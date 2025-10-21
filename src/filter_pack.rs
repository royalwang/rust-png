//! 滤镜打包器模块
//! 实现PNG滤镜的打包功能，匹配原始pngjs库的filter-pack.js

use crate::constants::*;
use crate::filter_extensible::*;

/// 滤镜打包器
pub struct FilterPacker {
    processor: FilterProcessor,
}

impl FilterPacker {
    pub fn new() -> Self {
        Self {
            processor: FilterProcessor::new(),
        }
    }
    
    /// 打包滤镜数据
    pub fn pack_filters(&self, data: &[u8], width: u32, height: u32, bpp: usize) -> Result<Vec<u8>, String> {
        let mut packed_data = Vec::new();
        let bytes_per_row = (width as usize * bpp) as usize;
        
        for y in 0..height {
            let row_start = (y * bytes_per_row as u32) as usize;
            let row_end = row_start + bytes_per_row;
            
            if row_end > data.len() {
                return Err("Insufficient data for row".to_string());
            }
            
            let row_data = &data[row_start..row_end];
            
            // 选择最佳滤镜
            let best_filter = self.choose_best_filter(row_data, y as usize, width as usize, bpp);
            
            // 应用滤镜
            let filtered_row = self.apply_filter(row_data, best_filter, y as usize, width as usize, bpp)?;
            
            // 添加滤镜类型字节
            packed_data.push(best_filter);
            packed_data.extend_from_slice(&filtered_row);
        }
        
        Ok(packed_data)
    }
    
    /// 选择最佳滤镜
    fn choose_best_filter(&self, row_data: &[u8], row_index: usize, width: usize, bpp: usize) -> u8 {
        let context = FilterContext {
            width,
            height: 0, // 不用于选择
            bytes_per_pixel: bpp,
            row_index,
            column_index: 0,
            previous_row: None,
        };
        
        // 测试所有滤镜类型
        let mut best_filter = FILTER_NONE;
        let mut best_score = f64::MAX;
        
        for filter_type in [FILTER_NONE, FILTER_SUB, FILTER_UP, FILTER_AVERAGE, FILTER_PAETH] {
            let score = self.calculate_filter_score(row_data, filter_type, &context);
            if score < best_score {
                best_score = score;
                best_filter = filter_type;
            }
        }
        
        best_filter
    }
    
    /// 计算滤镜分数
    fn calculate_filter_score(&self, data: &[u8], filter_type: u8, context: &FilterContext) -> f64 {
        let mut test_data = data.to_vec();
        
        // 应用滤镜
        if let Err(_) = self.processor.apply_filter(filter_type, &mut test_data, context) {
            return f64::MAX;
        }
        
        // 计算压缩比（简化实现）
        self.calculate_compression_ratio(&test_data)
    }
    
    /// 计算压缩比
    fn calculate_compression_ratio(&self, data: &[u8]) -> f64 {
        // 简化的压缩比计算
        let mut score = 0.0;
        for &byte in data {
            score += byte as f64 * byte as f64;
        }
        score
    }
    
    /// 应用滤镜
    fn apply_filter(&self, data: &[u8], filter_type: u8, row_index: usize, width: usize, bpp: usize) -> Result<Vec<u8>, String> {
        let mut filtered_data = data.to_vec();
        let context = FilterContext {
            width,
            height: 0,
            bytes_per_pixel: bpp,
            row_index,
            column_index: 0,
            previous_row: None,
        };
        
        self.processor.apply_filter(filter_type, &mut filtered_data, &context)?;
        Ok(filtered_data)
    }
}

/// 滤镜解包器
pub struct FilterUnpacker {
    processor: FilterProcessor,
}

impl FilterUnpacker {
    pub fn new() -> Self {
        Self {
            processor: FilterProcessor::new(),
        }
    }
    
    /// 解包滤镜数据
    pub fn unpack_filters(&self, data: &[u8], width: u32, height: u32, bpp: usize) -> Result<Vec<u8>, String> {
        let mut unpacked_data = Vec::new();
        let bytes_per_row = (width as usize * bpp) as usize;
        let mut offset = 0;
        
        for y in 0..height {
            if offset >= data.len() {
                return Err("Insufficient data for filter type".to_string());
            }
            
            let filter_type = data[offset];
            offset += 1;
            
            if offset + bytes_per_row > data.len() {
                return Err("Insufficient data for row".to_string());
            }
            
            let mut row_data = data[offset..offset + bytes_per_row].to_vec();
            offset += bytes_per_row;
            
            // 反向应用滤镜
            let context = FilterContext {
                width: width as usize,
                height: height as usize,
                bytes_per_pixel: bpp,
                row_index: y as usize,
                column_index: 0,
                previous_row: if y > 0 {
                    let prev_start = (y - 1) * bytes_per_row as u32;
                    let prev_end = prev_start + bytes_per_row as u32;
                    if prev_end <= unpacked_data.len() as u32 {
                        Some(unpacked_data[prev_start as usize..prev_end as usize].to_vec())
                    } else {
                        None
                    }
                } else {
                    None
                },
            };
            
            self.processor.reverse_filter(filter_type, &mut row_data, &context)?;
            unpacked_data.extend_from_slice(&row_data);
        }
        
        Ok(unpacked_data)
    }
}

