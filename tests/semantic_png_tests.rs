//! 语义化PNG结构测试

use rust_png::png_semantic::*;
use rust_png::png_structures::*;

#[test]
fn test_image_dimensions() {
    let dims = ImageDimensions::new(800, 600);
    
    assert_eq!(dims.width, 800);
    assert_eq!(dims.height, 600);
    assert_eq!(dims.aspect_ratio, 800.0 / 600.0);
    assert_eq!(dims.total_pixels(), 480000);
    assert!(dims.is_landscape());
    assert!(!dims.is_portrait());
    assert!(!dims.is_square());
}

#[test]
fn test_square_dimensions() {
    let dims = ImageDimensions::new(512, 512);
    
    assert_eq!(dims.width, 512);
    assert_eq!(dims.height, 512);
    assert_eq!(dims.aspect_ratio, 1.0);
    assert!(dims.is_square());
    assert!(!dims.is_landscape());
    assert!(!dims.is_portrait());
}

#[test]
fn test_color_info() {
    let color_info = ColorInfo::new(6, 8); // RGBA 8-bit
    
    assert!(matches!(color_info.color_type, ColorType::RGBA));
    assert!(matches!(color_info.bit_depth, BitDepth::Eight));
    assert!(color_info.has_alpha);
    assert_eq!(color_info.channels, 4);
    assert_eq!(color_info.bytes_per_pixel, 4);
    assert!(color_info.is_truecolor());
    assert!(!color_info.is_indexed());
}

#[test]
fn test_grayscale_color_info() {
    let color_info = ColorInfo::new(0, 8); // Grayscale 8-bit
    
    assert!(matches!(color_info.color_type, ColorType::Grayscale));
    assert!(matches!(color_info.bit_depth, BitDepth::Eight));
    assert!(!color_info.has_alpha);
    assert_eq!(color_info.channels, 1);
    assert_eq!(color_info.bytes_per_pixel, 1);
    assert!(!color_info.is_truecolor());
    assert!(!color_info.is_indexed());
}

#[test]
fn test_palette_color_info() {
    let color_info = ColorInfo::new(1, 8); // Palette 8-bit
    
    assert!(matches!(color_info.color_type, ColorType::Palette));
    assert!(matches!(color_info.bit_depth, BitDepth::Eight));
    assert!(!color_info.has_alpha);
    assert_eq!(color_info.channels, 1);
    assert_eq!(color_info.bytes_per_pixel, 1);
    assert!(!color_info.is_truecolor());
    assert!(color_info.is_indexed());
}

#[test]
fn test_compression_info() {
    let comp_info = CompressionInfo::new(0, 4); // Deflate + Paeth
    
    assert!(matches!(comp_info.method, CompressionMethod::Deflate));
    assert!(matches!(comp_info.filter, FilterMethod::Paeth));
    assert_eq!(comp_info.level, 6);
    assert_eq!(comp_info.estimated_ratio, 0.0);
}

#[test]
fn test_interlace_info() {
    let interlace_none = InterlaceInfo::new(0);
    assert!(matches!(interlace_none.method, InterlaceMethod::None));
    assert!(!interlace_none.is_interlaced);
    assert_eq!(interlace_none.passes, 1);
    assert!(!interlace_none.progressive_loading);
    
    let interlace_adam7 = InterlaceInfo::new(1);
    assert!(matches!(interlace_adam7.method, InterlaceMethod::Adam7));
    assert!(interlace_adam7.is_interlaced);
    assert_eq!(interlace_adam7.passes, 7);
    assert!(interlace_adam7.progressive_loading);
}

#[test]
fn test_palette_info() {
    let palette_data = vec![255, 0, 0, 0, 255, 0, 0, 0, 255]; // RGB colors
    let transparency_data = Some(vec![255, 128, 0]); // Alpha values
    
    let palette = PaletteInfo::new(&palette_data, transparency_data.as_deref());
    
    assert_eq!(palette.color_count, 3);
    assert!(palette.has_transparency);
    assert_eq!(palette.colors.len(), 3);
    
    // Test first color (red)
    assert_eq!(palette.colors[0].red, 255);
    assert_eq!(palette.colors[0].green, 0);
    assert_eq!(palette.colors[0].blue, 0);
    assert_eq!(palette.colors[0].alpha, 255);
    
    // Test second color (green)
    assert_eq!(palette.colors[1].red, 0);
    assert_eq!(palette.colors[1].green, 255);
    assert_eq!(palette.colors[1].blue, 0);
    assert_eq!(palette.colors[1].alpha, 128);
    
    // Test color lookup
    let color_index = palette.find_color(255, 0, 0);
    assert_eq!(color_index, Some(0));
    
    let color_index = palette.find_color(0, 255, 0);
    assert_eq!(color_index, Some(1));
    
    let color_index = palette.find_color(0, 0, 255);
    assert_eq!(color_index, Some(2));
}

