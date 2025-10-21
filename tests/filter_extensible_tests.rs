//! 可扩展滤镜系统测试

use rust_png::filter_extensible::*;
use rust_png::custom_filters::*;
use std::sync::Arc;

#[test]
fn test_filter_trait_implementation() {
    // 测试标准滤镜实现
    let filter = StandardFilter::new(FILTER_SUB);
    assert_eq!(filter.name(), "Sub");
    assert_eq!(filter.filter_type(), FILTER_SUB);
    assert!(filter.supports_parallel());
    assert_eq!(filter.priority(), 50);
}

#[test]
fn test_filter_registry() {
    let mut registry = FilterRegistry::new();
    
    // 测试获取标准滤镜
    assert!(registry.get_filter(FILTER_NONE).is_some());
    assert!(registry.get_filter(FILTER_SUB).is_some());
    assert!(registry.get_filter(FILTER_UP).is_some());
    assert!(registry.get_filter(FILTER_AVERAGE).is_some());
    assert!(registry.get_filter(FILTER_PAETH).is_some());
    
    // 测试获取不存在的滤镜
    assert!(registry.get_filter(99).is_none());
}

#[test]
fn test_custom_filter_registration() {
    let mut registry = FilterRegistry::new();
    
    // 注册自定义滤镜
    let adaptive_filter = Arc::new(AdaptiveFilter::new());
    registry.register_custom_filter("adaptive".to_string(), adaptive_filter);
    
    // 测试获取自定义滤镜
    assert!(registry.get_custom_filter("adaptive").is_some());
    assert!(registry.get_custom_filter("nonexistent").is_none());
}

#[test]
fn test_filter_processor() {
    let processor = FilterProcessor::new();
    
    // 测试滤镜信息获取
    assert!(processor.get_filter_info(FILTER_NONE).is_some());
    assert!(processor.get_filter_info(FILTER_SUB).is_some());
    assert!(processor.get_filter_info(99).is_none());
}

#[test]
fn test_adaptive_filter() {
    let filter = AdaptiveFilter::new();
    assert_eq!(filter.name(), "Adaptive");
    assert_eq!(filter.filter_type(), 10);
    assert_eq!(filter.priority(), 80);
    
    // 测试滤镜上下文
    let context = FilterContext {
        width: 8,
        height: 8,
        bytes_per_pixel: 3,
        row_index: 0,
        column_index: 0,
        previous_row: None,
    };
    
    // 创建测试数据
    let mut test_data = vec![0; 8 * 8 * 3];
    for i in 0..test_data.len() {
        test_data[i] = (i % 256) as u8;
    }
    
    // 测试压缩比计算
    let ratio = filter.calculate_compression_ratio(&test_data, &context);
    assert!(ratio > 0.0);
}

#[test]
fn test_edge_detection_filter() {
    let filter = EdgeDetectionFilter::new(10);
    assert_eq!(filter.name(), "EdgeDetection");
    assert_eq!(filter.filter_type(), 11);
    assert_eq!(filter.priority(), 70);
    
    let context = FilterContext {
        width: 4,
        height: 4,
        bytes_per_pixel: 3,
        row_index: 0,
        column_index: 0,
        previous_row: None,
    };
    
    // 创建有边缘的测试数据
    let mut test_data = vec![0; 4 * 4 * 3];
    for y in 0..4 {
        for x in 0..4 {
            let idx = (y * 4 + x) * 3;
            if x == 2 || y == 2 { // 创建边缘
                test_data[idx] = 255;
                test_data[idx + 1] = 255;
                test_data[idx + 2] = 255;
            }
        }
    }
    
    let ratio = filter.calculate_compression_ratio(&test_data, &context);
    assert!(ratio > 0.0);
}

#[test]
fn test_parallel_filter_processor() {
    let processor = ParallelFilterProcessor::new(Some(2));
    
    let context = FilterContext {
        width: 8,
        height: 8,
        bytes_per_pixel: 3,
        row_index: 0,
        column_index: 0,
        previous_row: None,
    };
    
    let mut test_data = vec![0; 8 * 8 * 3];
    for i in 0..test_data.len() {
        test_data[i] = (i % 256) as u8;
    }
    
    // 测试并行滤镜处理
    let result = processor.apply_filter_parallel(FILTER_SUB, &mut test_data, &context);
    assert!(result.is_ok());
}

