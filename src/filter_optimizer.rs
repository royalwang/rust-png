//! 滤镜性能优化模块
//! 提供并行处理、缓存优化和性能分析

use crate::filter_extensible::{Filter, FilterContext, FilterProcessor};
use std::sync::Arc;
use std::thread;
use std::sync::mpsc;

/// 并行滤镜处理器
pub struct ParallelFilterProcessor {
    processor: FilterProcessor,
    thread_count: usize,
}

impl ParallelFilterProcessor {
    pub fn new(thread_count: Option<usize>) -> Self {
        let thread_count = thread_count.unwrap_or_else(|| {
            thread::available_parallelism()
                .map(|n| n.get())
                .unwrap_or(4)
        });
        
        Self {
            processor: FilterProcessor::new(),
            thread_count,
        }
    }
    
    /// 并行应用滤镜
    pub fn apply_filter_parallel(&self, filter_type: u8, data: &mut [u8], context: &FilterContext) -> Result<(), String> {
        if let Some(filter) = self.processor.registry.get_filter(filter_type) {
            if filter.supports_parallel() {
                self.apply_parallel(data, context, |row_data, row_context| {
                    filter.apply(row_data, row_context)
                })
            } else {
                // 回退到串行处理
                filter.apply(data, context)
            }
        } else {
            Err(format!("Filter type {} not found", filter_type))
        }
    }
    
    /// 并行反向应用滤镜
    pub fn reverse_filter_parallel(&self, filter_type: u8, data: &mut [u8], context: &FilterContext) -> Result<(), String> {
        if let Some(filter) = self.processor.registry.get_filter(filter_type) {
            if filter.supports_parallel() {
                self.apply_parallel(data, context, |row_data, row_context| {
                    filter.reverse(row_data, row_context)
                })
            } else {
                filter.reverse(data, context)
            }
        } else {
            Err(format!("Filter type {} not found", filter_type))
        }
    }
    
    fn apply_parallel<F>(&self, data: &mut [u8], context: &FilterContext, filter_func: F) -> Result<(), String>
    where
        F: Fn(&mut [u8], &FilterContext) -> Result<(), String> + Send + Sync + 'static,
    {
        let bytes_per_row = context.width * context.bytes_per_pixel;
        let row_count = context.height;
        
        if row_count == 0 {
            return Ok(());
        }
        
        // 计算每个线程处理的行数
        let rows_per_thread = (row_count + self.thread_count - 1) / self.thread_count;
        
        // 创建通道用于线程间通信
        let (tx, rx) = mpsc::channel();
        
        // 启动工作线程
        let mut handles = Vec::new();
        
        for thread_id in 0..self.thread_count {
            let start_row = thread_id * rows_per_thread;
            let end_row = ((thread_id + 1) * rows_per_thread).min(row_count);
            
            if start_row >= end_row {
                break;
            }
            
            let tx = tx.clone();
            let filter_func = Arc::new(filter_func);
            let data_ptr = data.as_mut_ptr();
            let data_len = data.len();
            
            let handle = thread::spawn(move || {
                let mut local_data = unsafe {
                    std::slice::from_raw_parts_mut(data_ptr, data_len)
                };
                
                for row in start_row..end_row {
                    let row_start = row * bytes_per_row;
                    let row_end = row_start + bytes_per_row;
                    
                    if row_end <= local_data.len() {
                        let row_data = &mut local_data[row_start..row_end];
                        let row_context = FilterContext {
                            width: context.width,
                            height: context.height,
                            bytes_per_pixel: context.bytes_per_pixel,
                            row_index: row,
                            column_index: 0,
                            previous_row: if row > 0 {
                                Some(local_data[(row - 1) * bytes_per_row..row * bytes_per_row].to_vec())
                            } else {
                                None
                            },
                        };
                        
                        if let Err(e) = filter_func(row_data, &row_context) {
                            let _ = tx.send(Err(e));
                            return;
                        }
                    }
                }
                
                let _ = tx.send(Ok(()));
            });
            
            handles.push(handle);
        }
        
        // 等待所有线程完成
        for handle in handles {
            handle.join().unwrap();
        }
        
        // 检查是否有错误
        drop(tx);
        while let Ok(result) = rx.recv() {
            if let Err(e) = result {
                return Err(e);
            }
        }
        
        Ok(())
    }
}

/// 滤镜性能分析器
pub struct FilterProfiler {
    measurements: Vec<FilterMeasurement>,
}

#[derive(Debug, Clone)]
struct FilterMeasurement {
    filter_type: u8,
    filter_name: String,
    processing_time: u64, // 微秒
    compression_ratio: f64,
    memory_usage: usize,
    cache_hits: u64,
    cache_misses: u64,
}

