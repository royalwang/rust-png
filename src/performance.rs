//! 性能优化模块
//! 实现PNG处理的性能优化功能

use std::sync::Arc;
use std::thread;
use std::sync::mpsc;
use rayon::prelude::*;

/// 性能优化器
pub struct PerformanceOptimizer {
    num_threads: usize,
    chunk_size: usize,
    memory_limit: usize,
}

impl PerformanceOptimizer {
    pub fn new() -> Self {
        Self {
            num_threads: num_cpus::get(),
            chunk_size: 64 * 1024, // 64KB chunks
            memory_limit: 256 * 1024 * 1024, // 256MB
        }
    }
    
    pub fn with_threads(mut self, num_threads: usize) -> Self {
        self.num_threads = num_threads;
        self
    }
    
    pub fn with_chunk_size(mut self, chunk_size: usize) -> Self {
        self.chunk_size = chunk_size;
        self
    }
    
    pub fn with_memory_limit(mut self, memory_limit: usize) -> Self {
        self.memory_limit = memory_limit;
        self
    }
    
    /// 并行处理数据
    pub fn process_parallel<F, T>(&self, data: &[u8], processor: F) -> Result<Vec<T>, String>
    where
        F: Fn(&[u8]) -> Result<T, String> + Send + Sync,
        T: Send,
    {
        let chunks: Vec<&[u8]> = data.chunks(self.chunk_size).collect();
        
        chunks.par_iter()
            .map(|chunk| processor(chunk))
            .collect::<Result<Vec<T>, String>>()
    }
    
    /// 并行处理行数据
    pub fn process_rows_parallel<F, T>(&self, data: &[u8], width: u32, height: u32, processor: F) -> Result<Vec<T>, String>
    where
        F: Fn(&[u8], u32, u32) -> Result<T, String> + Send + Sync,
        T: Send,
    {
        let bytes_per_row = width as usize;
        let rows: Vec<&[u8]> = data.chunks(bytes_per_row).collect();
        
        rows.par_iter()
            .enumerate()
            .map(|(y, row)| processor(row, width, y as u32))
            .collect::<Result<Vec<T>, String>>()
    }
    
    /// 内存映射处理
    pub fn process_memory_mapped<F, T>(&self, data: &[u8], processor: F) -> Result<Vec<T>, String>
    where
        F: Fn(&[u8]) -> Result<T, String> + Send + Sync,
        T: Send,
    {
        if data.len() > self.memory_limit {
            return Err("Data size exceeds memory limit".to_string());
        }
        
        self.process_parallel(data, processor)
    }
}

/// 缓存优化器
pub struct CacheOptimizer {
    cache_size: usize,
    cache: std::collections::HashMap<String, Vec<u8>>,
    access_count: std::collections::HashMap<String, usize>,
}

impl CacheOptimizer {
    pub fn new(cache_size: usize) -> Self {
        Self {
            cache_size,
            cache: std::collections::HashMap::new(),
            access_count: std::collections::HashMap::new(),
        }
    }
    
    /// 获取缓存数据
    pub fn get(&mut self, key: &str) -> Option<&Vec<u8>> {
        if let Some(data) = self.cache.get(key) {
            *self.access_count.entry(key.to_string()).or_insert(0) += 1;
            Some(data)
        } else {
            None
        }
    }
    
    /// 设置缓存数据
    pub fn set(&mut self, key: String, data: Vec<u8>) {
        if self.cache.len() >= self.cache_size {
            self.evict_least_used();
        }
        
        self.cache.insert(key.clone(), data);
        self.access_count.insert(key, 1);
    }
    
    /// 清除最少使用的缓存
    fn evict_least_used(&mut self) {
        if let Some((key, _)) = self.access_count.iter()
            .min_by_key(|(_, &count)| count)
            .map(|(k, _)| (k.clone(), *k))
        {
            self.cache.remove(&key);
            self.access_count.remove(&key);
        }
    }
    
    /// 清除所有缓存
    pub fn clear(&mut self) {
        self.cache.clear();
        self.access_count.clear();
    }
    
    /// 获取缓存统计
    pub fn get_stats(&self) -> CacheStats {
        CacheStats {
            cache_size: self.cache.len(),
            total_access: self.access_count.values().sum(),
            hit_rate: self.calculate_hit_rate(),
        }
    }
    
    fn calculate_hit_rate(&self) -> f64 {
        let total_access: usize = self.access_count.values().sum();
        if total_access == 0 {
            0.0
        } else {
            let hits = self.cache.len();
            hits as f64 / total_access as f64
        }
    }
}

/// 缓存统计信息
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub cache_size: usize,
    pub total_access: usize,
    pub hit_rate: f64,
}

/// 内存池
pub struct MemoryPool {
    pool_size: usize,
    available: Vec<Vec<u8>>,
    in_use: std::collections::HashSet<*mut Vec<u8>>,
}

impl MemoryPool {
    pub fn new(pool_size: usize) -> Self {
        Self {
            pool_size,
            available: Vec::new(),
            in_use: std::collections::HashSet::new(),
        }
    }
    
    /// 获取内存块
    pub fn acquire(&mut self, size: usize) -> Option<Vec<u8>> {
        if let Some(mut buffer) = self.available.pop() {
            buffer.resize(size, 0);
            self.in_use.insert(buffer.as_mut() as *mut Vec<u8>);
            Some(buffer)
        } else if self.available.len() + self.in_use.len() < self.pool_size {
            let mut buffer = vec![0; size];
            self.in_use.insert(buffer.as_mut() as *mut Vec<u8>);
            Some(buffer)
        } else {
            None
        }
    }
    
