//! 基本使用示例
//! 展示Rust PNG Library的基本功能

use rust_png::{PNG, PNGSync};
use rust_png::constants::*;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rust PNG Library 基本使用示例 ===");
    
    // 示例1: 同步PNG解析
    example_sync_parsing()?;
    
    // 示例2: 异步PNG解析
    example_async_parsing()?;
    
    // 示例3: PNG编码
    example_png_encoding()?;
    
    // 示例4: 像素操作
    example_pixel_operations()?;
    
    // 示例5: 图像属性
    example_image_properties()?;
    
    println!("=== 基本使用示例完成 ===");
    Ok(())
}

/// 示例1: 同步PNG解析
fn example_sync_parsing() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n1. 同步PNG解析");
    
    // 创建测试PNG数据 (这里使用一个简单的1x1像素PNG)
    let test_png_data = create_test_png_data();
    
    // 创建同步PNG实例
    let png_sync = PNGSync::new();
    
    // 解析PNG数据
    let png = png_sync.read(&test_png_data)?;
    
    println!("   解析成功!");
    println!("   宽度: {}", png.get_width());
    println!("   高度: {}", png.get_height());
    println!("   位深度: {}", png.get_bit_depth());
    println!("   颜色类型: {}", png.get_color_type());
    
    Ok(())
}

/// 示例2: 异步PNG解析
fn example_async_parsing() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n2. 异步PNG解析");
    
    // 创建测试PNG数据
    let test_png_data = create_test_png_data();
    
    // 创建PNG实例
    let png = PNG::new();
    
    // 异步解析
    png.parse(&test_png_data, |result| {
        match result {
            Ok(png) => {
                println!("   异步解析成功!");
                println!("   宽度: {}, 高度: {}", png.get_width(), png.get_height());
            }
            Err(e) => {
                eprintln!("   异步解析失败: {}", e);
            }
        }
    });
    
    // 等待异步操作完成
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    Ok(())
}

/// 示例3: PNG编码
fn example_png_encoding() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n3. PNG编码");
    
    // 创建新的PNG图像
    let mut png = PNG::new();
    
    // 设置图像属性
    png.set_width(320);
    png.set_height(200);
    png.set_bit_depth(8);
    png.set_color_type(COLORTYPE_COLOR_ALPHA);
    
    // 创建像素数据 (渐变效果)
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
    fs::write("output_basic.png", png_data)?;
    println!("   PNG文件已保存: output_basic.png");
    
    Ok(())
}

/// 示例4: 像素操作
fn example_pixel_operations() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n4. 像素操作");
    
    // 创建测试PNG
    let test_png_data = create_test_png_data();
    let png_sync = PNGSync::new();
    let mut png = png_sync.read(&test_png_data)?;
    
    // 获取像素数据
    let mut pixels = png.get_rgba8_array();
    println!("   原始像素数据长度: {}", pixels.len());
    
    // 修改像素数据 (添加红色滤镜)
    for i in (0..pixels.len()).step_by(4) {
        if i + 3 < pixels.len() {
            pixels[i] = pixels[i].saturating_add(50); // 增加红色
        }
    }
    
    // 设置修改后的像素数据
    png.set_rgba8_array(pixels)?;
    
    // 获取特定像素
    if let Ok(pixel) = png.get_pixel(0, 0) {
        println!("   左上角像素: R={}, G={}, B={}, A={}", 
                 pixel[0], pixel[1], pixel[2], pixel[3]);
    }
    
    // 设置特定像素
    png.set_pixel(0, 0, [255, 0, 0, 255])?; // 设置为红色
    println!("   设置左上角像素为红色");
    
    Ok(())
}

/// 示例5: 图像属性
fn example_image_properties() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n5. 图像属性");
    
    // 创建测试PNG
    let test_png_data = create_test_png_data();
    let png_sync = PNGSync::new();
    let png = png_sync.read(&test_png_data)?;
    
    // 显示所有图像属性
    println!("   图像尺寸: {}x{}", png.get_width(), png.get_height());
    println!("   位深度: {}", png.get_bit_depth());
    println!("   颜色类型: {}", png.get_color_type());
    println!("   压缩方法: {}", png.get_compression_method());
    println!("   滤镜方法: {}", png.get_filter_method());
    println!("   交错方法: {}", png.get_interlace_method());
    println!("   是否交错: {}", png.is_interlaced());
    println!("   是否可读: {}", png.readable());
    println!("   是否可写: {}", png.writable());
    
    // 显示调色板信息
    if let Some(palette) = png.get_palette() {
        println!("   调色板长度: {}", palette.len());
    } else {
        println!("   无调色板");
    }
    
    // 显示Gamma信息
    println!("   Gamma值: {}", png.get_gamma());
    println!("   是否有Alpha: {}", png.get_alpha());
    
    Ok(())
}

/// 创建测试PNG数据
fn create_test_png_data() -> Vec<u8> {
    // 创建一个简单的1x1像素PNG数据
    // 这是一个最小的有效PNG文件
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
