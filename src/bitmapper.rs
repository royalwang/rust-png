//! 位图映射器模块
//! 实现PNG数据到像素数据的转换，匹配原始pngjs库的bitmapper.js

use crate::constants::*;
use crate::interlace::*;

/// 位图映射器
pub struct Bitmapper {
    width: u32,
    height: u32,
    color_type: u8,
    bit_depth: u8,
    palette: Option<Vec<u8>>,
    trans_color: Option<Vec<u16>>,
}

impl Bitmapper {
    pub fn new(width: u32, height: u32, color_type: u8, bit_depth: u8) -> Self {
        Self {
            width,
            height,
            color_type,
            bit_depth,
            palette: None,
            trans_color: None,
        }
    }
    
    pub fn set_palette(&mut self, palette: Vec<u8>) {
        self.palette = Some(palette);
    }
    
    pub fn set_trans_color(&mut self, trans_color: Vec<u16>) {
        self.trans_color = Some(trans_color);
    }
    
    /// 映射像素数据
    pub fn map_pixels(&self, data: &[u8], interlace: bool) -> Result<Vec<u8>, String> {
        if interlace {
            self.map_interlaced_pixels(data)
        } else {
            self.map_sequential_pixels(data)
        }
    }
    
    /// 映射顺序像素数据
    fn map_sequential_pixels(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let bytes_per_pixel = self.get_bytes_per_pixel();
        let bytes_per_row = (self.width as usize * bytes_per_pixel) as usize;
        let mut output = vec![0; (self.width * self.height * 4) as usize];
        
        for y in 0..self.height {
            let row_start = (y * bytes_per_row) as usize;
            let row_end = row_start + bytes_per_row;
            
            if row_end > data.len() {
                return Err("Insufficient data for row".to_string());
            }
            
            let row_data = &data[row_start..row_end];
            let output_start = (y * self.width * 4) as usize;
            
            self.map_row(row_data, &mut output[output_start..], y as usize)?;
        }
        
        Ok(output)
    }
    
    /// 映射交错像素数据
    fn map_interlaced_pixels(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let passes = get_interlace_passes(self.width, self.height);
        let mut output = vec![0; (self.width * self.height * 4) as usize];
        let mut data_offset = 0;
        
        for pass in &passes {
            let pass_data = &data[data_offset..data_offset + pass.data_size];
            data_offset += pass.data_size;
            
            self.map_interlace_pass(pass, pass_data, &mut output)?;
        }
        
        Ok(output)
    }
    
    /// 映射交错通道
    fn map_interlace_pass(&self, pass: &InterlacePass, data: &[u8], output: &mut [u8]) -> Result<(), String> {
        let bytes_per_pixel = self.get_bytes_per_pixel();
        let bytes_per_row = (pass.width * bytes_per_pixel) as usize;
        
        for y in 0..pass.height {
            let row_start = (y * bytes_per_row) as usize;
            let row_end = row_start + bytes_per_row;
            
            if row_end > data.len() {
                return Err("Insufficient data for interlace pass row".to_string());
            }
            
            let row_data = &data[row_start..row_end];
            let output_y = pass.y_offset + y * pass.y_step;
            let output_start = (output_y * self.width * 4 + pass.x_offset * 4) as usize;
            
            for x in 0..pass.width {
                let input_pos = x * bytes_per_pixel;
                let output_pos = output_start + x * pass.x_step * 4;
                
                if input_pos + bytes_per_pixel <= row_data.len() && output_pos + 3 < output.len() {
                    self.map_pixel(&row_data[input_pos..input_pos + bytes_per_pixel], &mut output[output_pos..])?;
                }
            }
        }
        
        Ok(())
    }
    
