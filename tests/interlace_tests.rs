//! 交错图像处理测试用例

use rust_png::interlace::*;

#[test]
fn test_interlace_pass_calculation() {
    // 测试8x8图像的交错通道计算
    let width = 8;
    let height = 8;
    let passes = get_interlace_passes(width, height);
    
    // 应该有7个通道
    assert_eq!(passes.len(), 7);
    
    // 验证每个通道的尺寸
    assert_eq!(passes[0].width, 1);  // pass 1: 1px
    assert_eq!(passes[0].height, 1);
    
    assert_eq!(passes[1].width, 1);  // pass 2: 1px
    assert_eq!(passes[1].height, 1);
    
    assert_eq!(passes[2].width, 2);  // pass 3: 2px
    assert_eq!(passes[2].height, 1);
    
    assert_eq!(passes[3].width, 2);  // pass 4: 4px
    assert_eq!(passes[3].height, 2);
    
    assert_eq!(passes[4].width, 4);  // pass 5: 8px
    assert_eq!(passes[4].height, 2);
    
    assert_eq!(passes[5].width, 4);  // pass 6: 16px
    assert_eq!(passes[5].height, 4);
    
    assert_eq!(passes[6].width, 8);  // pass 7: 32px
    assert_eq!(passes[6].height, 4);
}

#[test]
fn test_pixel_in_pass() {
    // 测试像素是否属于指定通道
    assert!(is_pixel_in_pass(0, 0, 0));  // pass 1: (0,0)
    assert!(is_pixel_in_pass(4, 0, 1)); // pass 2: (4,0)
    assert!(is_pixel_in_pass(0, 4, 2)); // pass 3: (0,4)
    assert!(is_pixel_in_pass(2, 0, 3)); // pass 4: (2,0)
    assert!(is_pixel_in_pass(0, 2, 4)); // pass 5: (0,2)
    assert!(is_pixel_in_pass(1, 0, 5)); // pass 6: (1,0)
    assert!(is_pixel_in_pass(0, 1, 6)); // pass 7: (0,1)
    
    // 测试不属于通道的像素
    assert!(!is_pixel_in_pass(1, 0, 0)); // (1,0) 不属于 pass 1
    assert!(!is_pixel_in_pass(0, 1, 0)); // (0,1) 不属于 pass 1
}

#[test]
fn test_pixel_position_in_pass() {
    // 测试像素在通道中的位置
    let pos = get_pixel_position_in_pass(0, 0, 0);
    assert_eq!(pos, Some((0, 0)));
    
    let pos = get_pixel_position_in_pass(8, 0, 0);
    assert_eq!(pos, Some((1, 0)));
    
    let pos = get_pixel_position_in_pass(0, 8, 0);
    assert_eq!(pos, Some((0, 1)));
    
    // 测试不属于通道的像素
    let pos = get_pixel_position_in_pass(1, 0, 0);
    assert_eq!(pos, None);
}

#[test]
fn test_interlace_deinterlace() {
    // 创建测试图像数据
    let width = 4;
    let height = 4;
    let bytes_per_pixel = 3; // RGB
    let mut image_data = vec![0; (width * height * bytes_per_pixel) as usize];
    
    // 填充测试数据
    for i in 0..image_data.len() {
        image_data[i] = (i % 256) as u8;
    }
    
    // 分解为交错通道
    let passes = interlace_image(&image_data, width, height, bytes_per_pixel);
    
    // 验证通道数量
    assert_eq!(passes.len(), 7);
    
    // 重构图像
    let reconstructed = deinterlace_passes(&passes, width, height, bytes_per_pixel);
    
    // 验证重构结果
    assert_eq!(reconstructed.len(), image_data.len());
    assert_eq!(reconstructed, image_data);
}

#[test]
fn test_interlace_stats() {
    let width = 16;
    let height = 16;
    let stats = get_interlace_stats(width, height);
    
    assert_eq!(stats.total_passes, 7);
    assert_eq!(stats.pass_sizes.len(), 7);
    
    // 验证总像素数
    let total_pixels: u32 = stats.pass_sizes.iter().sum();
    assert_eq!(total_pixels, width * height);
    
    // 验证压缩比
    assert!(stats.compression_ratio > 0.0);
    assert!(stats.compression_ratio <= 1.0);
}

#[test]
fn test_validate_interlace_data() {
    let width = 8;
    let height = 8;
    let bytes_per_pixel = 3;
    
    // 创建有效的交错数据
    let mut passes = Vec::new();
    for pass in 0..7 {
        let (pass_width, pass_height) = get_interlace_pass_size(width, height, pass);
        if pass_width > 0 && pass_height > 0 {
            let size = (pass_width * pass_height * bytes_per_pixel) as usize;
            passes.push(vec![0; size]);
        } else {
            passes.push(Vec::new());
        }
    }
    
    assert!(validate_interlace_data(&passes, width, height, bytes_per_pixel));
    
    // 测试无效数据
    let mut invalid_passes = passes.clone();
    invalid_passes[0].push(0); // 添加额外字节
    
    assert!(!validate_interlace_data(&invalid_passes, width, height, bytes_per_pixel));
}

#[test]
fn test_interlace_byte_width() {
    let width = 8;
    
    // 测试不同位深度
    assert_eq!(calculate_interlace_byte_width(width, 1), 1);   // 1位
    assert_eq!(calculate_interlace_byte_width(width, 2), 2);   // 2位
    assert_eq!(calculate_interlace_byte_width(width, 4), 4);   // 4位
    assert_eq!(calculate_interlace_byte_width(width, 8), 8);    // 8位
    assert_eq!(calculate_interlace_byte_width(width, 16), 16);  // 16位
}

#[test]
fn test_interlace_pixel_positions() {
    let width = 8;
    let height = 8;
    
    // 测试pass 0的像素位置
    let positions = get_interlace_pixel_positions(width, height, 0);
    assert_eq!(positions.len(), 1);
    assert_eq!(positions[0], (0, 0));
    
    // 测试pass 1的像素位置
    let positions = get_interlace_pixel_positions(width, height, 1);
    assert_eq!(positions.len(), 1);
    assert_eq!(positions[0], (4, 0));
    
    // 测试pass 6的像素位置
    let positions = get_interlace_pixel_positions(width, height, 6);
    assert_eq!(positions.len(), 16); // 4x4 = 16个像素
}

#[test]
fn test_large_image_interlace() {
    // 测试大图像的交错处理
    let width = 64;
    let height = 64;
    let bytes_per_pixel = 4; // RGBA
    
    let mut image_data = vec![0; (width * height * bytes_per_pixel) as usize];
    
    // 填充测试数据
    for i in 0..image_data.len() {
        image_data[i] = (i % 256) as u8;
    }
    
    // 分解为交错通道
    let passes = interlace_image(&image_data, width, height, bytes_per_pixel);
    
    // 重构图像
    let reconstructed = deinterlace_passes(&passes, width, height, bytes_per_pixel);
    
    // 验证重构结果
    assert_eq!(reconstructed.len(), image_data.len());
    assert_eq!(reconstructed, image_data);
    
    // 验证统计信息
    let stats = get_interlace_stats(width, height);
    assert_eq!(stats.total_passes, 7);
    
    let total_pixels: u32 = stats.pass_sizes.iter().sum();
    assert_eq!(total_pixels, width * height);
}
