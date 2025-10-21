//! 高级PNG功能模块
//! 实现高级PNG处理功能，包括16位处理、颜色类型转换等

use crate::constants::*;
use crate::bitmapper::*;
use crate::png_chunks::*;

/// 高级PNG处理器
pub struct AdvancedPNG {
    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: u8,
    input_color_type: u8,
    input_has_alpha: bool,
    data: Option<Vec<u8>>,
}

impl AdvancedPNG {
    pub fn new(options: AdvancedPNGOptions) -> Self {
        Self {
            width: options.width,
            height: options.height,
            bit_depth: options.bit_depth,
            color_type: options.color_type,
            input_color_type: options.input_color_type,
            input_has_alpha: options.input_has_alpha,
            data: None,
        }
    }
    
    /// 设置16位数据
    pub fn set_16bit_data(&mut self, data: &[u16]) -> Result<(), String> {
        if self.bit_depth != 16 {
            return Err("Bit depth must be 16 for 16-bit data".to_string());
        }
        
        let bytes_per_pixel = self.get_bytes_per_pixel();
        let expected_size = (self.width * self.height * bytes_per_pixel) as usize;
        
        if data.len() * 2 != expected_size {
            return Err("Data size mismatch".to_string());
        }
        
        // 转换16位数据为字节数组
        let mut byte_data = Vec::new();
        for &value in data {
            byte_data.extend_from_slice(&value.to_be_bytes());
        }
        
        self.data = Some(byte_data);
        Ok(())
    }
    
    /// 设置8位数据
    pub fn set_8bit_data(&mut self, data: &[u8]) -> Result<(), String> {
        if self.bit_depth != 8 {
            return Err("Bit depth must be 8 for 8-bit data".to_string());
        }
        
        let bytes_per_pixel = self.get_bytes_per_pixel();
        let expected_size = (self.width * self.height * bytes_per_pixel) as usize;
        
        if data.len() != expected_size {
            return Err("Data size mismatch".to_string());
        }
        
        self.data = Some(data.to_vec());
        Ok(())
    }
    
    /// 转换颜色类型
    pub fn convert_color_type(&mut self, target_color_type: u8) -> Result<(), String> {
        if self.color_type == target_color_type {
            return Ok(());
        }
        
        let data = self.data.as_ref().ok_or("No data to convert")?;
        let converted_data = self.perform_color_conversion(data, target_color_type)?;
        
        self.color_type = target_color_type;
        self.data = Some(converted_data);
        Ok(())
    }
    
    /// 执行颜色转换
    fn perform_color_conversion(&self, data: &[u8], target_color_type: u8) -> Result<Vec<u8>, String> {
        match (self.color_type, target_color_type) {
            (COLORTYPE_GRAYSCALE, COLORTYPE_COLOR) => {
                self.grayscale_to_rgb(data)
            }
            (COLORTYPE_COLOR, COLORTYPE_GRAYSCALE) => {
                self.rgb_to_grayscale(data)
            }
            (COLORTYPE_GRAYSCALE, COLORTYPE_COLOR_ALPHA) => {
                self.grayscale_to_rgba(data)
            }
            (COLORTYPE_COLOR, COLORTYPE_COLOR_ALPHA) => {
                self.rgb_to_rgba(data)
            }
            (COLORTYPE_COLOR_ALPHA, COLORTYPE_COLOR) => {
                self.rgba_to_rgb(data)
            }
            (COLORTYPE_COLOR_ALPHA, COLORTYPE_GRAYSCALE) => {
                self.rgba_to_grayscale(data)
            }
            _ => Err("Unsupported color type conversion".to_string()),
        }
    }
    
    /// 灰度转RGB
    fn grayscale_to_rgb(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        let bytes_per_pixel = if self.bit_depth == 16 { 2 } else { 1 };
        
        for chunk in data.chunks_exact(bytes_per_pixel) {
            let gray = if self.bit_depth == 16 {
                u16::from_be_bytes([chunk[0], chunk[1]]) as u8
            } else {
                chunk[0]
            };
            
            output.push(gray); // R
            output.push(gray); // G
            output.push(gray); // B
        }
        
        Ok(output)
    }
    
