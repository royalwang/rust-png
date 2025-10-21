//! 位图处理模块
//! 匹配原始pngjs库的bitmapper.js和format-normaliser.js

use crate::constants::*;

/// 位深度转换函数
pub fn scale_depth(input: &[u8], output: &mut [u8], width: usize, height: usize, input_depth: u8, output_depth: u8) {
    let input_bpp = if input_depth <= 8 { 1 } else { 2 };
    let output_bpp = if output_depth <= 8 { 1 } else { 2 };
    
    let max_input = (1u16 << input_depth) - 1;
    let max_output = (1u16 << output_depth) - 1;
    
    for y in 0..height {
        for x in 0..width {
            let input_idx = (y * width + x) * input_bpp;
            let output_idx = (y * width + x) * output_bpp;
            
            if input_depth <= 8 && output_depth <= 8 {
                // 8位到8位
                let value = input[input_idx] as u16;
                let scaled = (value * max_output / max_input) as u8;
                output[output_idx] = scaled;
            } else if input_depth <= 8 && output_depth > 8 {
                // 8位到16位
                let value = input[input_idx] as u16;
                let scaled = (value * max_output / max_input) as u16;
                output[output_idx] = (scaled >> 8) as u8;
                output[output_idx + 1] = (scaled & 0xff) as u8;
            } else if input_depth > 8 && output_depth <= 8 {
                // 16位到8位
                let value = ((input[input_idx] as u16) << 8) | (input[input_idx + 1] as u16);
                let scaled = (value * max_output / max_input) as u8;
                output[output_idx] = scaled;
            } else {
                // 16位到16位
                let value = ((input[input_idx] as u16) << 8) | (input[input_idx + 1] as u16);
                let scaled = (value * max_output / max_input) as u16;
                output[output_idx] = (scaled >> 8) as u8;
                output[output_idx + 1] = (scaled & 0xff) as u8;
            }
        }
    }
}

/// 调色板处理函数
pub fn de_palette(input: &[u8], output: &mut [u8], width: usize, height: usize, palette: &[u8]) {
    let mut px_pos = 0;
    
    for _y in 0..height {
        for _x in 0..width {
            let color_idx = input[px_pos] as usize * 3;
            
            if color_idx + 2 < palette.len() {
                output[px_pos * 4] = palette[color_idx];     // R
                output[px_pos * 4 + 1] = palette[color_idx + 1]; // G
                output[px_pos * 4 + 2] = palette[color_idx + 2]; // B
                output[px_pos * 4 + 3] = 255; // A
            } else {
                output[px_pos * 4] = 0;
                output[px_pos * 4 + 1] = 0;
                output[px_pos * 4 + 2] = 0;
                output[px_pos * 4 + 3] = 255;
            }
            
            px_pos += 1;
        }
    }
}

/// 透明度颜色处理
pub fn replace_transparent_color(input: &[u8], output: &mut [u8], width: usize, height: usize, trans_color: &[u16]) {
    let mut px_pos = 0;
    
    for _y in 0..height {
        for _x in 0..width {
            let mut make_trans = false;
            
            if trans_color.len() == 1 {
                // 灰度透明度
                if trans_color[0] == input[px_pos] as u16 {
                    make_trans = true;
                }
            } else if trans_color.len() == 3 {
                // RGB透明度
                if px_pos + 2 < input.len() &&
                   trans_color[0] == input[px_pos] as u16 &&
                   trans_color[1] == input[px_pos + 1] as u16 &&
                   trans_color[2] == input[px_pos + 2] as u16 {
                    make_trans = true;
                }
            }
            
            if make_trans {
                output[px_pos * 4] = 0;
                output[px_pos * 4 + 1] = 0;
                output[px_pos * 4 + 2] = 0;
                output[px_pos * 4 + 3] = 0;
            } else {
                output[px_pos * 4] = input[px_pos];
                output[px_pos * 4 + 1] = if px_pos + 1 < input.len() { input[px_pos + 1] } else { input[px_pos] };
                output[px_pos * 4 + 2] = if px_pos + 2 < input.len() { input[px_pos + 2] } else { input[px_pos] };
                output[px_pos * 4 + 3] = 255;
            }
            
            px_pos += 1;
        }
    }
}

/// 将PNG数据转换为RGBA格式
pub fn convert_to_rgba(data: &[u8], width: usize, height: usize, color_type: u8, bit_depth: u8, palette: Option<&[u8]>) -> Vec<u8> {
    let mut rgba = Vec::with_capacity(width * height * 4);
    
    match (color_type, bit_depth) {
        (COLORTYPE_GRAYSCALE, 8) => {
            // 灰度 8-bit
            for &gray in data {
                rgba.push(gray); // R
                rgba.push(gray); // G
                rgba.push(gray); // B
                rgba.push(255);  // A
            }
        }
        (COLORTYPE_GRAYSCALE | COLORTYPE_ALPHA, 8) => {
            // 灰度 + Alpha 8-bit
            for chunk in data.chunks_exact(2) {
                let gray = chunk[0];
                let alpha = chunk[1];
                rgba.push(gray); // R
                rgba.push(gray); // G
                rgba.push(gray); // B
                rgba.push(alpha); // A
            }
        }
        (COLORTYPE_COLOR, 8) => {
            // RGB 8-bit
            for chunk in data.chunks_exact(3) {
                rgba.push(chunk[0]); // R
                rgba.push(chunk[1]); // G
                rgba.push(chunk[2]); // B
                rgba.push(255);      // A
            }
        }
        (COLORTYPE_COLOR_ALPHA, 8) => {
            // RGBA 8-bit
            rgba.extend_from_slice(data);
        }
        (COLORTYPE_PALETTE_COLOR, 8) => {
            // 调色板 8-bit
            if let Some(palette) = palette {
                for &index in data {
                    let palette_index = (index as usize) * 3;
                    if palette_index + 2 < palette.len() {
                        rgba.push(palette[palette_index]);     // R
                        rgba.push(palette[palette_index + 1]); // G
                        rgba.push(palette[palette_index + 2]); // B
                        rgba.push(255);                        // A
                    } else {
                        rgba.extend_from_slice(&[0, 0, 0, 255]); // Default black
                    }
                }
            }
        }
        (COLORTYPE_COLOR, 16) => {
            // RGB 16-bit -> 8-bit
            for chunk in data.chunks_exact(6) {
                rgba.push(chunk[0]); // R (high byte)
                rgba.push(chunk[2]); // G (high byte)
                rgba.push(chunk[4]); // B (high byte)
                rgba.push(255);      // A
            }
        }
        (COLORTYPE_COLOR_ALPHA, 16) => {
            // RGBA 16-bit -> 8-bit
            for chunk in data.chunks_exact(8) {
                rgba.push(chunk[0]); // R (high byte)
                rgba.push(chunk[2]); // G (high byte)
                rgba.push(chunk[4]); // B (high byte)
                rgba.push(chunk[6]); // A (high byte)
            }
        }
        _ => {
            // 不支持的格式，返回黑色像素
            rgba.resize(width * height * 4, 0);
        }
    }
    
    rgba
}

/// 获取每像素字节数
pub fn get_bytes_per_pixel(color_type: u8) -> usize {
    match color_type {
        COLORTYPE_GRAYSCALE => 1,
        COLORTYPE_COLOR => 3,
        COLORTYPE_PALETTE_COLOR => 1,
        COLORTYPE_GRAYSCALE | COLORTYPE_ALPHA => 2,
        COLORTYPE_COLOR_ALPHA => 4,
        _ => 4, // 默认RGBA
    }
}