    /// 映射行数据
    fn map_row(&self, row_data: &[u8], output: &mut [u8], _y: usize) -> Result<(), String> {
        let bytes_per_pixel = self.get_bytes_per_pixel();
        
        for x in 0..self.width as usize {
            let input_pos = x * bytes_per_pixel;
            let output_pos = x * 4;
            
            if input_pos + bytes_per_pixel <= row_data.len() && output_pos + 3 < output.len() {
                self.map_pixel(&row_data[input_pos..input_pos + bytes_per_pixel], &mut output[output_pos..])?;
            }
        }
        
        Ok(())
    }
    
    /// 映射单个像素
    fn map_pixel(&self, pixel_data: &[u8], output: &mut [u8]) -> Result<(), String> {
        match self.color_type {
            COLORTYPE_GRAYSCALE => self.map_grayscale_pixel(pixel_data, output),
            COLORTYPE_COLOR => self.map_rgb_pixel(pixel_data, output),
            COLORTYPE_PALETTE_COLOR => self.map_palette_pixel(pixel_data, output),
            COLORTYPE_GRAYSCALE | COLORTYPE_ALPHA => self.map_grayscale_alpha_pixel(pixel_data, output),
            COLORTYPE_COLOR_ALPHA => self.map_rgba_pixel(pixel_data, output),
            _ => Err("Unsupported color type".to_string()),
        }
    }
    
    /// 映射灰度像素
    fn map_grayscale_pixel(&self, pixel_data: &[u8], output: &mut [u8]) -> Result<(), String> {
        let gray = self.scale_pixel_value(pixel_data[0]);
        output[0] = gray;
        output[1] = gray;
        output[2] = gray;
        output[3] = 0xff;
        Ok(())
    }
    
    /// 映射RGB像素
    fn map_rgb_pixel(&self, pixel_data: &[u8], output: &mut [u8]) -> Result<(), String> {
        if pixel_data.len() < 3 {
            return Err("Insufficient data for RGB pixel".to_string());
        }
        
        output[0] = self.scale_pixel_value(pixel_data[0]);
        output[1] = self.scale_pixel_value(pixel_data[1]);
        output[2] = self.scale_pixel_value(pixel_data[2]);
        output[3] = 0xff;
        Ok(())
    }
    
    /// 映射调色板像素
    fn map_palette_pixel(&self, pixel_data: &[u8], output: &mut [u8]) -> Result<(), String> {
        let index = pixel_data[0] as usize;
        
        if let Some(ref palette) = self.palette {
            if index * 3 + 2 < palette.len() {
                output[0] = palette[index * 3];
                output[1] = palette[index * 3 + 1];
                output[2] = palette[index * 3 + 2];
                output[3] = 0xff;
            } else {
                return Err("Palette index out of bounds".to_string());
            }
        } else {
            return Err("No palette available".to_string());
        }
        
        // 处理透明度
        if let Some(ref trans_color) = self.trans_color {
            if index < trans_color.len() {
                output[3] = trans_color[index] as u8;
            }
        }
        
        Ok(())
    }
    
    /// 映射灰度+Alpha像素
    fn map_grayscale_alpha_pixel(&self, pixel_data: &[u8], output: &mut [u8]) -> Result<(), String> {
        if pixel_data.len() < 2 {
            return Err("Insufficient data for grayscale+alpha pixel".to_string());
        }
        
        let gray = self.scale_pixel_value(pixel_data[0]);
        let alpha = self.scale_pixel_value(pixel_data[1]);
        
        output[0] = gray;
        output[1] = gray;
        output[2] = gray;
        output[3] = alpha;
        Ok(())
    }
    
    /// 映射RGBA像素
    fn map_rgba_pixel(&self, pixel_data: &[u8], output: &mut [u8]) -> Result<(), String> {
        if pixel_data.len() < 4 {
            return Err("Insufficient data for RGBA pixel".to_string());
        }
        
        output[0] = self.scale_pixel_value(pixel_data[0]);
        output[1] = self.scale_pixel_value(pixel_data[1]);
        output[2] = self.scale_pixel_value(pixel_data[2]);
        output[3] = self.scale_pixel_value(pixel_data[3]);
        Ok(())
    }
    
