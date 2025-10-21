//! 完整应用示例
//! 展示Rust PNG Library的完整应用场景

use rust_png::{PNG, PNGSync};
use rust_png::advanced_png::{AdvancedPNG, AdvancedPNGOptions, PNG16Bit};
use rust_png::wasm_optimization::{WASMOptimizer, WASMOptimizationConfig, WASMPerformanceMonitor};
use rust_png::advanced_filters::{AdvancedFilterProcessor, AdaptiveFilter, EdgeDetectionFilter, NoiseReductionFilter};
use rust_png::error_handling::{ErrorHandler, PngError};
use rust_png::constants::*;
use std::fs;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rust PNG Library 完整应用示例 ===");
    
    // 示例1: 图像处理流水线
    example_image_processing_pipeline()?;
    
    // 示例2: 批量图像处理
    example_batch_image_processing()?;
    
    // 示例3: 实时图像处理
    example_realtime_image_processing()?;
    
    // 示例4: 错误处理和恢复
    example_error_handling_recovery()?;
    
    // 示例5: 性能优化应用
    example_performance_optimization()?;
    
    println!("=== 完整应用示例完成 ===");
    Ok(())
}

/// 示例1: 图像处理流水线
fn example_image_processing_pipeline() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n1. 图像处理流水线");
    
    // 创建测试图像
    let test_data = create_test_image_data(800, 600);
    println!("   测试图像创建完成，大小: {} bytes", test_data.len());
    
    // 步骤1: 解析PNG
    let png_sync = PNGSync::new();
    let mut png = png_sync.read(&test_data)?;
    println!("   步骤1: PNG解析完成");
    
    // 步骤2: 应用滤镜
    let mut processor = AdvancedFilterProcessor::new();
    processor.register_filter(Box::new(AdaptiveFilter::new(0.5, true)));
    processor.register_filter(Box::new(EdgeDetectionFilter::new(0.3, 3)));
    
    let pixels = png.get_rgba8_array();
    let filtered_pixels = processor.process_image(&pixels, png.get_width(), png.get_height())?;
    png.set_rgba8_array(filtered_pixels)?;
    println!("   步骤2: 滤镜处理完成");
    
    // 步骤3: 颜色转换
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
    advanced_png.convert_color_type(COLORTYPE_GRAYSCALE)?;
    println!("   步骤3: 颜色转换完成");
    
    // 步骤4: 编码输出
    let output_data = advanced_png.pack()?;
    fs::write("output_pipeline.png", output_data)?;
    println!("   步骤4: 编码输出完成，文件已保存: output_pipeline.png");
    
    Ok(())
}

/// 示例2: 批量图像处理
fn example_batch_image_processing() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n2. 批量图像处理");
    
    // 创建多个测试图像
    let image_sizes = vec![(400, 300), (800, 600), (1200, 900)];
    let mut images = Vec::new();
    
    for (i, (width, height)) in image_sizes.iter().enumerate() {
        let data = create_test_image_data(*width, *height);
        images.push((format!("image_{}", i), data, *width, *height));
    }
    
    println!("   创建了 {} 个测试图像", images.len());
    
    // 使用并行处理
    let config = WASMOptimizationConfig {
        max_workers: 4,
        memory_limit: 512 * 1024 * 1024, // 512MB
        cache_size: 128 * 1024 * 1024,    // 128MB
        enable_simd: true,
        enable_parallel: true,
    };
    
    let mut optimizer = WASMOptimizer::new();
    let mut processor = AdvancedFilterProcessor::new();
    processor.register_filter(Box::new(AdaptiveFilter::new(0.5, true)));
    
    let start = Instant::now();
    let mut results = Vec::new();
    
    for (name, data, width, height) in images {
        // 并行处理
        let processed_data = optimizer.process_parallel(&data, width, height)?;
        
        // 应用滤镜
        let filtered_data = processor.process_image(&processed_data, width, height)?;
        
        // 保存结果
        let filename = format!("output_{}.png", name);
        save_processed_image(&filtered_data, width, height, &filename)?;
        
        results.push((name, filename));
    }
    
    let duration = start.elapsed();
    println!("   批量处理完成，耗时: {:?}", duration);
    println!("   处理速度: {:.2} 图像/秒", images.len() as f64 / duration.as_secs_f64());
    
    for (name, filename) in results {
        println!("   {} -> {}", name, filename);
    }
    
    Ok(())
}

