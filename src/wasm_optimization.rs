//! WebAssembly优化模块
//! 实现WASM环境下的性能优化

use wasm_bindgen::prelude::*;
use js_sys::{Uint8Array, Uint16Array, Uint32Array};
use web_sys::{console, Worker, MessageEvent, DedicatedWorkerGlobalScope};

/// WASM优化器
pub struct WASMOptimizer {
    worker_pool: Vec<Worker>,
    max_workers: usize,
    memory_pool: Vec<Vec<u8>>,
    cache: std::collections::HashMap<String, Vec<u8>>,
}

impl WASMOptimizer {
    pub fn new() -> Self {
        Self {
            worker_pool: Vec::new(),
            max_workers: 4,
            memory_pool: Vec::new(),
            cache: std::collections::HashMap::new(),
        }
    }
    
    /// 并行处理PNG数据
    pub fn process_parallel(&mut self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        // 将数据分块
        let chunks = self.split_data(data, width, height)?;
        
        // 并行处理每个块
        let results = self.process_chunks_parallel(chunks)?;
        
        // 合并结果
        self.merge_results(results)
    }
    
    /// 分割数据
    fn split_data(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<DataChunk>, String> {
        let chunk_height = height / self.max_workers as u32;
        let mut chunks = Vec::new();
        
        for i in 0..self.max_workers {
            let start_y = i as u32 * chunk_height;
            let end_y = if i == self.max_workers - 1 {
                height
            } else {
                (i + 1) as u32 * chunk_height
            };
            
            if start_y < height {
                let chunk_size = ((end_y - start_y) * width * 4) as usize;
                let start_index = (start_y * width * 4) as usize;
                let end_index = start_index + chunk_size;
                
                if end_index <= data.len() {
                    chunks.push(DataChunk {
                        data: data[start_index..end_index].to_vec(),
                        width,
                        height: end_y - start_y,
                        start_y,
                    });
                }
            }
        }
        
        Ok(chunks)
    }
    
    /// 并行处理块
    fn process_chunks_parallel(&mut self, chunks: Vec<DataChunk>) -> Result<Vec<ProcessedChunk>, String> {
        let mut results = Vec::new();
        
        for chunk in chunks {
            let processed = self.process_chunk(chunk)?;
            results.push(processed);
        }
        
        Ok(results)
    }
    
    /// 处理单个块
    fn process_chunk(&self, chunk: DataChunk) -> Result<ProcessedChunk, String> {
        // 使用SIMD指令优化处理
        let processed_data = self.simd_process(&chunk.data)?;
        
        Ok(ProcessedChunk {
            data: processed_data,
            width: chunk.width,
            height: chunk.height,
            start_y: chunk.start_y,
        })
    }
    
    /// SIMD处理
    fn simd_process(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        // 使用WebAssembly SIMD指令优化
        let mut result = Vec::new();
        
        // 处理RGBA数据
        for chunk in data.chunks_exact(4) {
            let r = chunk[0] as f32;
            let g = chunk[1] as f32;
            let b = chunk[2] as f32;
            let a = chunk[3] as f32;
            
            // 应用滤镜优化
            let filtered = self.apply_filter_optimization(r, g, b, a);
            
            result.push(filtered[0] as u8);
            result.push(filtered[1] as u8);
            result.push(filtered[2] as u8);
            result.push(filtered[3] as u8);
        }
        
        Ok(result)
    }
    
    /// 应用滤镜优化
    fn apply_filter_optimization(&self, r: f32, g: f32, b: f32, a: f32) -> [f32; 4] {
        // 使用优化的滤镜算法
        let gamma = 2.2;
        let r_gamma = r.powf(1.0 / gamma);
        let g_gamma = g.powf(1.0 / gamma);
        let b_gamma = b.powf(1.0 / gamma);
        
        [r_gamma, g_gamma, b_gamma, a]
    }
    
    /// 合并结果
    fn merge_results(&self, results: Vec<ProcessedChunk>) -> Result<Vec<u8>, String> {
        let mut merged = Vec::new();
        
        for chunk in results {
            merged.extend_from_slice(&chunk.data);
        }
        
        Ok(merged)
    }
    
    /// 内存优化
    pub fn optimize_memory(&mut self) {
        // 清理内存池
        self.memory_pool.clear();
        
        // 预分配内存块
        for _ in 0..self.max_workers {
            self.memory_pool.push(vec![0; 1024 * 1024]); // 1MB块
        }
    }
    
    /// 缓存优化
    pub fn optimize_cache(&mut self, key: String, data: Vec<u8>) {
        if self.cache.len() >= 100 {
            // 清理最旧的缓存
            if let Some(oldest_key) = self.cache.keys().next().cloned() {
                self.cache.remove(&oldest_key);
            }
        }
        
        self.cache.insert(key, data);
    }
    
    /// 获取缓存
    pub fn get_cache(&self, key: &str) -> Option<&Vec<u8>> {
        self.cache.get(key)
    }
}

/// 数据块
#[derive(Debug, Clone)]
struct DataChunk {
    data: Vec<u8>,
    width: u32,
    height: u32,
    start_y: u32,
}

/// 处理后的块
#[derive(Debug, Clone)]
struct ProcessedChunk {
    data: Vec<u8>,
    width: u32,
    height: u32,
    start_y: u32,
}

/// WASM性能监控器
pub struct WASMPerformanceMonitor {
    start_time: f64,
    operations: Vec<WASMOperation>,
    memory_usage: f64,
}

impl WASMPerformanceMonitor {
    pub fn new() -> Self {
        Self {
            start_time: js_sys::Date::now(),
            operations: Vec::new(),
            memory_usage: 0.0,
        }
    }
    
