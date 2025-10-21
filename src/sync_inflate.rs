//! 同步解压缩器模块
//! 实现PNG数据的同步解压缩，匹配原始pngjs库的sync-inflate.js

use flate2::read::DeflateDecoder;
use std::io::Read;

/// 同步解压缩器
pub struct SyncInflate {
    buffer: Vec<u8>,
}

impl SyncInflate {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
        }
    }
    
    /// 解压缩数据
    pub fn inflate(&mut self, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut decoder = DeflateDecoder::new(data);
        let mut decompressed = Vec::new();
        
        decoder.read_to_end(&mut decompressed)
            .map_err(|e| format!("Decompression error: {}", e))?;
        
        Ok(decompressed)
    }
    
    /// 解压缩数据到缓冲区
    pub fn inflate_to_buffer(&mut self, data: &[u8]) -> Result<(), String> {
        let decompressed = self.inflate(data)?;
        self.buffer.extend_from_slice(&decompressed);
        Ok(())
    }
    
    /// 获取缓冲区数据
    pub fn get_buffer(&self) -> &[u8] {
        &self.buffer
    }
    
    /// 清空缓冲区
    pub fn clear_buffer(&mut self) {
        self.buffer.clear();
    }
    
    /// 获取缓冲区大小
    pub fn buffer_size(&self) -> usize {
        self.buffer.len()
    }
}

/// 流式解压缩器
pub struct StreamingInflate {
    decoder: DeflateDecoder<&[u8]>,
    buffer: Vec<u8>,
    finished: bool,
}

impl StreamingInflate {
    pub fn new(data: &[u8]) -> Self {
        Self {
            decoder: DeflateDecoder::new(data),
            buffer: Vec::new(),
            finished: false,
        }
    }
    
    /// 读取解压缩数据
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, String> {
        if self.finished {
            return Ok(0);
        }
        
        let bytes_read = self.decoder.read(buf)
            .map_err(|e| format!("Read error: {}", e))?;
        
        if bytes_read == 0 {
            self.finished = true;
        }
        
        Ok(bytes_read)
    }
    
    /// 读取所有数据
    pub fn read_all(&mut self) -> Result<Vec<u8>, String> {
        let mut result = Vec::new();
        let mut buffer = [0; 4096];
        
        loop {
            let bytes_read = self.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            result.extend_from_slice(&buffer[..bytes_read]);
        }
        
        Ok(result)
    }
    
    /// 检查是否完成
    pub fn is_finished(&self) -> bool {
        self.finished
    }
}

/// 分块解压缩器
pub struct ChunkedInflate {
    chunks: Vec<Vec<u8>>,
    current_chunk: usize,
    decoder: Option<StreamingInflate>,
}

impl ChunkedInflate {
    pub fn new() -> Self {
        Self {
            chunks: Vec::new(),
            current_chunk: 0,
            decoder: None,
        }
    }
    
    /// 添加数据块
    pub fn add_chunk(&mut self, data: &[u8]) {
        self.chunks.push(data.to_vec());
    }
    
    /// 解压缩所有块
    pub fn inflate_all(&mut self) -> Result<Vec<u8>, String> {
        let mut result = Vec::new();
        
        for chunk in &self.chunks {
            let mut inflate = SyncInflate::new();
            let decompressed = inflate.inflate(chunk)?;
            result.extend_from_slice(&decompressed);
        }
        
        Ok(result)
    }
    
    /// 流式解压缩
    pub fn inflate_stream(&mut self, buf: &mut [u8]) -> Result<usize, String> {
        // 如果当前解码器已完成，移动到下一个块
        if let Some(ref mut decoder) = self.decoder {
            if decoder.is_finished() {
                self.current_chunk += 1;
                self.decoder = None;
            }
        }
        
        // 如果没有活动解码器，创建新的
        if self.decoder.is_none() && self.current_chunk < self.chunks.len() {
            let chunk_data = &self.chunks[self.current_chunk];
            self.decoder = Some(StreamingInflate::new(chunk_data));
        }
        
        // 从当前解码器读取数据
        if let Some(ref mut decoder) = self.decoder {
            decoder.read(buf)
        } else {
            Ok(0) // 没有更多数据
        }
    }
    
    /// 检查是否完成
    pub fn is_finished(&self) -> bool {
        self.current_chunk >= self.chunks.len() && 
        self.decoder.as_ref().map_or(true, |d| d.is_finished())
    }
}

/// 解压缩选项
#[derive(Debug, Clone)]
pub struct InflateOptions {
    pub chunk_size: usize,
    pub buffer_size: usize,
    pub max_memory: usize,
}