/// 示例3: 实时图像处理
fn example_realtime_image_processing() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n3. 实时图像处理");
    
    // 创建性能监控器
    let mut monitor = WASMPerformanceMonitor::new();
    
    // 创建优化器
    let config = WASMOptimizationConfig::default();
    let mut optimizer = WASMOptimizer::new();
    
    // 创建滤镜处理器
    let mut processor = AdvancedFilterProcessor::new();
    processor.register_filter(Box::new(AdaptiveFilter::new(0.5, true)));
    processor.register_filter(Box::new(NoiseReductionFilter::new(0.8, 5)));
    
    // 模拟实时处理
    let frame_count = 10;
    let mut total_processing_time = Duration::new(0, 0);
    
    for frame in 0..frame_count {
        let timer = monitor.start_operation(format!("Frame {}", frame));
        
        // 创建模拟帧数据
        let frame_data = create_test_image_data(640, 480);
        
        // 处理帧
        let start = Instant::now();
        let processed_data = optimizer.process_parallel(&frame_data, 640, 480)?;
        let filtered_data = processor.process_image(&processed_data, 640, 480)?;
        let processing_time = start.elapsed();
        
        total_processing_time += processing_time;
        
        drop(timer);
        
        println!("   帧 {} 处理完成，耗时: {:?}", frame, processing_time);
        
        // 模拟实时处理间隔
        std::thread::sleep(Duration::from_millis(16)); // 60 FPS
    }
    
    // 性能统计
    let report = monitor.get_report();
    println!("   实时处理性能报告: {}", report.get_summary());
    println!("   平均处理时间: {:?}", total_processing_time / frame_count);
    println!("   处理帧率: {:.2} FPS", 
             frame_count as f64 / total_processing_time.as_secs_f64());
    
    Ok(())
}

/// 示例4: 错误处理和恢复
fn example_error_handling_recovery() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n4. 错误处理和恢复");
    
    // 创建错误处理器
    let mut error_handler = ErrorHandler::new()
        .with_max_retries(3)
        .with_retry_delay(Duration::from_millis(100))
        .with_logging(true);
    
    // 测试各种错误情况
    let test_cases = vec![
        ("有效PNG", create_test_png_data()),
        ("无效数据", vec![0, 1, 2, 3, 4, 5]),
        ("空数据", vec![]),
        ("损坏的PNG", create_corrupted_png_data()),
    ];
    
    for (name, data) in test_cases {
        println!("   测试用例: {}", name);
        
        let result = error_handler.handle_error("PNG Processing", || {
            let png_sync = PNGSync::new();
            png_sync.read(&data)
        });
        
        match result {
            Ok(png) => {
                println!("     处理成功: {}x{}", png.get_width(), png.get_height());
            }
            Err(e) => {
                println!("     处理失败: {}", e);
                
                // 实现降级策略
                if let Ok(fallback_result) = fallback_processing(&data) {
                    println!("     降级处理成功: {}", fallback_result);
                } else {
                    println!("     降级处理也失败");
                }
            }
        }
    }
    
    Ok(())
}

/// 示例5: 性能优化应用
fn example_performance_optimization() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n5. 性能优化应用");
    
    // 创建测试数据
    let test_data = create_test_image_data(1920, 1080); // 4K图像
    println!("   测试数据创建完成，大小: {} bytes", test_data.len());
    
    // 性能基准测试
    let png_sync = PNGSync::new();
    
    // 1. 基本处理
    let start = Instant::now();
    let png = png_sync.read(&test_data)?;
    let basic_duration = start.elapsed();
    println!("   基本处理耗时: {:?}", basic_duration);
    
    // 2. 并行处理
    let config = WASMOptimizationConfig {
        max_workers: num_cpus::get(),
        memory_limit: 1024 * 1024 * 1024, // 1GB
        cache_size: 256 * 1024 * 1024,    // 256MB
        enable_simd: true,
        enable_parallel: true,
    };
    
    let mut optimizer = WASMOptimizer::new();
    let start = Instant::now();
    let parallel_result = optimizer.process_parallel(&test_data, png.get_width(), png.get_height())?;
    let parallel_duration = start.elapsed();
    println!("   并行处理耗时: {:?}", parallel_duration);
    
    // 3. 高级滤镜处理
    let mut processor = AdvancedFilterProcessor::new();
    processor.register_filter(Box::new(AdaptiveFilter::new(0.5, true)));
    processor.register_filter(Box::new(EdgeDetectionFilter::new(0.3, 3)));
    processor.register_filter(Box::new(NoiseReductionFilter::new(0.8, 5)));
    
    let start = Instant::now();
    let filtered_result = processor.process_image(&parallel_result, png.get_width(), png.get_height())?;
    let filter_duration = start.elapsed();
    println!("   滤镜处理耗时: {:?}", filter_duration);
    
    // 性能对比
    let total_optimized = parallel_duration + filter_duration;
    let speedup = basic_duration.as_nanos() as f64 / total_optimized.as_nanos() as f64;
    
    println!("   性能对比:");
    println!("     基本处理: {:?}", basic_duration);
    println!("     优化处理: {:?}", total_optimized);
    println!("     性能提升: {:.2}x", speedup);
    
    // 内存使用统计
    let memory_manager = rust_png::wasm_optimization::WASMMemoryManager::new(1024 * 1024 * 1024);
    let usage = memory_manager.get_memory_usage();
    println!("   内存使用: {} / {} ({}%)", 
             usage.current, usage.max, usage.percentage);
    
    Ok(())
}

