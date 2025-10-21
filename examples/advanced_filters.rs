//! 高级滤镜示例
//! 展示Rust PNG Library的高级滤镜功能

use rust_png::advanced_filters::{
    AdvancedFilterProcessor, AdaptiveFilter, EdgeDetectionFilter, NoiseReductionFilter
};
use rust_png::{PNG, PNGSync};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rust PNG Library 高级滤镜示例 ===");
    
    // 示例1: 自适应滤镜
    example_adaptive_filter()?;
    
    // 示例2: 边缘检测滤镜
    example_edge_detection_filter()?;
    
    // 示例3: 噪声减少滤镜
    example_noise_reduction_filter()?;
    
    // 示例4: 多滤镜组合
    example_multiple_filters()?;
    
    // 示例5: 滤镜性能对比
    example_filter_performance()?;
    
    println!("=== 高级滤镜示例完成 ===");
    Ok(())
}

/// 示例1: 自适应滤镜
fn example_adaptive_filter() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n1. 自适应滤镜");
    
    // 创建自适应滤镜
    let adaptive_filter = AdaptiveFilter::new(0.5, true);
    println!("   自适应滤镜创建成功 (阈值: 0.5, 上下文感知: true)");
    
    // 创建测试图像数据
    let test_data = create_test_image_data(400, 300);
    println!("   测试图像数据创建完成，大小: {} bytes", test_data.len());
    
    // 应用自适应滤镜
    let filtered_data = adaptive_filter.process(&test_data, 400, 300)?;
    println!("   自适应滤镜处理完成，结果长度: {}", filtered_data.len());
    
    // 计算压缩比
    let compression_ratio = adaptive_filter.get_compression_ratio(&filtered_data);
    println!("   压缩比: {:.2}", compression_ratio);
    
    // 检查是否支持并行处理
    println!("   支持并行处理: {}", adaptive_filter.supports_parallel());
    
    Ok(())
}

/// 示例2: 边缘检测滤镜
fn example_edge_detection_filter() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n2. 边缘检测滤镜");
    
    // 创建边缘检测滤镜
    let edge_filter = EdgeDetectionFilter::new(0.3, 3);
    println!("   边缘检测滤镜创建成功 (敏感度: 0.3, 核大小: 3)");
    
    // 创建测试图像数据
    let test_data = create_test_image_data(400, 300);
    println!("   测试图像数据创建完成，大小: {} bytes", test_data.len());
    
    // 应用边缘检测滤镜
    let filtered_data = edge_filter.process(&test_data, 400, 300)?;
    println!("   边缘检测滤镜处理完成，结果长度: {}", filtered_data.len());
    
    // 计算压缩比
    let compression_ratio = edge_filter.get_compression_ratio(&filtered_data);
    println!("   压缩比: {:.2}", compression_ratio);
    
    // 检查是否支持并行处理
    println!("   支持并行处理: {}", edge_filter.supports_parallel());
    
    Ok(())
}

/// 示例3: 噪声减少滤镜
fn example_noise_reduction_filter() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n3. 噪声减少滤镜");
    
    // 创建噪声减少滤镜
    let noise_filter = NoiseReductionFilter::new(0.8, 5);
    println!("   噪声减少滤镜创建成功 (强度: 0.8, 窗口大小: 5)");
    
    // 创建带噪声的测试图像数据
    let noisy_data = create_noisy_image_data(400, 300);
    println!("   带噪声的测试图像数据创建完成，大小: {} bytes", noisy_data.len());
    
    // 应用噪声减少滤镜
    let filtered_data = noise_filter.process(&noisy_data, 400, 300)?;
    println!("   噪声减少滤镜处理完成，结果长度: {}", filtered_data.len());
    
    // 计算压缩比
    let compression_ratio = noise_filter.get_compression_ratio(&filtered_data);
    println!("   压缩比: {:.2}", compression_ratio);
    
    // 检查是否支持并行处理
    println!("   支持并行处理: {}", noise_filter.supports_parallel());
    
    Ok(())
}