    /// RGB转灰度
    fn rgb_to_grayscale(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        let bytes_per_pixel = if self.bit_depth == 16 { 6 } else { 3 };
        
        for chunk in data.chunks_exact(bytes_per_pixel) {
            let (r, g, b) = if self.bit_depth == 16 {
                let r = u16::from_be_bytes([chunk[0], chunk[1]]);
                let g = u16::from_be_bytes([chunk[2], chunk[3]]);
                let b = u16::from_be_bytes([chunk[4], chunk[5]]);
                (r, g, b)
            } else {
                (chunk[0] as u16, chunk[1] as u16, chunk[2] as u16)
            };
            
            // 使用标准灰度转换公式
            let gray = (0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64) as u16;
            
            if self.bit_depth == 16 {
                output.extend_from_slice(&gray.to_be_bytes());
            } else {
                output.push((gray >> 8) as u8);
            }
        }
        
        Ok(output)
    }
    
    /// 灰度转RGBA
    fn grayscale_to_rgba(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        let bytes_per_pixel = if self.bit_depth == 16 { 2 } else { 1 };
        
        for chunk in data.chunks_exact(bytes_per_pixel) {
            let gray = if self.bit_depth == 16 {
                u16::from_be_bytes([chunk[0], chunk[1]]) as u8
            } else {
                chunk[0]
            };
            
            output.push(gray); // R
            output.push(gray); // G
            output.push(gray); // B
            output.push(255);  // A
        }
        
        Ok(output)
    }
    
    /// RGB转RGBA
    fn rgb_to_rgba(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        let bytes_per_pixel = if self.bit_depth == 16 { 6 } else { 3 };
        
        for chunk in data.chunks_exact(bytes_per_pixel) {
            output.extend_from_slice(chunk);
            output.push(255); // A
        }
        
        Ok(output)
    }
    
    /// RGBA转RGB
    fn rgba_to_rgb(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        let bytes_per_pixel = if self.bit_depth == 16 { 8 } else { 4 };
        
        for chunk in data.chunks_exact(bytes_per_pixel) {
            // 跳过alpha通道
            let rgb_bytes = if self.bit_depth == 16 { 6 } else { 3 };
            output.extend_from_slice(&chunk[..rgb_bytes]);
        }
        
        Ok(output)
    }
    
    /// RGBA转灰度
    fn rgba_to_grayscale(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        let bytes_per_pixel = if self.bit_depth == 16 { 8 } else { 4 };
        
        for chunk in data.chunks_exact(bytes_per_pixel) {
            let (r, g, b) = if self.bit_depth == 16 {
                let r = u16::from_be_bytes([chunk[0], chunk[1]]);
                let g = u16::from_be_bytes([chunk[2], chunk[3]]);
                let b = u16::from_be_bytes([chunk[4], chunk[5]]);
                (r, g, b)
            } else {
                (chunk[0] as u16, chunk[1] as u16, chunk[2] as u16)
            };
            
            // 使用标准灰度转换公式
            let gray = (0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64) as u16;
            
            if self.bit_depth == 16 {
                output.extend_from_slice(&gray.to_be_bytes());
            } else {
                output.push((gray >> 8) as u8);
            }
        }
        
        Ok(output)
    }
    
    /// 获取每像素字节数
    fn get_bytes_per_pixel(&self) -> u32 {
        match self.color_type {
            COLORTYPE_GRAYSCALE => if self.bit_depth == 16 { 2 } else { 1 },
            COLORTYPE_COLOR => if self.bit_depth == 16 { 6 } else { 3 },
            COLORTYPE_PALETTE_COLOR => 1,
            COLORTYPE_GRAYSCALE | COLORTYPE_ALPHA => if self.bit_depth == 16 { 4 } else { 2 },
            COLORTYPE_COLOR_ALPHA => if self.bit_depth == 16 { 8 } else { 4 },
            _ => 4,
        }
    }
    
    /// 打包PNG数据
    pub fn pack(&self) -> Result<Vec<u8>, String> {
        let data = self.data.as_ref().ok_or("No data to pack")?;
        
        // 使用位图映射器处理数据
        let bitmapper = Bitmapper::new(
            self.width,
            self.height,
            self.color_type,
            self.bit_depth,
        );
        
        let pixels = bitmapper.map_pixels(data, false)?;
        
        // 使用PNG打包器打包
        let options = crate::png_packer::PackerOptions {
            width: self.width,
            height: self.height,
            bit_depth: self.bit_depth,
            color_type: self.color_type,
            input_color_type: self.input_color_type,
            input_has_alpha: self.input_has_alpha,
            ..Default::default()
        };
        
        let packer = crate::png_packer::PNGPacker::new(options);
        packer.pack(&pixels)
    }
    
