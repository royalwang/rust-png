# Rust PNG Library API 文档

## 目录

- [核心API](#核心api)
- [高级功能API](#高级功能api)
- [WebAssembly优化API](#webassembly优化api)
- [高级滤镜API](#高级滤镜api)
- [性能监控API](#性能监控api)
- [错误处理API](#错误处理api)
- [常量定义](#常量定义)

## 核心API

### PNG 类

主要的PNG处理类，提供异步PNG解析和编码功能。

```rust
pub struct PNG {
    // 内部字段
}
```

#### 构造函数

```rust
impl PNG {
    /// 创建新的PNG实例
    pub fn new() -> Self;
    
    /// 从选项创建PNG实例
    pub fn with_options(options: PNGOptions) -> Self;
}
```

#### 解析方法

```rust
impl PNG {
    /// 异步解析PNG数据
    pub fn parse<F>(&self, data: &[u8], callback: F) -> Result<(), String>
        where F: FnOnce(Result<PNG, String>) + Send + 'static;
    
    /// 解析PNG数据（带选项）
    pub fn parse_with_options<F>(&self, data: &[u8], options: ParseOptions, callback: F) -> Result<(), String>
        where F: FnOnce(Result<PNG, String>) + Send + 'static;
}
```

#### 编码方法

```rust
impl PNG {
    /// 打包PNG数据
    pub fn pack(&self) -> Result<Vec<u8>, String>;
    
    /// 打包PNG数据（带选项）
    pub fn pack_with_options(&self, options: PackOptions) -> Result<Vec<u8>, String>;
    
    /// 写入文件
    pub fn write_file(&self, filename: &str) -> Result<(), String>;
    
    /// 转换为缓冲区
    pub fn to_buffer(&self) -> Result<Vec<u8>, String>;
}
```

#### 属性访问

```rust
impl PNG {
    /// 获取图像宽度
    pub fn get_width(&self) -> u32;
    
    /// 获取图像高度
    pub fn get_height(&self) -> u32;
    
    /// 获取位深度
    pub fn get_bit_depth(&self) -> u8;
    
    /// 获取颜色类型
    pub fn get_color_type(&self) -> u8;
    
    /// 获取压缩方法
    pub fn get_compression_method(&self) -> u8;
    
    /// 获取滤镜方法
    pub fn get_filter_method(&self) -> u8;
    
    /// 获取交错方法
    pub fn get_interlace_method(&self) -> u8;
    
    /// 获取调色板
    pub fn get_palette(&self) -> Option<Vec<u8>>;
    
    /// 获取Gamma值
    pub fn get_gamma(&self) -> f64;
    
    /// 获取Alpha通道
    pub fn get_alpha(&self) -> bool;
}
```

#### 像素操作

```rust
impl PNG {
    /// 获取像素值
    pub fn get_pixel(&self, x: u32, y: u32) -> Result<[u8; 4], String>;
    
    /// 设置像素值
    pub fn set_pixel(&mut self, x: u32, y: u32, rgba: [u8; 4]) -> Result<(), String>;
    
    /// 获取RGBA8数组
    pub fn get_rgba8_array(&self) -> Vec<u8>;
    
    /// 设置RGBA8数组
    pub fn set_rgba8_array(&mut self, data: Vec<u8>) -> Result<(), String>;
    
    /// 获取数据
    pub fn get_data(&self) -> &Vec<u8>;
    
    /// 设置数据
    pub fn set_data(&mut self, data: Vec<u8>);
}
```

#### 图像操作

```rust
impl PNG {
    /// 位块传输
    pub fn bitblt(&self, src: &PNG, dst: &mut PNG, src_x: u32, src_y: u32, 
                  width: u32, height: u32, delta_x: u32, delta_y: u32) -> Result<(), String>;
    
    /// 调整Gamma
    pub fn adjust_gamma(&self, gamma: f64) -> Result<(), String>;
    
    /// 检查是否交错
    pub fn is_interlaced(&self) -> bool;
    
    /// 获取交错通道
    pub fn get_interlace_passes(&self) -> Vec<InterlacePass>;
    
    /// 获取交错统计
    pub fn get_interlace_stats(&self) -> InterlaceStats;
}
```

#### 流属性

```rust
impl PNG {
    /// 是否可读
    pub fn readable(&self) -> bool;
    
    /// 是否可写
    pub fn writable(&self) -> bool;
}
```

### PNGSync 类

同步PNG处理类，提供同步PNG解析和编码功能。

```rust
pub struct PNGSync {
    // 内部字段
}
```

#### 构造函数

```rust
impl PNGSync {
    /// 创建新的同步PNG实例
    pub fn new() -> Self;
}
```

#### 同步方法

```rust
impl PNGSync {
    /// 同步读取PNG数据
    pub fn read(&self, data: &[u8]) -> Result<PNG, String>;
    
    /// 同步写入PNG数据
    pub fn write(&self, png: &PNG) -> Result<Vec<u8>, String>;
    
    /// 同步读取PNG数据（带选项）
    pub fn read_with_options(&self, data: &[u8], options: ParseOptions) -> Result<PNG, String>;
    
    /// 同步写入PNG数据（带选项）
    pub fn write_with_options(&self, png: &PNG, options: PackOptions) -> Result<Vec<u8>, String>;
}
```

## 高级功能API

### AdvancedPNG 类

高级PNG处理类，支持16位PNG和颜色类型转换。

```rust
pub struct AdvancedPNG {
    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: u8,
    input_color_type: u8,
    input_has_alpha: bool,
    data: Option<Vec<u8>>,
}
```

#### 构造函数

```rust
impl AdvancedPNG {
    /// 创建新的高级PNG实例
    pub fn new(options: AdvancedPNGOptions) -> Self;
}
```

#### 数据设置

```rust
impl AdvancedPNG {
    /// 设置16位数据
    pub fn set_16bit_data(&mut self, data: &[u16]) -> Result<(), String>;
    
    /// 设置8位数据
    pub fn set_8bit_data(&mut self, data: &[u8]) -> Result<(), String>;
    
    /// 获取数据
    pub fn get_data(&self) -> Option<&Vec<u8>>;
    
    /// 设置数据
    pub fn set_data(&mut self, data: Vec<u8>);
}
```

#### 颜色转换

```rust
impl AdvancedPNG {
    /// 转换颜色类型
    pub fn convert_color_type(&mut self, target_color_type: u8) -> Result<(), String>;
}
```

#### 打包

```rust
impl AdvancedPNG {
    /// 打包PNG数据
    pub fn pack(&self) -> Result<Vec<u8>, String>;
}
```

### PNG16Bit 类

16位PNG处理器，专门处理16位PNG数据。

```rust
pub struct PNG16Bit {
    width: u32,
    height: u32,
    color_type: u8,
    data: Vec<u16>,
}
```

#### 构造函数

```rust
impl PNG16Bit {
    /// 创建新的16位PNG实例
    pub fn new(width: u32, height: u32, color_type: u8) -> Self;
    
    /// 从字节数组创建
    pub fn from_bytes(data: &[u8], width: u32, height: u32, color_type: u8) -> Result<Self, String>;
}
```

#### 像素操作

```rust
impl PNG16Bit {
    /// 设置像素值
    pub fn set_pixel(&mut self, x: u32, y: u32, values: &[u16]) -> Result<(), String>;
    
    /// 获取像素值
    pub fn get_pixel(&self, x: u32, y: u32) -> Result<Vec<u16>, String>;
}
```

#### 数据转换

```rust
impl PNG16Bit {
    /// 转换为字节数组
    pub fn to_bytes(&self) -> Vec<u8>;
}
```

### ColorTypeConverter 类

颜色类型转换器，提供颜色类型转换功能。

```rust
pub struct ColorTypeConverter;
```

#### 转换方法

```rust
impl ColorTypeConverter {
    /// 转换颜色类型
    pub fn convert(data: &[u8], from_type: u8, to_type: u8, bit_depth: u8) -> Result<Vec<u8>, String>;
}
```

## WebAssembly优化API

### WASMOptimizer 类

WebAssembly优化器，提供并行处理和SIMD优化。

```rust
pub struct WASMOptimizer {
    worker_pool: Vec<Worker>,
    max_workers: usize,
    memory_pool: Vec<Vec<u8>>,
    cache: std::collections::HashMap<String, Vec<u8>>,
}
```

#### 构造函数

```rust
impl WASMOptimizer {
    /// 创建新的WASM优化器
    pub fn new() -> Self;
}
```

#### 并行处理

```rust
impl WASMOptimizer {
    /// 并行处理PNG数据
    pub fn process_parallel(&mut self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String>;
}
```

#### 内存优化

```rust
impl WASMOptimizer {
    /// 优化内存
    pub fn optimize_memory(&mut self);
    
    /// 优化缓存
    pub fn optimize_cache(&mut self, key: String, data: Vec<u8>);
    
    /// 获取缓存
    pub fn get_cache(&self, key: &str) -> Option<&Vec<u8>>;
}
```

### WASMPerformanceMonitor 类

性能监控器，提供实时性能统计。

```rust
pub struct WASMPerformanceMonitor {
    start_time: f64,
    operations: Vec<WASMOperation>,
    memory_usage: f64,
}
```

#### 构造函数

```rust
impl WASMPerformanceMonitor {
    /// 创建新的性能监控器
    pub fn new() -> Self;
}
```

#### 性能监控

```rust
impl WASMPerformanceMonitor {
    /// 开始监控操作
    pub fn start_operation(&mut self, name: String) -> WASMOperationTimer;
    
    /// 记录操作完成
    pub fn record_operation(&mut self, name: String, duration: f64, memory_used: f64);
    
    /// 获取性能报告
    pub fn get_report(&self) -> WASMPerformanceReport;
}
```

### WASMMemoryManager 类

内存管理器，提供智能内存管理。

```rust
pub struct WASMMemoryManager {
    allocated: Vec<Vec<u8>>,
    max_memory: usize,
    current_usage: usize,
}
```

#### 构造函数

```rust
impl WASMMemoryManager {
    /// 创建新的内存管理器
    pub fn new(max_memory: usize) -> Self;
}
```

#### 内存管理

```rust
impl WASMMemoryManager {
    /// 分配内存
    pub fn allocate(&mut self, size: usize) -> Result<Vec<u8>, String>;
    
    /// 释放内存
    pub fn deallocate(&mut self, buffer: Vec<u8>);
    
    /// 获取内存使用情况
    pub fn get_memory_usage(&self) -> MemoryUsage;
}
```

### WASMCacheManager 类

缓存管理器，提供高效的缓存系统。

```rust
pub struct WASMCacheManager {
    cache: std::collections::HashMap<String, CachedData>,
    max_size: usize,
    current_size: usize,
}
```

#### 构造函数

```rust
impl WASMCacheManager {
    /// 创建新的缓存管理器
    pub fn new(max_size: usize) -> Self;
}
```

#### 缓存管理

```rust
impl WASMCacheManager {
    /// 设置缓存
    pub fn set(&mut self, key: String, data: Vec<u8>, ttl: f64) -> Result<(), String>;
    
    /// 获取缓存
    pub fn get(&mut self, key: &str) -> Option<Vec<u8>>;
}
```

## 高级滤镜API

### AdvancedFilterProcessor 类

高级滤镜处理器，支持多种滤镜算法。

```rust
pub struct AdvancedFilterProcessor {
    filters: Vec<Box<dyn AdvancedFilter>>,
    optimizer: FilterOptimizer,
}
```

#### 构造函数

```rust
impl AdvancedFilterProcessor {
    /// 创建新的高级滤镜处理器
    pub fn new() -> Self;
}
```

#### 滤镜管理

```rust
impl AdvancedFilterProcessor {
    /// 注册高级滤镜
    pub fn register_filter(&mut self, filter: Box<dyn AdvancedFilter>);
    
    /// 处理图像数据
    pub fn process_image(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String>;
}
```

### AdvancedFilter Trait

高级滤镜trait，定义滤镜接口。

```rust
pub trait AdvancedFilter: Send + Sync {
    /// 获取滤镜名称
    fn name(&self) -> &str;
    
    /// 处理图像数据
    fn process(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String>;
    
    /// 获取压缩比
    fn get_compression_ratio(&self, data: &[u8]) -> f64;
    
    /// 是否支持并行处理
    fn supports_parallel(&self) -> bool;
}
```

### AdaptiveFilter 类

自适应滤镜，根据图像内容自动选择最佳滤镜。

```rust
pub struct AdaptiveFilter {
    threshold: f64,
    context_aware: bool,
}
```

#### 构造函数

```rust
impl AdaptiveFilter {
    /// 创建新的自适应滤镜
    pub fn new(threshold: f64, context_aware: bool) -> Self;
}
```

### EdgeDetectionFilter 类

边缘检测滤镜，使用Sobel算子进行边缘检测。

```rust
pub struct EdgeDetectionFilter {
    sensitivity: f64,
    kernel_size: usize,
}
```

#### 构造函数

```rust
impl EdgeDetectionFilter {
    /// 创建新的边缘检测滤镜
    pub fn new(sensitivity: f64, kernel_size: usize) -> Self;
}
```

### NoiseReductionFilter 类

噪声减少滤镜，使用中值滤波减少噪声。

```rust
pub struct NoiseReductionFilter {
    strength: f64,
    window_size: usize,
}
```

#### 构造函数

```rust
impl NoiseReductionFilter {
    /// 创建新的噪声减少滤镜
    pub fn new(strength: f64, window_size: usize) -> Self;
}
```

## 性能监控API

### WASMOperation 结构体

WASM操作结构体，记录操作信息。

```rust
pub struct WASMOperation {
    pub name: String,
    pub duration: f64,
    pub memory_used: f64,
}
```

### WASMPerformanceReport 结构体

性能报告结构体，包含性能统计信息。

```rust
pub struct WASMPerformanceReport {
    pub total_duration: f64,
    pub total_memory: f64,
    pub operations: Vec<WASMOperation>,
    pub throughput: f64,
}
```

#### 方法

```rust
impl WASMPerformanceReport {
    /// 获取性能摘要
    pub fn get_summary(&self) -> String;
}
```

### MemoryUsage 结构体

内存使用情况结构体。

```rust
pub struct MemoryUsage {
    pub current: usize,
    pub max: usize,
    pub percentage: f64,
}
```

## 错误处理API

### PngError 枚举

PNG错误枚举，定义所有可能的错误类型。

```rust
pub enum PngError {
    InvalidSignature,
    InvalidChunkLength(String),
    CrcMismatch(String),
    MissingChunk(String),
    UnsupportedFeature(String),
    DecodingError(String),
    EncodingError(String),
    IoError(io::Error),
    Custom(String),
}
```

### ErrorHandler 类

错误处理器，提供错误恢复策略。

```rust
pub struct ErrorHandler {
    max_retries: u32,
    retry_delay: Duration,
    log_errors: bool,
    total_errors: u32,
    total_recoveries: u32,
}
```

#### 构造函数

```rust
impl ErrorHandler {
    /// 创建新的错误处理器
    pub fn new() -> Self;
    
    /// 设置最大重试次数
    pub fn with_max_retries(mut self, retries: u32) -> Self;
    
    /// 设置重试延迟
    pub fn with_retry_delay(mut self, delay: Duration) -> Self;
    
    /// 设置日志记录
    pub fn with_logging(mut self, log: bool) -> Self;
}
```

#### 错误处理

```rust
impl ErrorHandler {
    /// 处理错误
    pub fn handle_error<F, T>(&mut self, operation: &str, func: F) -> Result<T, PngError>
        where F: Fn() -> Result<T, PngError>;
}
```

## 常量定义

### PNG 常量

```rust
// PNG 签名
pub const PNG_SIGNATURE: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

// Chunk 类型
pub const TYPE_IHDR: u32 = 0x49484452;
pub const TYPE_IEND: u32 = 0x49454E44;
pub const TYPE_IDAT: u32 = 0x49444154;
pub const TYPE_PLTE: u32 = 0x504C5445;
pub const TYPE_tRNS: u32 = 0x74524E53;
pub const TYPE_gAMA: u32 = 0x67414D41;
pub const TYPE_cHRM: u32 = 0x6348524D;
pub const TYPE_sRGB: u32 = 0x73524742;
pub const TYPE_iCCP: u32 = 0x69434350;
pub const TYPE_tEXt: u32 = 0x74455874;
pub const TYPE_zTXt: u32 = 0x7A545874;
pub const TYPE_iTXt: u32 = 0x69545874;

// 颜色类型
pub const COLORTYPE_GRAYSCALE: u8 = 0;
pub const COLORTYPE_COLOR: u8 = 2;
pub const COLORTYPE_PALETTE_COLOR: u8 = 3;
pub const COLORTYPE_GRAYSCALE_ALPHA: u8 = 4;
pub const COLORTYPE_COLOR_ALPHA: u8 = 6;

// 滤镜类型
pub const FILTER_NONE: u8 = 0;
pub const FILTER_SUB: u8 = 1;
pub const FILTER_UP: u8 = 2;
pub const FILTER_AVERAGE: u8 = 3;
pub const FILTER_PAETH: u8 = 4;
pub const FILTER_ADAPTIVE: u8 = 5;

// 交错方法
pub const INTERLACE_NONE: u8 = 0;
pub const INTERLACE_ADAM7: u8 = 1;
```

### 选项结构体

```rust
// PNG 选项
pub struct PNGOptions {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: u8,
    pub input_color_type: u8,
    pub input_has_alpha: bool,
}

// 解析选项
pub struct ParseOptions {
    pub data: bool,
    pub skip_validation: bool,
    pub max_file_size: Option<usize>,
}

// 打包选项
pub struct PackOptions {
    pub deflate_chunk_size: usize,
    pub deflate_level: u32,
    pub deflate_strategy: u32,
    pub filter_method: u8,
    pub interlace_method: u8,
}

// 高级PNG选项
pub struct AdvancedPNGOptions {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: u8,
    pub input_color_type: u8,
    pub input_has_alpha: bool,
}

// WASM优化配置
pub struct WASMOptimizationConfig {
    pub max_workers: usize,
    pub memory_limit: usize,
    pub cache_size: usize,
    pub enable_simd: bool,
    pub enable_parallel: bool,
}
```

## 使用示例

### 基本使用

```rust
use rust_png::{PNG, PNGSync};

// 异步处理
let png = PNG::new();
png.parse(data, |result| {
    match result {
        Ok(png) => println!("Width: {}, Height: {}", png.get_width(), png.get_height()),
        Err(e) => eprintln!("Error: {}", e),
    }
});

// 同步处理
let png_sync = PNGSync::new();
let result = png_sync.read(data)?;
println!("Width: {}, Height: {}", result.get_width(), result.get_height());
```

### 高级功能

```rust
use rust_png::advanced_png::{AdvancedPNG, AdvancedPNGOptions};
use rust_png::constants::*;

let options = AdvancedPNGOptions {
    width: 320,
    height: 200,
    bit_depth: 16,
    color_type: COLORTYPE_COLOR_ALPHA,
    input_color_type: COLORTYPE_COLOR_ALPHA,
    input_has_alpha: true,
};

let mut advanced_png = AdvancedPNG::new(options);
advanced_png.set_16bit_data(&data)?;
advanced_png.convert_color_type(COLORTYPE_GRAYSCALE)?;
let packed = advanced_png.pack()?;
```

### WebAssembly优化

```rust
use rust_png::wasm_optimization::{WASMOptimizer, WASMOptimizationConfig};

let config = WASMOptimizationConfig::default();
let mut optimizer = WASMOptimizer::new();
let result = optimizer.process_parallel(&data, width, height)?;
```

### 高级滤镜

```rust
use rust_png::advanced_filters::{
    AdvancedFilterProcessor, AdaptiveFilter, EdgeDetectionFilter, NoiseReductionFilter
};

let mut processor = AdvancedFilterProcessor::new();
processor.register_filter(Box::new(AdaptiveFilter::new(0.5, true)));
processor.register_filter(Box::new(EdgeDetectionFilter::new(0.3, 3)));
processor.register_filter(Box::new(NoiseReductionFilter::new(0.8, 5)));

let filtered = processor.process_image(&data, width, height)?;
```
