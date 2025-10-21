//! 自定义滤镜实现示例
//! 展示如何创建和注册自定义滤镜

use crate::filter_extensible::{Filter, FilterContext};
use std::sync::Arc;

/// 自适应滤镜 - 根据图像内容自动选择最佳策略
pub struct AdaptiveFilter {
    name: String,
    filter_type: u8,
}

impl AdaptiveFilter {
    pub fn new() -> Self {
        Self {
            name: "Adaptive".to_string(),
            filter_type: 10, // 自定义滤镜类型
        }
    }
}

impl Filter for AdaptiveFilter {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn filter_type(&self) -> u8 {
        self.filter_type
    }
    
    fn apply(&self, data: &mut [u8], context: &FilterContext) -> Result<(), String> {
        // 分析图像内容，选择最佳滤镜策略
        let strategy = self.analyze_image_content(data, context);
        
        match strategy {
            FilterStrategy::Horizontal => self.apply_horizontal_filter(data, context),
            FilterStrategy::Vertical => self.apply_vertical_filter(data, context),
            FilterStrategy::Diagonal => self.apply_diagonal_filter(data, context),
            FilterStrategy::Gradient => self.apply_gradient_filter(data, context),
        }
    }
    
    fn reverse(&self, data: &mut [u8], context: &FilterContext) -> Result<(), String> {
        // 反向应用相同的策略
        let strategy = self.analyze_image_content(data, context);
        
        match strategy {
            FilterStrategy::Horizontal => self.reverse_horizontal_filter(data, context),
            FilterStrategy::Vertical => self.reverse_vertical_filter(data, context),
            FilterStrategy::Diagonal => self.reverse_diagonal_filter(data, context),
            FilterStrategy::Gradient => self.reverse_gradient_filter(data, context),
        }
    }
    
    fn calculate_compression_ratio(&self, data: &[u8], context: &FilterContext) -> f64 {
        // 计算自适应滤镜的压缩效果
        let strategy = self.analyze_image_content(data, context);
        self.calculate_strategy_compression(data, context, strategy)
    }
    
    fn priority(&self) -> u8 {
        80 // 高优先级
    }
}

impl AdaptiveFilter {
    fn analyze_image_content(&self, data: &[u8], context: &FilterContext) -> FilterStrategy {
        let bytes_per_row = context.width * context.bytes_per_pixel;
        let mut horizontal_variance = 0.0;
        let mut vertical_variance = 0.0;
        let mut diagonal_variance = 0.0;
        
        // 分析水平变化
        for y in 0..context.height {
            for x in 1..context.width {
                let idx1 = (y * context.width + x - 1) * context.bytes_per_pixel;
                let idx2 = (y * context.width + x) * context.bytes_per_pixel;
                
                if idx2 + context.bytes_per_pixel <= data.len() {
                    for i in 0..context.bytes_per_pixel {
                        let diff = (data[idx2 + i] as i16 - data[idx1 + i] as i16).abs() as f64;
                        horizontal_variance += diff;
                    }
                }
            }
        }
        
        // 分析垂直变化
        for y in 1..context.height {
            for x in 0..context.width {
                let idx1 = ((y - 1) * context.width + x) * context.bytes_per_pixel;
                let idx2 = (y * context.width + x) * context.bytes_per_pixel;
                
                if idx2 + context.bytes_per_pixel <= data.len() {
                    for i in 0..context.bytes_per_pixel {
                        let diff = (data[idx2 + i] as i16 - data[idx1 + i] as i16).abs() as f64;
                        vertical_variance += diff;
                    }
                }
            }
        }
        
        // 分析对角线变化
        for y in 1..context.height {
            for x in 1..context.width {
                let idx1 = ((y - 1) * context.width + x - 1) * context.bytes_per_pixel;
                let idx2 = (y * context.width + x) * context.bytes_per_pixel;
                
                if idx2 + context.bytes_per_pixel <= data.len() {
                    for i in 0..context.bytes_per_pixel {
                        let diff = (data[idx2 + i] as i16 - data[idx1 + i] as i16).abs() as f64;
                        diagonal_variance += diff;
                    }
                }
            }
        }
        
        // 选择最佳策略
        if horizontal_variance < vertical_variance && horizontal_variance < diagonal_variance {
            FilterStrategy::Horizontal
        } else if vertical_variance < diagonal_variance {
            FilterStrategy::Vertical
        } else if diagonal_variance < 100.0 {
            FilterStrategy::Diagonal
        } else {
            FilterStrategy::Gradient
        }
    }
    