    /// 获取数据
    pub fn get_data(&self) -> Option<&Vec<u8>> {
        self.data.as_ref()
    }
    
    /// 设置数据
    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = Some(data);
    }
}

/// 高级PNG选项
#[derive(Debug, Clone)]
pub struct AdvancedPNGOptions {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: u8,
    pub input_color_type: u8,
    pub input_has_alpha: bool,
}

impl Default for AdvancedPNGOptions {
    fn default() -> Self {
        Self {
            width: 0,
            height: 0,
            bit_depth: 8,
            color_type: COLORTYPE_COLOR_ALPHA,
            input_color_type: COLORTYPE_COLOR_ALPHA,
            input_has_alpha: true,
        }
    }
}

/// 16位PNG处理器
pub struct PNG16Bit {
    width: u32,
    height: u32,
    color_type: u8,
    data: Vec<u16>,
}

impl PNG16Bit {
    pub fn new(width: u32, height: u32, color_type: u8) -> Self {
        let bytes_per_pixel = match color_type {
            COLORTYPE_GRAYSCALE => 1,
            COLORTYPE_COLOR => 3,
            COLORTYPE_PALETTE_COLOR => 1,
            COLORTYPE_GRAYSCALE | COLORTYPE_ALPHA => 2,
            COLORTYPE_COLOR_ALPHA => 4,
            _ => 4,
        };
        
        let data_size = (width * height * bytes_per_pixel) as usize;
        
        Self {
            width,
            height,
            color_type,
            data: vec![0; data_size],
        }
    }
    
    /// 设置像素值
    pub fn set_pixel(&mut self, x: u32, y: u32, values: &[u16]) -> Result<(), String> {
        if x >= self.width || y >= self.height {
            return Err("Pixel coordinates out of bounds".to_string());
        }
        
        let bytes_per_pixel = self.get_bytes_per_pixel();
        if values.len() != bytes_per_pixel as usize {
            return Err("Invalid number of color components".to_string());
        }
        
        let index = (y * self.width * bytes_per_pixel + x * bytes_per_pixel) as usize;
        
        for (i, &value) in values.iter().enumerate() {
            self.data[index + i] = value;
        }
        
        Ok(())
    }
    
    /// 获取像素值
    pub fn get_pixel(&self, x: u32, y: u32) -> Result<Vec<u16>, String> {
        if x >= self.width || y >= self.height {
            return Err("Pixel coordinates out of bounds".to_string());
        }
        
        let bytes_per_pixel = self.get_bytes_per_pixel();
        let index = (y * self.width * bytes_per_pixel + x * bytes_per_pixel) as usize;
        
        let mut values = Vec::new();
        for i in 0..bytes_per_pixel as usize {
            values.push(self.data[index + i]);
        }
        
        Ok(values)
    }
    
    /// 获取每像素字节数
    fn get_bytes_per_pixel(&self) -> u32 {
        match self.color_type {
            COLORTYPE_GRAYSCALE => 1,
            COLORTYPE_COLOR => 3,
            COLORTYPE_PALETTE_COLOR => 1,
            COLORTYPE_GRAYSCALE | COLORTYPE_ALPHA => 2,
            COLORTYPE_COLOR_ALPHA => 4,
            _ => 4,
        }
    }
    
    /// 转换为字节数组
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for &value in &self.data {
            bytes.extend_from_slice(&value.to_be_bytes());
        }
        bytes
    }
    
    /// 从字节数组创建
    pub fn from_bytes(data: &[u8], width: u32, height: u32, color_type: u8) -> Result<Self, String> {
        if data.len() % 2 != 0 {
            return Err("Data length must be even for 16-bit data".to_string());
        }
        
        let mut png_16bit = Self::new(width, height, color_type);
        
        for (i, chunk) in data.chunks_exact(2).enumerate() {
            if i < png_16bit.data.len() {
                png_16bit.data[i] = u16::from_be_bytes([chunk[0], chunk[1]]);
            }
        }
        
        Ok(png_16bit)
    }
}

/// 颜色类型转换器
pub struct ColorTypeConverter;

