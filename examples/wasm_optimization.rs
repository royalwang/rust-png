//! WebAssembly优化示例
//! 展示Rust PNG Library的WebAssembly优化功能

use rust_png::wasm_optimization::{
    WASMOptimizer, WASMOptimizationConfig, WASMPerformanceMonitor, 
    WASMMemoryManager, WASMCacheManager, WASMOptimizerFactory
};
use rust_png::{PNG, PNGSync};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rust PNG Library WebAssembly优化示例 ===");
    
    // 示例1: 并行处理
    example_parallel_processing()?;
    
    // 示例2: 性能监控
    example_performance_monitoring()?;
    
    // 示例3: 内存管理
    example_memory_management()?;
    
    // 示例4: 缓存优化
    example_cache_optimization()?;
    
    // 示例5: 性能基准测试
    example_performance_benchmark()?;
    
    println!("=== WebAssembly优化示例完成 ===");
    Ok(())
}

/// 示例1: 并行处理
fn example_parallel_processing() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n1. 并行处理");
    
    // 创建WASM优化配置
    let config = WASMOptimizationConfig {
        max_workers: 4,
        memory_limit: 256 * 1024 * 1024, // 256MB
        cache_size: 64 * 1024 * 1024,    // 64MB
        enable_simd: true,
        enable_parallel: true,
    };
    
    // 创建WASM优化器
    let mut optimizer = WASMOptimizerFactory::create_optimizer(config);
    
    // 创建测试数据
    let test_data = create_test_image_data(800, 600);
    println!("   测试数据创建完成，大小: {} bytes", test_data.len());
    
    // 并行处理
    let start = Instant::now();
    let result = optimizer.process_parallel(&test_data, 800, 600)?;
    let duration = start.elapsed();
    
    println!("   并行处理完成，耗时: {:?}", duration);
    println!("   处理结果长度: {}", result.len());
    println!("   处理速度: {:.2} MB/s", 
             test_data.len() as f64 / duration.as_secs_f64() / 1024.0 / 1024.0);
    
    Ok(())
}

/// 示例2: 性能监控
fn example_performance_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n2. 性能监控");
    
    // 创建性能监控器
    let mut monitor = WASMPerformanceMonitor::new();
    
    // 监控多个操作
    let operations = vec![
        "PNG Parsing",
        "Image Processing", 
        "Filter Application",
        "PNG Encoding",
    ];
    
    for operation in operations {
        let timer = monitor.start_operation(operation.to_string());
        
        // 模拟操作
        std::thread::sleep(std::time::Duration::from_millis(50));
        
        drop(timer);
        println!("   操作 '{}' 完成", operation);
    }
    
    // 获取性能报告
    let report = monitor.get_report();
    println!("   性能报告: {}", report.get_summary());
    
    // 详细统计
    for operation in &report.operations {
        println!("   操作: {}, 耗时: {:.2}ms, 内存: {:.2}MB", 
                 operation.name, operation.duration, operation.memory_used / 1024.0 / 1024.0);
    }
    
    Ok(())
}

/// 示例3: 内存管理
fn example_memory_management() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n3. 内存管理");
    
    // 创建内存管理器
    let memory_manager = WASMMemoryManager::new(128 * 1024 * 1024); // 128MB
    
    // 获取初始内存使用情况
    let initial_usage = memory_manager.get_memory_usage();
    println!("   初始内存使用: {} / {} ({}%)", 
             initial_usage.current, initial_usage.max, initial_usage.percentage);
    
    // 分配内存
    let buffer1 = memory_manager.allocate(1024 * 1024)?; // 1MB
    let buffer2 = memory_manager.allocate(512 * 1024)?;  // 512KB
    
    let usage_after_alloc = memory_manager.get_memory_usage();
    println!("   分配后内存使用: {} / {} ({}%)", 
             usage_after_alloc.current, usage_after_alloc.max, usage_after_alloc.percentage);
    
    // 使用缓冲区
    println!("   缓冲区1长度: {}", buffer1.len());
    println!("   缓冲区2长度: {}", buffer2.len());
    
    // 释放内存
    memory_manager.deallocate(buffer1);
    memory_manager.deallocate(buffer2);
    
    let final_usage = memory_manager.get_memory_usage();
    println!("   释放后内存使用: {} / {} ({}%)", 
             final_usage.current, final_usage.max, final_usage.percentage);
    
    Ok(())
}

