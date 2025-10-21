//! 可扩展滤镜处理模块
//! 支持自定义滤镜和插件式架构

use crate::constants::*;
use std::collections::HashMap;
use std::sync::Arc;

/// 滤镜上下文信息
#[derive(Debug, Clone)]
pub struct FilterContext {
    pub width: usize,
    pub height: usize,
    pub bytes_per_pixel: usize,
    pub row_index: usize,
    pub column_index: usize,
    pub previous_row: Option<Vec<u8>>,
}

/// 滤镜处理结果
#[derive(Debug, Clone)]
pub struct FilterResult {
    pub filtered_data: Vec<u8>,
    pub compression_ratio: f64,
    pub processing_time: u64, // 微秒
}

/// 滤镜trait - 定义滤镜的基本接口
pub trait Filter: Send + Sync {
    /// 滤镜名称
    fn name(&self) -> &str;
    
    /// 滤镜类型ID
    fn filter_type(&self) -> u8;
    
    /// 应用滤镜（解码时使用）
    fn apply(&self, data: &mut [u8], context: &FilterContext) -> Result<(), String>;
    
    /// 反向应用滤镜（编码时使用）
    fn reverse(&self, data: &mut [u8], context: &FilterContext) -> Result<(), String>;
    
    /// 计算滤镜的压缩效果
    fn calculate_compression_ratio(&self, data: &[u8], context: &FilterContext) -> f64;
    
    /// 滤镜是否支持并行处理
    fn supports_parallel(&self) -> bool {
        true
    }
    
    /// 滤镜的优先级（用于自动选择）
    fn priority(&self) -> u8 {
        50 // 默认优先级
    }
}

/// 标准PNG滤镜实现
pub struct StandardFilter {
    filter_type: u8,
    name: String,
}

impl StandardFilter {
    pub fn new(filter_type: u8) -> Self {
        let name = match filter_type {
            FILTER_NONE => "None",
            FILTER_SUB => "Sub",
            FILTER_UP => "Up", 
            FILTER_AVERAGE => "Average",
            FILTER_PAETH => "Paeth",
            _ => "Unknown",
        }.to_string();
        
        Self { filter_type, name }
    }
}

impl Filter for StandardFilter {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn filter_type(&self) -> u8 {
        self.filter_type
    }
    
    fn apply(&self, data: &mut [u8], context: &FilterContext) -> Result<(), String> {
        match self.filter_type {
            FILTER_NONE => Ok(()),
            FILTER_SUB => self.apply_sub_filter(data, context),
            FILTER_UP => self.apply_up_filter(data, context),
            FILTER_AVERAGE => self.apply_average_filter(data, context),
            FILTER_PAETH => self.apply_paeth_filter(data, context),
            _ => Err(format!("Unsupported filter type: {}", self.filter_type)),
        }
    }
    
    fn reverse(&self, data: &mut [u8], context: &FilterContext) -> Result<(), String> {
        match self.filter_type {
            FILTER_NONE => Ok(()),
            FILTER_SUB => self.reverse_sub_filter(data, context),
            FILTER_UP => self.reverse_up_filter(data, context),
            FILTER_AVERAGE => self.reverse_average_filter(data, context),
            FILTER_PAETH => self.reverse_paeth_filter(data, context),
            _ => Err(format!("Unsupported filter type: {}", self.filter_type)),
        }
    }
    
    fn calculate_compression_ratio(&self, data: &[u8], context: &FilterContext) -> f64 {
        // 简化的压缩比计算
        let mut test_data = data.to_vec();
        if let Ok(_) = self.apply(&mut test_data, context) {
            // 计算熵值作为压缩效果的指标
            self.calculate_entropy(&test_data)
        } else {
            1.0
        }
    }
}

impl StandardFilter {
    fn apply_sub_filter(&self, data: &mut [u8], context: &FilterContext) -> Result<(), String> {
        let bytes_per_row = context.width * context.bytes_per_pixel;
        let row_start = context.row_index * bytes_per_row;
        let row_end = row_start + bytes_per_row;
        
        if row_end > data.len() {
            return Err("Row index out of bounds".to_string());
        }
        
        let row = &mut data[row_start..row_end];
        for x in context.bytes_per_pixel..bytes_per_row {
            row[x] = row[x].wrapping_add(row[x - context.bytes_per_pixel]);
        }
        Ok(())
    }
    
    fn reverse_sub_filter(&self, data: &mut [u8], context: &FilterContext) -> Result<(), String> {
        let bytes_per_row = context.width * context.bytes_per_pixel;
        let row_start = context.row_index * bytes_per_row;
        let row_end = row_start + bytes_per_row;
        
        if row_end > data.len() {
            return Err("Row index out of bounds".to_string());
        }
        
        let row = &mut data[row_start..row_end];
        for x in context.bytes_per_pixel..bytes_per_row {
            row[x] = row[x].wrapping_sub(row[x - context.bytes_per_pixel]);
        }
        Ok(())
    }
    
