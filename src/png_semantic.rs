//! 语义化PNG操作接口
//! 基于组合型结构提供更富有语义的PNG操作

use wasm_bindgen::prelude::*;
use js_sys::{Array, Uint8Array, Uint8ClampedArray, Object};
use std::collections::HashMap;
use png::{Decoder, ColorType as PngColorType, BitDepth as PngBitDepth, Transformations};
use std::io::Cursor;

use crate::png_structures::*;
use crate::constants::*;
use crate::bitmap::*;
use crate::utils::*;
use crate::interlace::*;

/// 语义化PNG结构体 - 使用组合型设计
#[wasm_bindgen]
pub struct SemanticPNG {
    metadata: PNGMetadata,
    pixel_data: PixelData,
    operation_state: OperationState,
    statistics: PNGStatistics,
}

#[wasm_bindgen]
impl SemanticPNG {
    /// 构造函数 - 创建新的PNG实例
    #[wasm_bindgen(constructor)]
    pub fn new(options: Option<JsValue>) -> SemanticPNG {
        let mut width = 0;
        let mut height = 0;
        let mut fill = false;
        
        // 解析选项
        if let Some(opts) = options {
            if let Ok(parsed) = serde_wasm_bindgen::from_value::<serde_json::Value>(opts) {
                width = parsed.get("width").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                height = parsed.get("height").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
                fill = parsed.get("fill").and_then(|v| v.as_bool()).unwrap_or(false);
            }
        }
        
        let metadata = PNGMetadata::new(width, height, 2, 8); // 默认RGB 8位
        let mut pixel_data = if width > 0 && height > 0 {
            let data_size = (4 * width * height) as usize;
            let mut data = vec![0; data_size];
            if fill {
                data.fill(0);
            }
            PixelData::new(data, DataFormat::RGBA)
        } else {
            PixelData::new(Vec::new(), DataFormat::RGBA)
        };
        
        let mut statistics = PNGStatistics::new();
        statistics.calculate_from_data(&metadata, &pixel_data);
        
        SemanticPNG {
            metadata,
            pixel_data,
            operation_state: OperationState::new(),
            statistics,
        }
    }

    /// 解析PNG数据
    #[wasm_bindgen]
    pub fn parse(&mut self, data: &[u8], callback: Option<js_sys::Function>) -> Result<(), JsValue> {
        console_log!("Parsing PNG data with length: {}", data.len());
        
        // 验证PNG签名
        if !validate_png_signature(data) {
            return Err(JsValue::from_str("Invalid PNG signature"));
        }
        
        let mut decoder = Decoder::new(Cursor::new(data));
        decoder.set_transformations(Transformations::EXPAND);
        
        let mut reader = match decoder.read_info() {
            Ok(reader) => reader,
            Err(e) => {
                console_log!("Error reading PNG info: {:?}", e);
                return Err(JsValue::from_str(&format!("Failed to read PNG info: {}", e)));
            }
        };

        let info = reader.info();
        
        // 更新元数据
        self.metadata.dimensions = ImageDimensions::new(info.width, info.height);
        self.metadata.color_info = ColorInfo::new(info.color_type as u8, info.bit_depth as u8);
        self.metadata.compression = CompressionInfo::new(info.compression_method, info.filter_method);
        self.metadata.interlace = InterlaceInfo::new(info.interlace_method as u8);
        
        // 处理调色板
        if let Some(palette) = info.palette() {
            let transparency = info.trns();
            self.metadata.palette = Some(PaletteInfo::new(palette, transparency));
        }
        
        // 处理Gamma
        if let Some(gamma) = info.gamma() {
            self.metadata.gamma = GammaInfo::new(gamma);
        }
        
        // 读取像素数据
        let mut buffer = vec![0; reader.output_buffer_size()];
        match reader.next_frame(&mut buffer) {
            Ok(_) => {
                // 转换为RGBA格式
                let rgba_data = convert_to_rgba(
                    &buffer,
                    self.metadata.dimensions.width as usize,
                    self.metadata.dimensions.height as usize,
                    self.metadata.color_info.color_type as u8,
                    self.metadata.color_info.bit_depth as u8,
                    self.metadata.palette.as_ref().map(|p| p.colors.as_slice())
                );
                
                self.pixel_data = PixelData::new(rgba_data, DataFormat::RGBA);
                self.operation_state.mark_parsed();
                
                // 更新统计信息
                self.statistics.calculate_from_data(&self.metadata, &self.pixel_data);
                
                console_log!("PNG parsed successfully: {}x{}, color_type: {:?}, bit_depth: {:?}", 
                    self.metadata.dimensions.width, 
                    self.metadata.dimensions.height, 
                    self.metadata.color_info.color_type,
                    self.metadata.color_info.bit_depth);
                
                // 调用回调函数
                if let Some(cb) = callback {
                    let _ = cb.call0(&JsValue::null());
                }
            }
            Err(e) => {
                console_log!("Error reading PNG frame: {:?}", e);
                return Err(JsValue::from_str(&format!("Failed to read PNG frame: {}", e)));
            }
        }
        
        Ok(())
    }

