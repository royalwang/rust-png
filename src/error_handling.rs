//! 错误处理模块
//! 实现完善的错误处理和恢复机制

use std::fmt;
use std::error::Error as StdError;

/// PNG处理错误类型
#[derive(Debug, Clone)]
pub enum PNGError {
    /// 文件格式错误
    InvalidFormat(String),
    /// 数据不足错误
    InsufficientData(String),
    /// 解析错误
    ParseError(String),
    /// 编码错误
    EncodeError(String),
    /// 解码错误
    DecodeError(String),
    /// 内存错误
    MemoryError(String),
    /// 性能错误
    PerformanceError(String),
    /// 未知错误
    Unknown(String),
}

impl fmt::Display for PNGError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PNGError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            PNGError::InsufficientData(msg) => write!(f, "Insufficient data: {}", msg),
            PNGError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            PNGError::EncodeError(msg) => write!(f, "Encode error: {}", msg),
            PNGError::DecodeError(msg) => write!(f, "Decode error: {}", msg),
            PNGError::MemoryError(msg) => write!(f, "Memory error: {}", msg),
            PNGError::PerformanceError(msg) => write!(f, "Performance error: {}", msg),
            PNGError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl StdError for PNGError {}

/// 错误恢复策略
#[derive(Debug, Clone)]
pub enum RecoveryStrategy {
    /// 忽略错误
    Ignore,
    /// 使用默认值
    UseDefault,
    /// 重试操作
    Retry,
    /// 降级处理
    Degrade,
    /// 失败
    Fail,
}

/// 错误处理器
pub struct ErrorHandler {
    max_retries: usize,
    retry_delay: std::time::Duration,
    recovery_strategies: std::collections::HashMap<String, RecoveryStrategy>,
}

impl ErrorHandler {
    pub fn new() -> Self {
        Self {
            max_retries: 3,
            retry_delay: std::time::Duration::from_millis(100),
            recovery_strategies: std::collections::HashMap::new(),
        }
    }
    
    pub fn with_max_retries(mut self, max_retries: usize) -> Self {
        self.max_retries = max_retries;
        self
    }
    
    pub fn with_retry_delay(mut self, delay: std::time::Duration) -> Self {
        self.retry_delay = delay;
        self
    }
    
    pub fn set_recovery_strategy(&mut self, error_type: String, strategy: RecoveryStrategy) {
        self.recovery_strategies.insert(error_type, strategy);
    }
    
    /// 处理错误
    pub fn handle_error<F, T>(&self, operation: F) -> Result<T, PNGError>
    where
        F: Fn() -> Result<T, PNGError>,
    {
        let mut last_error = None;
        
        for attempt in 0..=self.max_retries {
            match operation() {
                Ok(result) => return Ok(result),
                Err(error) => {
                    last_error = Some(error.clone());
                    
                    if attempt < self.max_retries {
                        let strategy = self.get_recovery_strategy(&error);
                        match strategy {
                            RecoveryStrategy::Retry => {
                                std::thread::sleep(self.retry_delay);
                                continue;
                            }
                            RecoveryStrategy::Ignore => {
                                return Err(error);
                            }
                            RecoveryStrategy::UseDefault => {
                                return self.get_default_value();
                            }
                            RecoveryStrategy::Degrade => {
                                return self.degrade_operation();
                            }
                            RecoveryStrategy::Fail => {
                                return Err(error);
                            }
                        }
                    }
                }
            }
        }
        
        Err(last_error.unwrap_or(PNGError::Unknown("Max retries exceeded".to_string())))
    }
    
    fn get_recovery_strategy(&self, error: &PNGError) -> RecoveryStrategy {
        let error_type = match error {
            PNGError::InvalidFormat(_) => "InvalidFormat",
            PNGError::InsufficientData(_) => "InsufficientData",
            PNGError::ParseError(_) => "ParseError",
            PNGError::EncodeError(_) => "EncodeError",
            PNGError::DecodeError(_) => "DecodeError",
            PNGError::MemoryError(_) => "MemoryError",
            PNGError::PerformanceError(_) => "PerformanceError",
            PNGError::Unknown(_) => "Unknown",
        };
        
        self.recovery_strategies
            .get(error_type)
            .cloned()
            .unwrap_or(RecoveryStrategy::Fail)
    }
    
    fn get_default_value<T>(&self) -> Result<T, PNGError> {
        // 这里需要根据具体类型实现默认值
        Err(PNGError::Unknown("No default value available".to_string()))
    }
    
    fn degrade_operation<T>(&self) -> Result<T, PNGError> {
        // 这里需要实现降级操作
        Err(PNGError::Unknown("Degrade operation not implemented".to_string()))
    }
}

/// 错误上下文
#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub operation: String,
    pub file_name: Option<String>,
    pub line_number: Option<u32>,
    pub column_number: Option<u32>,
    pub additional_info: std::collections::HashMap<String, String>,
}

impl ErrorContext {
    pub fn new(operation: String) -> Self {
        Self {
            operation,
            file_name: None,
            line_number: None,
            column_number: None,
            additional_info: std::collections::HashMap::new(),
        }
    }
    
    pub fn with_file_name(mut self, file_name: String) -> Self {
        self.file_name = Some(file_name);
        self
    }
    
    pub fn with_location(mut self, line: u32, column: u32) -> Self {
        self.line_number = Some(line);
        self.column_number = Some(column);
        self
    }
    
    pub fn with_info(mut self, key: String, value: String) -> Self {
        self.additional_info.insert(key, value);
        self
    }
}

/// 错误报告器
pub struct ErrorReporter {
    errors: Vec<(PNGError, ErrorContext)>,
    max_errors: usize,
}