/// 创建测试图像数据
fn create_test_image_data(width: u32, height: u32) -> Vec<u8> {
    let mut data = Vec::new();
    
    for y in 0..height {
        for x in 0..width {
            let r = (x * 255 / width) as u8;
            let g = (y * 255 / height) as u8;
            let b = 128u8;
            let a = 255u8;
            data.extend_from_slice(&[r, g, b, a]);
        }
    }
    
    data
}

/// 创建测试PNG数据
fn create_test_png_data() -> Vec<u8> {
    // 创建一个简单的1x1像素PNG数据
    vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG签名
        0x00, 0x00, 0x00, 0x0D, // IHDR长度
        0x49, 0x48, 0x44, 0x52, // IHDR类型
        0x00, 0x00, 0x00, 0x01, // 宽度: 1
        0x00, 0x00, 0x00, 0x01, // 高度: 1
        0x08, // 位深度: 8
        0x06, // 颜色类型: RGBA
        0x00, // 压缩方法: 0
        0x00, // 滤镜方法: 0
        0x00, // 交错方法: 0
        0x1F, 0x15, 0xC4, 0x89, // IHDR CRC
        0x00, 0x00, 0x00, 0x0C, // IDAT长度
        0x49, 0x44, 0x41, 0x54, // IDAT类型
        0x78, 0x9C, 0x62, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, // 压缩数据
        0xE5, 0x21, 0xBC, 0x33, // IDAT CRC
        0x00, 0x00, 0x00, 0x00, // IEND长度
        0x49, 0x45, 0x4E, 0x44, // IEND类型
        0xAE, 0x42, 0x60, 0x82, // IEND CRC
    ]
}

/// 创建损坏的PNG数据
fn create_corrupted_png_data() -> Vec<u8> {
    // 创建一个损坏的PNG数据
    vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG签名
        0x00, 0x00, 0x00, 0x0D, // IHDR长度
        0x49, 0x48, 0x44, 0x52, // IHDR类型
        0x00, 0x00, 0x00, 0x01, // 宽度: 1
        0x00, 0x00, 0x00, 0x01, // 高度: 1
        0x08, // 位深度: 8
        0x06, // 颜色类型: RGBA
        0x00, // 压缩方法: 0
        0x00, // 滤镜方法: 0
        0x00, // 交错方法: 0
        0x00, 0x00, 0x00, 0x00, // 错误的CRC
        // 缺少IDAT和IEND块
    ]
}

/// 保存处理后的图像
fn save_processed_image(data: &[u8], width: u32, height: u32, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut png = PNG::new();
    png.set_width(width);
    png.set_height(height);
    png.set_bit_depth(8);
    png.set_color_type(COLORTYPE_COLOR_ALPHA);
    png.set_rgba8_array(data.to_vec())?;
    
    let png_data = png.pack()?;
    fs::write(filename, png_data)?;
    
    Ok(())
}

/// 降级处理
fn fallback_processing(data: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    if data.len() < 8 {
        return Err("数据太短".into());
    }
    
    // 简单的降级处理
    Ok(format!("降级处理完成，数据长度: {}", data.len()))
}