    /// 获取图像尺寸信息
    #[wasm_bindgen]
    pub fn get_dimensions(&self) -> Result<Object, JsValue> {
        let obj = Object::new();
        js_sys::Reflect::set(&obj, &"width".into(), &self.metadata.dimensions.width.into())?;
        js_sys::Reflect::set(&obj, &"height".into(), &self.metadata.dimensions.height.into())?;
        js_sys::Reflect::set(&obj, &"aspectRatio".into(), &self.metadata.dimensions.aspect_ratio.into())?;
        js_sys::Reflect::set(&obj, &"totalPixels".into(), &self.metadata.dimensions.total_pixels().into())?;
        js_sys::Reflect::set(&obj, &"isSquare".into(), &self.metadata.dimensions.is_square().into())?;
        js_sys::Reflect::set(&obj, &"isLandscape".into(), &self.metadata.dimensions.is_landscape().into())?;
        js_sys::Reflect::set(&obj, &"isPortrait".into(), &self.metadata.dimensions.is_portrait().into())?;
        Ok(obj)
    }

    /// 获取颜色信息
    #[wasm_bindgen]
    pub fn get_color_info(&self) -> Result<Object, JsValue> {
        let obj = Object::new();
        js_sys::Reflect::set(&obj, &"colorType".into(), &(self.metadata.color_info.color_type as u8).into())?;
        js_sys::Reflect::set(&obj, &"bitDepth".into(), &(self.metadata.color_info.bit_depth as u8).into())?;
        js_sys::Reflect::set(&obj, &"hasAlpha".into(), &self.metadata.color_info.has_alpha.into())?;
        js_sys::Reflect::set(&obj, &"hasTransparency".into(), &self.metadata.color_info.has_transparency.into())?;
        js_sys::Reflect::set(&obj, &"channels".into(), &self.metadata.color_info.channels.into())?;
        js_sys::Reflect::set(&obj, &"bytesPerPixel".into(), &self.metadata.color_info.bytes_per_pixel.into())?;
        js_sys::Reflect::set(&obj, &"isIndexed".into(), &self.metadata.color_info.is_indexed().into())?;
        js_sys::Reflect::set(&obj, &"isTruecolor".into(), &self.metadata.color_info.is_truecolor().into())?;
        Ok(obj)
    }

    /// 获取压缩信息
    #[wasm_bindgen]
    pub fn get_compression_info(&self) -> Result<Object, JsValue> {
        let obj = Object::new();
        js_sys::Reflect::set(&obj, &"method".into(), &(self.metadata.compression.method as u8).into())?;
        js_sys::Reflect::set(&obj, &"filter".into(), &(self.metadata.compression.filter as u8).into())?;
        js_sys::Reflect::set(&obj, &"level".into(), &self.metadata.compression.level.into())?;
        js_sys::Reflect::set(&obj, &"estimatedRatio".into(), &self.metadata.compression.estimated_ratio.into())?;
        Ok(obj)
    }

