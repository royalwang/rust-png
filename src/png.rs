//! PNG主结构体模块
//! 包含PNG类和PNGSync类

use wasm_bindgen::prelude::*;
use js_sys::{Array, Uint8Array, Uint8ClampedArray};
use std::collections::HashMap;
use png::{Decoder, ColorType, BitDepth, Transformations};
use std::io::Cursor;

use crate::constants::*;
use crate::bitmap::*;
use crate::utils::*;
use crate::interlace::*;
use crate::png_packer::*;
use crate::png_chunks::*;
use crate::filter_pack::*;
use crate::sync_inflate::*;
use crate::bitmapper::*;

/// PNG结构体 - 匹配原始pngjs库的PNG类
#[wasm_bindgen]
pub struct PNG {
    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: u8,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,
    palette: Option<Vec<u8>>,
    pixel_data: Option<Vec<u8>>,
    rgba_data: Option<Vec<u8>>,
    gamma: f64,
    trans_color: Option<Vec<u16>>,
    alpha: bool,
    readable: bool,
    writable: bool,
    // 新增的元数据字段
    chunks: HashMap<String, Vec<u8>>,
    has_ihdr: bool,
    has_iend: bool,
    interlace: bool,
    bpp: u8, // bytes per pixel
    depth: u8, // bit depth
    chunk_parser: PNGChunkParser,
}

#[wasm_bindgen]
impl PNG {
    /// 构造函数 - 匹配原始pngjs库
    #[wasm_bindgen(constructor)]
    pub fn new(options: Option<JsValue>) -> PNG {
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
        
        let mut rgba_data = None;
        if width > 0 && height > 0 {
            let data_size = (4 * width * height) as usize;
            let mut data = vec![0; data_size];
            if fill {
                data.fill(0);
            }
            rgba_data = Some(data);
        }
        
        PNG {
            width,
            height,
            bit_depth: 8,
            color_type: 2,
            compression_method: 0,
            filter_method: 0,
            interlace_method: 0,
            palette: None,
            pixel_data: None,
            rgba_data,
            gamma: 0.0,
            trans_color: None,
            alpha: false,
            readable: true,
            writable: true,
            chunks: HashMap::new(),
            has_ihdr: false,
            has_iend: false,
            interlace: false,
            bpp: 4, // RGBA = 4 bytes per pixel
            depth: 8,
            chunk_parser: PNGChunkParser::new(),
        }
    }

    /// 解析PNG数据 - 匹配原始pngjs库的parse方法
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
        self.width = info.width;
        self.height = info.height;
        self.bit_depth = info.bit_depth as u8;
        self.color_type = info.color_type as u8;
        self.compression_method = info.compression_method;
        self.filter_method = info.filter_method;
        self.interlace_method = info.interlace_method;
        self.interlace = info.interlace_method != png::InterlaceMethod::None;
        
        // 设置每像素字节数
        self.bpp = get_bytes_per_pixel(self.color_type) as u8;
        self.depth = self.bit_depth;
        
        // 处理调色板
        if let Some(palette) = info.palette() {
            self.palette = Some(palette.to_vec());
        }
        
        // 处理透明度
        if let Some(trns) = info.trns() {
            self.trans_color = Some(trns.iter().map(|&x| x as u16).collect());
            self.alpha = true;
        }
        
        // 处理Gamma
        if let Some(gamma) = info.gamma() {
            self.gamma = gamma;
        }
        
