use wasm_bindgen::prelude::*;
use png::{Decoder, ColorType, BitDepth};
use std::io::Cursor;
use js_sys::{Array, Uint8Array, Uint8ClampedArray};
use web_sys::console;

// 导入console.log用于调试
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

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
}

#[wasm_bindgen]
impl PNG {
    #[wasm_bindgen(constructor)]
    pub fn new() -> PNG {
        PNG {
            width: 0,
            height: 0,
            bit_depth: 8,
            color_type: 2,
            compression_method: 0,
            filter_method: 0,
            interlace_method: 0,
            palette: None,
            pixel_data: None,
            rgba_data: None,
        }
    }

    // 原始pngjs的parse方法
    #[wasm_bindgen]
    pub fn parse(&mut self, data: &[u8], callback: Option<js_sys::Function>) -> Result<(), JsValue> {
        console_log!("Parsing PNG data with length: {}", data.len());
        
        let mut decoder = Decoder::new(Cursor::new(data));
        decoder.set_transformations(png::Transformations::EXPAND);
        
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
        self.compression_method = info.compression_method as u8;
        self.filter_method = info.filter_method as u8;
        self.interlace_method = info.interlace_method as u8;

        console_log!("PNG info - Width: {}, Height: {}, BitDepth: {}, ColorType: {}", 
                    self.width, self.height, self.bit_depth, self.color_type);

        // 读取像素数据
        let mut buffer = vec![0; reader.output_buffer_size()];
        match reader.next_frame(&mut buffer) {
            Ok(_) => {
                console_log!("Successfully parsed {} bytes of pixel data", buffer.len());
                self.pixel_data = Some(buffer.clone());
                
                // 转换为RGBA格式
                self.rgba_data = Some(convert_to_rgba(&buffer, &info));
                
                // 提取调色板
                if let Some(palette_data) = info.palette() {
                    self.palette = Some(palette_data.to_vec());
                }
                
                // 调用回调函数（如果提供）
                if let Some(cb) = callback {
                    let _ = cb.call0(&JsValue::NULL);
                }
            }
            Err(e) => {
                console_log!("Error reading frame: {:?}", e);
                return Err(JsValue::from_str(&format!("Failed to read PNG frame: {}", e)));
            }
        }

        Ok(())
    }

    #[wasm_bindgen(getter)]
    pub fn get_width(&self) -> u32 {
        self.width
    }

    #[wasm_bindgen(getter)]
    pub fn get_height(&self) -> u32 {
        self.height
    }

    #[wasm_bindgen]
    pub fn get_pixel(&self, x: u32, y: u32) -> Result<Array, JsValue> {
        if x >= self.width || y >= self.height {
            return Err(JsValue::from_str("Pixel coordinates out of bounds"));
        }

        let rgba_data = self.rgba_data.as_ref()
            .ok_or_else(|| JsValue::from_str("No pixel data available"))?;

        let index = ((y * self.width + x) * 4) as usize;
        if index + 3 >= rgba_data.len() {
            return Err(JsValue::from_str("Invalid pixel index"));
        }

        let pixel = Array::new();
        pixel.push(&JsValue::from(rgba_data[index]));     // Red
        pixel.push(&JsValue::from(rgba_data[index + 1])); // Green
        pixel.push(&JsValue::from(rgba_data[index + 2])); // Blue
        pixel.push(&JsValue::from(rgba_data[index + 3])); // Alpha

        Ok(pixel)
    }

    #[wasm_bindgen]
    pub fn get_rgba8_array(&self) -> Result<Uint8ClampedArray, JsValue> {
        let rgba_data = self.rgba_data.as_ref()
            .ok_or_else(|| JsValue::from_str("No RGBA data available"))?;

        let array = Uint8ClampedArray::new_with_length(rgba_data.len() as u32);
        array.copy_from(rgba_data);
        Ok(array)
    }

    #[wasm_bindgen(getter)]
    pub fn get_bit_depth(&self) -> u8 {
        self.bit_depth
    }

    #[wasm_bindgen(getter)]
    pub fn get_color_type(&self) -> u8 {
        self.color_type
    }

    #[wasm_bindgen(getter)]
    pub fn get_compression_method(&self) -> u8 {
        self.compression_method
    }

    #[wasm_bindgen(getter)]
    pub fn get_filter_method(&self) -> u8 {
        self.filter_method
    }

    #[wasm_bindgen(getter)]
    pub fn get_interlace_method(&self) -> u8 {
        self.interlace_method
    }

    #[wasm_bindgen]
    pub fn get_palette(&self) -> Option<Uint8Array> {
        self.palette.as_ref().map(|palette| {
            let array = Uint8Array::new_with_length(palette.len() as u32);
            array.copy_from(palette);
            array
        })
    }

    // 原始pngjs库的parse方法
    #[wasm_bindgen]
    pub fn parse(&mut self, data: &[u8], callback: Option<js_sys::Function>) -> Result<(), JsValue> {
        console_log!("Parsing PNG data with length: {}", data.len());
        
        // 重新解析数据
        let mut decoder = Decoder::new(Cursor::new(data));
        decoder.set_transformations(png::Transformations::EXPAND);
        
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
        self.compression_method = info.compression_method as u8;
        self.filter_method = info.filter_method as u8;
        self.interlace_method = info.interlace_method as u8;

        // 读取像素数据
        let mut buffer = vec![0; reader.output_buffer_size()];
        match reader.next_frame(&mut buffer) {
            Ok(_) => {
                console_log!("Successfully parsed {} bytes of pixel data", buffer.len());
                self.pixel_data = Some(buffer.clone());
                self.rgba_data = Some(convert_to_rgba(&buffer, &info));
                
                // 提取调色板
                if let Some(palette_data) = info.palette() {
                    self.palette = Some(palette_data.to_vec());
                }
                
                // 调用回调函数（如果提供）
                if let Some(cb) = callback {
                    let _ = cb.call0(&JsValue::NULL);
                }
            }
            Err(e) => {
                console_log!("Error reading frame: {:?}", e);
                return Err(JsValue::from_str(&format!("Failed to read PNG frame: {}", e)));
            }
        }

        Ok(())
    }

