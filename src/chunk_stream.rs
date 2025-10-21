//! Chunk流处理模块
//! 实现PNG数据流处理，匹配原始pngjs库的chunkstream.js

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Chunk流读取请求
pub struct ReadRequest {
    pub length: usize,
    pub allow_less: bool,
    pub callback: Box<dyn Fn(Vec<u8>) + Send + Sync>,
}

/// Chunk流处理器
pub struct ChunkStream {
    buffers: Arc<Mutex<VecDeque<Vec<u8>>>>,
    buffered: Arc<Mutex<usize>>,
    reads: Arc<Mutex<VecDeque<ReadRequest>>>,
    paused: Arc<Mutex<bool>>,
    writable: Arc<Mutex<bool>>,
    encoding: String,
}

impl ChunkStream {
    pub fn new() -> Self {
        Self {
            buffers: Arc::new(Mutex::new(VecDeque::new())),
            buffered: Arc::new(Mutex::new(0)),
            reads: Arc::new(Mutex::new(VecDeque::new())),
            paused: Arc::new(Mutex::new(false)),
            writable: Arc::new(Mutex::new(true)),
            encoding: "utf8".to_string(),
        }
    }
    
    /// 读取数据
    pub fn read<F>(&self, length: isize, callback: F) -> Result<(), String>
    where
        F: Fn(Vec<u8>) + Send + Sync + 'static,
    {
        let read_request = ReadRequest {
            length: length.abs() as usize,
            allow_less: length < 0,
            callback: Box::new(callback),
        };
        
        {
            let mut reads = self.reads.lock().map_err(|e| e.to_string())?;
            reads.push_back(read_request);
        }
        
        // 异步处理
        self.process_async()?;
        
        Ok(())
    }
    
    /// 写入数据
    pub fn write(&self, data: &[u8]) -> Result<bool, String> {
        let writable = *self.writable.lock().map_err(|e| e.to_string())?;
        if !writable {
            return Err("Stream not writable".to_string());
        }
        
        {
            let mut buffers = self.buffers.lock().map_err(|e| e.to_string())?;
            buffers.push_back(data.to_vec());
        }
        
        {
            let mut buffered = self.buffered.lock().map_err(|e| e.to_string())?;
            *buffered += data.len();
        }
        
        self.process_async()?;
        Ok(true)
    }
    
    /// 异步处理
    fn process_async(&self) -> Result<(), String> {
        let buffers = Arc::clone(&self.buffers);
        let buffered = Arc::clone(&self.buffered);
        let reads = Arc::clone(&self.reads);
        let paused = Arc::clone(&self.paused);
        
        thread::spawn(move || {
            if let Err(e) = Self::process_internal(&buffers, &buffered, &reads, &paused) {
                eprintln!("ChunkStream processing error: {}", e);
            }
        });
        
        Ok(())
    }
    
    /// 内部处理逻辑
    fn process_internal(
        buffers: &Arc<Mutex<VecDeque<Vec<u8>>>>,
        buffered: &Arc<Mutex<usize>>,
        reads: &Arc<Mutex<VecDeque<ReadRequest>>>,
        paused: &Arc<Mutex<bool>>,
    ) -> Result<(), String> {
        loop {
            let mut current_data = Vec::new();
            let mut current_length = 0;
            
            // 收集所有缓冲数据
            {
                let mut buffers_guard = buffers.lock().map_err(|e| e.to_string())?;
                while let Some(buffer) = buffers_guard.pop_front() {
                    current_data.extend_from_slice(&buffer);
                    current_length += buffer.len();
                }
            }
            
            if current_data.is_empty() {
                break;
            }
            
            // 处理读取请求
            {
                let mut reads_guard = reads.lock().map_err(|e| e.to_string())?;
                let mut paused_guard = paused.lock().map_err(|e| e.to_string())?;
                
                while let Some(read_request) = reads_guard.pop_front() {
                    let available = current_data.len();
                    let needed = read_request.length;
                    
                    if available >= needed || read_request.allow_less {
                        let take_length = if available >= needed { needed } else { available };
                        let data = current_data.drain(0..take_length).collect();
                        
                        // 执行回调
                        (read_request.callback)(data);
                        
                        if current_data.is_empty() {
                            break;
                        }
                    } else {
                        // 数据不足，重新放回请求
                        reads_guard.push_front(read_request);
                        *paused_guard = true;
                        break;
                    }
                }
            }
            
            // 如果还有剩余数据，放回缓冲区
            if !current_data.is_empty() {
                let mut buffers_guard = buffers.lock().map_err(|e| e.to_string())?;
                buffers_guard.push_back(current_data);
            }
            
            // 更新缓冲大小
            {
                let mut buffered_guard = buffered.lock().map_err(|e| e.to_string())?;
                *buffered_guard = current_length;
            }
            
            // 短暂休眠避免CPU占用过高
            thread::sleep(Duration::from_millis(1));
        }
        
        Ok(())
    }
    
    /// 暂停流
    pub fn pause(&self) -> Result<(), String> {
        let mut paused = self.paused.lock().map_err(|e| e.to_string())?;
        *paused = true;
        Ok(())
    }
    
    /// 恢复流
    pub fn resume(&self) -> Result<(), String> {
        let mut paused = self.paused.lock().map_err(|e| e.to_string())?;
        *paused = false;
        self.process_async()?;
        Ok(())
    }
    