#[test]
fn test_filter_profiler() {
    let mut profiler = FilterProfiler::new();
    
    let context = FilterContext {
        width: 4,
        height: 4,
        bytes_per_pixel: 3,
        row_index: 0,
        column_index: 0,
        previous_row: None,
    };
    
    let test_data = vec![0; 4 * 4 * 3];
    
    // 测试性能分析
    let measurement = profiler.profile_filter(FILTER_NONE, &test_data, &context);
    assert_eq!(measurement.filter_type, FILTER_NONE);
    assert!(measurement.processing_time > 0);
    
    // 测试性能报告
    let report = profiler.get_performance_report();
    assert_eq!(report.total_measurements, 1);
    assert!(report.average_processing_time > 0);
}

#[test]
fn test_filter_cache() {
    let mut cache = FilterCache::new(10);
    
    let key = "test_filter_123".to_string();
    let result = CachedFilterResult {
        filtered_data: vec![1, 2, 3, 4],
        compression_ratio: 0.8,
        timestamp: std::time::SystemTime::now(),
    };
    
    // 测试缓存存储和检索
    cache.cache_result(key.clone(), result.clone());
    assert!(cache.get_cached_result(&key).is_some());
    
    // 测试缓存键生成
    let context = FilterContext {
        width: 8,
        height: 8,
        bytes_per_pixel: 3,
        row_index: 0,
        column_index: 0,
        previous_row: None,
    };
    
    let cache_key = cache.generate_cache_key(FILTER_SUB, 12345, &context);
    assert!(cache_key.contains("filter_1_12345_8x8"));
}

#[test]
fn test_smart_filter_selector() {
    let mut selector = SmartFilterSelector::new();
    
    let context = FilterContext {
        width: 8,
        height: 8,
        bytes_per_pixel: 3,
        row_index: 0,
        column_index: 0,
        previous_row: None,
    };
    
    let test_data = vec![0; 8 * 8 * 3];
    
    // 测试智能滤镜选择
    let best_filter = selector.select_best_filter(&test_data, &context);
    assert!(best_filter <= FILTER_PAETH);
}

#[test]
fn test_filter_context_validation() {
    let context = FilterContext {
        width: 8,
        height: 8,
        bytes_per_pixel: 3,
        row_index: 0,
        column_index: 0,
        previous_row: None,
    };
    
    // 测试上下文信息
    assert_eq!(context.width, 8);
    assert_eq!(context.height, 8);
    assert_eq!(context.bytes_per_pixel, 3);
    assert_eq!(context.row_index, 0);
    assert_eq!(context.column_index, 0);
    assert!(context.previous_row.is_none());
}

#[test]
fn test_filter_error_handling() {
    let processor = FilterProcessor::new();
    
    let context = FilterContext {
        width: 8,
        height: 8,
        bytes_per_pixel: 3,
        row_index: 0,
        column_index: 0,
        previous_row: None,
    };
    
    let mut test_data = vec![0; 4]; // 数据太小
    
    // 测试错误处理
    let result = processor.apply_filter(FILTER_SUB, &mut test_data, &context);
    assert!(result.is_err());
}

#[test]
fn test_filter_compression_ratio_calculation() {
    let filter = StandardFilter::new(FILTER_SUB);
    
    let context = FilterContext {
        width: 4,
        height: 4,
        bytes_per_pixel: 3,
        row_index: 0,
        column_index: 0,
        previous_row: None,
    };
    
    // 创建有规律的数据（应该压缩得很好）
    let mut test_data = vec![0; 4 * 4 * 3];
    for i in 0..test_data.len() {
        test_data[i] = (i % 10) as u8; // 重复模式
    }
    
    let ratio = filter.calculate_compression_ratio(&test_data, &context);
    assert!(ratio > 0.0);
    assert!(ratio <= 1.0);
}

#[test]
fn test_filter_priority_system() {
    let adaptive_filter = AdaptiveFilter::new();
    let edge_filter = EdgeDetectionFilter::new(10);
    let standard_filter = StandardFilter::new(FILTER_SUB);
    
    // 测试优先级
    assert!(adaptive_filter.priority() > standard_filter.priority());
    assert!(edge_filter.priority() > standard_filter.priority());
    assert!(adaptive_filter.priority() > edge_filter.priority());
}

#[test]
fn test_filter_parallel_support() {
    let adaptive_filter = AdaptiveFilter::new();
    let edge_filter = EdgeDetectionFilter::new(10);
    let standard_filter = StandardFilter::new(FILTER_SUB);
    
    // 测试并行支持
    assert!(adaptive_filter.supports_parallel());
    assert!(edge_filter.supports_parallel());
    assert!(standard_filter.supports_parallel());
}