    fn apply_up_filter(&self, data: &mut [u8], context: &FilterContext) -> Result<(), String> {
        if context.row_index == 0 {
            return Ok(());
        }
        
        let bytes_per_row = context.width * context.bytes_per_pixel;
        let row_start = context.row_index * bytes_per_row;
        let row_end = row_start + bytes_per_row;
        let prev_row_start = (context.row_index - 1) * bytes_per_row;
        
        if row_end > data.len() || prev_row_start + bytes_per_row > data.len() {
            return Err("Row index out of bounds".to_string());
        }
        
        let row = &mut data[row_start..row_end];
        let prev_row = &data[prev_row_start..prev_row_start + bytes_per_row];
        
        for x in 0..bytes_per_row {
            row[x] = row[x].wrapping_add(prev_row[x]);
        }
        Ok(())
    }
    
    fn reverse_up_filter(&self, data: &mut [u8], context: &FilterContext) -> Result<(), String> {
        if context.row_index == 0 {
            return Ok(());
        }
        
        let bytes_per_row = context.width * context.bytes_per_pixel;
        let row_start = context.row_index * bytes_per_row;
        let row_end = row_start + bytes_per_row;
        let prev_row_start = (context.row_index - 1) * bytes_per_row;
        
        if row_end > data.len() || prev_row_start + bytes_per_row > data.len() {
            return Err("Row index out of bounds".to_string());
        }
        
        let row = &mut data[row_start..row_end];
        let prev_row = &data[prev_row_start..prev_row_start + bytes_per_row];
        
        for x in 0..bytes_per_row {
            row[x] = row[x].wrapping_sub(prev_row[x]);
        }
        Ok(())
    }
    
    fn apply_average_filter(&self, data: &mut [u8], context: &FilterContext) -> Result<(), String> {
        let bytes_per_row = context.width * context.bytes_per_pixel;
        let row_start = context.row_index * bytes_per_row;
        let row_end = row_start + bytes_per_row;
        
        if row_end > data.len() {
            return Err("Row index out of bounds".to_string());
        }
        
        let row = &mut data[row_start..row_end];
        
        for x in 0..bytes_per_row {
            let left = if x >= context.bytes_per_pixel { row[x - context.bytes_per_pixel] } else { 0 };
            let up = if context.row_index > 0 && x < data.len() - (context.row_index - 1) * bytes_per_row {
                data[(context.row_index - 1) * bytes_per_row + x]
            } else { 0 };
            let average = ((left as u16 + up as u16) / 2) as u8;
            row[x] = row[x].wrapping_add(average);
        }
        Ok(())
    }
    
    fn reverse_average_filter(&self, data: &mut [u8], context: &FilterContext) -> Result<(), String> {
        let bytes_per_row = context.width * context.bytes_per_pixel;
        let row_start = context.row_index * bytes_per_row;
        let row_end = row_start + bytes_per_row;
        
        if row_end > data.len() {
            return Err("Row index out of bounds".to_string());
        }
        
        let row = &mut data[row_start..row_end];
        
        for x in 0..bytes_per_row {
            let left = if x >= context.bytes_per_pixel { row[x - context.bytes_per_pixel] } else { 0 };
            let up = if context.row_index > 0 && x < data.len() - (context.row_index - 1) * bytes_per_row {
                data[(context.row_index - 1) * bytes_per_row + x]
            } else { 0 };
            let average = ((left as u16 + up as u16) / 2) as u8;
            row[x] = row[x].wrapping_sub(average);
        }
        Ok(())
    }
    
    fn apply_paeth_filter(&self, data: &mut [u8], context: &FilterContext) -> Result<(), String> {
        let bytes_per_row = context.width * context.bytes_per_pixel;
        let row_start = context.row_index * bytes_per_row;
        let row_end = row_start + bytes_per_row;
        
        if row_end > data.len() {
            return Err("Row index out of bounds".to_string());
        }
        
        let row = &mut data[row_start..row_end];
        
        for x in 0..bytes_per_row {
            let left = if x >= context.bytes_per_pixel { row[x - context.bytes_per_pixel] } else { 0 };
            let up = if context.row_index > 0 && x < data.len() - (context.row_index - 1) * bytes_per_row {
                data[(context.row_index - 1) * bytes_per_row + x]
            } else { 0 };
            let up_left = if context.row_index > 0 && x >= context.bytes_per_pixel && 
                           x < data.len() - (context.row_index - 1) * bytes_per_row {
                data[(context.row_index - 1) * bytes_per_row + x - context.bytes_per_pixel]
            } else { 0 };
            
            let predictor = self.paeth_predictor(left, up, up_left);
            row[x] = row[x].wrapping_add(predictor);
        }
        Ok(())
    }
    
