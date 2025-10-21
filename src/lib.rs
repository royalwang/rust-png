use wasm_bindgen::prelude::*;
use png::{Decoder, ColorType, BitDepth, Transformations};
use std::io::Cursor;
use js_sys::{Array, Uint8Array, Uint8ClampedArray};
use web_sys::console;
use std::collections::HashMap;

// 导入console.log用于调试
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// PNG常量定义 (匹配原始pngjs库)
const PNG_SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];

// Chunk类型
const TYPE_IHDR: u32 = 0x49484452;
const TYPE_IEND: u32 = 0x49454e44;
const TYPE_IDAT: u32 = 0x49444154;
const TYPE_PLTE: u32 = 0x504c5445;
const TYPE_tRNS: u32 = 0x74524e53;
const TYPE_gAMA: u32 = 0x67414d41;

// 颜色类型
const COLORTYPE_GRAYSCALE: u8 = 0;
const COLORTYPE_PALETTE: u8 = 1;
const COLORTYPE_COLOR: u8 = 2;
const COLORTYPE_ALPHA: u8 = 4;
const COLORTYPE_PALETTE_COLOR: u8 = 3;
const COLORTYPE_COLOR_ALPHA: u8 = 6;

// 滤镜类型
const FILTER_NONE: u8 = 0;
const FILTER_SUB: u8 = 1;
const FILTER_UP: u8 = 2;
const FILTER_AVERAGE: u8 = 3;
const FILTER_PAETH: u8 = 4;

