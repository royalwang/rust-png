//! 工具函数模块
//! 包含各种辅助函数

use wasm_bindgen::prelude::*;
use js_sys::{Array, Uint8Array, Uint8ClampedArray};

/// 调试日志宏
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub use console_log;

/// 将Rust Vec<u8>转换为JavaScript Uint8Array
pub fn vec_to_uint8_array(data: &[u8]) -> Uint8Array {
    let array = Uint8Array::new_with_length(data.len() as u32);
    array.copy_from(data);
    array
}

/// 将Rust Vec<u8>转换为JavaScript Uint8ClampedArray
pub fn vec_to_uint8_clamped_array(data: &[u8]) -> Uint8ClampedArray {
    let array = Uint8ClampedArray::new_with_length(data.len() as u32);
    array.copy_from(data);
    array
}

/// 将像素数组转换为JavaScript Array
pub fn pixel_to_array(pixel: [u8; 4]) -> Array {
    let array = Array::new();
    array.push(&JsValue::from(pixel[0])); // R
    array.push(&JsValue::from(pixel[1])); // G
    array.push(&JsValue::from(pixel[2])); // B
    array.push(&JsValue::from(pixel[3])); // A
    array
}

/// 验证PNG文件签名
pub fn validate_png_signature(data: &[u8]) -> bool {
    const PNG_SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];
    
    if data.len() < 8 {
        return false;
    }
    
    data[0..8] == PNG_SIGNATURE
}

/// 计算图像数据大小
pub fn calculate_image_size(width: u32, height: u32, bytes_per_pixel: usize) -> usize {
    (width * height * bytes_per_pixel as u32) as usize
}

/// 检查坐标是否在边界内
pub fn is_coordinate_valid(x: u32, y: u32, width: u32, height: u32) -> bool {
    x < width && y < height
}

/// 计算像素索引
pub fn calculate_pixel_index(x: u32, y: u32, width: u32, bytes_per_pixel: usize) -> usize {
    ((y * width + x) * bytes_per_pixel as u32) as usize
}

/// 安全的数组访问
pub fn safe_array_access<T: Copy>(data: &[T], index: usize, default: T) -> T {
    data.get(index).copied().unwrap_or(default)
}

/// 安全的数组切片
pub fn safe_array_slice<T>(data: &[T], start: usize, end: usize) -> &[T] {
    let start = start.min(data.len());
    let end = end.min(data.len());
    if start >= end {
        &[]
    } else {
        &data[start..end]
    }
}

/// 将u16数组转换为u8数组
pub fn u16_to_u8_array(data: &[u16]) -> Vec<u8> {
    let mut result = Vec::with_capacity(data.len() * 2);
    for &value in data {
        result.push((value >> 8) as u8);
        result.push((value & 0xff) as u8);
    }
    result
}

/// 将u8数组转换为u16数组
pub fn u8_to_u16_array(data: &[u8]) -> Vec<u16> {
    let mut result = Vec::with_capacity(data.len() / 2);
    for chunk in data.chunks_exact(2) {
        let value = ((chunk[0] as u16) << 8) | (chunk[1] as u16);
        result.push(value);
    }
    result
}

/// 计算行字节数
pub fn calculate_row_bytes(width: u32, bits_per_pixel: u8) -> usize {
    ((width * bits_per_pixel as u32 + 7) / 8) as usize
}

/// 计算交错通道的偏移量
pub fn calculate_interlace_offset(x: u32, y: u32, pass: usize) -> (u32, u32) {
    if pass >= 7 {
        return (0, 0);
    }
    
    use crate::constants::{INTERLACE_PASSES, INTERLACE_Y_PASSES};
    
    let x_pass = INTERLACE_PASSES[pass];
    let y_pass = INTERLACE_Y_PASSES[pass];
    
    let mut offset_x = 0;
    let mut offset_y = 0;
    
    for i in 0..8 {
        if x_pass[i] != 0 && (x % 8) == x_pass[i] as u32 {
            offset_x = i as u32;
            break;
        }
    }
    
    for i in 0..8 {
        if y_pass[i] != 0 && (y % 8) == y_pass[i] as u32 {
            offset_y = i as u32;
            break;
        }
    }
    
    (offset_x, offset_y)
}
