//! PNG交错处理模块
//! 实现Adam7交错算法，匹配原始pngjs库的interlace.js

use crate::constants::*;

/// Adam7交错通道信息
#[derive(Debug, Clone)]
pub struct InterlacePass {
    pub pass: usize,
    pub width: u32,
    pub height: u32,
    pub x_offset: u32,
    pub y_offset: u32,
    pub x_step: u32,
    pub y_step: u32,
    pub data_size: usize,
}

/// Adam7交错模式
const ADAM7_PATTERN: [[[u8; 8]; 8]; 7] = [
    // Pass 1
    [
        [1, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ],
    // Pass 2
    [
        [0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ],
    // Pass 3
    [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ],
    // Pass 4
    [
        [0, 0, 1, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ],
    // Pass 5
    [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 1, 0, 1, 0, 1, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 1, 0, 1, 0, 1, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ],
    // Pass 6
    [
        [0, 1, 0, 1, 0, 1, 0, 1],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 1, 0, 1, 0, 1],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 1, 0, 1, 0, 1],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 1, 0, 1, 0, 1, 0, 1],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ],
    // Pass 7
    [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [1, 1, 1, 1, 1, 1, 1, 1],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [1, 1, 1, 1, 1, 1, 1, 1],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [1, 1, 1, 1, 1, 1, 1, 1],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [1, 1, 1, 1, 1, 1, 1, 1],
    ],
];

/// 获取所有交错通道信息
pub fn get_interlace_passes(width: u32, height: u32) -> Vec<InterlacePass> {
    let mut passes = Vec::new();
    
    for pass in 0..7 {
        let (pass_width, pass_height) = get_interlace_pass_size(width, height, pass);
        if pass_width > 0 && pass_height > 0 {
            let (x_offset, y_offset, x_step, y_step) = get_interlace_offsets(pass);
            let data_size = (pass_width * pass_height) as usize;
            passes.push(InterlacePass {
                pass,
                width: pass_width,
                height: pass_height,
                x_offset,
                y_offset,
                x_step,
                y_step,
                data_size,
            });
        }
    }
    
    passes
}

/// 获取交错通道的偏移量和步长
fn get_interlace_offsets(pass: usize) -> (u32, u32, u32, u32) {
    match pass {
        0 => (0, 0, 8, 8),  // pass 1: 1px
        1 => (4, 0, 8, 8),  // pass 2: 1px
        2 => (0, 4, 4, 8),  // pass 3: 2px
        3 => (2, 0, 4, 4),  // pass 4: 4px
        4 => (0, 2, 2, 4),  // pass 5: 8px
        5 => (1, 0, 2, 2),  // pass 6: 16px
        6 => (0, 1, 1, 2),  // pass 7: 32px
        _ => (0, 0, 1, 1),
    }
}

/// 检查像素是否属于指定通道
pub fn is_pixel_in_pass(x: u32, y: u32, pass: usize) -> bool {
    if pass >= 7 {
        return false;
    }
    
    let (x_offset, y_offset, x_step, y_step) = get_interlace_offsets(pass);
    
    (x % 8 == x_offset) && (y % 8 == y_offset)
}

/// 获取像素在交错通道中的位置
pub fn get_pixel_position_in_pass(x: u32, y: u32, pass: usize) -> Option<(u32, u32)> {
    if !is_pixel_in_pass(x, y, pass) {
        return None;
    }
    
    let (x_offset, y_offset, x_step, y_step) = get_interlace_offsets(pass);
    
    let pass_x = (x - x_offset) / x_step;
    let pass_y = (y - y_offset) / y_step;
    
    Some((pass_x, pass_y))
}

/// 从交错通道重构完整图像
pub fn deinterlace_passes(passes: &[Vec<u8>], width: u32, height: u32, bytes_per_pixel: usize) -> Vec<u8> {
    let mut result = vec![0; (width * height * bytes_per_pixel) as usize];
    
    for pass in 0..7 {
        if pass >= passes.len() {
            continue;
        }
        
        let pass_data = &passes[pass];
        let (pass_width, pass_height) = get_interlace_pass_size(width, height, pass);
        
        if pass_width == 0 || pass_height == 0 {
            continue;
        }
        
        let (x_offset, y_offset, x_step, y_step) = get_interlace_offsets(pass);
        
        for y in 0..pass_height {
            for x in 0..pass_width {
                let source_idx = (y * pass_width + x) * bytes_per_pixel as u32;
                let target_x = x_offset + x * x_step;
                let target_y = y_offset + y * y_step;
                
                if target_x < width && target_y < height {
                    let target_idx = (target_y * width + target_x) * bytes_per_pixel as u32;
                    
                    if source_idx + bytes_per_pixel as u32 <= pass_data.len() as u32 && 
                       target_idx + bytes_per_pixel as u32 <= result.len() as u32 {
                        for i in 0..bytes_per_pixel {
                            result[(target_idx + i as u32) as usize] = pass_data[(source_idx + i as u32) as usize];
                        }
                    }
                }
            }
        }
    }
    
    result
}

