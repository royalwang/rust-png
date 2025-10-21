//! 集成测试 - 测试PNG类的交错功能

use wasm_bindgen_test::*;
use rust_png::{PNG, PNGSync};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_png_interlace_detection() {
    // 创建PNG实例
    let png = PNG::new(None);
    
    // 测试交错检测
    assert!(!png.is_interlaced());
}

#[wasm_bindgen_test]
fn test_png_interlace_passes() {
    // 创建PNG实例
    let png = PNG::new(None);
    
    // 获取交错通道信息
    let passes = png.get_interlace_passes().unwrap();
    
    // 验证通道数量
    assert_eq!(passes.length(), 7);
    
    // 验证第一个通道
    let first_pass = js_sys::Reflect::get(&passes.get(0), &"pass".into()).unwrap();
    assert_eq!(first_pass.as_f64().unwrap(), 0.0);
}

#[wasm_bindgen_test]
fn test_png_interlace_stats() {
    // 创建PNG实例
    let png = PNG::new(None);
    
    // 获取交错统计信息
    let stats = png.get_interlace_stats().unwrap();
    
    // 验证统计信息
    let total_passes = js_sys::Reflect::get(&stats, &"totalPasses".into()).unwrap();
    assert_eq!(total_passes.as_f64().unwrap(), 7.0);
    
    let total_pixels = js_sys::Reflect::get(&stats, &"totalPixels".into()).unwrap();
    assert!(total_pixels.as_f64().unwrap() > 0.0);
    
    let compression_ratio = js_sys::Reflect::get(&stats, &"compressionRatio".into()).unwrap();
    assert!(compression_ratio.as_f64().unwrap() > 0.0);
    assert!(compression_ratio.as_f64().unwrap() <= 1.0);
}

#[wasm_bindgen_test]
fn test_png_sync_interlace() {
    // 创建测试数据 (这里需要实际的PNG数据)
    let test_data = vec![
        0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, // PNG签名
        // 这里需要完整的PNG数据，为了测试简化处理
    ];
    
    // 测试同步读取
    let result = PNGSync::read(&test_data, None);
    
    // 由于测试数据不完整，预期会失败
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_png_interlace_with_options() {
    // 使用选项创建PNG
    let options = js_sys::Object::new();
    js_sys::Reflect::set(&options, &"width".into(), &32.into()).unwrap();
    js_sys::Reflect::set(&options, &"height".into(), &32.into()).unwrap();
    
    let png = PNG::new(Some(options.into()));
    
    // 验证尺寸
    assert_eq!(png.width(), 32);
    assert_eq!(png.height(), 32);
    
    // 测试交错功能
    let passes = png.get_interlace_passes().unwrap();
    assert_eq!(passes.length(), 7);
    
    let stats = png.get_interlace_stats().unwrap();
    let total_pixels = js_sys::Reflect::get(&stats, &"totalPixels".into()).unwrap();
    assert_eq!(total_pixels.as_f64().unwrap(), 1024.0); // 32*32
}