    /// 结束流
    pub fn end(&self) -> Result<(), String> {
        let mut writable = self.writable.lock().map_err(|e| e.to_string())?;
        *writable = false;
        Ok(())
    }
    
    /// 获取缓冲大小
    pub fn get_buffered_size(&self) -> Result<usize, String> {
        let buffered = self.buffered.lock().map_err(|e| e.to_string())?;
        Ok(*buffered)
    }
    
    /// 检查是否可写
    pub fn is_writable(&self) -> Result<bool, String> {
        let writable = self.writable.lock().map_err(|e| e.to_string())?;
        Ok(*writable)
    }
    
    /// 检查是否暂停
    pub fn is_paused(&self) -> Result<bool, String> {
        let paused = self.paused.lock().map_err(|e| e.to_string())?;
        Ok(*paused)
    }
}

/// 同步读取器
pub struct SyncReader {
    buffer: Vec<u8>,
    reads: VecDeque<ReadRequest>,
}

impl SyncReader {
    pub fn new(buffer: Vec<u8>) -> Self {
        Self {
            buffer,
            reads: VecDeque::new(),
        }
    }
    
    /// 读取数据
    pub fn read<F>(&mut self, length: isize, callback: F) -> Result<(), String>
    where
        F: Fn(Vec<u8>) + Send + Sync + 'static,
    {
        let read_request = ReadRequest {
            length: length.abs() as usize,
            allow_less: length < 0,
            callback: Box::new(callback),
        };
        
        self.reads.push_back(read_request);
        Ok(())
    }
    
    /// 处理所有读取请求
    pub fn process(&mut self) -> Result<(), String> {
        while !self.reads.is_empty() && !self.buffer.is_empty() {
            let read = self.reads.pop_front().ok_or("No read request")?;
            
            let available = self.buffer.len();
            let needed = read.length;
            
            if available >= needed || read.allow_less {
                let take_length = if available >= needed { needed } else { available };
                let data = self.buffer.drain(0..take_length).collect();
                
                // 执行回调
                (read.callback)(data);
            } else {
                // 数据不足，重新放回请求
                self.reads.push_front(read);
                break;
            }
        }
        
        if !self.reads.is_empty() {
            return Err("There are some read requests waiting on finished stream".to_string());
        }
        
        if !self.buffer.is_empty() {
            return Err("Unrecognised content at end of stream".to_string());
        }
        
        Ok(())
    }
    
    /// 获取剩余缓冲区大小
    pub fn get_remaining_size(&self) -> usize {
        self.buffer.len()
    }
    
    /// 检查是否完成
    pub fn is_finished(&self) -> bool {
        self.buffer.is_empty() && self.reads.is_empty()
    }
}

/// 流统计信息
#[derive(Debug, Clone)]
pub struct StreamStats {
    pub buffered_size: usize,
    pub pending_reads: usize,
    pub is_paused: bool,
    pub is_writable: bool,
}

impl ChunkStream {
    /// 获取流统计信息
    pub fn get_stats(&self) -> Result<StreamStats, String> {
        let buffered = self.buffered.lock().map_err(|e| e.to_string())?;
        let reads = self.reads.lock().map_err(|e| e.to_string())?;
        let paused = self.paused.lock().map_err(|e| e.to_string())?;
        let writable = self.writable.lock().map_err(|e| e.to_string())?;
        
        Ok(StreamStats {
            buffered_size: *buffered,
            pending_reads: reads.len(),
            is_paused: *paused,
            is_writable: *writable,
        })
    }
}

/// 流事件处理器
pub struct StreamEventHandler {
    on_data: Option<Box<dyn Fn(Vec<u8>) + Send + Sync>>,
    on_end: Option<Box<dyn Fn() + Send + Sync>>,
    on_error: Option<Box<dyn Fn(String) + Send + Sync>>,
    on_drain: Option<Box<dyn Fn() + Send + Sync>>,
}

impl StreamEventHandler {
    pub fn new() -> Self {
        Self {
            on_data: None,
            on_end: None,
            on_error: None,
            on_drain: None,
        }
    }
    
    pub fn on_data<F>(mut self, handler: F) -> Self
    where
        F: Fn(Vec<u8>) + Send + Sync + 'static,
    {
        self.on_data = Some(Box::new(handler));
        self
    }
    
    pub fn on_end<F>(mut self, handler: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_end = Some(Box::new(handler));
        self
    }
    
    pub fn on_error<F>(mut self, handler: F) -> Self
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        self.on_error = Some(Box::new(handler));
        self
    }
    
    pub fn on_drain<F>(mut self, handler: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_drain = Some(Box::new(handler));
        self
    }
    
    pub fn emit_data(&self, data: Vec<u8>) {
        if let Some(ref handler) = self.on_data {
            handler(data);
        }
    }
    
    pub fn emit_end(&self) {
        if let Some(ref handler) = self.on_end {
            handler();
        }
    }
    
    pub fn emit_error(&self, error: String) {
        if let Some(ref handler) = self.on_error {
            handler(error);
        }
    }
    
    pub fn emit_drain(&self) {
        if let Some(ref handler) = self.on_drain {
            handler();
        }
    }
}
