//! PNG语义化结构设计
//! 使用组合型结构提供更富有语义的PNG表示

use wasm_bindgen::prelude::*;
use js_sys::{Array, Uint8Array, Uint8ClampedArray, Object};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// PNG图像尺寸信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageDimensions {
    pub width: u32,
    pub height: u32,
    pub aspect_ratio: f64,
}

impl ImageDimensions {
    pub fn new(width: u32, height: u32) -> Self {
        let aspect_ratio = if height > 0 { width as f64 / height as f64 } else { 1.0 };
        Self { width, height, aspect_ratio }
    }
    
    pub fn total_pixels(&self) -> u64 {
        self.width as u64 * self.height as u64
    }
    
    pub fn is_square(&self) -> bool {
        (self.aspect_ratio - 1.0).abs() < 0.01
    }
    
    pub fn is_landscape(&self) -> bool {
        self.aspect_ratio > 1.0
    }
    
    pub fn is_portrait(&self) -> bool {
        self.aspect_ratio < 1.0
    }
}

/// PNG颜色信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorInfo {
    pub color_type: ColorType,
    pub bit_depth: BitDepth,
    pub has_alpha: bool,
    pub has_transparency: bool,
    pub channels: u8,
    pub bytes_per_pixel: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColorType {
    Grayscale = 0,
    Palette = 1,
    RGB = 2,
    GrayscaleAlpha = 4,
    RGBA = 6,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BitDepth {
    One = 1,
    Two = 2,
    Four = 4,
    Eight = 8,
    Sixteen = 16,
}

impl ColorInfo {
    pub fn new(color_type: u8, bit_depth: u8) -> Self {
        let color_type_enum = match color_type {
            0 => ColorType::Grayscale,
            1 => ColorType::Palette,
            2 => ColorType::RGB,
            4 => ColorType::GrayscaleAlpha,
            6 => ColorType::RGBA,
            _ => ColorType::RGB,
        };
        
        let bit_depth_enum = match bit_depth {
            1 => BitDepth::One,
            2 => BitDepth::Two,
            4 => BitDepth::Four,
            8 => BitDepth::Eight,
            16 => BitDepth::Sixteen,
            _ => BitDepth::Eight,
        };
        
        let has_alpha = matches!(color_type_enum, ColorType::GrayscaleAlpha | ColorType::RGBA);
        let channels = match color_type_enum {
            ColorType::Grayscale => 1,
            ColorType::Palette => 1,
            ColorType::RGB => 3,
            ColorType::GrayscaleAlpha => 2,
            ColorType::RGBA => 4,
        };
        
        let bytes_per_pixel = match bit_depth_enum {
            BitDepth::One => 1,
            BitDepth::Two => 1,
            BitDepth::Four => 1,
            BitDepth::Eight => channels,
            BitDepth::Sixteen => channels * 2,
        };
        
        Self {
            color_type: color_type_enum,
            bit_depth: bit_depth_enum,
            has_alpha,
            has_transparency: false, // 将在解析时设置
            channels,
            bytes_per_pixel,
        }
    }
    
    pub fn supports_transparency(&self) -> bool {
        self.has_alpha || self.has_transparency
    }
    
    pub fn is_indexed(&self) -> bool {
        matches!(self.color_type, ColorType::Palette)
    }
    
    pub fn is_truecolor(&self) -> bool {
        matches!(self.color_type, ColorType::RGB | ColorType::RGBA)
    }
}

/// PNG压缩信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionInfo {
    pub method: CompressionMethod,
    pub filter: FilterMethod,
    pub level: u8,
    pub estimated_ratio: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompressionMethod {
    Deflate = 0,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterMethod {
    None = 0,
    Sub = 1,
    Up = 2,
    Average = 3,
    Paeth = 4,
}

impl CompressionInfo {
    pub fn new(method: u8, filter: u8) -> Self {
        let method_enum = match method {
            0 => CompressionMethod::Deflate,
            _ => CompressionMethod::Deflate,
        };
        
        let filter_enum = match filter {
            0 => FilterMethod::None,
            1 => FilterMethod::Sub,
            2 => FilterMethod::Up,
            3 => FilterMethod::Average,
            4 => FilterMethod::Paeth,
            _ => FilterMethod::None,
        };
        
        Self {
            method: method_enum,
            filter: filter_enum,
            level: 6, // 默认压缩级别
            estimated_ratio: 0.0,
        }
    }
}

/// PNG交错信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterlaceInfo {
    pub method: InterlaceMethod,
    pub is_interlaced: bool,
    pub passes: u8,
    pub progressive_loading: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterlaceMethod {
    None = 0,
    Adam7 = 1,
}

impl InterlaceInfo {
    pub fn new(method: u8) -> Self {
        let method_enum = match method {
            0 => InterlaceMethod::None,
            1 => InterlaceMethod::Adam7,
            _ => InterlaceMethod::None,
        };
        
        let is_interlaced = matches!(method_enum, InterlaceMethod::Adam7);
        let passes = if is_interlaced { 7 } else { 1 };
        
        Self {
            method: method_enum,
            is_interlaced,
            passes,
            progressive_loading: is_interlaced,
        }
    }
}

/// PNG调色板信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaletteInfo {
    pub colors: Vec<PaletteColor>,
    pub color_count: u32,
    pub has_transparency: bool,
    pub transparency_colors: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaletteColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl PaletteInfo {
    pub fn new(palette_data: &[u8], transparency_data: Option<&[u8]>) -> Self {
        let mut colors = Vec::new();
        let color_count = palette_data.len() / 3;
        
        for i in 0..color_count {
            let red = palette_data[i * 3];
            let green = palette_data[i * 3 + 1];
            let blue = palette_data[i * 3 + 2];
            let alpha = if let Some(trans_data) = transparency_data {
                if i < trans_data.len() {
                    trans_data[i]
                } else {
                    255
                }
            } else {
                255
            };
            
            colors.push(PaletteColor { red, green, blue, alpha });
        }
        
        let has_transparency = transparency_data.is_some();
        let transparency_colors = transparency_data.map(|d| d.to_vec()).unwrap_or_default();
        
        Self {
            colors,
            color_count: color_count as u32,
            has_transparency,
            transparency_colors,
        }
    }
    
    pub fn get_color(&self, index: usize) -> Option<&PaletteColor> {
        self.colors.get(index)
    }
    
    pub fn find_color(&self, r: u8, g: u8, b: u8) -> Option<usize> {
        self.colors.iter().position(|c| c.red == r && c.green == g && c.blue == b)
    }
}

/// PNG Gamma信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GammaInfo {
    pub gamma: f64,
    pub has_gamma: bool,
    pub gamma_corrected: bool,
}

impl GammaInfo {
    pub fn new(gamma: f64) -> Self {
        Self {
            gamma,
            has_gamma: gamma > 0.0,
            gamma_corrected: false,
        }
    }
    
    pub fn apply_gamma_correction(&mut self) {
        if self.has_gamma && !self.gamma_corrected {
            self.gamma_corrected = true;
        }
    }
}

/// PNG元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PNGMetadata {
    pub dimensions: ImageDimensions,
    pub color_info: ColorInfo,
    pub compression: CompressionInfo,
    pub interlace: InterlaceInfo,
    pub palette: Option<PaletteInfo>,
    pub gamma: GammaInfo,
    pub chunks: HashMap<String, Vec<u8>>,
    pub creation_time: Option<String>,
    pub software: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
}