impl Default for InflateOptions {
    fn default() -> Self {
        Self {
            chunk_size: 32 * 1024, // 32KB
            buffer_size: 64 * 1024, // 64KB
            max_memory: 16 * 1024 * 1024, // 16MB
        }
    }
}

/// 高级解压缩器
pub struct AdvancedInflate {
    options: InflateOptions,
    buffer: Vec<u8>,
    memory_usage: usize,
}

impl AdvancedInflate {
    pub fn new(options: InflateOptions) -> Self {
        Self {
            options,
            buffer: Vec::new(),
            memory_usage: 0,
        }
    }
    
    /// 解压缩数据
    pub fn inflate(&mut self, data: &[u8]) -> Result<Vec<u8>, String> {
        // 检查内存使用
        if self.memory_usage > self.options.max_memory {
            return Err("Memory limit exceeded".to_string());
        }
        
        let mut result = Vec::new();
        let mut offset = 0;
        
        while offset < data.len() {
            let chunk_size = std::cmp::min(self.options.chunk_size, data.len() - offset);
            let chunk = &data[offset..offset + chunk_size];
            
            let mut inflate = SyncInflate::new();
            let decompressed = inflate.inflate(chunk)?;
            
            // 更新内存使用
            self.memory_usage += decompressed.len();
            if self.memory_usage > self.options.max_memory {
                return Err("Memory limit exceeded during decompression".to_string());
            }
            
            result.extend_from_slice(&decompressed);
            offset += chunk_size;
        }
        
        Ok(result)
    }
    
    /// 流式解压缩
    pub fn inflate_stream(&mut self, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut result = Vec::new();
        let mut offset = 0;
        
        while offset < data.len() {
            let chunk_size = std::cmp::min(self.options.chunk_size, data.len() - offset);
            let chunk = &data[offset..offset + chunk_size];
            
            let mut inflate = SyncInflate::new();
            let decompressed = inflate.inflate(chunk)?;
            
            // 检查缓冲区大小
            if self.buffer.len() + decompressed.len() > self.options.buffer_size {
                // 刷新缓冲区
                result.extend_from_slice(&self.buffer);
                self.buffer.clear();
            }
            
            self.buffer.extend_from_slice(&decompressed);
            offset += chunk_size;
        }
        
        // 添加剩余数据
        result.extend_from_slice(&self.buffer);
        self.buffer.clear();
        
        Ok(result)
    }
    
    /// 获取内存使用情况
    pub fn get_memory_usage(&self) -> usize {
        self.memory_usage
    }
    
    /// 重置内存使用
    pub fn reset_memory_usage(&mut self) {
        self.memory_usage = 0;
    }
}

/// 解压缩统计信息
#[derive(Debug, Clone)]
pub struct InflateStats {
    pub input_size: usize,
    pub output_size: usize,
    pub compression_ratio: f64,
    pub processing_time: std::time::Duration,
    pub memory_usage: usize,
}

impl InflateStats {
    pub fn new(input_size: usize, output_size: usize, processing_time: std::time::Duration, memory_usage: usize) -> Self {
        let compression_ratio = if input_size > 0 {
            output_size as f64 / input_size as f64
        } else {
            0.0
        };
        
        Self {
            input_size,
            output_size,
            compression_ratio,
            processing_time,
            memory_usage,
        }
    }
    
    pub fn get_compression_percentage(&self) -> f64 {
        (1.0 - self.compression_ratio) * 100.0
    }
}

/// 带统计信息的解压缩器
pub struct StatsInflate {
    inflate: AdvancedInflate,
    stats: InflateStats,
}

impl StatsInflate {
    pub fn new(options: InflateOptions) -> Self {
        Self {
            inflate: AdvancedInflate::new(options),
            stats: InflateStats::new(0, 0, std::time::Duration::from_secs(0), 0),
        }
    }
    
    /// 解压缩数据并收集统计信息
    pub fn inflate_with_stats(&mut self, data: &[u8]) -> Result<Vec<u8>, String> {
        let start_time = std::time::Instant::now();
        let input_size = data.len();
        
        let result = self.inflate.inflate(data)?;
        let output_size = result.len();
        let processing_time = start_time.elapsed();
        let memory_usage = self.inflate.get_memory_usage();
        
        self.stats = InflateStats::new(input_size, output_size, processing_time, memory_usage);
        
        Ok(result)
    }
    
    /// 获取统计信息
    pub fn get_stats(&self) -> &InflateStats {
        &self.stats
    }
    
    /// 重置统计信息
    pub fn reset_stats(&mut self) {
        self.stats = InflateStats::new(0, 0, std::time::Duration::from_secs(0), 0);
        self.inflate.reset_memory_usage();
    }
}
