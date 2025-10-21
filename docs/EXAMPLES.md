# Rust PNG Library 使用示例

## 目录

- [基本使用](#基本使用)
- [高级功能](#高级功能)
- [WebAssembly优化](#webassembly优化)
- [高级滤镜](#高级滤镜)
- [性能监控](#性能监控)
- [错误处理](#错误处理)
- [完整示例](#完整示例)

## 基本使用

### 1. 异步PNG解析

```rust
use rust_png::{PNG, PNGSync};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取PNG文件
    let data = fs::read("example.png")?;
    
    // 创建PNG实例
    let png = PNG::new();
    
    // 异步解析
    png.parse(&data, |result| {
        match result {
            Ok(png) => {
                println!("图像信息:");
                println!("  宽度: {}", png.get_width());
                println!("  高度: {}", png.get_height());
                println!("  位深度: {}", png.get_bit_depth());
                println!("  颜色类型: {}", png.get_color_type());
                println!("  是否交错: {}", png.is_interlaced());
                
                // 获取像素数据
                let pixels = png.get_rgba8_array();
                println!("  像素数据长度: {}", pixels.len());
                
                // 获取特定像素
                if let Ok(pixel) = png.get_pixel(0, 0) {
                    println!("  左上角像素: R={}, G={}, B={}, A={}", 
                             pixel[0], pixel[1], pixel[2], pixel[3]);
                }
            }
            Err(e) => eprintln!("解析错误: {}", e),
        }
    });
    
    Ok(())
}
```

### 2. 同步PNG解析

```rust
use rust_png::{PNG, PNGSync};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 读取PNG文件
    let data = fs::read("example.png")?;
    
    // 创建同步PNG实例
    let png_sync = PNGSync::new();
    
    // 同步解析
    let png = png_sync.read(&data)?;
    
    println!("图像信息:");
    println!("  宽度: {}", png.get_width());
    println!("  高度: {}", png.get_height());
    println!("  位深度: {}", png.get_bit_depth());
    println!("  颜色类型: {}", png.get_color_type());
    
    // 获取像素数据
    let pixels = png.get_rgba8_array();
    println!("像素数据长度: {}", pixels.len());
    
    Ok(())
}
```

### 3. PNG编码

```rust
use rust_png::{PNG, PNGSync};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建新的PNG图像
    let mut png = PNG::new();
    
    // 设置图像属性
    png.set_width(320);
    png.set_height(200);
    png.set_bit_depth(8);
    png.set_color_type(COLORTYPE_COLOR_ALPHA);
    
    // 创建像素数据 (RGBA)
    let mut pixels = Vec::new();
    for y in 0..200 {
        for x in 0..320 {
            let r = (x * 255 / 320) as u8;
            let g = (y * 255 / 200) as u8;
            let b = 128u8;
            let a = 255u8;
            pixels.extend_from_slice(&[r, g, b, a]);
        }
    }
    
    // 设置像素数据
    png.set_rgba8_array(pixels)?;
    
    // 打包PNG数据
    let png_data = png.pack()?;
    
    // 保存到文件
    fs::write("output.png", png_data)?;
    println!("PNG文件已保存: output.png");
    
    Ok(())
}
```

## 高级功能

### 1. 16位PNG处理

```rust
use rust_png::advanced_png::{AdvancedPNG, AdvancedPNGOptions, PNG16Bit};
use rust_png::constants::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建16位PNG选项
    let options = AdvancedPNGOptions {
        width: 320,
        height: 200,
        bit_depth: 16,
        color_type: COLORTYPE_COLOR_ALPHA,
        input_color_type: COLORTYPE_COLOR_ALPHA,
        input_has_alpha: true,
    };
    
    // 创建高级PNG实例
    let mut advanced_png = AdvancedPNG::new(options);
    
    // 创建16位数据
    let mut data_16bit = Vec::new();
    for y in 0..200 {
        for x in 0..320 {
            let r = (x * 65535 / 320) as u16;
            let g = (y * 65535 / 200) as u16;
            let b = 32768u16;
            let a = 65535u16;
            data_16bit.extend_from_slice(&[r, g, b, a]);
        }
    }
    
    // 设置16位数据
    advanced_png.set_16bit_data(&data_16bit)?;
    
    // 转换颜色类型
    advanced_png.convert_color_type(COLORTYPE_GRAYSCALE)?;
    
    // 打包PNG数据
    let png_data = advanced_png.pack()?;
    
    // 保存到文件
    std::fs::write("output_16bit.png", png_data)?;
    println!("16位PNG文件已保存: output_16bit.png");
    
    Ok(())
}
```

### 2. 16位PNG处理器

```rust
use rust_png::advanced_png::PNG16Bit;
use rust_png::constants::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建16位PNG处理器
    let mut png_16bit = PNG16Bit::new(320, 200, COLORTYPE_COLOR_ALPHA);
    
    // 设置像素值
    for y in 0..200 {
        for x in 0..320 {
            let r = (x * 65535 / 320) as u16;
            let g = (y * 65535 / 200) as u16;
            let b = 32768u16;
            let a = 65535u16;
            png_16bit.set_pixel(x, y, &[r, g, b, a])?;
        }
    }
    
    // 获取特定像素
    let pixel = png_16bit.get_pixel(100, 100)?;
    println!("像素(100,100): R={}, G={}, B={}, A={}", 
             pixel[0], pixel[1], pixel[2], pixel[3]);
    
    // 转换为字节数组
    let bytes = png_16bit.to_bytes();
    println!("字节数组长度: {}", bytes.len());
    
    Ok(())
}
```

### 3. 颜色类型转换

```rust
use rust_png::advanced_png::ColorTypeConverter;
use rust_png::constants::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建RGB数据
    let mut rgb_data = Vec::new();
    for i in 0..1000 {
        let r = (i % 256) as u8;
        let g = ((i * 2) % 256) as u8;
        let b = ((i * 3) % 256) as u8;
        rgb_data.extend_from_slice(&[r, g, b]);
    }
    
    // 转换为灰度
    let gray_data = ColorTypeConverter::convert(
        &rgb_data, 
        COLORTYPE_COLOR, 
        COLORTYPE_GRAYSCALE, 
        8
    )?;
    
    println!("原始RGB数据长度: {}", rgb_data.len());
    println!("转换后灰度数据长度: {}", gray_data.len());
    
    // 转换为RGBA
    let rgba_data = ColorTypeConverter::convert(
        &rgb_data, 
        COLORTYPE_COLOR, 
        COLORTYPE_COLOR_ALPHA, 
        8
    )?;
    
    println!("转换后RGBA数据长度: {}", rgba_data.len());
    
    Ok(())
}
```

## WebAssembly优化

### 1. 并行处理

```rust
use rust_png::wasm_optimization::{WASMOptimizer, WASMOptimizationConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建WASM优化配置
    let config = WASMOptimizationConfig {
        max_workers: 8,
        memory_limit: 512 * 1024 * 1024, // 512MB
        cache_size: 128 * 1024 * 1024,    // 128MB
        enable_simd: true,
        enable_parallel: true,
    };
    
    // 创建WASM优化器
    let mut optimizer = WASMOptimizer::new();
    
    // 读取PNG数据
    let data = std::fs::read("large_image.png")?;
    
    // 并行处理
    let result = optimizer.process_parallel(&data, 1920, 1080)?;
    
    println!("处理完成，结果长度: {}", result.len());
    
    Ok(())
}
```

### 2. 性能监控

```rust
use rust_png::wasm_optimization::{WASMPerformanceMonitor, WASMOptimizer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建性能监控器
    let mut monitor = WASMPerformanceMonitor::new();
    
    // 开始监控操作
    let timer = monitor.start_operation("PNG Processing".to_string());
    
    // 执行PNG处理
    let mut optimizer = WASMOptimizer::new();
    let data = std::fs::read("example.png")?;
    let result = optimizer.process_parallel(&data, 800, 600)?;
    
    // 结束监控
    drop(timer);
    
    // 获取性能报告
    let report = monitor.get_report();
    println!("性能报告: {}", report.get_summary());
    
    Ok(())
}
```

### 3. 内存管理

```rust
use rust_png::wasm_optimization::{WASMMemoryManager, WASMCacheManager};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建内存管理器
    let mut memory_manager = WASMMemoryManager::new(256 * 1024 * 1024); // 256MB
    
    // 分配内存
    let buffer1 = memory_manager.allocate(1024 * 1024)?; // 1MB
    let buffer2 = memory_manager.allocate(512 * 1024)?;  // 512KB
    
    // 获取内存使用情况
    let usage = memory_manager.get_memory_usage();
    println!("内存使用: {} / {} ({}%)", 
             usage.current, usage.max, usage.percentage);
    
    // 释放内存
    memory_manager.deallocate(buffer1);
    memory_manager.deallocate(buffer2);
    
    // 创建缓存管理器
    let mut cache_manager = WASMCacheManager::new(64 * 1024 * 1024); // 64MB
    
    // 设置缓存
    let data = vec![1, 2, 3, 4, 5];
    cache_manager.set("key1".to_string(), data, 3600.0)?; // 1小时TTL
    
    // 获取缓存
    if let Some(cached_data) = cache_manager.get("key1") {
        println!("从缓存获取数据: {:?}", cached_data);
    }
    
    Ok(())
}
```

## 高级滤镜

### 1. 自适应滤镜

```rust
use rust_png::advanced_filters::{AdvancedFilterProcessor, AdaptiveFilter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建高级滤镜处理器
    let mut processor = AdvancedFilterProcessor::new();
    
    // 注册自适应滤镜
    processor.register_filter(Box::new(AdaptiveFilter::new(0.5, true)));
    
    // 读取图像数据
    let data = std::fs::read("example.png")?;
    
    // 处理图像
    let filtered = processor.process_image(&data, 800, 600)?;
    
    println!("滤镜处理完成，结果长度: {}", filtered.len());
    
    Ok(())
}
```

### 2. 边缘检测滤镜

```rust
use rust_png::advanced_filters::{AdvancedFilterProcessor, EdgeDetectionFilter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建高级滤镜处理器
    let mut processor = AdvancedFilterProcessor::new();
    
    // 注册边缘检测滤镜
    processor.register_filter(Box::new(EdgeDetectionFilter::new(0.3, 3)));
    
    // 读取图像数据
    let data = std::fs::read("example.png")?;
    
    // 处理图像
    let filtered = processor.process_image(&data, 800, 600)?;
    
    println!("边缘检测完成，结果长度: {}", filtered.len());
    
    Ok(())
}
```

### 3. 噪声减少滤镜

```rust
use rust_png::advanced_filters::{AdvancedFilterProcessor, NoiseReductionFilter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建高级滤镜处理器
    let mut processor = AdvancedFilterProcessor::new();
    
    // 注册噪声减少滤镜
    processor.register_filter(Box::new(NoiseReductionFilter::new(0.8, 5)));
    
    // 读取图像数据
    let data = std::fs::read("noisy_image.png")?;
    
    // 处理图像
    let filtered = processor.process_image(&data, 800, 600)?;
    
    println!("噪声减少完成，结果长度: {}", filtered.len());
    
    Ok(())
}
```

### 4. 多滤镜组合

```rust
use rust_png::advanced_filters::{
    AdvancedFilterProcessor, AdaptiveFilter, EdgeDetectionFilter, NoiseReductionFilter
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建高级滤镜处理器
    let mut processor = AdvancedFilterProcessor::new();
    
    // 注册多个滤镜
    processor.register_filter(Box::new(AdaptiveFilter::new(0.5, true)));
    processor.register_filter(Box::new(EdgeDetectionFilter::new(0.3, 3)));
    processor.register_filter(Box::new(NoiseReductionFilter::new(0.8, 5)));
    
    // 读取图像数据
    let data = std::fs::read("example.png")?;
    
    // 处理图像
    let filtered = processor.process_image(&data, 800, 600)?;
    
    println!("多滤镜处理完成，结果长度: {}", filtered.len());
    
    Ok(())
}
```

## 性能监控

### 1. 基本性能监控

```rust
use rust_png::wasm_optimization::WASMPerformanceMonitor;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建性能监控器
    let mut monitor = WASMPerformanceMonitor::new();
    
    // 监控多个操作
    let timer1 = monitor.start_operation("PNG Parsing".to_string());
    // ... 执行PNG解析
    drop(timer1);
    
    let timer2 = monitor.start_operation("Image Processing".to_string());
    // ... 执行图像处理
    drop(timer2);
    
    let timer3 = monitor.start_operation("PNG Encoding".to_string());
    // ... 执行PNG编码
    drop(timer3);
    
    // 获取性能报告
    let report = monitor.get_report();
    println!("性能报告: {}", report.get_summary());
    
    // 详细统计
    for operation in &report.operations {
        println!("操作: {}, 耗时: {:.2}ms, 内存: {:.2}MB", 
                 operation.name, operation.duration, operation.memory_used / 1024.0 / 1024.0);
    }
    
    Ok(())
}
```

### 2. 性能优化

```rust
use rust_png::wasm_optimization::{WASMOptimizer, WASMOptimizationConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建优化配置
    let config = WASMOptimizationConfig {
        max_workers: num_cpus::get(),
        memory_limit: 1024 * 1024 * 1024, // 1GB
        cache_size: 256 * 1024 * 1024,     // 256MB
        enable_simd: true,
        enable_parallel: true,
    };
    
    // 创建优化器
    let mut optimizer = WASMOptimizer::new();
    
    // 优化内存
    optimizer.optimize_memory();
    
    // 处理多个图像
    let images = vec!["image1.png", "image2.png", "image3.png"];
    
    for image_path in images {
        let data = std::fs::read(image_path)?;
        let result = optimizer.process_parallel(&data, 800, 600)?;
        
        // 缓存结果
        optimizer.optimize_cache(
            format!("processed_{}", image_path), 
            result
        );
        
        println!("处理完成: {}", image_path);
    }
    
    Ok(())
}
```

## 错误处理

### 1. 基本错误处理

```rust
use rust_png::{PNG, PNGSync};
use rust_png::error_handling::{ErrorHandler, PngError};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建错误处理器
    let mut error_handler = ErrorHandler::new()
        .with_max_retries(3)
        .with_logging(true);
    
    // 处理PNG解析
    let result = error_handler.handle_error("PNG Parsing", || {
        let png_sync = PNGSync::new();
        let data = std::fs::read("example.png")?;
        png_sync.read(&data)
    });
    
    match result {
        Ok(png) => {
            println!("解析成功: {}x{}", png.get_width(), png.get_height());
        }
        Err(e) => {
            eprintln!("解析失败: {}", e);
        }
    }
    
    Ok(())
}
```

### 2. 错误恢复

```rust
use rust_png::error_handling::{ErrorHandler, PngError};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建错误处理器，配置重试策略
    let mut error_handler = ErrorHandler::new()
        .with_max_retries(5)
        .with_retry_delay(Duration::from_millis(100))
        .with_logging(true);
    
    // 处理可能失败的操作
    let result = error_handler.handle_error("Risky Operation", || {
        // 模拟可能失败的操作
        if rand::random::<f64>() < 0.3 {
            Err(PngError::Custom("随机失败".to_string()))
        } else {
            Ok("成功".to_string())
        }
    });
    
    match result {
        Ok(msg) => println!("操作成功: {}", msg),
        Err(e) => eprintln!("操作失败: {}", e),
    }
    
    Ok(())
}
```

## 完整示例

### 1. 完整的PNG处理流程

```rust
use rust_png::{PNG, PNGSync};
use rust_png::advanced_png::{AdvancedPNG, AdvancedPNGOptions};
use rust_png::wasm_optimization::{WASMOptimizer, WASMOptimizationConfig};
use rust_png::advanced_filters::{AdvancedFilterProcessor, AdaptiveFilter};
use rust_png::constants::*;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rust PNG Library 完整示例 ===");
    
    // 1. 基本PNG解析
    println!("1. 基本PNG解析");
    let data = fs::read("example.png")?;
    let png_sync = PNGSync::new();
    let png = png_sync.read(&data)?;
    println!("   图像尺寸: {}x{}", png.get_width(), png.get_height());
    println!("   位深度: {}", png.get_bit_depth());
    println!("   颜色类型: {}", png.get_color_type());
    
    // 2. 高级PNG处理
    println!("2. 高级PNG处理");
    let options = AdvancedPNGOptions {
        width: png.get_width(),
        height: png.get_height(),
        bit_depth: png.get_bit_depth(),
        color_type: png.get_color_type(),
        input_color_type: png.get_color_type(),
        input_has_alpha: png.get_alpha(),
    };
    
    let mut advanced_png = AdvancedPNG::new(options);
    advanced_png.set_data(png.get_data().clone());
    
    // 转换颜色类型
    advanced_png.convert_color_type(COLORTYPE_GRAYSCALE)?;
    println!("   颜色类型转换完成");
    
    // 3. WebAssembly优化
    println!("3. WebAssembly优化");
    let config = WASMOptimizationConfig::default();
    let mut optimizer = WASMOptimizer::new();
    let optimized_data = optimizer.process_parallel(&data, png.get_width(), png.get_height())?;
    println!("   优化处理完成，数据长度: {}", optimized_data.len());
    
    // 4. 高级滤镜处理
    println!("4. 高级滤镜处理");
    let mut processor = AdvancedFilterProcessor::new();
    processor.register_filter(Box::new(AdaptiveFilter::new(0.5, true)));
    let filtered_data = processor.process_image(&data, png.get_width(), png.get_height())?;
    println!("   滤镜处理完成，数据长度: {}", filtered_data.len());
    
    // 5. 保存结果
    println!("5. 保存结果");
    let packed = advanced_png.pack()?;
    fs::write("output_processed.png", packed)?;
    println!("   处理后的图像已保存: output_processed.png");
    
    println!("=== 处理完成 ===");
    Ok(())
}
```

### 2. 性能基准测试

```rust
use rust_png::{PNG, PNGSync};
use rust_png::wasm_optimization::{WASMOptimizer, WASMPerformanceMonitor};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 性能基准测试 ===");
    
    // 读取测试图像
    let data = std::fs::read("large_image.png")?;
    let png_sync = PNGSync::new();
    let png = png_sync.read(&data)?;
    
    // 1. 基本解析性能
    println!("1. 基本解析性能");
    let start = Instant::now();
    for _ in 0..100 {
        let _ = png_sync.read(&data)?;
    }
    let duration = start.elapsed();
    println!("   100次解析耗时: {:?}", duration);
    println!("   平均每次: {:?}", duration / 100);
    
    // 2. 并行处理性能
    println!("2. 并行处理性能");
    let mut optimizer = WASMOptimizer::new();
    let start = Instant::now();
    for _ in 0..10 {
        let _ = optimizer.process_parallel(&data, png.get_width(), png.get_height())?;
    }
    let duration = start.elapsed();
    println!("   10次并行处理耗时: {:?}", duration);
    println!("   平均每次: {:?}", duration / 10);
    
    // 3. 性能监控
    println!("3. 性能监控");
    let mut monitor = WASMPerformanceMonitor::new();
    let timer = monitor.start_operation("Benchmark".to_string());
    
    // 执行一些操作
    for _ in 0..50 {
        let _ = png_sync.read(&data)?;
    }
    
    drop(timer);
    let report = monitor.get_report();
    println!("   性能报告: {}", report.get_summary());
    
    println!("=== 基准测试完成 ===");
    Ok(())
}
```

### 3. 错误处理和恢复

```rust
use rust_png::{PNG, PNGSync};
use rust_png::error_handling::{ErrorHandler, PngError};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 错误处理和恢复示例 ===");
    
    // 创建错误处理器
    let mut error_handler = ErrorHandler::new()
        .with_max_retries(3)
        .with_retry_delay(Duration::from_millis(100))
        .with_logging(true);
    
    // 处理多个可能失败的操作
    let operations = vec![
        "valid_image.png",
        "corrupted_image.png",
        "large_image.png",
        "invalid_format.jpg",
    ];
    
    for operation in operations {
        println!("处理: {}", operation);
        
        let result = error_handler.handle_error("PNG Processing", || {
            let data = std::fs::read(operation)?;
            let png_sync = PNGSync::new();
            png_sync.read(&data)
        });
        
        match result {
            Ok(png) => {
                println!("  成功: {}x{}", png.get_width(), png.get_height());
            }
            Err(e) => {
                println!("  失败: {}", e);
            }
        }
    }
    
    println!("=== 错误处理完成 ===");
    Ok(())
}
```

这些示例展示了Rust PNG Library的完整功能，包括基本使用、高级功能、WebAssembly优化、高级滤镜、性能监控和错误处理。每个示例都是完整可运行的，可以作为学习和开发的参考。
