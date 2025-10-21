//! PNG打包器模块
//! 实现PNG编码和打包功能，匹配原始pngjs库的packer.js

use std::io::{Write, Cursor};
use flate2::write::DeflateEncoder;
use flate2::Compression;
use crate::constants::*;
use crate::crc::crc32;
use crate::filter_extensible::*;
use crate::bitmap::*;

/// PNG打包选项
#[derive(Debug, Clone)]
pub struct PackerOptions {
    pub deflate_chunk_size: usize,
    pub deflate_level: u8,
    pub deflate_strategy: u8,
    pub input_has_alpha: bool,
    pub bit_depth: u8,
    pub color_type: u8,
    pub input_color_type: u8,
    pub width: u32,
    pub height: u32,
}

impl Default for PackerOptions {
    fn default() -> Self {
        Self {
            deflate_chunk_size: 32 * 1024,
            deflate_level: 9,
            deflate_strategy: 3,
            input_has_alpha: true,
            bit_depth: 8,
            color_type: COLORTYPE_COLOR_ALPHA,
            input_color_type: COLORTYPE_COLOR_ALPHA,
            width: 0,
            height: 0,
        }
    }
}

/// PNG打包器
pub struct PNGPacker {
    options: PackerOptions,
}

impl PNGPacker {
    pub fn new(options: PackerOptions) -> Self {
        Self { options }
    }
    
    /// 打包PNG数据
    pub fn pack(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut output = Vec::new();
        
        // 写入PNG签名
        output.extend_from_slice(&PNG_SIGNATURE);
        
        // 写入IHDR chunk
        self.write_ihdr_chunk(&mut output)?;
        
        // 处理像素数据
        let processed_data = self.process_pixel_data(data)?;
        
        // 写入IDAT chunks
        self.write_idat_chunks(&mut output, &processed_data)?;
        
        // 写入IEND chunk
        self.write_iend_chunk(&mut output)?;
        
        Ok(output)
    }
    
    /// 写入IHDR chunk
    fn write_ihdr_chunk(&self, output: &mut Vec<u8>) -> Result<(), String> {
        let mut ihdr_data = Vec::new();
        
        // 写入IHDR数据
        ihdr_data.write_all(&self.options.width.to_be_bytes()).map_err(|e| e.to_string())?;
        ihdr_data.write_all(&self.options.height.to_be_bytes()).map_err(|e| e.to_string())?;
        ihdr_data.write_all(&[self.options.bit_depth]).map_err(|e| e.to_string())?;
        ihdr_data.write_all(&[self.options.color_type]).map_err(|e| e.to_string())?;
        ihdr_data.write_all(&[0])?; // compression method
        ihdr_data.write_all(&[0])?; // filter method
        ihdr_data.write_all(&[0])?; // interlace method
        
        // 写入chunk
        self.write_chunk(output, TYPE_IHDR, &ihdr_data)?;
        
        Ok(())
    }
    
    /// 处理像素数据
    fn process_pixel_data(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let bytes_per_row = self.calculate_bytes_per_row();
        let mut processed_data = Vec::new();
        
        // 按行处理数据
        for y in 0..self.options.height {
            let row_start = (y * bytes_per_row) as usize;
            let row_end = row_start + bytes_per_row;
            
            if row_end > data.len() {
                return Err("Insufficient pixel data".to_string());
            }
            
            let row_data = &data[row_start..row_end];
            
            // 选择最佳滤镜
            let best_filter = self.choose_best_filter(row_data, y as usize);
            processed_data.push(best_filter);
            
            // 应用滤镜
            let filtered_row = self.apply_filter(row_data, best_filter, y as usize)?;
            processed_data.extend_from_slice(&filtered_row);
        }
        
        // 压缩数据
        self.compress_data(&processed_data)
    }
    