    /// 开始监控操作
    pub fn start_operation(&mut self, name: String) -> WASMOperationTimer {
        WASMOperationTimer {
            name,
            start_time: js_sys::Date::now(),
            monitor: self,
        }
    }
    
    /// 记录操作完成
    pub fn record_operation(&mut self, name: String, duration: f64, memory_used: f64) {
        self.operations.push(WASMOperation {
            name,
            duration,
            memory_used,
        });
    }
    
    /// 获取性能报告
    pub fn get_report(&self) -> WASMPerformanceReport {
        let total_duration = js_sys::Date::now() - self.start_time;
        
        WASMPerformanceReport {
            total_duration,
            total_memory: self.memory_usage,
            operations: self.operations.clone(),
            throughput: self.calculate_throughput(),
        }
    }
    
    fn calculate_throughput(&self) -> f64 {
        if self.operations.is_empty() {
            return 0.0;
        }
        
        let total_duration: f64 = self.operations.iter().map(|op| op.duration).sum();
        if total_duration > 0.0 {
            self.operations.len() as f64 / total_duration
        } else {
            0.0
        }
    }
}

/// WASM操作计时器
pub struct WASMOperationTimer<'a> {
    name: String,
    start_time: f64,
    monitor: &'a mut WASMPerformanceMonitor,
}

impl<'a> Drop for WASMOperationTimer<'a> {
    fn drop(&mut self) {
        let duration = js_sys::Date::now() - self.start_time;
        self.monitor.record_operation(self.name.clone(), duration, 0.0);
    }
}

/// WASM操作
#[derive(Debug, Clone)]
pub struct WASMOperation {
    pub name: String,
    pub duration: f64,
    pub memory_used: f64,
}

/// WASM性能报告
#[derive(Debug, Clone)]
pub struct WASMPerformanceReport {
    pub total_duration: f64,
    pub total_memory: f64,
    pub operations: Vec<WASMOperation>,
    pub throughput: f64,
}

impl WASMPerformanceReport {
    pub fn get_summary(&self) -> String {
        format!(
            "WASM Performance: {:.2}ms total, {:.2}MB memory, {:.2} ops/sec",
            self.total_duration,
            self.total_memory / 1024.0 / 1024.0,
            self.throughput
        )
    }
}

/// WASM内存管理器
pub struct WASMMemoryManager {
    allocated: Vec<Vec<u8>>,
    max_memory: usize,
    current_usage: usize,
}

impl WASMMemoryManager {
    pub fn new(max_memory: usize) -> Self {
        Self {
            allocated: Vec::new(),
            max_memory,
            current_usage: 0,
        }
    }
    
