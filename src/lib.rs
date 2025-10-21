//! Rust PNG处理库
//! 完全兼容原始pngjs库的API

use wasm_bindgen::prelude::*;

// 模块声明
mod constants;
mod crc;
mod filter;
mod filter_extensible;
mod custom_filters;
mod filter_optimizer;
mod bitmap;
mod bitmapper;
mod utils;
mod interlace;
mod png_packer;
mod png_chunks;
mod filter_pack;
mod sync_inflate;
mod performance;
mod chunk_stream;
mod error_handling;
mod testing;
mod advanced_png;
mod wasm_optimization;
mod advanced_filters;
mod png;
mod png_structures;
mod png_semantic;

// 重新导出主要类型
pub use png::{PNG, PNGSync};
pub use png_semantic::{SemanticPNG, SemanticPNGSync};

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