    /// 计算每行字节数
    fn calculate_bytes_per_row(&self) -> u32 {
        let bits_per_pixel = match self.options.color_type {
            COLORTYPE_GRAYSCALE => self.options.bit_depth,
            COLORTYPE_COLOR => self.options.bit_depth * 3,
            COLORTYPE_PALETTE_COLOR => self.options.bit_depth,
            COLORTYPE_GRAYSCALE | COLORTYPE_ALPHA => self.options.bit_depth * 2,
            COLORTYPE_COLOR_ALPHA => self.options.bit_depth * 4,
            _ => 8,
        };
        
        ((self.options.width * bits_per_pixel as u32 + 7) / 8) as u32
    }
    
    /// 选择最佳滤镜
    fn choose_best_filter(&self, row_data: &[u8], row_index: usize) -> u8 {
        let context = FilterContext {
            width: self.options.width as usize,
            height: self.options.height as usize,
            bytes_per_pixel: self.get_bytes_per_pixel(),
            row_index,
            column_index: 0,
            previous_row: None,
        };
        
        let processor = FilterProcessor::new();
        processor.choose_best_filter(row_data, &context).unwrap_or(FILTER_NONE)
    }
    
    /// 获取每像素字节数
    fn get_bytes_per_pixel(&self) -> usize {
        match self.options.color_type {
            COLORTYPE_GRAYSCALE => 1,
            COLORTYPE_COLOR => 3,
            COLORTYPE_PALETTE_COLOR => 1,
            COLORTYPE_GRAYSCALE | COLORTYPE_ALPHA => 2,
            COLORTYPE_COLOR_ALPHA => 4,
            _ => 4,
        }
    }
    
    /// 应用滤镜
    fn apply_filter(&self, row_data: &[u8], filter_type: u8, row_index: usize) -> Result<Vec<u8>, String> {
        let mut filtered_data = row_data.to_vec();
        let context = FilterContext {
            width: self.options.width as usize,
            height: self.options.height as usize,
            bytes_per_pixel: self.get_bytes_per_pixel(),
            row_index,
            column_index: 0,
            previous_row: None,
        };
        
        let processor = FilterProcessor::new();
        processor.apply_filter(filter_type, &mut filtered_data, &context)?;
        
        Ok(filtered_data)
    }
    
    /// 压缩数据
    fn compress_data(&self, data: &[u8]) -> Result<Vec<u8>, String> {
        let mut encoder = DeflateEncoder::new(Vec::new(), Compression::new(self.options.deflate_level as u32));
        encoder.write_all(data).map_err(|e| e.to_string())?;
        encoder.finish().map_err(|e| e.to_string())
    }
    
    /// 写入IDAT chunks
    fn write_idat_chunks(&self, output: &mut Vec<u8>, data: &[u8]) -> Result<(), String> {
        let chunk_size = self.options.deflate_chunk_size;
        
        for chunk in data.chunks(chunk_size) {
            self.write_chunk(output, TYPE_IDAT, chunk)?;
        }
        
        Ok(())
    }
    
    /// 写入IEND chunk
    fn write_iend_chunk(&self, output: &mut Vec<u8>) -> Result<(), String> {
        self.write_chunk(output, TYPE_IEND, &[])?;
        Ok(())
    }
    
    /// 写入chunk
    fn write_chunk(&self, output: &mut Vec<u8>, chunk_type: u32, data: &[u8]) -> Result<(), String> {
        // 写入长度
        output.write_all(&(data.len() as u32).to_be_bytes()).map_err(|e| e.to_string())?;
        
        // 写入chunk类型
        output.write_all(&chunk_type.to_be_bytes()).map_err(|e| e.to_string())?;
        
        // 写入数据
        output.write_all(data).map_err(|e| e.to_string())?;
        
        // 计算并写入CRC
        let mut crc_data = Vec::new();
        crc_data.write_all(&chunk_type.to_be_bytes()).map_err(|e| e.to_string())?;
        crc_data.write_all(data).map_err(|e| e.to_string())?;
        
        let crc = crc32(&crc_data);
        output.write_all(&crc.to_be_bytes()).map_err(|e| e.to_string())?;
        
        Ok(())
    }
}

/// 位打包器
pub struct BitPacker {
    bit_depth: u8,
    color_type: u8,
}