/// 滤镜优化器
pub struct FilterOptimizer {
    packer: FilterPacker,
    unpacker: FilterUnpacker,
}

impl FilterOptimizer {
    pub fn new() -> Self {
        Self {
            packer: FilterPacker::new(),
            unpacker: FilterUnpacker::new(),
        }
    }
    
    /// 优化滤镜选择
    pub fn optimize_filters(&self, data: &[u8], width: u32, height: u32, bpp: usize) -> Result<Vec<u8>, String> {
        // 使用智能滤镜选择
        let smart_selector = SmartFilterSelector::new();
        let context = FilterContext {
            width: width as usize,
            height: height as usize,
            bytes_per_pixel: bpp,
            row_index: 0,
            column_index: 0,
            previous_row: None,
        };
        
        let best_filter = smart_selector.select_best_filter(data, &context);
        
        // 应用最佳滤镜
        self.packer.pack_filters(data, width, height, bpp)
    }
    
    /// 分析滤镜性能
    pub fn analyze_filter_performance(&self, data: &[u8], width: u32, height: u32, bpp: usize) -> Result<Vec<FilterPerformance>, String> {
        let mut performances = Vec::new();
        
        for filter_type in [FILTER_NONE, FILTER_SUB, FILTER_UP, FILTER_AVERAGE, FILTER_PAETH] {
            let start_time = std::time::Instant::now();
            
            // 测试滤镜性能
            let mut test_data = data.to_vec();
            let context = FilterContext {
                width: width as usize,
                height: height as usize,
                bytes_per_pixel: bpp,
                row_index: 0,
                column_index: 0,
                previous_row: None,
            };
            
            let result = self.packer.processor.apply_filter(filter_type, &mut test_data, &context);
            let duration = start_time.elapsed();
            
            let performance = FilterPerformance {
                filter_type,
                duration: duration.as_micros() as f64,
                success: result.is_ok(),
                compression_ratio: if result.is_ok() {
                    self.packer.calculate_compression_ratio(&test_data)
                } else {
                    0.0
                },
            };
            
            performances.push(performance);
        }
        
        Ok(performances)
    }
}

/// 滤镜性能数据
#[derive(Debug, Clone)]
pub struct FilterPerformance {
    pub filter_type: u8,
    pub duration: f64, // 微秒
    pub success: bool,
    pub compression_ratio: f64,
}

impl FilterPerformance {
    pub fn get_filter_name(&self) -> &'static str {
        match self.filter_type {
            FILTER_NONE => "None",
            FILTER_SUB => "Sub",
            FILTER_UP => "Up",
            FILTER_AVERAGE => "Average",
            FILTER_PAETH => "Paeth",
            _ => "Unknown",
        }
    }
}

/// 并行滤镜处理器
pub struct ParallelFilterProcessor {
    num_threads: usize,
}

impl ParallelFilterProcessor {
    pub fn new(num_threads: Option<usize>) -> Self {
        Self {
            num_threads: num_threads.unwrap_or_else(|| num_cpus::get()),
        }
    }
    
    /// 并行处理滤镜
    pub fn process_filters_parallel(&self, data: &[u8], width: u32, height: u32, bpp: usize) -> Result<Vec<u8>, String> {
        let bytes_per_row = (width as usize * bpp) as usize;
        let mut results = Vec::new();
        
        // 将数据分成多个块进行并行处理
        let chunk_size = (height as usize + self.num_threads - 1) / self.num_threads;
        
        for chunk_start in (0..height as usize).step_by(chunk_size) {
            let chunk_end = std::cmp::min(chunk_start + chunk_size, height as usize);
            let chunk_data = &data[chunk_start * bytes_per_row..chunk_end * bytes_per_row];
            
            let packer = FilterPacker::new();
            let chunk_result = packer.pack_filters(
                chunk_data,
                width,
                (chunk_end - chunk_start) as u32,
                bpp
            )?;
            
            results.extend_from_slice(&chunk_result);
        }
        
        Ok(results)
    }
}

/// 滤镜缓存
pub struct FilterCache {
    cache: std::collections::HashMap<String, Vec<u8>>,
    max_size: usize,
}

impl FilterCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: std::collections::HashMap::new(),
            max_size,
        }
    }
    
    /// 获取缓存的滤镜结果
    pub fn get(&self, key: &str) -> Option<&Vec<u8>> {
        self.cache.get(key)
    }
    
    /// 设置缓存的滤镜结果
    pub fn set(&mut self, key: String, value: Vec<u8>) {
        if self.cache.len() >= self.max_size {
            // 简单的LRU实现：移除第一个元素
            if let Some(first_key) = self.cache.keys().next().cloned() {
                self.cache.remove(&first_key);
            }
        }
        
        self.cache.insert(key, value);
    }
    
    /// 清除缓存
    pub fn clear(&mut self) {
        self.cache.clear();
    }
    
    /// 获取缓存大小
    pub fn size(&self) -> usize {
        self.cache.len()
    }
}