        // 读取像素数据
        let mut buffer = vec![0; reader.output_buffer_size()];
        match reader.next_frame(&mut buffer) {
            Ok(_) => {
                // 转换为RGBA格式
                self.rgba_data = Some(convert_to_rgba(
                    &buffer,
                    self.width as usize,
                    self.height as usize,
                    self.color_type,
                    self.bit_depth,
                    self.palette.as_deref()
                ));
                
                self.pixel_data = Some(buffer);
                self.has_ihdr = true;
                self.has_iend = true;
                
                console_log!("PNG parsed successfully: {}x{}, color_type: {}, bit_depth: {}", 
                    self.width, self.height, self.color_type, self.bit_depth);
                
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

    /// 打包PNG数据 - 匹配原始pngjs库的pack方法
    #[wasm_bindgen]
    pub fn pack(&self) -> Result<Vec<u8>, JsValue> {
        if let Some(ref data) = self.rgba_data {
            let options = PackerOptions {
                width: self.width,
                height: self.height,
                bit_depth: self.bit_depth,
                color_type: self.color_type,
                input_color_type: self.color_type,
                input_has_alpha: self.alpha,
                ..Default::default()
            };
            
            let packer = PNGPacker::new(options);
            match packer.pack(data) {
                Ok(packed_data) => Ok(packed_data),
                Err(e) => Err(JsValue::from_str(&e)),
            }
        } else {
            Err(JsValue::from_str("No image data to pack"))
        }
    }

    /// 写入文件 - 匹配原始pngjs库的writeFile方法
    #[wasm_bindgen]
    pub fn write_file(&self, _filename: &str) -> Result<(), JsValue> {
        // 在WASM环境中，文件写入通常通过JavaScript File API处理
        console_log!("writeFile called (WASM environment)");
        Ok(())
    }

    /// 获取缓冲区 - 匹配原始pngjs库的toBuffer方法
    #[wasm_bindgen]
    pub fn to_buffer(&self) -> Result<Vec<u8>, JsValue> {
        if let Some(rgba_data) = &self.rgba_data {
            Ok(rgba_data.clone())
        } else {
            Err(JsValue::from_str("No image data available"))
        }
    }

    /// 位块传输 - 匹配原始pngjs库的bitblt方法
    #[wasm_bindgen]
    pub fn bitblt(&self, dst: &mut PNG, src_x: u32, src_y: u32, width: u32, height: u32, delta_x: u32, delta_y: u32) -> Result<(), JsValue> {
        if src_x + width > self.width || src_y + height > self.height {
            return Err(JsValue::from_str("Source coordinates out of bounds"));
        }
        
        if delta_x + width > dst.width || delta_y + height > dst.height {
            return Err(JsValue::from_str("Destination coordinates out of bounds"));
        }
        
        if let (Some(src_data), Some(dst_data)) = (&self.rgba_data, &mut dst.rgba_data) {
            for y in 0..height {
                for x in 0..width {
                    let src_idx = ((src_y + y) * self.width + src_x + x) * 4;
                    let dst_idx = ((delta_y + y) * dst.width + delta_x + x) * 4;
                    
                    if src_idx + 3 < src_data.len() && dst_idx + 3 < dst_data.len() {
                        dst_data[dst_idx] = src_data[src_idx];
                        dst_data[dst_idx + 1] = src_data[src_idx + 1];
                        dst_data[dst_idx + 2] = src_data[src_idx + 2];
                        dst_data[dst_idx + 3] = src_data[src_idx + 3];
                    }
                }
            }
        }
        
        Ok(())
    }

    /// 调整Gamma - 匹配原始pngjs库的adjustGamma方法
    #[wasm_bindgen]
    pub fn adjust_gamma(&mut self) {
        if self.gamma > 0.0 {
            if let Some(rgba_data) = &mut self.rgba_data {
                let gamma_correction = 1.0 / self.gamma;
                for i in (0..rgba_data.len()).step_by(4) {
                    if i + 2 < rgba_data.len() {
                        rgba_data[i] = ((rgba_data[i] as f64 / 255.0).powf(gamma_correction) * 255.0) as u8;
                        rgba_data[i + 1] = ((rgba_data[i + 1] as f64 / 255.0).powf(gamma_correction) * 255.0) as u8;
                        rgba_data[i + 2] = ((rgba_data[i + 2] as f64 / 255.0).powf(gamma_correction) * 255.0) as u8;
                    }
                }
            }
        }
    }

    /// 获取像素值 - 匹配原始pngjs库的getPixel方法
    #[wasm_bindgen]
    pub fn get_pixel(&self, x: u32, y: u32) -> Result<Array, JsValue> {
        if x >= self.width || y >= self.height {
            return Err(JsValue::from_str("Pixel coordinates out of bounds"));
        }

        if let Some(rgba_data) = &self.rgba_data {
            let index = ((y * self.width + x) * 4) as usize;
            if index + 3 < rgba_data.len() {
                let pixel = [
                    rgba_data[index],
                    rgba_data[index + 1],
                    rgba_data[index + 2],
                    rgba_data[index + 3],
                ];
                Ok(pixel_to_array(pixel))
            } else {
                Err(JsValue::from_str("Pixel data out of bounds"))
            }
        } else {
            Err(JsValue::from_str("No image data available"))
        }
    }

    /// 设置像素值 - 匹配原始pngjs库的setPixel方法
    #[wasm_bindgen]
    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) -> Result<(), JsValue> {
        if x >= self.width || y >= self.height {
            return Err(JsValue::from_str("Pixel coordinates out of bounds"));
        }

        if let Some(rgba_data) = &mut self.rgba_data {
            let index = ((y * self.width + x) * 4) as usize;
            if index + 3 < rgba_data.len() {
                rgba_data[index] = r;
                rgba_data[index + 1] = g;
                rgba_data[index + 2] = b;
                rgba_data[index + 3] = a;
            }
        }

        Ok(())
    }

    // Getter方法
    #[wasm_bindgen(getter)]
    pub fn width(&self) -> u32 { self.width }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 { self.height }

    #[wasm_bindgen(getter)]
    pub fn bit_depth(&self) -> u8 { self.bit_depth }

    #[wasm_bindgen(getter)]
    pub fn color_type(&self) -> u8 { self.color_type }

    #[wasm_bindgen(getter)]
    pub fn compression_method(&self) -> u8 { self.compression_method }

    #[wasm_bindgen(getter)]
    pub fn filter_method(&self) -> u8 { self.filter_method }

    #[wasm_bindgen(getter)]
    pub fn interlace_method(&self) -> u8 { self.interlace_method }

    #[wasm_bindgen(getter)]
    pub fn gamma(&self) -> f64 { self.gamma }

    #[wasm_bindgen(setter)]
    pub fn set_gamma(&mut self, gamma: f64) { self.gamma = gamma; }

    #[wasm_bindgen(getter)]
    pub fn alpha(&self) -> bool { self.alpha }

    #[wasm_bindgen(getter)]
    pub fn readable(&self) -> bool { self.readable }

    #[wasm_bindgen(getter)]
    pub fn writable(&self) -> bool { self.writable }

    /// 获取RGBA数据
    #[wasm_bindgen]
    pub fn get_rgba8_array(&self) -> Result<Uint8ClampedArray, JsValue> {
        if let Some(rgba_data) = &self.rgba_data {
            Ok(vec_to_uint8_clamped_array(rgba_data))
        } else {
            Err(JsValue::from_str("No image data available"))
        }
    }

    /// 获取调色板数据
    #[wasm_bindgen]
    pub fn get_palette(&self) -> Option<Uint8Array> {
        self.palette.as_ref().map(|palette| vec_to_uint8_array(palette))
    }

    /// 获取透明度颜色
    #[wasm_bindgen]
    pub fn get_trans_color(&self) -> Option<Uint8Array> {
        self.trans_color.as_ref().map(|colors| {
            let array = Uint8Array::new_with_length(colors.len() as u32);
            for (i, &color) in colors.iter().enumerate() {
                array.set_index(i as u32, color as u8);
            }
            array
        })
    }

    /// 获取交错通道信息
    #[wasm_bindgen]
    pub fn get_interlace_passes(&self) -> Result<Array, JsValue> {
        let passes = get_interlace_passes(self.width, self.height);
        let array = Array::new();
        
        for pass in passes {
            let pass_obj = js_sys::Object::new();
            js_sys::Reflect::set(&pass_obj, &"pass".into(), &pass.pass.into())?;
            js_sys::Reflect::set(&pass_obj, &"width".into(), &pass.width.into())?;
            js_sys::Reflect::set(&pass_obj, &"height".into(), &pass.height.into())?;
            js_sys::Reflect::set(&pass_obj, &"xOffset".into(), &pass.x_offset.into())?;
            js_sys::Reflect::set(&pass_obj, &"yOffset".into(), &pass.y_offset.into())?;
            js_sys::Reflect::set(&pass_obj, &"xStep".into(), &pass.x_step.into())?;
            js_sys::Reflect::set(&pass_obj, &"yStep".into(), &pass.y_step.into())?;
            array.push(&pass_obj);
        }
        
        Ok(array)
    }

    /// 检查是否为交错图像
    #[wasm_bindgen]
    pub fn is_interlaced(&self) -> bool {
        self.interlace
    }

    /// 获取交错统计信息
    #[wasm_bindgen]
    pub fn get_interlace_stats(&self) -> Result<js_sys::Object, JsValue> {
        let stats = get_interlace_stats(self.width, self.height);
        let obj = js_sys::Object::new();
        
        js_sys::Reflect::set(&obj, &"totalPasses".into(), &stats.total_passes.into())?;
        js_sys::Reflect::set(&obj, &"totalPixels".into(), &stats.total_pixels.into())?;
        js_sys::Reflect::set(&obj, &"compressionRatio".into(), &stats.compression_ratio.into())?;
        
        let pass_sizes = Array::new();
        for size in stats.pass_sizes {
            pass_sizes.push(&size.into());
        }
        js_sys::Reflect::set(&obj, &"passSizes".into(), &pass_sizes)?;
        
        Ok(obj)
    }
}

/// 同步PNG处理类 - 匹配原始pngjs库的PNGSync
#[wasm_bindgen]
pub struct PNGSync;

#[wasm_bindgen]
impl PNGSync {
    /// 同步读取PNG
    #[wasm_bindgen]
    pub fn read(buffer: &[u8], options: Option<JsValue>) -> Result<PNG, JsValue> {
        let mut png = PNG::new(options);
        png.parse(buffer, None)?;
        Ok(png)
    }
    
    /// 同步写入PNG
    #[wasm_bindgen]
    pub fn write(png: &PNG, options: Option<JsValue>) -> Result<Vec<u8>, JsValue> {
        png.pack()
    }
}

impl PNG {
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