/// 将完整图像分解为交错通道
pub fn interlace_image(image_data: &[u8], width: u32, height: u32, bytes_per_pixel: usize) -> Vec<Vec<u8>> {
    let mut passes = Vec::new();
    
    for pass in 0..7 {
        let (pass_width, pass_height) = get_interlace_pass_size(width, height, pass);
        
        if pass_width == 0 || pass_height == 0 {
            passes.push(Vec::new());
            continue;
        }
        
        let mut pass_data = vec![0; (pass_width * pass_height * bytes_per_pixel as u32) as usize];
        let (x_offset, y_offset, x_step, y_step) = get_interlace_offsets(pass);
        
        for y in 0..pass_height {
            for x in 0..pass_width {
                let source_x = x_offset + x * x_step;
                let source_y = y_offset + y * y_step;
                
                if source_x < width && source_y < height {
                let source_idx = (source_y * width + source_x) * bytes_per_pixel as u32;
                let target_idx = (y * pass_width + x) * bytes_per_pixel as u32;
                    
                    if source_idx + bytes_per_pixel as u32 <= image_data.len() as u32 && 
                       target_idx + bytes_per_pixel as u32 <= pass_data.len() as u32 {
                        for i in 0..bytes_per_pixel {
                            pass_data[(target_idx + i as u32) as usize] = image_data[(source_idx + i as u32) as usize];
                        }
                    }
                }
            }
        }
        
        passes.push(pass_data);
    }
    
    passes
}

/// 计算交错通道的字节宽度
pub fn calculate_interlace_byte_width(width: u32, bits_per_pixel: u8) -> u32 {
    let bytes_per_pixel = if bits_per_pixel <= 8 { 1 } else { 2 };
    ((width * bits_per_pixel as u32 + 7) / 8) * bytes_per_pixel
}

/// 获取交错通道的像素位置
pub fn get_interlace_pixel_positions(width: u32, height: u32, pass: usize) -> Vec<(u32, u32)> {
    let mut positions = Vec::new();
    let (x_offset, y_offset, x_step, y_step) = get_interlace_offsets(pass);
    
    for y in (y_offset..height).step_by(y_step as usize) {
        for x in (x_offset..width).step_by(x_step as usize) {
            positions.push((x, y));
        }
    }
    
    positions
}

/// 验证交错图像数据
pub fn validate_interlace_data(passes: &[Vec<u8>], width: u32, height: u32, bytes_per_pixel: usize) -> bool {
    for (pass, pass_data) in passes.iter().enumerate() {
        let (pass_width, pass_height) = get_interlace_pass_size(width, height, pass);
        
        if pass_width == 0 || pass_height == 0 {
            if !pass_data.is_empty() {
                return false;
            }
            continue;
        }
        
        let expected_size = (pass_width * pass_height * bytes_per_pixel as u32) as usize;
        if pass_data.len() != expected_size {
            return false;
        }
    }
    
    true
}

/// 获取交错通道的统计信息
pub fn get_interlace_stats(width: u32, height: u32) -> InterlaceStats {
    let mut total_pixels = 0;
    let mut pass_sizes = Vec::new();
    
    for pass in 0..7 {
        let (pass_width, pass_height) = get_interlace_pass_size(width, height, pass);
        let pass_pixels = pass_width * pass_height;
        total_pixels += pass_pixels;
        pass_sizes.push(pass_pixels);
    }
    
    InterlaceStats {
        total_passes: 7,
        total_pixels,
        pass_sizes,
        compression_ratio: if total_pixels > 0 { 
            (width * height) as f64 / total_pixels as f64 
        } else { 
            1.0 
        },
    }
}

/// 交错统计信息
#[derive(Debug, Clone)]
pub struct InterlaceStats {
    pub total_passes: usize,
    pub total_pixels: u32,
    pub pass_sizes: Vec<u32>,
    pub compression_ratio: f64,
}