    /// 释放内存块
    pub fn release(&mut self, buffer: Vec<u8>) {
        let ptr = buffer.as_ptr() as *mut Vec<u8>;
        if self.in_use.remove(&ptr) {
            self.available.push(buffer);
        }
    }
    
    /// 获取池统计
    pub fn get_stats(&self) -> MemoryPoolStats {
        MemoryPoolStats {
            pool_size: self.pool_size,
            available: self.available.len(),
            in_use: self.in_use.len(),
        }
    }
}

/// 内存池统计信息
#[derive(Debug, Clone)]
pub struct MemoryPoolStats {
    pub pool_size: usize,
    pub available: usize,
    pub in_use: usize,
}

/// 性能监控器
pub struct PerformanceMonitor {
    start_time: std::time::Instant,
    operations: Vec<OperationMetrics>,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
            operations: Vec::new(),
        }
    }
    
    /// 开始监控操作
    pub fn start_operation(&mut self, name: String) -> OperationTimer {
        OperationTimer {
            name,
            start_time: std::time::Instant::now(),
            monitor: self,
        }
    }
    
    /// 记录操作完成
    pub fn record_operation(&mut self, name: String, duration: std::time::Duration, memory_used: usize) {
        self.operations.push(OperationMetrics {
            name,
            duration,
            memory_used,
        });
    }
    
    /// 获取性能报告
    pub fn get_report(&self) -> PerformanceReport {
        let total_duration = self.start_time.elapsed();
        let total_memory: usize = self.operations.iter().map(|op| op.memory_used).sum();
        
        PerformanceReport {
            total_duration,
            total_memory,
            operations: self.operations.clone(),
            throughput: self.calculate_throughput(),
        }
    }
    
    fn calculate_throughput(&self) -> f64 {
        let total_duration = self.start_time.elapsed();
        if total_duration.as_secs_f64() > 0.0 {
            self.operations.len() as f64 / total_duration.as_secs_f64()
        } else {
            0.0
        }
    }
}

/// 操作计时器
pub struct OperationTimer<'a> {
    name: String,
    start_time: std::time::Instant,
    monitor: &'a mut PerformanceMonitor,
}

impl<'a> Drop for OperationTimer<'a> {
    fn drop(&mut self) {
        let duration = self.start_time.elapsed();
        self.monitor.record_operation(self.name.clone(), duration, 0);
    }
}

/// 操作指标
#[derive(Debug, Clone)]
pub struct OperationMetrics {
    pub name: String,
    pub duration: std::time::Duration,
    pub memory_used: usize,
}

/// 性能报告
#[derive(Debug, Clone)]
pub struct PerformanceReport {
    pub total_duration: std::time::Duration,
    pub total_memory: usize,
    pub operations: Vec<OperationMetrics>,
    pub throughput: f64,
}

impl PerformanceReport {
    pub fn get_operation_summary(&self) -> String {
        let mut summary = String::new();
        summary.push_str(&format!("Total Duration: {:?}\n", self.total_duration));
        summary.push_str(&format!("Total Memory: {} bytes\n", self.total_memory));
        summary.push_str(&format!("Throughput: {:.2} ops/sec\n", self.throughput));
        summary.push_str("Operations:\n");
        
        for op in &self.operations {
            summary.push_str(&format!("  {}: {:?} ({} bytes)\n", op.name, op.duration, op.memory_used));
        }
        
        summary
    }
}

/// 智能内存管理器
pub struct SmartMemoryManager {
    pool: MemoryPool,
    cache: CacheOptimizer,
    monitor: PerformanceMonitor,
}

impl SmartMemoryManager {
    pub fn new() -> Self {
        Self {
            pool: MemoryPool::new(100),
            cache: CacheOptimizer::new(50),
            monitor: PerformanceMonitor::new(),
        }
    }
    
    /// 智能分配内存
    pub fn smart_allocate(&mut self, size: usize, key: Option<String>) -> Result<Vec<u8>, String> {
        let _timer = self.monitor.start_operation("smart_allocate".to_string());
        
        // 首先尝试从缓存获取
        if let Some(key) = key {
            if let Some(cached) = self.cache.get(&key) {
                return Ok(cached.clone());
            }
        }
        
        // 尝试从内存池获取
        if let Some(mut buffer) = self.pool.acquire(size) {
            if let Some(key) = key {
                self.cache.set(key, buffer.clone());
            }
            return Ok(buffer);
        }
        
        // 分配新内存
        let buffer = vec![0; size];
        if let Some(key) = key {
            self.cache.set(key, buffer.clone());
        }
        Ok(buffer)
    }
    
    /// 智能释放内存
    pub fn smart_release(&mut self, buffer: Vec<u8>, key: Option<String>) {
        let _timer = self.monitor.start_operation("smart_release".to_string());
        
        // 如果有关联的key，更新缓存
        if let Some(key) = key {
            self.cache.set(key, buffer.clone());
        }
        
        // 释放到内存池
        self.pool.release(buffer);
    }
    
    /// 获取内存统计
    pub fn get_memory_stats(&self) -> MemoryStats {
        MemoryStats {
            pool_stats: self.pool.get_stats(),
            cache_stats: self.cache.get_stats(),
            performance_report: self.monitor.get_report(),
        }
    }
}

/// 内存统计信息
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub pool_stats: MemoryPoolStats,
    pub cache_stats: CacheStats,
    pub performance_report: PerformanceReport,
}