#[test]
fn test_gamma_info() {
    let gamma_info = GammaInfo::new(2.2);
    
    assert_eq!(gamma_info.gamma, 2.2);
    assert!(gamma_info.has_gamma);
    assert!(!gamma_info.gamma_corrected);
    
    gamma_info.apply_gamma_correction();
    assert!(gamma_info.gamma_corrected);
}

#[test]
fn test_png_metadata() {
    let metadata = PNGMetadata::new(800, 600, 6, 8);
    
    assert_eq!(metadata.dimensions.width, 800);
    assert_eq!(metadata.dimensions.height, 600);
    assert!(matches!(metadata.color_info.color_type, ColorType::RGBA));
    assert!(matches!(metadata.color_info.bit_depth, BitDepth::Eight));
    
    // Test chunk management
    assert!(!metadata.has_chunk("tEXt"));
    metadata.add_chunk("tEXt".to_string(), b"Title:Test Image".to_vec());
    assert!(metadata.has_chunk("tEXt"));
    
    let chunk_data = metadata.get_chunk("tEXt");
    assert!(chunk_data.is_some());
    assert_eq!(chunk_data.unwrap(), b"Title:Test Image");
}

#[test]
fn test_pixel_data() {
    let mut pixel_data = PixelData::new(vec![255, 0, 0, 255, 0, 255, 0, 255], DataFormat::RGBA);
    
    assert_eq!(pixel_data.raw_data.len(), 8);
    assert!(pixel_data.rgba_data.is_none());
    assert!(!pixel_data.is_modified);
    
    // Test pixel access
    let metadata = PNGMetadata::new(2, 1, 6, 8);
    let pixel = pixel_data.get_pixel(0, 0, &metadata).unwrap();
    assert_eq!(pixel, (255, 0, 0, 255));
    
    let pixel = pixel_data.get_pixel(1, 0, &metadata).unwrap();
    assert_eq!(pixel, (0, 255, 0, 255));
    
    // Test pixel modification
    pixel_data.set_pixel(0, 0, 0, 0, 255, 255, &metadata).unwrap();
    assert!(pixel_data.is_modified);
    
    let pixel = pixel_data.get_pixel(0, 0, &metadata).unwrap();
    assert_eq!(pixel, (0, 0, 255, 255));
}

#[test]
fn test_operation_state() {
    let mut state = OperationState::new();
    
    assert!(state.is_readable);
    assert!(state.is_writable);
    assert!(!state.is_parsed);
    assert!(!state.is_modified);
    assert!(!state.has_errors);
    assert!(state.error_message.is_none());
    
    state.mark_parsed();
    assert!(state.is_parsed);
    
    state.mark_modified();
    assert!(state.is_modified);
    
    state.set_error("Test error".to_string());
    assert!(state.has_errors);
    assert_eq!(state.error_message, Some("Test error".to_string()));
    
    state.clear_errors();
    assert!(!state.has_errors);
    assert!(state.error_message.is_none());
}

#[test]
fn test_png_statistics() {
    let mut stats = PNGStatistics::new();
    
    assert_eq!(stats.file_size, 0);
    assert_eq!(stats.pixel_count, 0);
    assert_eq!(stats.compression_ratio, 0.0);
    assert_eq!(stats.color_entropy, 0.0);
    assert_eq!(stats.unique_colors, 0);
    assert_eq!(stats.transparency_ratio, 0.0);
    assert_eq!(stats.processing_time, 0);
    
    // Test with sample data
    let metadata = PNGMetadata::new(2, 2, 6, 8);
    let pixel_data = PixelData::new(vec![255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 255, 255, 255, 255], DataFormat::RGBA);
    
    stats.calculate_from_data(&metadata, &pixel_data);
    
    assert_eq!(stats.pixel_count, 4);
    assert!(stats.file_size > 0);
    assert!(stats.compression_ratio > 0.0);
    assert!(stats.color_entropy > 0.0);
    assert!(stats.unique_colors > 0);
}