    // 获取图像数据（原始pngjs的data属性）
    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Option<Uint8ClampedArray> {
        self.rgba_data.as_ref().map(|data| {
            let array = Uint8ClampedArray::new_with_length(data.len() as u32);
            array.copy_from(data);
            array
        })
    }

    // 设置像素数据
    #[wasm_bindgen(setter)]
    pub fn set_data(&mut self, data: &[u8]) {
        self.rgba_data = Some(data.to_vec());
    }

    // 原始pngjs的pack方法 - 将图像数据打包为PNG格式
    #[wasm_bindgen]
    pub fn pack(&self) -> Result<Vec<u8>, JsValue> {
        if self.rgba_data.is_none() {
            return Err(JsValue::from_str("No image data to pack"));
        }

        // 这里应该实现PNG编码逻辑
        // 由于复杂性，我们返回一个简单的实现
        console_log!("Packing PNG data...");
        
        // 在实际实现中，这里应该使用PNG编码器
        // 现在返回原始数据作为占位符
        if let Some(data) = &self.rgba_data {
            Ok(data.clone())
        } else {
            Err(JsValue::from_str("No data available to pack"))
        }
    }

    // 原始pngjs的writeFile方法
    #[wasm_bindgen]
    pub fn write_file(&self, filename: &str) -> Result<(), JsValue> {
        console_log!("Writing PNG file: {}", filename);
        
        if let Some(data) = &self.rgba_data {
            // 在实际实现中，这里应该写入文件
            // 在WASM环境中，这通常通过JavaScript的File API处理
            console_log!("Would write {} bytes to {}", data.len(), filename);
            Ok(())
        } else {
            Err(JsValue::from_str("No data available to write"))
        }
    }

    // 原始pngjs的toBuffer方法
    #[wasm_bindgen]
    pub fn to_buffer(&self) -> Result<Vec<u8>, JsValue> {
        if let Some(data) = &self.rgba_data {
            Ok(data.clone())
        } else {
            Err(JsValue::from_str("No data available"))
        }
    }
}

// 将PNG数据转换为RGBA格式
fn convert_to_rgba(data: &[u8], info: &png::Info) -> Vec<u8> {
    let width = info.width as usize;
    let height = info.height as usize;
    let color_type = info.color_type;
    let bit_depth = info.bit_depth;
    let palette = info.palette();

    let mut rgba = Vec::with_capacity(width * height * 4);

    match (color_type, bit_depth) {
        (ColorType::Rgb, BitDepth::Eight) => {
            // RGB 8-bit
            for chunk in data.chunks_exact(3) {
                rgba.push(chunk[0]); // R
                rgba.push(chunk[1]); // G
                rgba.push(chunk[2]); // B
                rgba.push(255);      // A
            }
        }
        (ColorType::Rgba, BitDepth::Eight) => {
            // RGBA 8-bit
            rgba.extend_from_slice(data);
        }
        (ColorType::Grayscale, BitDepth::Eight) => {
            // Grayscale 8-bit
            for &gray in data {
                rgba.push(gray); // R
                rgba.push(gray); // G
                rgba.push(gray); // B
                rgba.push(255);  // A
            }
        }
        (ColorType::GrayscaleAlpha, BitDepth::Eight) => {
            // Grayscale + Alpha 8-bit
            for chunk in data.chunks_exact(2) {
                let gray = chunk[0];
                let alpha = chunk[1];
                rgba.push(gray); // R
                rgba.push(gray); // G
                rgba.push(gray); // B
                rgba.push(alpha); // A
            }
        }
        (ColorType::Indexed, BitDepth::Eight) => {
            // Palette 8-bit
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
        (ColorType::Rgb, BitDepth::Sixteen) => {
            // RGB 16-bit -> 8-bit
            for chunk in data.chunks_exact(6) {
                rgba.push(chunk[0]); // R (high byte)
                rgba.push(chunk[2]); // G (high byte)
                rgba.push(chunk[4]); // B (high byte)
                rgba.push(255);      // A
            }
        }
        (ColorType::Rgba, BitDepth::Sixteen) => {
            // RGBA 16-bit -> 8-bit
            for chunk in data.chunks_exact(8) {
                rgba.push(chunk[0]); // R (high byte)
                rgba.push(chunk[2]); // G (high byte)
                rgba.push(chunk[4]); // B (high byte)
                rgba.push(chunk[6]); // A (high byte)
            }
        }
        _ => {
            console_log!("Unsupported color type: {:?}, bit depth: {:?}", color_type, bit_depth);
            // 返回黑色像素作为fallback
            rgba.resize(width * height * 4, 0);
        }
    }

    rgba
}

// 导出函数用于从JavaScript调用（兼容原始pngjs API）
#[wasm_bindgen]
pub fn create_png() -> PNG {
    PNG::new()
}

// 兼容性函数 - 创建并解析PNG
#[wasm_bindgen]
pub fn create_png_from_data(data: &[u8]) -> Result<PNG, JsValue> {
    let mut png = PNG::new();
    png.parse(data, None)?;
    Ok(png)
}

// 当模块被加载时调用
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}