impl BitPacker {
    pub fn new(bit_depth: u8, color_type: u8) -> Self {
        Self { bit_depth, color_type }
    }
    
    /// 打包位数据
    pub fn pack_bits(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        match self.bit_depth {
            1 => self.pack_1bit(data, width, height),
            2 => self.pack_2bit(data, width, height),
            4 => self.pack_4bit(data, width, height),
            8 => self.pack_8bit(data, width, height),
            16 => self.pack_16bit(data, width, height),
            _ => Err(format!("Unsupported bit depth: {}", self.bit_depth)),
        }
    }
    
    /// 打包1位数据
    fn pack_1bit(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        let mut packed = Vec::new();
        let bytes_per_row = (width + 7) / 8;
        
        for y in 0..height {
            let mut row = Vec::new();
            let mut bit_buffer = 0u8;
            let mut bit_count = 0;
            
            for x in 0..width {
                let pixel_index = (y * width + x) as usize;
                if pixel_index < data.len() {
                    let pixel = data[pixel_index];
                    bit_buffer |= (pixel & 1) << (7 - bit_count);
                    bit_count += 1;
                    
                    if bit_count == 8 {
                        row.push(bit_buffer);
                        bit_buffer = 0;
                        bit_count = 0;
                    }
                }
            }
            
            if bit_count > 0 {
                row.push(bit_buffer);
            }
            
            packed.extend_from_slice(&row);
        }
        
        Ok(packed)
    }
    
    /// 打包2位数据
    fn pack_2bit(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        let mut packed = Vec::new();
        let bytes_per_row = (width * 2 + 7) / 8;
        
        for y in 0..height {
            let mut row = Vec::new();
            let mut bit_buffer = 0u8;
            let mut bit_count = 0;
            
            for x in 0..width {
                let pixel_index = (y * width + x) as usize;
                if pixel_index < data.len() {
                    let pixel = data[pixel_index] & 0x03; // 只取低2位
                    bit_buffer |= pixel << (6 - bit_count);
                    bit_count += 2;
                    
                    if bit_count == 8 {
                        row.push(bit_buffer);
                        bit_buffer = 0;
                        bit_count = 0;
                    }
                }
            }
            
            if bit_count > 0 {
                row.push(bit_buffer);
            }
            
            packed.extend_from_slice(&row);
        }
        
        Ok(packed)
    }
    
    /// 打包4位数据
    fn pack_4bit(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        let mut packed = Vec::new();
        let bytes_per_row = (width * 4 + 7) / 8;
        
        for y in 0..height {
            let mut row = Vec::new();
            let mut bit_buffer = 0u8;
            let mut bit_count = 0;
            
            for x in 0..width {
                let pixel_index = (y * width + x) as usize;
                if pixel_index < data.len() {
                    let pixel = data[pixel_index] & 0x0F; // 只取低4位
                    bit_buffer |= pixel << (4 - bit_count);
                    bit_count += 4;
                    
                    if bit_count == 8 {
                        row.push(bit_buffer);
                        bit_buffer = 0;
                        bit_count = 0;
                    }
                }
            }
            
            if bit_count > 0 {
                row.push(bit_buffer);
            }
            
            packed.extend_from_slice(&row);
        }
        
        Ok(packed)
    }
    
    /// 打包8位数据
    fn pack_8bit(&self, data: &[u8], _width: u32, _height: u32) -> Result<Vec<u8>, String> {
        Ok(data.to_vec())
    }
    