    /// 获取交错信息
    #[wasm_bindgen]
    pub fn get_interlace_info(&self) -> Result<Object, JsValue> {
        let obj = Object::new();
        js_sys::Reflect::set(&obj, &"method".into(), &(self.metadata.interlace.method as u8).into())?;
        js_sys::Reflect::set(&obj, &"isInterlaced".into(), &self.metadata.interlace.is_interlaced.into())?;
        js_sys::Reflect::set(&obj, &"passes".into(), &self.metadata.interlace.passes.into())?;
        js_sys::Reflect::set(&obj, &"progressiveLoading".into(), &self.metadata.interlace.progressive_loading.into())?;
        Ok(obj)
    }

    /// 获取调色板信息
    #[wasm_bindgen]
    pub fn get_palette_info(&self) -> Result<Option<Object>, JsValue> {
        if let Some(palette) = &self.metadata.palette {
            let obj = Object::new();
            js_sys::Reflect::set(&obj, &"colorCount".into(), &palette.color_count.into())?;
            js_sys::Reflect::set(&obj, &"hasTransparency".into(), &palette.has_transparency.into())?;
            
            let colors = Array::new();
            for color in &palette.colors {
                let color_obj = Object::new();
                js_sys::Reflect::set(&color_obj, &"red".into(), &color.red.into())?;
                js_sys::Reflect::set(&color_obj, &"green".into(), &color.green.into())?;
                js_sys::Reflect::set(&color_obj, &"blue".into(), &color.blue.into())?;
                js_sys::Reflect::set(&color_obj, &"alpha".into(), &color.alpha.into())?;
                colors.push(&color_obj);
            }
            js_sys::Reflect::set(&obj, &"colors".into(), &colors)?;
            Ok(Some(obj))
        } else {
            Ok(None)
        }
    }

    /// 获取Gamma信息
    #[wasm_bindgen]
    pub fn get_gamma_info(&self) -> Result<Object, JsValue> {
        let obj = Object::new();
        js_sys::Reflect::set(&obj, &"gamma".into(), &self.metadata.gamma.gamma.into())?;
        js_sys::Reflect::set(&obj, &"hasGamma".into(), &self.metadata.gamma.has_gamma.into())?;
        js_sys::Reflect::set(&obj, &"gammaCorrected".into(), &self.metadata.gamma.gamma_corrected.into())?;
        Ok(obj)
    }

    /// 获取操作状态
    #[wasm_bindgen]
    pub fn get_operation_state(&self) -> Result<Object, JsValue> {
        let obj = Object::new();
        js_sys::Reflect::set(&obj, &"isReadable".into(), &self.operation_state.is_readable.into())?;
        js_sys::Reflect::set(&obj, &"isWritable".into(), &self.operation_state.is_writable.into())?;
        js_sys::Reflect::set(&obj, &"isParsed".into(), &self.operation_state.is_parsed.into())?;
        js_sys::Reflect::set(&obj, &"isModified".into(), &self.operation_state.is_modified.into())?;
        js_sys::Reflect::set(&obj, &"hasErrors".into(), &self.operation_state.has_errors.into())?;
        
        if let Some(error_msg) = &self.operation_state.error_message {
            js_sys::Reflect::set(&obj, &"errorMessage".into(), &error_msg.into())?;
        }
        
        Ok(obj)
    }

    /// 获取统计信息
    #[wasm_bindgen]
    pub fn get_statistics(&self) -> Result<Object, JsValue> {
        let obj = Object::new();
        js_sys::Reflect::set(&obj, &"fileSize".into(), &self.statistics.file_size.into())?;
        js_sys::Reflect::set(&obj, &"pixelCount".into(), &self.statistics.pixel_count.into())?;
        js_sys::Reflect::set(&obj, &"compressionRatio".into(), &self.statistics.compression_ratio.into())?;
        js_sys::Reflect::set(&obj, &"colorEntropy".into(), &self.statistics.color_entropy.into())?;
        js_sys::Reflect::set(&obj, &"uniqueColors".into(), &self.statistics.unique_colors.into())?;
        js_sys::Reflect::set(&obj, &"transparencyRatio".into(), &self.statistics.transparency_ratio.into())?;
        js_sys::Reflect::set(&obj, &"processingTime".into(), &self.statistics.processing_time.into())?;
        Ok(obj)
    }