    fn apply_horizontal_filter(&self, data: &mut [u8], context: &FilterContext) -> Result<(), String> {
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
    
    fn reverse_horizontal_filter(&self, data: &mut [u8], context: &FilterContext) -> Result<(), String> {
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
    
    fn apply_vertical_filter(&self, data: &mut [u8], context: &FilterContext) -> Result<(), String> {
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
    
    fn reverse_vertical_filter(&self, data: &mut [u8], context: &FilterContext) -> Result<(), String> {
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
    
    fn apply_diagonal_filter(&self, data: &mut [u8], context: &FilterContext) -> Result<(), String> {
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
            
            // 对角线预测器
            let predictor = ((left as u16 + up as u16 + up_left as u16) / 3) as u8;
            row[x] = row[x].wrapping_add(predictor);
        }
        Ok(())
    }
    
    fn reverse_diagonal_filter(&self, data: &mut [u8], context: &FilterContext) -> Result<(), String> {
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
            
            let predictor = ((left as u16 + up as u16 + up_left as u16) / 3) as u8;
            row[x] = row[x].wrapping_sub(predictor);
        }
        Ok(())
    }
    
    fn apply_gradient_filter(&self, data: &mut [u8], context: &FilterContext) -> Result<(), String> {
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
            
            // 梯度预测器
            let gradient = ((left as i16 + up as i16) / 2) as u8;
            row[x] = row[x].wrapping_add(gradient);
        }
        Ok(())
    }
    
    fn reverse_gradient_filter(&self, data: &mut [u8], context: &FilterContext) -> Result<(), String> {
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
            
            let gradient = ((left as i16 + up as i16) / 2) as u8;
            row[x] = row[x].wrapping_sub(gradient);
        }
        Ok(())
    }
    
    fn calculate_strategy_compression(&self, data: &[u8], context: &FilterContext, strategy: FilterStrategy) -> f64 {
        // 根据策略计算压缩效果
        match strategy {
            FilterStrategy::Horizontal => 0.8,
            FilterStrategy::Vertical => 0.7,
            FilterStrategy::Diagonal => 0.9,
            FilterStrategy::Gradient => 0.6,
        }
    }
}

/// 滤镜策略枚举
#[derive(Debug, Clone, Copy)]
enum FilterStrategy {
    Horizontal,
    Vertical,
    Diagonal,
    Gradient,
}

/// 边缘检测滤镜
pub struct EdgeDetectionFilter {
    name: String,
    filter_type: u8,
    threshold: u8,
}

impl EdgeDetectionFilter {
    pub fn new(threshold: u8) -> Self {
        Self {
            name: "EdgeDetection".to_string(),
            filter_type: 11,
            threshold,
        }
    }
}

impl Filter for EdgeDetectionFilter {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn filter_type(&self) -> u8 {
        self.filter_type
    }
    
    fn apply(&self, data: &mut [u8], context: &FilterContext) -> Result<(), String> {
        let bytes_per_row = context.width * context.bytes_per_pixel;
        let row_start = context.row_index * bytes_per_row;
        let row_end = row_start + bytes_per_row;
        
        if row_end > data.len() {
            return Err("Row index out of bounds".to_string());
        }
        
        let row = &mut data[row_start..row_end];
        
        for x in 0..bytes_per_row {
            let current = row[x];
            let left = if x >= context.bytes_per_pixel { row[x - context.bytes_per_pixel] } else { current };
            let up = if context.row_index > 0 && x < data.len() - (context.row_index - 1) * bytes_per_row {
                data[(context.row_index - 1) * bytes_per_row + x]
            } else { current };
            
            // 边缘检测算法
            let edge_strength = ((current as i16 - left as i16).abs() + 
                                (current as i16 - up as i16).abs()) as u8;
            
            if edge_strength > self.threshold {
                row[x] = row[x].wrapping_add(edge_strength);
            }
        }
        Ok(())
    }
    
    fn reverse(&self, data: &mut [u8], context: &FilterContext) -> Result<(), String> {
        let bytes_per_row = context.width * context.bytes_per_pixel;
        let row_start = context.row_index * bytes_per_row;
        let row_end = row_start + bytes_per_row;
        
        if row_end > data.len() {
            return Err("Row index out of bounds".to_string());
        }
        
        let row = &mut data[row_start..row_end];
        
        for x in 0..bytes_per_row {
            let current = row[x];
            let left = if x >= context.bytes_per_pixel { row[x - context.bytes_per_pixel] } else { current };
            let up = if context.row_index > 0 && x < data.len() - (context.row_index - 1) * bytes_per_row {
                data[(context.row_index - 1) * bytes_per_row + x]
            } else { current };
            
            let edge_strength = ((current as i16 - left as i16).abs() + 
                                (current as i16 - up as i16).abs()) as u8;
            
            if edge_strength > self.threshold {
                row[x] = row[x].wrapping_sub(edge_strength);
            }
        }
        Ok(())
    }
    
    fn calculate_compression_ratio(&self, data: &[u8], context: &FilterContext) -> f64 {
        // 计算边缘检测滤镜的压缩效果
        let mut edge_count = 0;
        let bytes_per_row = context.width * context.bytes_per_pixel;
        
        for y in 0..context.height {
            for x in 0..context.width {
                let idx = (y * context.width + x) * context.bytes_per_pixel;
                if idx + context.bytes_per_pixel <= data.len() {
                    let current = data[idx];
                    let left = if x > 0 { data[idx - context.bytes_per_pixel] } else { current };
                    let up = if y > 0 { data[idx - bytes_per_row] } else { current };
                    
                    let edge_strength = ((current as i16 - left as i16).abs() + 
                                        (current as i16 - up as i16).abs()) as u8;
                    
                    if edge_strength > self.threshold {
                        edge_count += 1;
                    }
                }
            }
        }
        
        let total_pixels = context.width * context.height;
        if total_pixels > 0 {
            1.0 - (edge_count as f64 / total_pixels as f64)
        } else {
            1.0
        }
    }
    
    fn priority(&self) -> u8 {
        70 // 中等优先级
    }
}