impl PNGMetadata {
    pub fn new(width: u32, height: u32, color_type: u8, bit_depth: u8) -> Self {
        Self {
            dimensions: ImageDimensions::new(width, height),
            color_info: ColorInfo::new(color_type, bit_depth),
            compression: CompressionInfo::new(0, 0),
            interlace: InterlaceInfo::new(0),
            palette: None,
            gamma: GammaInfo::new(0.0),
            chunks: HashMap::new(),
            creation_time: None,
            software: None,
            title: None,
            author: None,
            description: None,
        }
    }
    
    pub fn add_chunk(&mut self, chunk_type: String, data: Vec<u8>) {
        self.chunks.insert(chunk_type, data);
    }
    
    pub fn get_chunk(&self, chunk_type: &str) -> Option<&Vec<u8>> {
        self.chunks.get(chunk_type)
    }
    
    pub fn has_chunk(&self, chunk_type: &str) -> bool {
        self.chunks.contains_key(chunk_type)
    }
}

/// PNG像素数据管理
#[derive(Debug, Clone)]
pub struct PixelData {
    pub raw_data: Vec<u8>,
    pub rgba_data: Option<Vec<u8>>,
    pub data_format: DataFormat,
    pub is_modified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataFormat {
    Raw,
    RGBA,
    RGB,
    Grayscale,
    Indexed,
}

impl PixelData {
    pub fn new(raw_data: Vec<u8>, format: DataFormat) -> Self {
        Self {
            raw_data,
            rgba_data: None,
            data_format: format,
            is_modified: false,
        }
    }
    
    pub fn get_rgba_data(&mut self, metadata: &PNGMetadata) -> &Vec<u8> {
        if self.rgba_data.is_none() {
            self.convert_to_rgba(metadata);
        }
        self.rgba_data.as_ref().unwrap()
    }
    
    fn convert_to_rgba(&mut self, metadata: &PNGMetadata) {
        // 这里应该实现实际的格式转换逻辑
        // 简化实现
        self.rgba_data = Some(self.raw_data.clone());
        self.data_format = DataFormat::RGBA;
    }
    
    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8, metadata: &PNGMetadata) -> Result<(), String> {
        if x >= metadata.dimensions.width || y >= metadata.dimensions.height {
            return Err("Pixel coordinates out of bounds".to_string());
        }
        
        let index = ((y * metadata.dimensions.width + x) * 4) as usize;
        if index + 3 < self.raw_data.len() {
            self.raw_data[index] = r;
            self.raw_data[index + 1] = g;
            self.raw_data[index + 2] = b;
            self.raw_data[index + 3] = a;
            self.is_modified = true;
        }
        
        Ok(())
    }
    