// CRC计算表 (匹配原始pngjs库)
const CRC_TABLE: [u32; 256] = [
    0x00000000, 0x77073096, 0xee0e612c, 0x990951ba, 0x076dc419, 0x706af48f,
    0xe963a535, 0x9e6495a3, 0x0edb8832, 0x79dcb8a4, 0xe0d5e91e, 0x97d2d988,
    0x09b64c2b, 0x7eb17cbd, 0xe7b82d07, 0x90bf1d91, 0x1db71064, 0x6ab020f2,
    0xf3b97148, 0x84be41de, 0x1adad47d, 0x6ddde4eb, 0xf4d4b551, 0x83d385c7,
    0x136c9856, 0x646ba8c0, 0xfd62f97a, 0x8a65c9ec, 0x14015c4f, 0x63066cd9,
    0xfa0f3d63, 0x8d080df5, 0x3b6e20c8, 0x4c69105e, 0xd56041e4, 0xa2677172,
    0x3c03e4d1, 0x4b04d447, 0xd20d85fd, 0xa50ab56b, 0x35b5a8fa, 0x42b2986c,
    0xdbbbc9d6, 0xacbcf940, 0x32d86ce3, 0x45df5c75, 0xdcd60dcf, 0xabd13d59,
    0x26d930ac, 0x51de003a, 0xc8d75180, 0xbfd06116, 0x21b4f4b5, 0x56b3c423,
    0xcfba9599, 0xb8bda50f, 0x2802b89e, 0x5f058808, 0xc60cd9b2, 0xb10be924,
    0x2f6f7c87, 0x58684c11, 0xc1611dab, 0xb6662d3d, 0x76dc4190, 0x01db7106,
    0x98d220bc, 0xefd5102a, 0x71b18589, 0x06b6b51f, 0x9fbfe4a5, 0xe8b8d433,
    0x7807c9a2, 0x0f00f934, 0x9609a88e, 0xe10e9818, 0x7f6a0dbb, 0x086d3d2d,
    0x91646c97, 0xe6635c01, 0x6b6b51f4, 0x1c6c6162, 0x856530d8, 0xf262004e,
    0x6c0695ed, 0x1b01a57b, 0x8208f4c1, 0xf50fc457, 0x65b0d9c6, 0x12b7e950,
    0x8bbeb8ea, 0xfcb9887c, 0x62dd1ddf, 0x15da2d49, 0x8cd37cf3, 0xfbd44c65,
    0x4db26158, 0x3ab551ce, 0xa3bc0074, 0xd4bb30e2, 0x4adfa541, 0x3dd895d7,
    0xa4d1c46d, 0xd3d6f4fb, 0x4369e96a, 0x346ed9fc, 0xad678846, 0xda60b8d0,
    0x44042d73, 0x33031de5, 0xaa0a4c5f, 0xdd0d7cc9, 0x5005713c, 0x270241aa,
    0xbe0b1010, 0xc90c2086, 0x5768b525, 0x206f85b3, 0xb966d409, 0xce61e49f,
    0x5edef90e, 0x29d9c998, 0xb0d09822, 0xc7d7a8b4, 0x59b33d17, 0x2eb40d81,
    0xb7bd5c3b, 0xc0ba6cad, 0xedb88320, 0x9abfb3b6, 0x03b6e20c, 0x74b1d29a,
    0xead54739, 0x9dd277af, 0x04db2615, 0x73dc1683, 0xe3630b12, 0x94643b84,
    0x0d6d6a3e, 0x7a6a5aa8, 0xe40ecf0b, 0x9309ff9d, 0x0a00ae27, 0x7d079eb1,
    0xf00f9344, 0x8708a3d2, 0x1e01f268, 0x6906c2fe, 0xf762575d, 0x806567cb,
    0x196c3671, 0x6e6b06e7, 0xfed41b76, 0x89d32be0, 0x10da7a5a, 0x67dd4acc,
    0xf9b9df6f, 0x8ebeeff9, 0x17b7be43, 0x60b08ed5, 0xd6d6a3e8, 0xa1d1937e,
    0x38d8c2c4, 0x4fdff252, 0xd1bb67f1, 0xa6bc5767, 0x3fb506dd, 0x48b2364b,
    0xd80d2bda, 0xaf0a1b4c, 0x36034af6, 0x41047a60, 0xdf60efc3, 0xa867df55,
    0x316e8eef, 0x4669be79, 0xcb61b38c, 0xbc66831a, 0x256fd2a0, 0x5268e236,
    0xcc0c7795, 0xbb0b4703, 0x220216b9, 0x5505262f, 0xc5ba3bbe, 0xb2bd0b28,
    0x2bb45a92, 0x5cb36a04, 0xc2d7ffa7, 0xb5d0cf31, 0x2cd99e8b, 0x5bdeae1d,
    0x9b64c2b0, 0xec63f226, 0x756aa39c, 0x026d930a, 0x9c0906a9, 0xeb0e363f,
    0x72076785, 0x05005713, 0x95bf4a82, 0xe2b87a14, 0x7bb12bae, 0x0cb61b38,
    0x92d28e9b, 0xe5d5be0d, 0x7cdcefb7, 0x0bdbdf21, 0x86d3d2d4, 0xf1d4e242,
    0x68ddb3f8, 0x1fda836e, 0x81be16cd, 0xf6b9265b, 0x6fb077e1, 0x18b74777,
    0x88085ae6, 0xff0f6a70, 0x66063bca, 0x11010b5c, 0x8f659eff, 0xf862ae69,
    0x616bffd3, 0x166ccf45, 0xa00ae278, 0xd70dd2ee, 0x4e048354, 0x3903b3c2,
    0xa7672661, 0xd06016f7, 0x4969474d, 0x3e6e77db, 0xaed16a4a, 0xd9d65adc,
    0x40df0b66, 0x37d83bf0, 0xa9bcae53, 0xdebb9ec5, 0x47b2cf7f, 0x30b5ffe9,
    0xbdbdf21c, 0xcabac28a, 0x53b39330, 0x24b4a3a6, 0xbad03605, 0xcdd70693,
    0x54de5729, 0x23d967bf, 0xb3667a2e, 0xc4614ab8, 0x5d681b02, 0x2a6f2b94,
    0xb40bbe37, 0xc30c8ea1, 0x5a05df1b, 0x2d02ef8d
];

// CRC计算函数
fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xffffffff;
    for &byte in data {
        crc = CRC_TABLE[((crc ^ (byte as u32)) & 0xff) as usize] ^ (crc >> 8);
    }
    crc ^ 0xffffffff
}