/// 示例4: 缓存优化
fn example_cache_optimization() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n4. 缓存优化");
    
    // 创建缓存管理器
    let mut cache_manager = WASMCacheManager::new(32 * 1024 * 1024); // 32MB
    
    // 创建测试数据
    let test_data = create_test_image_data(400, 300);
    let processed_data = process_image_data(&test_data);
    
    // 设置缓存
    cache_manager.set("processed_image".to_string(), processed_data, 3600.0)?; // 1小时TTL
    println!("   缓存设置完成: processed_image");
    
    // 从缓存获取
    if let Some(cached) = cache_manager.get("processed_image") {
        println!("   从缓存获取数据，长度: {}", cached.len());
    } else {
        println!("   缓存未命中");
    }
    
    // 缓存预热
    let common_images = vec!["logo", "background", "icon"];
    for image_name in common_images {
        let data = create_test_image_data(200, 200);
        let processed = process_image_data(&data);
        cache_manager.set(
            format!("cached_{}", image_name), 
            processed, 
            7200.0 // 2小时TTL
        )?;
        println!("   缓存预热完成: {}", image_name);
    }
    
    // 测试缓存命中
    for image_name in &["logo", "background", "icon"] {
        if let Some(cached) = cache_manager.get(&format!("cached_{}", image_name)) {
            println!("   缓存命中: {} (长度: {})", image_name, cached.len());
        } else {
            println!("   缓存未命中: {}", image_name);
        }
    }
    
    Ok(())
}

/// 示例5: 性能基准测试
fn example_performance_benchmark() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n5. 性能基准测试");
    
    // 创建测试数据
    let test_data = create_test_image_data(800, 600);
    let png_sync = PNGSync::new();
    
    // 1. 基本解析性能
    println!("   1. 基本解析性能");
    let start = Instant::now();
    for _ in 0..10 {
        let _ = png_sync.read(&test_data)?;
    }
    let basic_duration = start.elapsed();
    println!("     10次解析耗时: {:?}", basic_duration);
    println!("     平均每次: {:?}", basic_duration / 10);
    
    // 2. 并行处理性能
    println!("   2. 并行处理性能");
    let config = WASMOptimizationConfig::default();
    let mut optimizer = WASMOptimizer::new();
    
    let start = Instant::now();
    for _ in 0..5 {
        let _ = optimizer.process_parallel(&test_data, 800, 600)?;
    }
    let parallel_duration = start.elapsed();
    println!("     5次并行处理耗时: {:?}", parallel_duration);
    println!("     平均每次: {:?}", parallel_duration / 5);
    
    // 3. 性能对比
    println!("   3. 性能对比");
    let basic_avg = basic_duration.as_nanos() / 10;
    let parallel_avg = parallel_duration.as_nanos() / 5;
    let speedup = basic_avg as f64 / parallel_avg as f64;
    
    println!("     基本解析平均耗时: {} ns", basic_avg);
    println!("     并行处理平均耗时: {} ns", parallel_avg);
    println!("     性能提升: {:.2}x", speedup);
    
    // 4. 内存使用对比
    println!("   4. 内存使用对比");
    let memory_manager = WASMMemoryManager::new(256 * 1024 * 1024); // 256MB
    
    let buffer = memory_manager.allocate(test_data.len())?;
    let usage = memory_manager.get_memory_usage();
    
    println!("     分配内存: {} bytes", buffer.len());
    println!("     内存使用: {} / {} ({}%)", 
             usage.current, usage.max, usage.percentage);
    
    memory_manager.deallocate(buffer);
    
    Ok(())
}

/// 创建测试图像数据
fn create_test_image_data(width: u32, height: u32) -> Vec<u8> {
    let mut data = Vec::new();
    
    // 创建简单的测试图像数据
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

/// 处理图像数据
fn process_image_data(data: &[u8]) -> Vec<u8> {
    // 简单的图像处理：增加亮度
    data.iter().map(|&byte| byte.saturating_add(20)).collect()
}
