//! 高级功能示例
//! 展示Rust PNG Library的高级功能

use rust_png::advanced_png::{AdvancedPNG, AdvancedPNGOptions, PNG16Bit, ColorTypeConverter};
use rust_png::constants::*;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Rust PNG Library 高级功能示例 ===");
    
    // 示例1: 16位PNG处理
    example_16bit_png()?;
    
    // 示例2: 颜色类型转换
    example_color_type_conversion()?;
    
    // 示例3: 高级PNG处理
    example_advanced_png_processing()?;
    
    // 示例4: 16位PNG处理器
    example_16bit_processor()?;
    
    // 示例5: 颜色转换器
    example_color_converter()?;
    
    println!("=== 高级功能示例完成 ===");
    Ok(())
}

/// 示例1: 16位PNG处理
fn example_16bit_png() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n1. 16位PNG处理");
    
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
    println!("   16位数据设置成功，数据长度: {}", data_16bit.len());
    
    // 转换颜色类型
    advanced_png.convert_color_type(COLORTYPE_GRAYSCALE)?;
    println!("   颜色类型转换完成: RGBA -> 灰度");
    
    // 打包PNG数据
    let png_data = advanced_png.pack()?;
    fs::write("output_16bit.png", png_data)?;
    println!("   16位PNG文件已保存: output_16bit.png");
    
    Ok(())
}

/// 示例2: 颜色类型转换
fn example_color_type_conversion() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n2. 颜色类型转换");
    
    // 创建RGB数据
    let mut rgb_data = Vec::new();
    for i in 0..1000 {
        let r = (i % 256) as u8;
        let g = ((i * 2) % 256) as u8;
        let b = ((i * 3) % 256) as u8;
        rgb_data.extend_from_slice(&[r, g, b]);
    }
    
    println!("   原始RGB数据长度: {}", rgb_data.len());
    
    // 转换为灰度
    let gray_data = ColorTypeConverter::convert(
        &rgb_data, 
        COLORTYPE_COLOR, 
        COLORTYPE_GRAYSCALE, 
        8
    )?;
    println!("   转换为灰度数据长度: {}", gray_data.len());
    
    // 转换为RGBA
    let rgba_data = ColorTypeConverter::convert(
        &rgb_data, 
        COLORTYPE_COLOR, 
        COLORTYPE_COLOR_ALPHA, 
        8
    )?;
    println!("   转换为RGBA数据长度: {}", rgba_data.len());
    
    // 从RGBA转换回RGB
    let rgb_data_back = ColorTypeConverter::convert(
        &rgba_data, 
        COLORTYPE_COLOR_ALPHA, 
        COLORTYPE_COLOR, 
        8
    )?;
    println!("   从RGBA转换回RGB数据长度: {}", rgb_data_back.len());
    
    Ok(())
}

/// 示例3: 高级PNG处理
fn example_advanced_png_processing() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n3. 高级PNG处理");
    
    // 创建高级PNG选项
    let options = AdvancedPNGOptions {
        width: 160,
        height: 100,
        bit_depth: 8,
        color_type: COLORTYPE_COLOR,
        input_color_type: COLORTYPE_COLOR,
        input_has_alpha: false,
    };
    
    // 创建高级PNG实例
    let mut advanced_png = AdvancedPNG::new(options);
    
    // 创建8位RGB数据
    let mut rgb_data = Vec::new();
    for y in 0..100 {
        for x in 0..160 {
            let r = (x * 255 / 160) as u8;
            let g = (y * 255 / 100) as u8;
            let b = 128u8;
            rgb_data.extend_from_slice(&[r, g, b]);
        }
    }
    
    // 设置8位数据
    advanced_png.set_8bit_data(&rgb_data)?;
    println!("   8位RGB数据设置成功，数据长度: {}", rgb_data.len());
    
    // 转换为RGBA
    advanced_png.convert_color_type(COLORTYPE_COLOR_ALPHA)?;
    println!("   颜色类型转换完成: RGB -> RGBA");
    
    // 获取转换后的数据
    if let Some(data) = advanced_png.get_data() {
        println!("   转换后数据长度: {}", data.len());
    }
    
    // 打包PNG数据
    let png_data = advanced_png.pack()?;
    fs::write("output_advanced.png", png_data)?;
    println!("   高级PNG文件已保存: output_advanced.png");
    
    Ok(())
}

/// 示例4: 16位PNG处理器
fn example_16bit_processor() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n4. 16位PNG处理器");
    
    // 创建16位PNG处理器
    let mut png_16bit = PNG16Bit::new(64, 64, COLORTYPE_COLOR_ALPHA);
    println!("   16位PNG处理器创建成功: 64x64 RGBA");
    
    // 设置像素值
    for y in 0..64 {
        for x in 0..64 {
            let r = (x * 65535 / 64) as u16;
            let g = (y * 65535 / 64) as u16;
            let b = 32768u16;
            let a = 65535u16;
            png_16bit.set_pixel(x, y, &[r, g, b, a])?;
        }
    }
    println!("   像素值设置完成");
    
    // 获取特定像素
    let pixel = png_16bit.get_pixel(32, 32)?;
    println!("   中心像素(32,32): R={}, G={}, B={}, A={}", 
             pixel[0], pixel[1], pixel[2], pixel[3]);
    
    // 转换为字节数组
    let bytes = png_16bit.to_bytes();
    println!("   转换为字节数组，长度: {}", bytes.len());
    
    // 从字节数组创建
    let png_16bit_from_bytes = PNG16Bit::from_bytes(&bytes, 64, 64, COLORTYPE_COLOR_ALPHA)?;
    println!("   从字节数组创建16位PNG处理器成功");
    
    // 验证数据一致性
    let pixel_from_bytes = png_16bit_from_bytes.get_pixel(32, 32)?;
    println!("   从字节数组创建的像素(32,32): R={}, G={}, B={}, A={}", 
             pixel_from_bytes[0], pixel_from_bytes[1], pixel_from_bytes[2], pixel_from_bytes[3]);
    
    Ok(())
}

/// 示例5: 颜色转换器
fn example_color_converter() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n5. 颜色转换器");
    
    // 创建测试数据
    let test_data = vec![
        255, 0, 0,    // 红色
        0, 255, 0,    // 绿色
        0, 0, 255,    // 蓝色
        128, 128, 128, // 灰色
    ];
    
    println!("   原始RGB数据: {:?}", test_data);
    
    // RGB -> 灰度
    let gray_data = ColorTypeConverter::convert(
        &test_data, 
        COLORTYPE_COLOR, 
        COLORTYPE_GRAYSCALE, 
        8
    )?;
    println!("   转换为灰度: {:?}", gray_data);
    
    // RGB -> RGBA
    let rgba_data = ColorTypeConverter::convert(
        &test_data, 
        COLORTYPE_COLOR, 
        COLORTYPE_COLOR_ALPHA, 
        8
    )?;
    println!("   转换为RGBA: {:?}", rgba_data);
    
    // RGBA -> RGB
    let rgb_data = ColorTypeConverter::convert(
        &rgba_data, 
        COLORTYPE_COLOR_ALPHA, 
        COLORTYPE_COLOR, 
        8
    )?;
    println!("   从RGBA转换回RGB: {:?}", rgb_data);
    
    // RGBA -> 灰度
    let gray_from_rgba = ColorTypeConverter::convert(
        &rgba_data, 
        COLORTYPE_COLOR_ALPHA, 
        COLORTYPE_GRAYSCALE, 
        8
    )?;
    println!("   从RGBA转换为灰度: {:?}", gray_from_rgba);
    
    Ok(())
}