#[test]
fn test_semantic_png_creation() {
    let png = SemanticPNG::new(None);
    
    assert_eq!(png.width(), 0);
    assert_eq!(png.height(), 0);
    assert!(png.readable());
    assert!(png.writable());
}

#[test]
fn test_semantic_png_with_options() {
    let options = serde_json::json!({
        "width": 800,
        "height": 600,
        "fill": true
    });
    
    let png = SemanticPNG::new(Some(serde_wasm_bindgen::to_value(&options).unwrap()));
    
    assert_eq!(png.width(), 800);
    assert_eq!(png.height(), 600);
}

#[test]
fn test_semantic_png_get_dimensions() {
    let png = SemanticPNG::new(None);
    let dims = png.get_dimensions().unwrap();
    
    // 验证返回的对象包含正确的属性
    assert!(js_sys::Reflect::has(&dims, &"width".into()).unwrap());
    assert!(js_sys::Reflect::has(&dims, &"height".into()).unwrap());
    assert!(js_sys::Reflect::has(&dims, &"aspectRatio".into()).unwrap());
    assert!(js_sys::Reflect::has(&dims, &"totalPixels".into()).unwrap());
    assert!(js_sys::Reflect::has(&dims, &"isSquare".into()).unwrap());
    assert!(js_sys::Reflect::has(&dims, &"isLandscape".into()).unwrap());
    assert!(js_sys::Reflect::has(&dims, &"isPortrait".into()).unwrap());
}

#[test]
fn test_semantic_png_get_color_info() {
    let png = SemanticPNG::new(None);
    let color_info = png.get_color_info().unwrap();
    
    // 验证返回的对象包含正确的属性
    assert!(js_sys::Reflect::has(&color_info, &"colorType".into()).unwrap());
    assert!(js_sys::Reflect::has(&color_info, &"bitDepth".into()).unwrap());
    assert!(js_sys::Reflect::has(&color_info, &"hasAlpha".into()).unwrap());
    assert!(js_sys::Reflect::has(&color_info, &"hasTransparency".into()).unwrap());
    assert!(js_sys::Reflect::has(&color_info, &"channels".into()).unwrap());
    assert!(js_sys::Reflect::has(&color_info, &"bytesPerPixel".into()).unwrap());
    assert!(js_sys::Reflect::has(&color_info, &"isIndexed".into()).unwrap());
    assert!(js_sys::Reflect::has(&color_info, &"isTruecolor".into()).unwrap());
}

#[test]
fn test_semantic_png_get_operation_state() {
    let png = SemanticPNG::new(None);
    let state = png.get_operation_state().unwrap();
    
    // 验证返回的对象包含正确的属性
    assert!(js_sys::Reflect::has(&state, &"isReadable".into()).unwrap());
    assert!(js_sys::Reflect::has(&state, &"isWritable".into()).unwrap());
    assert!(js_sys::Reflect::has(&state, &"isParsed".into()).unwrap());
    assert!(js_sys::Reflect::has(&state, &"isModified".into()).unwrap());
    assert!(js_sys::Reflect::has(&state, &"hasErrors".into()).unwrap());
}

#[test]
fn test_semantic_png_get_statistics() {
    let png = SemanticPNG::new(None);
    let stats = png.get_statistics().unwrap();
    
    // 验证返回的对象包含正确的属性
    assert!(js_sys::Reflect::has(&stats, &"fileSize".into()).unwrap());
    assert!(js_sys::Reflect::has(&stats, &"pixelCount".into()).unwrap());
    assert!(js_sys::Reflect::has(&stats, &"compressionRatio".into()).unwrap());
    assert!(js_sys::Reflect::has(&stats, &"colorEntropy".into()).unwrap());
    assert!(js_sys::Reflect::has(&stats, &"uniqueColors".into()).unwrap());
    assert!(js_sys::Reflect::has(&stats, &"transparencyRatio".into()).unwrap());
    assert!(js_sys::Reflect::has(&stats, &"processingTime".into()).unwrap());
}