    /// 缩放像素值
    fn scale_pixel_value(&self, value: u8) -> u8 {
        match self.bit_depth {
            1 => (value & 0x80) >> 7,
            2 => (value & 0xc0) >> 6,
            4 => (value & 0xf0) >> 4,
            8 => value,
            16 => value, // 简化处理，实际应该处理16位数据
            _ => value,
        }
    }
    
    /// 获取每像素字节数
    fn get_bytes_per_pixel(&self) -> usize {
        match self.color_type {
            COLORTYPE_GRAYSCALE => 1,
            COLORTYPE_COLOR => 3,
            COLORTYPE_PALETTE_COLOR => 1,
            COLORTYPE_GRAYSCALE | COLORTYPE_ALPHA => 2,
            COLORTYPE_COLOR_ALPHA => 4,
            _ => 4,
        }
    }
}

/// 位深度转换器
pub struct BitDepthConverter {
    input_depth: u8,
    output_depth: u8,
}

impl BitDepthConverter {
    pub fn new(input_depth: u8, output_depth: u8) -> Self {
        Self {
            input_depth,
            output_depth,
        }
    }
    
    /// 转换位深度
    pub fn convert(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        if self.input_depth == self.output_depth {
            return Ok(data.to_vec());
        }
        
        match (self.input_depth, self.output_depth) {
            (1, 8) => self.convert_1_to_8(data, width, height),
            (2, 8) => self.convert_2_to_8(data, width, height),
            (4, 8) => self.convert_4_to_8(data, width, height),
            (8, 1) => self.convert_8_to_1(data, width, height),
            (8, 2) => self.convert_8_to_2(data, width, height),
            (8, 4) => self.convert_8_to_4(data, width, height),
            (8, 16) => self.convert_8_to_16(data, width, height),
            (16, 8) => self.convert_16_to_8(data, width, height),
            _ => Err(format!("Unsupported bit depth conversion: {} to {}", self.input_depth, self.output_depth)),
        }
    }
    
    /// 1位转8位
    fn convert_1_to_8(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        let bytes_per_row = (width + 7) / 8;
        
        for y in 0..height {
            for x in 0..width {
                let byte_index = (y * bytes_per_row + x / 8) as usize;
                let bit_index = 7 - (x % 8);
                
                if byte_index < data.len() {
                    let bit = (data[byte_index] >> bit_index) & 1;
                    output.push(if bit == 1 { 255 } else { 0 });
                }
            }
        }
        
        Ok(output)
    }
    
    /// 2位转8位
    fn convert_2_to_8(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        let bytes_per_row = (width * 2 + 7) / 8;
        
        for y in 0..height {
            for x in 0..width {
                let byte_index = (y * bytes_per_row + x / 4) as usize;
                let bit_index = 6 - (x % 4) * 2;
                
                if byte_index < data.len() {
                    let value = (data[byte_index] >> bit_index) & 3;
                    output.push((value * 85) as u8); // 0, 85, 170, 255
                }
            }
        }
        
        Ok(output)
    }
    
    /// 4位转8位
    fn convert_4_to_8(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        let bytes_per_row = (width * 4 + 7) / 8;
        
        for y in 0..height {
            for x in 0..width {
                let byte_index = (y * bytes_per_row + x / 2) as usize;
                let bit_index = 4 - (x % 2) * 4;
                
                if byte_index < data.len() {
                    let value = (data[byte_index] >> bit_index) & 15;
                    output.push((value * 17) as u8); // 0, 17, 34, ..., 255
                }
            }
        }
        
        Ok(output)
    }
    
    /// 8位转1位
    fn convert_8_to_1(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        let bytes_per_row = (width + 7) / 8;
        
        for y in 0..height {
            let mut byte = 0u8;
            let mut bit_count = 0;
            
            for x in 0..width {
                let pixel_index = (y * width + x) as usize;
                if pixel_index < data.len() {
                    let bit = if data[pixel_index] > 127 { 1 } else { 0 };
                    byte |= bit << (7 - bit_count);
                    bit_count += 1;
                    
                    if bit_count == 8 {
                        output.push(byte);
                        byte = 0;
                        bit_count = 0;
                    }
                }
            }
            
            if bit_count > 0 {
                output.push(byte);
            }
        }
        
        Ok(output)
    }
    