impl ErrorReporter {
    pub fn new(max_errors: usize) -> Self {
        Self {
            errors: Vec::new(),
            max_errors,
        }
    }
    
    pub fn report_error(&mut self, error: PNGError, context: ErrorContext) {
        if self.errors.len() < self.max_errors {
            self.errors.push((error, context));
        }
    }
    
    pub fn get_errors(&self) -> &[(PNGError, ErrorContext)] {
        &self.errors
    }
    
    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }
    
    pub fn get_error_summary(&self) -> String {
        let mut summary = String::new();
        summary.push_str(&format!("Total errors: {}\n", self.errors.len()));
        
        for (error, context) in &self.errors {
            summary.push_str(&format!("Error: {} in {}\n", error, context.operation));
            if let Some(file) = &context.file_name {
                summary.push_str(&format!("  File: {}\n", file));
            }
            if let Some(line) = context.line_number {
                summary.push_str(&format!("  Line: {}\n", line));
            }
        }
        
        summary
    }
}

/// 错误验证器
pub struct ErrorValidator {
    strict_mode: bool,
    warnings_as_errors: bool,
}

impl ErrorValidator {
    pub fn new() -> Self {
        Self {
            strict_mode: false,
            warnings_as_errors: false,
        }
    }
    
    pub fn with_strict_mode(mut self, strict: bool) -> Self {
        self.strict_mode = strict;
        self
    }
    
    pub fn with_warnings_as_errors(mut self, warnings_as_errors: bool) -> Self {
        self.warnings_as_errors = warnings_as_errors;
        self
    }
    
    pub fn validate_png_data(&self, data: &[u8]) -> Result<(), PNGError> {
        if data.len() < 8 {
            return Err(PNGError::InsufficientData("PNG signature too short".to_string()));
        }
        
        // 检查PNG签名
        let signature = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];
        if &data[0..8] != &signature {
            return Err(PNGError::InvalidFormat("Invalid PNG signature".to_string()));
        }
        
        if self.strict_mode {
            // 严格模式下的额外验证
            self.validate_chunks(data)?;
        }
        
        Ok(())
    }
    
    fn validate_chunks(&self, data: &[u8]) -> Result<(), PNGError> {
        let mut offset = 8; // 跳过PNG签名
        
        while offset < data.len() {
            if offset + 12 > data.len() {
                return Err(PNGError::InsufficientData("Incomplete chunk".to_string()));
            }
            
            let length = u32::from_be_bytes([
                data[offset], data[offset + 1], data[offset + 2], data[offset + 3]
            ]) as usize;
            
            if offset + 12 + length > data.len() {
                return Err(PNGError::InsufficientData("Chunk data incomplete".to_string()));
            }
            
            offset += 12 + length; // 移动到下一个chunk
        }
        
        Ok(())
    }
}

/// 错误恢复器
pub struct ErrorRecovery {
    strategies: std::collections::HashMap<String, Box<dyn Fn(&PNGError) -> Result<(), PNGError> + Send + Sync>>,
}

impl ErrorRecovery {
    pub fn new() -> Self {
        Self {
            strategies: std::collections::HashMap::new(),
        }
    }
    
    pub fn register_strategy<F>(&mut self, error_type: String, strategy: F)
    where
        F: Fn(&PNGError) -> Result<(), PNGError> + Send + Sync + 'static,
    {
        self.strategies.insert(error_type, Box::new(strategy));
    }
    
    pub fn recover(&self, error: &PNGError) -> Result<(), PNGError> {
        let error_type = match error {
            PNGError::InvalidFormat(_) => "InvalidFormat",
            PNGError::InsufficientData(_) => "InsufficientData",
            PNGError::ParseError(_) => "ParseError",
            PNGError::EncodeError(_) => "EncodeError",
            PNGError::DecodeError(_) => "DecodeError",
            PNGError::MemoryError(_) => "MemoryError",
            PNGError::PerformanceError(_) => "PerformanceError",
            PNGError::Unknown(_) => "Unknown",
        };
        
        if let Some(strategy) = self.strategies.get(error_type) {
            strategy(error)
        } else {
            Err(PNGError::Unknown("No recovery strategy available".to_string()))
        }
    }
}

/// 错误统计
#[derive(Debug, Clone)]
pub struct ErrorStats {
    pub total_errors: usize,
    pub error_types: std::collections::HashMap<String, usize>,
    pub recovery_attempts: usize,
    pub recovery_successes: usize,
}

impl ErrorStats {
    pub fn new() -> Self {
        Self {
            total_errors: 0,
            error_types: std::collections::HashMap::new(),
            recovery_attempts: 0,
            recovery_successes: 0,
        }
    }
    
    pub fn record_error(&mut self, error: &PNGError) {
        self.total_errors += 1;
        
        let error_type = match error {
            PNGError::InvalidFormat(_) => "InvalidFormat",
            PNGError::InsufficientData(_) => "InsufficientData",
            PNGError::ParseError(_) => "ParseError",
            PNGError::EncodeError(_) => "EncodeError",
            PNGError::DecodeError(_) => "DecodeError",
            PNGError::MemoryError(_) => "MemoryError",
            PNGError::PerformanceError(_) => "PerformanceError",
            PNGError::Unknown(_) => "Unknown",
        };
        
        *self.error_types.entry(error_type.to_string()).or_insert(0) += 1;
    }
    
    pub fn record_recovery_attempt(&mut self, success: bool) {
        self.recovery_attempts += 1;
        if success {
            self.recovery_successes += 1;
        }
    }
    
    pub fn get_recovery_rate(&self) -> f64 {
        if self.recovery_attempts == 0 {
            0.0
        } else {
            self.recovery_successes as f64 / self.recovery_attempts as f64
        }
    }
}