impl FilterProfiler {
    pub fn new() -> Self {
        Self {
            measurements: Vec::new(),
        }
    }
    
    /// 分析滤镜性能
    pub fn profile_filter(&mut self, filter_type: u8, data: &[u8], context: &FilterContext) -> FilterMeasurement {
        let start_time = std::time::Instant::now();
        let start_memory = self.get_memory_usage();
        
        // 创建滤镜处理器
        let processor = FilterProcessor::new();
        let mut test_data = data.to_vec();
        
        // 应用滤镜
        let result = processor.apply_filter(filter_type, &mut test_data, context);
        
        let processing_time = start_time.elapsed().as_micros() as u64;
        let end_memory = self.get_memory_usage();
        let memory_usage = end_memory.saturating_sub(start_memory);
        
        // 计算压缩比
        let compression_ratio = if let Some(filter) = processor.registry.get_filter(filter_type) {
            filter.calculate_compression_ratio(data, context)
        } else {
            1.0
        };
        
        let measurement = FilterMeasurement {
            filter_type,
            filter_name: processor.get_filter_info(filter_type).unwrap_or_else(|| "Unknown".to_string()),
            processing_time,
            compression_ratio,
            memory_usage,
            cache_hits: 0, // 简化实现
            cache_misses: 0,
        };
        
        self.measurements.push(measurement.clone());
        measurement
    }
    
    /// 获取性能报告
    pub fn get_performance_report(&self) -> PerformanceReport {
        if self.measurements.is_empty() {
            return PerformanceReport::default();
        }
        
        let total_time: u64 = self.measurements.iter().map(|m| m.processing_time).sum();
        let avg_time = total_time / self.measurements.len() as u64;
        
        let best_compression = self.measurements.iter()
            .max_by(|a, b| a.compression_ratio.partial_cmp(&b.compression_ratio).unwrap())
            .cloned();
        
        let fastest_filter = self.measurements.iter()
            .min_by(|a, b| a.processing_time.cmp(&b.processing_time))
            .cloned();
        
        PerformanceReport {
            total_measurements: self.measurements.len(),
            average_processing_time: avg_time,
            best_compression_filter: best_compression,
            fastest_filter,
            total_memory_usage: self.measurements.iter().map(|m| m.memory_usage).sum(),
        }
    }
    
    fn get_memory_usage(&self) -> usize {
        // 简化的内存使用量计算
        std::process::id() as usize * 1024 // 占位符实现
    }
}

#[derive(Debug, Clone, Default)]
pub struct PerformanceReport {
    pub total_measurements: usize,
    pub average_processing_time: u64,
    pub best_compression_filter: Option<FilterMeasurement>,
    pub fastest_filter: Option<FilterMeasurement>,
    pub total_memory_usage: usize,
}

/// 滤镜缓存优化器
pub struct FilterCache {
    cache: std::collections::HashMap<String, CachedFilterResult>,
    max_cache_size: usize,
}

#[derive(Debug, Clone)]
struct CachedFilterResult {
    filtered_data: Vec<u8>,
    compression_ratio: f64,
    timestamp: std::time::SystemTime,
}

impl FilterCache {
    pub fn new(max_cache_size: usize) -> Self {
        Self {
            cache: std::collections::HashMap::new(),
            max_cache_size,
        }
    }
    
    /// 获取缓存的滤镜结果
    pub fn get_cached_result(&self, key: &str) -> Option<&CachedFilterResult> {
        self.cache.get(key)
    }
    
    /// 缓存滤镜结果
    pub fn cache_result(&mut self, key: String, result: CachedFilterResult) {
        if self.cache.len() >= self.max_cache_size {
            // 简单的LRU策略：删除最旧的条目
            if let Some(oldest_key) = self.cache.keys().next().cloned() {
                self.cache.remove(&oldest_key);
            }
        }
        
        self.cache.insert(key, result);
    }
    
    /// 生成缓存键
    pub fn generate_cache_key(&self, filter_type: u8, data_hash: u64, context: &FilterContext) -> String {
        format!("filter_{}_{}_{}x{}", filter_type, data_hash, context.width, context.height)
    }
    
    /// 清理过期缓存
    pub fn cleanup_expired(&mut self, max_age: std::time::Duration) {
        let now = std::time::SystemTime::now();
        self.cache.retain(|_, result| {
            now.duration_since(result.timestamp).unwrap_or_default() < max_age
        });
    }
}