    pub fn get_pixel(&self, x: u32, y: u32, metadata: &PNGMetadata) -> Result<(u8, u8, u8, u8), String> {
        if x >= metadata.dimensions.width || y >= metadata.dimensions.height {
            return Err("Pixel coordinates out of bounds".to_string());
        }
        
        let index = ((y * metadata.dimensions.width + x) * 4) as usize;
        if index + 3 < self.raw_data.len() {
            Ok((
                self.raw_data[index],
                self.raw_data[index + 1],
                self.raw_data[index + 2],
                self.raw_data[index + 3],
            ))
        } else {
            Err("Pixel data out of bounds".to_string())
        }
    }
}

/// PNG操作状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationState {
    pub is_readable: bool,
    pub is_writable: bool,
    pub is_parsed: bool,
    pub is_modified: bool,
    pub has_errors: bool,
    pub error_message: Option<String>,
}

impl OperationState {
    pub fn new() -> Self {
        Self {
            is_readable: true,
            is_writable: true,
            is_parsed: false,
            is_modified: false,
            has_errors: false,
            error_message: None,
        }
    }
    
    pub fn mark_parsed(&mut self) {
        self.is_parsed = true;
    }
    
    pub fn mark_modified(&mut self) {
        self.is_modified = true;
    }
    
    pub fn set_error(&mut self, message: String) {
        self.has_errors = true;
        self.error_message = Some(message);
    }
    
    pub fn clear_errors(&mut self) {
        self.has_errors = false;
        self.error_message = None;
    }
}

/// PNG统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PNGStatistics {
    pub file_size: u64,
    pub pixel_count: u64,
    pub compression_ratio: f64,
    pub color_entropy: f64,
    pub unique_colors: u32,
    pub transparency_ratio: f64,
    pub processing_time: u64, // 微秒
}

impl PNGStatistics {
    pub fn new() -> Self {
        Self {
            file_size: 0,
            pixel_count: 0,
            compression_ratio: 0.0,
            color_entropy: 0.0,
            unique_colors: 0,
            transparency_ratio: 0.0,
            processing_time: 0,
        }
    }
    
    pub fn calculate_from_data(&mut self, metadata: &PNGMetadata, pixel_data: &PixelData) {
        self.pixel_count = metadata.dimensions.total_pixels();
        self.file_size = pixel_data.raw_data.len() as u64;
        
        if self.file_size > 0 {
            self.compression_ratio = (self.pixel_count * metadata.color_info.bytes_per_pixel as u64) as f64 / self.file_size as f64;
        }
        
        // 计算颜色熵
        self.color_entropy = self.calculate_entropy(&pixel_data.raw_data);
        
        // 计算唯一颜色数
        self.unique_colors = self.count_unique_colors(&pixel_data.raw_data, &metadata.color_info);
        
        // 计算透明度比例
        if metadata.color_info.has_alpha {
            self.transparency_ratio = self.calculate_transparency_ratio(&pixel_data.raw_data, &metadata.color_info);
        }
    }
    
    fn calculate_entropy(&self, data: &[u8]) -> f64 {
        let mut histogram = [0u32; 256];
        for &byte in data {
            histogram[byte as usize] += 1;
        }
        
        let total = data.len() as f64;
        let mut entropy = 0.0;
        
        for &count in &histogram {
            if count > 0 {
                let probability = count as f64 / total;
                entropy -= probability * probability.log2();
            }
        }
        
        entropy
    }
    
    fn count_unique_colors(&self, data: &[u8], color_info: &ColorInfo) -> u32 {
        let mut unique_colors = std::collections::HashSet::new();
        let bytes_per_pixel = color_info.bytes_per_pixel as usize;
        
        for chunk in data.chunks_exact(bytes_per_pixel) {
            let color_key = chunk.to_vec();
            unique_colors.insert(color_key);
        }
        
        unique_colors.len() as u32
    }
    
    fn calculate_transparency_ratio(&self, data: &[u8], color_info: &ColorInfo) -> f64 {
        if !color_info.has_alpha {
            return 0.0;
        }
        
        let mut transparent_pixels = 0;
        let bytes_per_pixel = color_info.bytes_per_pixel as usize;
        let alpha_offset = bytes_per_pixel - 1; // Alpha通道在最后
        
        for chunk in data.chunks_exact(bytes_per_pixel) {
            if chunk[alpha_offset] < 255 {
                transparent_pixels += 1;
            }
        }
        
        let total_pixels = data.len() / bytes_per_pixel;
        if total_pixels > 0 {
            transparent_pixels as f64 / total_pixels as f64
        } else {
            0.0
        }
    }
}