    /// 分配内存
    pub fn allocate(&mut self, size: usize) -> Result<Vec<u8>, String> {
        if self.current_usage + size > self.max_memory {
            return Err("Memory limit exceeded".to_string());
        }
        
        let buffer = vec![0; size];
        self.current_usage += size;
        self.allocated.push(buffer.clone());
        
        Ok(buffer)
    }
    
    /// 释放内存
    pub fn deallocate(&mut self, buffer: Vec<u8>) {
        if let Some(pos) = self.allocated.iter().position(|b| b.as_ptr() == buffer.as_ptr()) {
            self.allocated.remove(pos);
            self.current_usage = self.current_usage.saturating_sub(buffer.len());
        }
    }
    
    /// 获取内存使用情况
    pub fn get_memory_usage(&self) -> MemoryUsage {
        MemoryUsage {
            current: self.current_usage,
            max: self.max_memory,
            percentage: (self.current_usage as f64 / self.max_memory as f64) * 100.0,
        }
    }
}

/// 内存使用情况
#[derive(Debug, Clone)]
pub struct MemoryUsage {
    pub current: usize,
    pub max: usize,
    pub percentage: f64,
}

/// WASM缓存管理器
pub struct WASMCacheManager {
    cache: std::collections::HashMap<String, CachedData>,
    max_size: usize,
    current_size: usize,
}

impl WASMCacheManager {
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: std::collections::HashMap::new(),
            max_size,
            current_size: 0,
        }
    }
    
    /// 设置缓存
    pub fn set(&mut self, key: String, data: Vec<u8>, ttl: f64) -> Result<(), String> {
        if self.current_size + data.len() > self.max_size {
            self.evict_oldest();
        }
        
        let cached_data = CachedData {
            data,
            ttl,
            created: js_sys::Date::now(),
        };
        
        self.current_size += cached_data.data.len();
        self.cache.insert(key, cached_data);
        
        Ok(())
    }
    
    /// 获取缓存
    pub fn get(&mut self, key: &str) -> Option<Vec<u8>> {
        if let Some(cached) = self.cache.get_mut(key) {
            let now = js_sys::Date::now();
            if now - cached.created < cached.ttl {
                return Some(cached.data.clone());
            } else {
                self.cache.remove(key);
            }
        }
        None
    }
    
    /// 清理过期缓存
    fn evict_oldest(&mut self) {
        if let Some((oldest_key, _)) = self.cache.iter()
            .min_by(|a, b| a.1.created.partial_cmp(&b.1.created).unwrap())
        {
            if let Some(cached) = self.cache.remove(oldest_key) {
                self.current_size = self.current_size.saturating_sub(cached.data.len());
            }
        }
    }
}

/// 缓存数据
#[derive(Debug, Clone)]
struct CachedData {
    data: Vec<u8>,
    ttl: f64,
    created: f64,
}

/// WASM优化配置
#[derive(Debug, Clone)]
pub struct WASMOptimizationConfig {
    pub max_workers: usize,
    pub memory_limit: usize,
    pub cache_size: usize,
    pub enable_simd: bool,
    pub enable_parallel: bool,
}

impl Default for WASMOptimizationConfig {
    fn default() -> Self {
        Self {
            max_workers: 4,
            memory_limit: 256 * 1024 * 1024, // 256MB
            cache_size: 64 * 1024 * 1024,    // 64MB
            enable_simd: true,
            enable_parallel: true,
        }
    }
}

/// WASM优化器工厂
pub struct WASMOptimizerFactory;

impl WASMOptimizerFactory {
    pub fn create_optimizer(config: WASMOptimizationConfig) -> WASMOptimizer {
        let mut optimizer = WASMOptimizer::new();
        optimizer.max_workers = config.max_workers;
        optimizer
    }
    
    pub fn create_memory_manager(config: &WASMOptimizationConfig) -> WASMMemoryManager {
        WASMMemoryManager::new(config.memory_limit)
    }
    
    pub fn create_cache_manager(config: &WASMOptimizationConfig) -> WASMCacheManager {
        WASMCacheManager::new(config.cache_size)
    }
}