/// 智能滤镜选择器
pub struct SmartFilterSelector {
    profiler: FilterProfiler,
    cache: FilterCache,
    learning_data: Vec<FilterSelection>,
}

#[derive(Debug, Clone)]
struct FilterSelection {
    image_characteristics: ImageCharacteristics,
    best_filter: u8,
    performance_score: f64,
}

#[derive(Debug, Clone)]
struct ImageCharacteristics {
    width: usize,
    height: usize,
    bytes_per_pixel: usize,
    entropy: f64,
    edge_density: f64,
    color_variance: f64,
}

impl SmartFilterSelector {
    pub fn new() -> Self {
        Self {
            profiler: FilterProfiler::new(),
            cache: FilterCache::new(1000),
            learning_data: Vec::new(),
        }
    }
    
    /// 智能选择最佳滤镜
    pub fn select_best_filter(&mut self, data: &[u8], context: &FilterContext) -> u8 {
        // 分析图像特征
        let characteristics = self.analyze_image_characteristics(data, context);
        
        // 查找相似的历史选择
        if let Some(best_filter) = self.find_similar_selection(&characteristics) {
            return best_filter;
        }
        
        // 如果没有历史数据，使用性能分析选择
        self.select_by_performance(data, context)
    }
    
    fn analyze_image_characteristics(&self, data: &[u8], context: &FilterContext) -> ImageCharacteristics {
        let entropy = self.calculate_entropy(data);
        let edge_density = self.calculate_edge_density(data, context);
        let color_variance = self.calculate_color_variance(data, context);
        
        ImageCharacteristics {
            width: context.width,
            height: context.height,
            bytes_per_pixel: context.bytes_per_pixel,
            entropy,
            edge_density,
            color_variance,
        }
    }
    
    fn calculate_entropy(&self, data: &[u8]) -> f64 {
        let mut histogram = [0u32; 256];
        for &byte in data {
            histogram[byte as usize] += 1;
        }
        
        let total = data.len() as f64;
        let mut entropy = 0.0;
        
        for &count in &histogram {
            if count > 0 {
                let probability = count as f64 / total;
                entropy -= probability * probability.log2();
            }
        }
        
        entropy
    }
    
    fn calculate_edge_density(&self, data: &[u8], context: &FilterContext) -> f64 {
        let mut edge_count = 0;
        let bytes_per_row = context.width * context.bytes_per_pixel;
        
        for y in 1..context.height {
            for x in 1..context.width {
                let idx = (y * context.width + x) * context.bytes_per_pixel;
                if idx + context.bytes_per_pixel <= data.len() {
                    let current = data[idx];
                    let left = data[idx - context.bytes_per_pixel];
                    let up = data[idx - bytes_per_row];
                    
                    let edge_strength = ((current as i16 - left as i16).abs() + 
                                        (current as i16 - up as i16).abs()) as u8;
                    
                    if edge_strength > 10 { // 阈值
                        edge_count += 1;
                    }
                }
            }
        }
        
        let total_pixels = (context.width - 1) * (context.height - 1);
        if total_pixels > 0 {
            edge_count as f64 / total_pixels as f64
        } else {
            0.0
        }
    }
    
    fn calculate_color_variance(&self, data: &[u8], context: &FilterContext) -> f64 {
        let mut sum = 0.0;
        let mut sum_squares = 0.0;
        let count = data.len() as f64;
        
        for &byte in data {
            let value = byte as f64;
            sum += value;
            sum_squares += value * value;
        }
        
        let mean = sum / count;
        let variance = (sum_squares / count) - (mean * mean);
        variance.sqrt()
    }
    
    fn find_similar_selection(&self, characteristics: &ImageCharacteristics) -> Option<u8> {
        // 简化的相似性匹配
        for selection in &self.learning_data {
            if self.is_similar(characteristics, &selection.image_characteristics) {
                return Some(selection.best_filter);
            }
        }
        None
    }
    
    fn is_similar(&self, a: &ImageCharacteristics, b: &ImageCharacteristics) -> bool {
        let size_diff = (a.width as f64 - b.width as f64).abs() / a.width as f64;
        let entropy_diff = (a.entropy - b.entropy).abs();
        let edge_diff = (a.edge_density - b.edge_density).abs();
        
        size_diff < 0.1 && entropy_diff < 0.5 && edge_diff < 0.1
    }
    
    fn select_by_performance(&mut self, data: &[u8], context: &FilterContext) -> u8 {
        // 使用性能分析选择最佳滤镜
        let processor = FilterProcessor::new();
        processor.choose_best_filter(data, context).unwrap_or(0)
    }
}