impl ColorTypeConverter {
    /// 转换颜色类型
    pub fn convert(data: &[u8], from_type: u8, to_type: u8, bit_depth: u8) -> Result<Vec<u8>, String> {
        if from_type == to_type {
            return Ok(data.to_vec());
        }
        
        match (from_type, to_type) {
            (COLORTYPE_GRAYSCALE, COLORTYPE_COLOR) => {
                Self::grayscale_to_rgb(data, bit_depth)
            }
            (COLORTYPE_COLOR, COLORTYPE_GRAYSCALE) => {
                Self::rgb_to_grayscale(data, bit_depth)
            }
            (COLORTYPE_GRAYSCALE, COLORTYPE_COLOR_ALPHA) => {
                Self::grayscale_to_rgba(data, bit_depth)
            }
            (COLORTYPE_COLOR, COLORTYPE_COLOR_ALPHA) => {
                Self::rgb_to_rgba(data, bit_depth)
            }
            (COLORTYPE_COLOR_ALPHA, COLORTYPE_COLOR) => {
                Self::rgba_to_rgb(data, bit_depth)
            }
            (COLORTYPE_COLOR_ALPHA, COLORTYPE_GRAYSCALE) => {
                Self::rgba_to_grayscale(data, bit_depth)
            }
            _ => Err("Unsupported color type conversion".to_string()),
        }
    }
    
    fn grayscale_to_rgb(data: &[u8], bit_depth: u8) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        let bytes_per_pixel = if bit_depth == 16 { 2 } else { 1 };
        
        for chunk in data.chunks_exact(bytes_per_pixel) {
            let gray = if bit_depth == 16 {
                u16::from_be_bytes([chunk[0], chunk[1]]) as u8
            } else {
                chunk[0]
            };
            
            output.push(gray);
            output.push(gray);
            output.push(gray);
        }
        
        Ok(output)
    }
    
    fn rgb_to_grayscale(data: &[u8], bit_depth: u8) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        let bytes_per_pixel = if bit_depth == 16 { 6 } else { 3 };
        
        for chunk in data.chunks_exact(bytes_per_pixel) {
            let (r, g, b) = if bit_depth == 16 {
                let r = u16::from_be_bytes([chunk[0], chunk[1]]);
                let g = u16::from_be_bytes([chunk[2], chunk[3]]);
                let b = u16::from_be_bytes([chunk[4], chunk[5]]);
                (r, g, b)
            } else {
                (chunk[0] as u16, chunk[1] as u16, chunk[2] as u16)
            };
            
            let gray = (0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64) as u16;
            
            if bit_depth == 16 {
                output.extend_from_slice(&gray.to_be_bytes());
            } else {
                output.push((gray >> 8) as u8);
            }
        }
        
        Ok(output)
    }
    
    fn grayscale_to_rgba(data: &[u8], bit_depth: u8) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        let bytes_per_pixel = if bit_depth == 16 { 2 } else { 1 };
        
        for chunk in data.chunks_exact(bytes_per_pixel) {
            let gray = if bit_depth == 16 {
                u16::from_be_bytes([chunk[0], chunk[1]]) as u8
            } else {
                chunk[0]
            };
            
            output.push(gray);
            output.push(gray);
            output.push(gray);
            output.push(255);
        }
        
        Ok(output)
    }
    
    fn rgb_to_rgba(data: &[u8], bit_depth: u8) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        let bytes_per_pixel = if bit_depth == 16 { 6 } else { 3 };
        
        for chunk in data.chunks_exact(bytes_per_pixel) {
            output.extend_from_slice(chunk);
            output.push(255);
        }
        
        Ok(output)
    }
    
    fn rgba_to_rgb(data: &[u8], bit_depth: u8) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        let bytes_per_pixel = if bit_depth == 16 { 8 } else { 4 };
        
        for chunk in data.chunks_exact(bytes_per_pixel) {
            let rgb_bytes = if bit_depth == 16 { 6 } else { 3 };
            output.extend_from_slice(&chunk[..rgb_bytes]);
        }
        
        Ok(output)
    }
    
    fn rgba_to_grayscale(data: &[u8], bit_depth: u8) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        let bytes_per_pixel = if bit_depth == 16 { 8 } else { 4 };
        
        for chunk in data.chunks_exact(bytes_per_pixel) {
            let (r, g, b) = if bit_depth == 16 {
                let r = u16::from_be_bytes([chunk[0], chunk[1]]);
                let g = u16::from_be_bytes([chunk[2], chunk[3]]);
                let b = u16::from_be_bytes([chunk[4], chunk[5]]);
                (r, g, b)
            } else {
                (chunk[0] as u16, chunk[1] as u16, chunk[2] as u16)
            };
            
            let gray = (0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64) as u16;
            
            if bit_depth == 16 {
                output.extend_from_slice(&gray.to_be_bytes());
            } else {
                output.push((gray >> 8) as u8);
            }
        }
        
        Ok(output)
    }
}