    fn reverse_paeth_filter(&self, data: &mut [u8], context: &FilterContext) -> Result<(), String> {
        let bytes_per_row = context.width * context.bytes_per_pixel;
        let row_start = context.row_index * bytes_per_row;
        let row_end = row_start + bytes_per_row;
        
        if row_end > data.len() {
            return Err("Row index out of bounds".to_string());
        }
        
        let row = &mut data[row_start..row_end];
        
        for x in 0..bytes_per_row {
            let left = if x >= context.bytes_per_pixel { row[x - context.bytes_per_pixel] } else { 0 };
            let up = if context.row_index > 0 && x < data.len() - (context.row_index - 1) * bytes_per_row {
                data[(context.row_index - 1) * bytes_per_row + x]
            } else { 0 };
            let up_left = if context.row_index > 0 && x >= context.bytes_per_pixel && 
                           x < data.len() - (context.row_index - 1) * bytes_per_row {
                data[(context.row_index - 1) * bytes_per_row + x - context.bytes_per_pixel]
            } else { 0 };
            
            let predictor = self.paeth_predictor(left, up, up_left);
            row[x] = row[x].wrapping_sub(predictor);
        }
        Ok(())
    }
    
    fn paeth_predictor(&self, a: u8, b: u8, c: u8) -> u8 {
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
    
    fn calculate_entropy(&self, data: &[u8]) -> f64 {
        let mut histogram = [0u32; 256];
        for &byte in data {
            histogram[byte as usize] += 1;
        }
        
        let total = data.len() as f64;
        let mut entropy = 0.0;
        
        for &count in &histogram {
            if count > 0 {
                let probability = count as f64 / total;
                entropy -= probability * probability.log2();
            }
        }
        
        entropy
    }
}

/// 滤镜注册表
pub struct FilterRegistry {
    filters: HashMap<u8, Arc<dyn Filter>>,
    custom_filters: HashMap<String, Arc<dyn Filter>>,
}

impl FilterRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            filters: HashMap::new(),
            custom_filters: HashMap::new(),
        };
        
        // 注册标准滤镜
        registry.register_standard_filters();
        registry
    }
    
    fn register_standard_filters(&mut self) {
        for filter_type in [FILTER_NONE, FILTER_SUB, FILTER_UP, FILTER_AVERAGE, FILTER_PAETH] {
            let filter = Arc::new(StandardFilter::new(filter_type));
            self.filters.insert(filter_type, filter);
        }
    }
    
    /// 注册自定义滤镜
    pub fn register_custom_filter(&mut self, name: String, filter: Arc<dyn Filter>) {
        self.custom_filters.insert(name, filter);
    }
    
    /// 获取滤镜
    pub fn get_filter(&self, filter_type: u8) -> Option<Arc<dyn Filter>> {
        self.filters.get(&filter_type).cloned()
    }
    
    /// 获取自定义滤镜
    pub fn get_custom_filter(&self, name: &str) -> Option<Arc<dyn Filter>> {
        self.custom_filters.get(name).cloned()
    }
    
    /// 获取所有可用滤镜
    pub fn get_all_filters(&self) -> Vec<Arc<dyn Filter>> {
        let mut all_filters = Vec::new();
        all_filters.extend(self.filters.values().cloned());
        all_filters.extend(self.custom_filters.values().cloned());
        all_filters
    }
    
    /// 选择最佳滤镜
    pub fn choose_best_filter(&self, data: &[u8], context: &FilterContext) -> Option<Arc<dyn Filter>> {
        let mut best_filter = None;
        let mut best_ratio = 0.0;
        
        for filter in self.get_all_filters() {
            let ratio = filter.calculate_compression_ratio(data, context);
            if ratio > best_ratio {
                best_ratio = ratio;
                best_filter = Some(filter);
            }
        }
        
        best_filter
    }
}

/// 滤镜处理器
pub struct FilterProcessor {
    registry: FilterRegistry,
}

impl FilterProcessor {
    pub fn new() -> Self {
        Self {
            registry: FilterRegistry::new(),
        }
    }
    
    /// 应用滤镜
    pub fn apply_filter(&self, filter_type: u8, data: &mut [u8], context: &FilterContext) -> Result<(), String> {
        if let Some(filter) = self.registry.get_filter(filter_type) {
            filter.apply(data, context)
        } else {
            Err(format!("Filter type {} not found", filter_type))
        }
    }
    
    /// 反向应用滤镜
    pub fn reverse_filter(&self, filter_type: u8, data: &mut [u8], context: &FilterContext) -> Result<(), String> {
        if let Some(filter) = self.registry.get_filter(filter_type) {
            filter.reverse(data, context)
        } else {
            Err(format!("Filter type {} not found", filter_type))
        }
    }
    
    /// 选择最佳滤镜
    pub fn choose_best_filter(&self, data: &[u8], context: &FilterContext) -> Option<u8> {
        self.registry.choose_best_filter(data, context)
            .map(|filter| filter.filter_type())
    }
    
    /// 注册自定义滤镜
    pub fn register_custom_filter(&mut self, name: String, filter: Arc<dyn Filter>) {
        self.registry.register_custom_filter(name, filter);
    }
    
    /// 获取滤镜信息
    pub fn get_filter_info(&self, filter_type: u8) -> Option<String> {
        self.registry.get_filter(filter_type)
            .map(|filter| format!("{} (type: {})", filter.name(), filter.filter_type()))
    }
}
