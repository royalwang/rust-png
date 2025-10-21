//! Rust PNG处理库
//! 完全兼容原始pngjs库的API

use wasm_bindgen::prelude::*;

// 模块声明
mod constants;
mod crc;
mod filter;
mod bitmap;
mod utils;
mod interlace;
mod png;

// 重新导出主要类型
pub use png::{PNG, PNGSync};

// 当模块被加载时调用
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
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