    /// 打包16位数据
    fn pack_16bit(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        let mut packed = Vec::new();
        let bytes_per_pixel = self.get_bytes_per_pixel() * 2; // 16位 = 2字节
        
        for y in 0..height {
            for x in 0..width {
                let pixel_index = (y * width + x) as usize * bytes_per_pixel;
                if pixel_index + bytes_per_pixel <= data.len() {
                    for i in 0..bytes_per_pixel {
                        packed.push(data[pixel_index + i]);
                    }
                }
            }
        }
        
        Ok(packed)
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

/// 格式标准化器
pub struct FormatNormalizer {
    input_color_type: u8,
    output_color_type: u8,
    input_bit_depth: u8,
    output_bit_depth: u8,
}

impl FormatNormalizer {
    pub fn new(input_color_type: u8, output_color_type: u8, input_bit_depth: u8, output_bit_depth: u8) -> Self {
        Self {
            input_color_type,
            output_color_type,
            input_bit_depth,
            output_bit_depth,
        }
    }
    
    /// 标准化格式
    pub fn normalize(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        if self.input_color_type == self.output_color_type && self.input_bit_depth == self.output_bit_depth {
            return Ok(data.to_vec());
        }
        
        // 先转换位深度
        let depth_converted = self.convert_bit_depth(data, width, height)?;
        
        // 再转换颜色类型
        self.convert_color_type(&depth_converted, width, height)
    }
    
    /// 转换位深度
    fn convert_bit_depth(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        if self.input_bit_depth == self.output_bit_depth {
            return Ok(data.to_vec());
        }
        
        let mut output = Vec::new();
        let input_bpp = if self.input_bit_depth <= 8 { 1 } else { 2 };
        let output_bpp = if self.output_bit_depth <= 8 { 1 } else { 2 };
        
        scale_depth(data, &mut output, width as usize, height as usize, self.input_bit_depth, self.output_bit_depth);
        
        Ok(output)
    }
    
    /// 转换颜色类型
    fn convert_color_type(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        if self.input_color_type == self.output_color_type {
            return Ok(data.to_vec());
        }
        
        match (self.input_color_type, self.output_color_type) {
            (COLORTYPE_PALETTE_COLOR, COLORTYPE_COLOR_ALPHA) => {
                self.palette_to_rgba(data, width, height)
            }
            (COLORTYPE_GRAYSCALE, COLORTYPE_COLOR_ALPHA) => {
                self.grayscale_to_rgba(data, width, height)
            }
            (COLORTYPE_COLOR, COLORTYPE_COLOR_ALPHA) => {
                self.rgb_to_rgba(data, width, height)
            }
            _ => {
                // 默认转换为RGBA
                self.convert_to_rgba(data, width, height)
            }
        }
    }
    
    /// 调色板转RGBA
    fn palette_to_rgba(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        let mut output = vec![0; (width * height * 4) as usize];
        
        for (i, &index) in data.iter().enumerate() {
            let output_index = i * 4;
            if output_index + 3 < output.len() {
                // 这里需要调色板数据，简化处理
                output[output_index] = index;     // R
                output[output_index + 1] = index; // G
                output[output_index + 2] = index; // B
                output[output_index + 3] = 255;   // A
            }
        }
        
        Ok(output)
    }
    
    /// 灰度转RGBA
    fn grayscale_to_rgba(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        let mut output = vec![0; (width * height * 4) as usize];
        
        for (i, &gray) in data.iter().enumerate() {
            let output_index = i * 4;
            if output_index + 3 < output.len() {
                output[output_index] = gray;     // R
                output[output_index + 1] = gray; // G
                output[output_index + 2] = gray; // B
                output[output_index + 3] = 255;   // A
            }
        }
        
        Ok(output)
    }
    
    /// RGB转RGBA
    fn rgb_to_rgba(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        let mut output = vec![0; (width * height * 4) as usize];
        
        for (i, chunk) in data.chunks_exact(3).enumerate() {
            let output_index = i * 4;
            if output_index + 3 < output.len() {
                output[output_index] = chunk[0];     // R
                output[output_index + 1] = chunk[1]; // G
                output[output_index + 2] = chunk[2]; // B
                output[output_index + 3] = 255;       // A
            }
        }
        
        Ok(output)
    }
    
    /// 转换为RGBA
    fn convert_to_rgba(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        match self.input_color_type {
            COLORTYPE_GRAYSCALE => self.grayscale_to_rgba(data, width, height),
            COLORTYPE_COLOR => self.rgb_to_rgba(data, width, height),
            COLORTYPE_PALETTE_COLOR => self.palette_to_rgba(data, width, height),
            _ => Ok(data.to_vec()),
        }
    }
}
