//! PNG滤镜处理模块
//! 匹配原始pngjs库的filter-parse.js和filter-pack.js

use crate::constants::*;

/// Paeth预测器算法
/// 匹配原始pngjs库的paeth-predictor.js
pub fn paeth_predictor(a: u8, b: u8, c: u8) -> u8 {
    let p = (a as i16) + (b as i16) - (c as i16);
    let pa = (p - (a as i16)).abs();
    let pb = (p - (b as i16)).abs();
    let pc = (p - (c as i16)).abs();
    
    if pa <= pb && pa <= pc {
        a
    } else if pb <= pc {
        b
    } else {
        c
    }
}

/// 应用PNG滤镜
pub fn apply_filter(filter_type: u8, data: &mut [u8], width: usize, bpp: usize) {
    let bytes_per_row = width * bpp;
    
    for y in 0..(data.len() / bytes_per_row) {
        let row_start = y * bytes_per_row;
        let row_end = row_start + bytes_per_row;
        
        if row_end > data.len() {
            break;
        }
        
        let row = &mut data[row_start..row_end];
        
        match filter_type {
            FILTER_NONE => {
                // 无滤镜，直接使用原始数据
            }
            FILTER_SUB => {
                // Sub滤镜：当前像素 - 左像素
                for x in bpp..bytes_per_row {
                    row[x] = row[x].wrapping_add(row[x - bpp]);
                }
            }
            FILTER_UP => {
                // Up滤镜：当前像素 - 上像素
                if y > 0 {
                    let prev_row_start = (y - 1) * bytes_per_row;
                    for x in 0..bytes_per_row {
                        row[x] = row[x].wrapping_add(data[prev_row_start + x]);
                    }
                }
            }
            FILTER_AVERAGE => {
                // Average滤镜：当前像素 - (左像素 + 上像素) / 2
                for x in 0..bytes_per_row {
                    let left = if x >= bpp { row[x - bpp] } else { 0 };
                    let up = if y > 0 { data[(y - 1) * bytes_per_row + x] } else { 0 };
                    let average = ((left as u16 + up as u16) / 2) as u8;
                    row[x] = row[x].wrapping_add(average);
                }
            }
            FILTER_PAETH => {
                // Paeth滤镜：使用Paeth预测器
                for x in 0..bytes_per_row {
                    let left = if x >= bpp { row[x - bpp] } else { 0 };
                    let up = if y > 0 { data[(y - 1) * bytes_per_row + x] } else { 0 };
                    let up_left = if y > 0 && x >= bpp { data[(y - 1) * bytes_per_row + x - bpp] } else { 0 };
                    let predictor = paeth_predictor(left, up, up_left);
                    row[x] = row[x].wrapping_add(predictor);
                }
            }
            _ => {
                // 未知滤镜类型，保持原样
            }
        }
    }
}

/// 反向应用PNG滤镜（用于编码）
pub fn reverse_filter(filter_type: u8, data: &mut [u8], width: usize, bpp: usize) {
    let bytes_per_row = width * bpp;
    
    for y in 0..(data.len() / bytes_per_row) {
        let row_start = y * bytes_per_row;
        let row_end = row_start + bytes_per_row;
        
        if row_end > data.len() {
            break;
        }
        
        let row = &mut data[row_start..row_end];
        
        match filter_type {
            FILTER_NONE => {
                // 无滤镜，直接使用原始数据
            }
            FILTER_SUB => {
                // Sub滤镜：当前像素 = 原始像素 + 左像素
                for x in bpp..bytes_per_row {
                    row[x] = row[x].wrapping_sub(row[x - bpp]);
                }
            }
            FILTER_UP => {
                // Up滤镜：当前像素 = 原始像素 + 上像素
                if y > 0 {
                    let prev_row_start = (y - 1) * bytes_per_row;
                    for x in 0..bytes_per_row {
                        row[x] = row[x].wrapping_sub(data[prev_row_start + x]);
                    }
                }
            }
            FILTER_AVERAGE => {
                // Average滤镜：当前像素 = 原始像素 + (左像素 + 上像素) / 2
                for x in 0..bytes_per_row {
                    let left = if x >= bpp { row[x - bpp] } else { 0 };
                    let up = if y > 0 { data[(y - 1) * bytes_per_row + x] } else { 0 };
                    let average = ((left as u16 + up as u16) / 2) as u8;
                    row[x] = row[x].wrapping_sub(average);
                }
            }
            FILTER_PAETH => {
                // Paeth滤镜：使用Paeth预测器
                for x in 0..bytes_per_row {
                    let left = if x >= bpp { row[x - bpp] } else { 0 };
                    let up = if y > 0 { data[(y - 1) * bytes_per_row + x] } else { 0 };
                    let up_left = if y > 0 && x >= bpp { data[(y - 1) * bytes_per_row + x - bpp] } else { 0 };
                    let predictor = paeth_predictor(left, up, up_left);
                    row[x] = row[x].wrapping_sub(predictor);
                }
            }
            _ => {
                // 未知滤镜类型，保持原样
            }
        }
    }
}

/// 选择最佳滤镜类型
pub fn choose_best_filter(data: &[u8], width: usize, bpp: usize) -> u8 {
    let bytes_per_row = width * bpp;
    let mut best_filter = FILTER_NONE;
    let mut best_size = data.len();
    
    for filter_type in [FILTER_NONE, FILTER_SUB, FILTER_UP, FILTER_AVERAGE, FILTER_PAETH] {
        let mut test_data = data.to_vec();
        apply_filter(filter_type, &mut test_data, width, bpp);
        
        // 计算压缩后的大小（这里简化处理）
        let compressed_size = test_data.len();
        
        if compressed_size < best_size {
            best_size = compressed_size;
            best_filter = filter_type;
        }
    }
    
    best_filter
}