    /// 8位转2位
    fn convert_8_to_2(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        let bytes_per_row = (width * 2 + 7) / 8;
        
        for y in 0..height {
            let mut byte = 0u8;
            let mut bit_count = 0;
            
            for x in 0..width {
                let pixel_index = (y * width + x) as usize;
                if pixel_index < data.len() {
                    let value = (data[pixel_index] / 85) & 3; // 0-3
                    byte |= value << (6 - bit_count);
                    bit_count += 2;
                    
                    if bit_count == 8 {
                        output.push(byte);
                        byte = 0;
                        bit_count = 0;
                    }
                }
            }
            
            if bit_count > 0 {
                output.push(byte);
            }
        }
        
        Ok(output)
    }
    
    /// 8位转4位
    fn convert_8_to_4(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        let bytes_per_row = (width * 4 + 7) / 8;
        
        for y in 0..height {
            let mut byte = 0u8;
            let mut bit_count = 0;
            
            for x in 0..width {
                let pixel_index = (y * width + x) as usize;
                if pixel_index < data.len() {
                    let value = (data[pixel_index] / 17) & 15; // 0-15
                    byte |= value << (4 - bit_count);
                    bit_count += 4;
                    
                    if bit_count == 8 {
                        output.push(byte);
                        byte = 0;
                        bit_count = 0;
                    }
                }
            }
            
            if bit_count > 0 {
                output.push(byte);
            }
        }
        
        Ok(output)
    }
    
    /// 8位转16位
    fn convert_8_to_16(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        
        for &value in data {
            let scaled = (value as u16 * 65535 / 255) as u16;
            output.extend_from_slice(&scaled.to_be_bytes());
        }
        
        Ok(output)
    }
    
    /// 16位转8位
    fn convert_16_to_8(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        
        for chunk in data.chunks_exact(2) {
            let value = u16::from_be_bytes([chunk[0], chunk[1]]);
            let scaled = (value * 255 / 65535) as u8;
            output.push(scaled);
        }
        
        Ok(output)
    }
}

/// 调色板处理器
pub struct PaletteProcessor {
    palette: Vec<[u8; 3]>,
    trans_color: Option<Vec<u8>>,
}

impl PaletteProcessor {
    pub fn new(palette: Vec<[u8; 3]>) -> Self {
        Self {
            palette,
            trans_color: None,
        }
    }
    
    pub fn set_trans_color(&mut self, trans_color: Vec<u8>) {
        self.trans_color = Some(trans_color);
    }
    
    /// 处理调色板像素
    pub fn process_palette_pixel(&self, index: u8) -> Result<[u8; 4], String> {
        let idx = index as usize;
        
        if idx >= self.palette.len() {
            return Err("Palette index out of bounds".to_string());
        }
        
        let [r, g, b] = self.palette[idx];
        let mut alpha = 255u8;
        
        if let Some(ref trans_color) = self.trans_color {
            if idx < trans_color.len() {
                alpha = trans_color[idx];
            }
        }
        
        Ok([r, g, b, alpha])
    }
    
    /// 处理调色板数据
    pub fn process_palette_data(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        
        for &index in data {
            let [r, g, b, a] = self.process_palette_pixel(index)?;
            output.push(r);
            output.push(g);
            output.push(b);
            output.push(a);
        }
        
        Ok(output)
    }
    
    /// 获取调色板大小
    pub fn get_palette_size(&self) -> usize {
        self.palette.len()
    }
    
    /// 获取调色板颜色
    pub fn get_palette_color(&self, index: usize) -> Option<[u8; 3]> {
        if index < self.palette.len() {
            Some(self.palette[index])
        } else {
            None
        }
    }
}