/// 示例4: 多滤镜组合
fn example_multiple_filters() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n4. 多滤镜组合");
    
    // 创建高级滤镜处理器
    let mut processor = AdvancedFilterProcessor::new();
    println!("   高级滤镜处理器创建成功");
    
    // 注册多个滤镜
    processor.register_filter(Box::new(AdaptiveFilter::new(0.5, true)));
    processor.register_filter(Box::new(EdgeDetectionFilter::new(0.3, 3)));
    processor.register_filter(Box::new(NoiseReductionFilter::new(0.8, 5)));
    println!("   注册了3个滤镜: 自适应、边缘检测、噪声减少");
    
    // 创建测试图像数据
    let test_data = create_test_image_data(400, 300);
    println!("   测试图像数据创建完成，大小: {} bytes", test_data.len());
    
    // 应用多滤镜处理
    let filtered_data = processor.process_image(&test_data, 400, 300)?;
    println!("   多滤镜处理完成，结果长度: {}", filtered_data.len());
    
    // 保存处理结果
    save_filtered_image(&filtered_data, 400, 300, "output_filtered.png")?;
    println!("   处理结果已保存: output_filtered.png");
    
    Ok(())
}

/// 示例5: 滤镜性能对比
fn example_filter_performance() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n5. 滤镜性能对比");
    
    // 创建测试数据
    let test_data = create_test_image_data(800, 600);
    println!("   测试数据创建完成，大小: {} bytes", test_data.len());
    
    // 测试不同滤镜的性能
    let filters = vec![
        ("自适应滤镜", Box::new(AdaptiveFilter::new(0.5, true)) as Box<dyn rust_png::advanced_filters::AdvancedFilter>),
        ("边缘检测滤镜", Box::new(EdgeDetectionFilter::new(0.3, 3)) as Box<dyn rust_png::advanced_filters::AdvancedFilter>),
        ("噪声减少滤镜", Box::new(NoiseReductionFilter::new(0.8, 5)) as Box<dyn rust_png::advanced_filters::AdvancedFilter>),
    ];
    
    for (name, filter) in filters {
        let start = std::time::Instant::now();
        let result = filter.process(&test_data, 800, 600)?;
        let duration = start.elapsed();
        
        let compression_ratio = filter.get_compression_ratio(&result);
        let supports_parallel = filter.supports_parallel();
        
        println!("   {}:", name);
        println!("     处理时间: {:?}", duration);
        println!("     结果长度: {}", result.len());
        println!("     压缩比: {:.2}", compression_ratio);
        println!("     支持并行: {}", supports_parallel);
    }
    
    Ok(())
}

/// 创建测试图像数据
fn create_test_image_data(width: u32, height: u32) -> Vec<u8> {
    let mut data = Vec::new();
    
    // 创建渐变图像
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

/// 创建带噪声的图像数据
fn create_noisy_image_data(width: u32, height: u32) -> Vec<u8> {
    let mut data = Vec::new();
    
    // 创建带噪声的图像
    for y in 0..height {
        for x in 0..width {
            let base_r = (x * 255 / width) as u8;
            let base_g = (y * 255 / height) as u8;
            let base_b = 128u8;
            
            // 添加随机噪声
            let noise = (rand::random::<u8>() % 50) as i16 - 25;
            let r = (base_r as i16 + noise).clamp(0, 255) as u8;
            let g = (base_g as i16 + noise).clamp(0, 255) as u8;
            let b = (base_b as i16 + noise).clamp(0, 255) as u8;
            let a = 255u8;
            
            data.extend_from_slice(&[r, g, b, a]);
        }
    }
    
    data
}

/// 保存处理后的图像
fn save_filtered_image(data: &[u8], width: u32, height: u32, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 创建PNG实例
    let mut png = PNG::new();
    png.set_width(width);
    png.set_height(height);
    png.set_bit_depth(8);
    png.set_color_type(rust_png::constants::COLORTYPE_COLOR_ALPHA);
    
    // 设置像素数据
    png.set_rgba8_array(data.to_vec())?;
    
    // 打包PNG数据
    let png_data = png.pack()?;
    
    // 保存到文件
    fs::write(filename, png_data)?;
    
    Ok(())
}