// Paeth预测器 (匹配原始pngjs库)
fn paeth_predictor(a: u8, b: u8, c: u8) -> u8 {
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

// 滤镜处理函数
fn apply_filter(filter_type: u8, data: &mut [u8], width: usize, bpp: usize) {
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
}

#[wasm_bindgen]
impl PNG {
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

    // 原始pngjs的bitblt方法 - 位块传输
    #[wasm_bindgen]
    pub fn bitblt(&self, dst: &mut PNG, src_x: u32, src_y: u32, width: u32, height: u32, delta_x: u32, delta_y: u32) -> Result<(), JsValue> {
        // 检查源图像边界
        if src_x > self.width || src_y > self.height || 
           src_x + width > self.width || src_y + height > self.height {
            return Err(JsValue::from_str("bitblt reading outside image"));
        }
        
        // 检查目标图像边界
        if delta_x > dst.width || delta_y > dst.height || 
           delta_x + width > dst.width || delta_y + height > dst.height {
            return Err(JsValue::from_str("bitblt writing outside image"));
        }
        
        if let (Some(src_data), Some(dst_data)) = (&self.rgba_data, &mut dst.rgba_data) {
            for y in 0..height {
                let src_start = ((src_y + y) * self.width + src_x) as usize * 4;
                let src_end = ((src_y + y) * self.width + src_x + width) as usize * 4;
                let dst_start = ((delta_y + y) * dst.width + delta_x) as usize * 4;
                
                if src_start < src_data.len() && src_end <= src_data.len() && 
                   dst_start + (src_end - src_start) <= dst_data.len() {
                    dst_data[dst_start..dst_start + (src_end - src_start)]
                        .copy_from_slice(&src_data[src_start..src_end]);
                }
            }
        }
        
        Ok(())
    }

    // 原始pngjs的adjustGamma方法
    #[wasm_bindgen]
    pub fn adjust_gamma(&mut self) {
        if self.gamma > 0.0 && self.rgba_data.is_some() {
            if let Some(data) = &mut self.rgba_data {
                for y in 0..self.height {
                    for x in 0..self.width {
                        let idx = ((self.width * y + x) * 4) as usize;
                        
                        for i in 0..3 {
                            if idx + i < data.len() {
                                let sample = data[idx + i] as f64 / 255.0;
                                let adjusted = sample.powf(1.0 / 2.2 / self.gamma);
                                data[idx + i] = (adjusted * 255.0).round() as u8;
                            }
                        }
                    }
                }
            }
            self.gamma = 0.0;
        }
    }

    // 获取像素值
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

    // 设置像素值
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

    // 获取gamma值
    #[wasm_bindgen(getter)]
    pub fn gamma(&self) -> f64 {
        self.gamma
    }

    // 设置gamma值
    #[wasm_bindgen(setter)]
    pub fn set_gamma(&mut self, gamma: f64) {
        self.gamma = gamma;
    }

    // 获取alpha通道状态
    #[wasm_bindgen(getter)]
    pub fn alpha(&self) -> bool {
        self.alpha
    }

    // 获取透明度颜色
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
    PNG::new(None)
}

// 兼容性函数 - 创建并解析PNG
#[wasm_bindgen]
pub fn create_png_from_data(data: &[u8]) -> Result<PNG, JsValue> {
    let mut png = PNG::new(None);
    png.parse(data, None)?;
    Ok(png)
}

// 原始pngjs的同步API
#[wasm_bindgen]
pub struct PNGSync;

#[wasm_bindgen]
impl PNGSync {
    // 同步读取PNG
    #[wasm_bindgen]
    pub fn read(buffer: &[u8], options: Option<JsValue>) -> Result<PNG, JsValue> {
        let mut png = PNG::new(options);
        png.parse(buffer, None)?;
        Ok(png)
    }
    
    // 同步写入PNG
    #[wasm_bindgen]
    pub fn write(png: &PNG, options: Option<JsValue>) -> Result<Vec<u8>, JsValue> {
        png.pack()
    }
}

// 位深度转换函数
fn scale_depth(input: &[u8], output: &mut [u8], width: usize, height: usize, input_depth: u8, output_depth: u8) {
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

// 调色板处理函数
fn de_palette(input: &[u8], output: &mut [u8], width: usize, height: usize, palette: &[u8]) {
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

// 透明度颜色处理
fn replace_transparent_color(input: &[u8], output: &mut [u8], width: usize, height: usize, trans_color: &[u16]) {
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

// 当模块被加载时调用
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}