    /// 获取像素值
    #[wasm_bindgen]
    pub fn get_pixel(&self, x: u32, y: u32) -> Result<Array, JsValue> {
        match self.pixel_data.get_pixel(x, y, &self.metadata) {
            Ok((r, g, b, a)) => {
                let array = Array::new();
                array.push(&r.into());
                array.push(&g.into());
                array.push(&b.into());
                array.push(&a.into());
                Ok(array)
            }
            Err(e) => Err(JsValue::from_str(&e))
        }
    }

    /// 设置像素值
    #[wasm_bindgen]
    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) -> Result<(), JsValue> {
        match self.pixel_data.set_pixel(x, y, r, g, b, a, &self.metadata) {
            Ok(_) => {
                self.operation_state.mark_modified();
                Ok(())
            }
            Err(e) => Err(JsValue::from_str(&e))
        }
    }

    /// 获取RGBA数据
    #[wasm_bindgen]
    pub fn get_rgba_data(&mut self) -> Result<Uint8ClampedArray, JsValue> {
        let rgba_data = self.pixel_data.get_rgba_data(&self.metadata);
        Ok(vec_to_uint8_clamped_array(rgba_data))
    }

    /// 应用Gamma校正
    #[wasm_bindgen]
    pub fn apply_gamma_correction(&mut self) {
        if self.metadata.gamma.has_gamma {
            self.metadata.gamma.apply_gamma_correction();
            // 这里应该实现实际的Gamma校正逻辑
            self.operation_state.mark_modified();
        }
    }

    /// 获取完整的元数据JSON
    #[wasm_bindgen]
    pub fn get_metadata_json(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.metadata)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// 获取统计信息JSON
    #[wasm_bindgen]
    pub fn get_statistics_json(&self) -> Result<String, JsValue> {
        serde_json::to_string(&self.statistics)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    // 兼容性方法 - 保持与原始PNG类的兼容性
    #[wasm_bindgen(getter)]
    pub fn width(&self) -> u32 { self.metadata.dimensions.width }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 { self.metadata.dimensions.height }

    #[wasm_bindgen(getter)]
    pub fn bit_depth(&self) -> u8 { self.metadata.color_info.bit_depth as u8 }

    #[wasm_bindgen(getter)]
    pub fn color_type(&self) -> u8 { self.metadata.color_info.color_type as u8 }

    #[wasm_bindgen(getter)]
    pub fn gamma(&self) -> f64 { self.metadata.gamma.gamma }

    #[wasm_bindgen(setter)]
    pub fn set_gamma(&mut self, gamma: f64) { 
        self.metadata.gamma = GammaInfo::new(gamma);
    }

    #[wasm_bindgen(getter)]
    pub fn alpha(&self) -> bool { self.metadata.color_info.has_alpha }

    #[wasm_bindgen(getter)]
    pub fn readable(&self) -> bool { self.operation_state.is_readable }

    #[wasm_bindgen(getter)]
    pub fn writable(&self) -> bool { self.operation_state.is_writable }
}

/// 语义化PNG同步处理类
#[wasm_bindgen]
pub struct SemanticPNGSync;

#[wasm_bindgen]
impl SemanticPNGSync {
    /// 同步读取PNG
    #[wasm_bindgen]
    pub fn read(buffer: &[u8], options: Option<JsValue>) -> Result<SemanticPNG, JsValue> {
        let mut png = SemanticPNG::new(options);
        png.parse(buffer, None)?;
        Ok(png)
    }
    
    /// 同步写入PNG
    #[wasm_bindgen]
    pub fn write(png: &SemanticPNG, options: Option<JsValue>) -> Result<Vec<u8>, JsValue> {
        // 这里应该实现PNG编码逻辑
        // 目前返回像素数据作为占位符
        Ok(png.pixel_data.raw_data.clone())
    }
}
