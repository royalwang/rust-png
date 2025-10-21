//! 高级滤镜算法模块
//! 实现高级PNG滤镜算法和优化

use crate::constants::*;
use crate::filter_extensible::*;

/// 高级滤镜处理器
pub struct AdvancedFilterProcessor {
    filters: Vec<Box<dyn AdvancedFilter>>,
    optimizer: FilterOptimizer,
}

impl AdvancedFilterProcessor {
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
            optimizer: FilterOptimizer::new(),
        }
    }
    
    /// 注册高级滤镜
    pub fn register_filter(&mut self, filter: Box<dyn AdvancedFilter>) {
        self.filters.push(filter);
    }
    
    /// 处理图像数据
    pub fn process_image(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        let mut best_result = data.to_vec();
        let mut best_score = f64::MAX;
        
        for filter in &self.filters {
            let result = filter.process(data, width, height)?;
            let score = self.calculate_score(&result);
            
            if score < best_score {
                best_score = score;
                best_result = result;
            }
        }
        
        Ok(best_result)
    }
    
    /// 计算滤镜分数
    fn calculate_score(&self, data: &[u8]) -> f64 {
        // 计算压缩比和复杂度
        let mut score = 0.0;
        let mut prev = 0u8;
        
        for &byte in data {
            score += (byte as f64 - prev as f64).abs();
            prev = byte;
        }
        
        score / data.len() as f64
    }
}

/// 高级滤镜trait
pub trait AdvancedFilter: Send + Sync {
    fn name(&self) -> &str;
    fn process(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String>;
    fn get_compression_ratio(&self, data: &[u8]) -> f64;
    fn supports_parallel(&self) -> bool;
}

/// 自适应滤镜
pub struct AdaptiveFilter {
    threshold: f64,
    context_aware: bool,
}

impl AdaptiveFilter {
    pub fn new(threshold: f64, context_aware: bool) -> Self {
        Self {
            threshold,
            context_aware,
        }
    }
    
    fn analyze_context(&self, data: &[u8], width: u32, height: u32) -> AnalysisResult {
        let mut complexity = 0.0;
        let mut variance = 0.0;
        let mut edge_density = 0.0;
        
        // 计算复杂度
        for chunk in data.chunks_exact(4) {
            let r = chunk[0] as f64;
            let g = chunk[1] as f64;
            let b = chunk[2] as f64;
            let a = chunk[3] as f64;
            
            complexity += (r - g).abs() + (g - b).abs() + (b - r).abs();
        }
        
        // 计算方差
        let mean = data.iter().map(|&x| x as f64).sum::<f64>() / data.len() as f64;
        for &byte in data {
            variance += (byte as f64 - mean).powi(2);
        }
        variance /= data.len() as f64;
        
        // 计算边缘密度
        edge_density = self.calculate_edge_density(data, width, height);
        
        AnalysisResult {
            complexity,
            variance,
            edge_density,
            mean,
        }
    }
    
    fn calculate_edge_density(&self, data: &[u8], width: u32, height: u32) -> f64 {
        let mut edges = 0;
        let mut total = 0;
        
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                let center = self.get_pixel(data, x, y, width);
                let left = self.get_pixel(data, x - 1, y, width);
                let right = self.get_pixel(data, x + 1, y, width);
                let top = self.get_pixel(data, x, y - 1, width);
                let bottom = self.get_pixel(data, x, y + 1, width);
                
                let gradient = (left - right).abs() + (top - bottom).abs();
                if gradient > self.threshold {
                    edges += 1;
                }
                total += 1;
            }
        }
        
        edges as f64 / total as f64
    }
    
    fn get_pixel(&self, data: &[u8], x: u32, y: u32, width: u32) -> f64 {
        let index = ((y * width + x) * 4) as usize;
        if index + 3 < data.len() {
            (data[index] as f64 + data[index + 1] as f64 + data[index + 2] as f64) / 3.0
        } else {
            0.0
        }
    }
}

impl AdvancedFilter for AdaptiveFilter {
    fn name(&self) -> &str {
        "AdaptiveFilter"
    }
    
    fn process(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        let analysis = self.analyze_context(data, width, height);
        
        // 根据分析结果选择最佳滤镜
        let filter_type = if analysis.complexity > self.threshold {
            if analysis.edge_density > 0.1 {
                FILTER_PAETH
            } else {
                FILTER_AVERAGE
            }
        } else if analysis.variance > 1000.0 {
            FILTER_SUB
        } else {
            FILTER_NONE
        };
        
        // 应用选择的滤镜
        self.apply_filter(data, width, height, filter_type)
    }
    
    fn get_compression_ratio(&self, data: &[u8]) -> f64 {
        // 计算压缩比
        let mut score = 0.0;
        for chunk in data.chunks_exact(4) {
            score += (chunk[0] as f64 + chunk[1] as f64 + chunk[2] as f64) / 3.0;
        }
        score / (data.len() / 4) as f64
    }
    
    fn supports_parallel(&self) -> bool {
        true
    }
}

impl AdaptiveFilter {
    fn apply_filter(&self, data: &[u8], width: u32, height: u32, filter_type: u8) -> Result<Vec<u8>, String> {
        let mut result = Vec::new();
        let bytes_per_row = (width * 4) as usize;
        
        for y in 0..height {
            let row_start = (y * bytes_per_row) as usize;
            let row_end = row_start + bytes_per_row;
            
            if row_end > data.len() {
                return Err("Insufficient data for row".to_string());
            }
            
            let row_data = &data[row_start..row_end];
            let mut filtered_row = row_data.to_vec();
            
            // 应用滤镜
            match filter_type {
                FILTER_NONE => {
                    // 无滤镜
                }
                FILTER_SUB => {
                    self.apply_sub_filter(&mut filtered_row, width as usize);
                }
                FILTER_UP => {
                    if y > 0 {
                        let prev_row_start = ((y - 1) * bytes_per_row) as usize;
                        let prev_row = &data[prev_row_start..prev_row_start + bytes_per_row];
                        self.apply_up_filter(&mut filtered_row, prev_row);
                    }
                }
                FILTER_AVERAGE => {
                    self.apply_average_filter(&mut filtered_row, width as usize, y, &data, bytes_per_row);
                }
                FILTER_PAETH => {
                    self.apply_paeth_filter(&mut filtered_row, width as usize, y, &data, bytes_per_row);
                }
                _ => return Err("Unknown filter type".to_string()),
            }
            
            result.extend_from_slice(&filtered_row);
        }
        
        Ok(result)
    }
    
    fn apply_sub_filter(&self, data: &mut [u8], width: usize) {
        for i in 4..data.len() {
            data[i] = data[i].wrapping_sub(data[i - 4]);
        }
    }
    
    fn apply_up_filter(&self, data: &mut [u8], prev_row: &[u8]) {
        for i in 0..data.len() {
            data[i] = data[i].wrapping_sub(prev_row[i]);
        }
    }
    
    fn apply_average_filter(&self, data: &mut [u8], width: usize, y: u32, full_data: &[u8], bytes_per_row: usize) {
        for i in 0..data.len() {
            let left = if i >= 4 { data[i - 4] } else { 0 };
            let up = if y > 0 {
                let prev_row_start = ((y - 1) * bytes_per_row) as usize;
                if prev_row_start + i < full_data.len() {
                    full_data[prev_row_start + i]
                } else {
                    0
                }
            } else {
                0
            };
            
            data[i] = data[i].wrapping_sub((left + up) / 2);
        }
    }
    
    fn apply_paeth_filter(&self, data: &mut [u8], width: usize, y: u32, full_data: &[u8], bytes_per_row: usize) {
        for i in 0..data.len() {
            let left = if i >= 4 { data[i - 4] } else { 0 };
            let up = if y > 0 {
                let prev_row_start = ((y - 1) * bytes_per_row) as usize;
                if prev_row_start + i < full_data.len() {
                    full_data[prev_row_start + i]
                } else {
                    0
                }
            } else {
                0
            };
            let up_left = if y > 0 && i >= 4 {
                let prev_row_start = ((y - 1) * bytes_per_row) as usize;
                if prev_row_start + i - 4 < full_data.len() {
                    full_data[prev_row_start + i - 4]
                } else {
                    0
                }
            } else {
                0
            };
            
            data[i] = data[i].wrapping_sub(self.paeth_predictor(left, up, up_left));
        }
    }
    
    fn paeth_predictor(&self, a: u8, b: u8, c: u8) -> u8 {
        let p = a as i16 + b as i16 - c as i16;
        let pa = (p - a as i16).abs();
        let pb = (p - b as i16).abs();
        let pc = (p - c as i16).abs();
        
        if pa <= pb && pa <= pc {
            a
        } else if pb <= pc {
            b
        } else {
            c
        }
    }
}

/// 分析结果
#[derive(Debug, Clone)]
struct AnalysisResult {
    complexity: f64,
    variance: f64,
    edge_density: f64,
    mean: f64,
}

/// 边缘检测滤镜
pub struct EdgeDetectionFilter {
    sensitivity: f64,
    kernel_size: usize,
}

impl EdgeDetectionFilter {
    pub fn new(sensitivity: f64, kernel_size: usize) -> Self {
        Self {
            sensitivity,
            kernel_size,
        }
    }
    
    fn detect_edges(&self, data: &[u8], width: u32, height: u32) -> Vec<f64> {
        let mut edges = Vec::new();
        
        for y in 0..height {
            for x in 0..width {
                let gradient = self.calculate_gradient(data, x, y, width, height);
                edges.push(gradient);
            }
        }
        
        edges
    }
    
    fn calculate_gradient(&self, data: &[u8], x: u32, y: u32, width: u32, height: u32) -> f64 {
        let mut gx = 0.0;
        let mut gy = 0.0;
        
        // Sobel算子
        let sobel_x = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];
        let sobel_y = [[-1, -2, -1], [0, 0, 0], [1, 2, 1]];
        
        for ky in 0..3 {
            for kx in 0..3 {
                let nx = x as i32 + kx as i32 - 1;
                let ny = y as i32 + ky as i32 - 1;
                
                if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                    let pixel = self.get_pixel_intensity(data, nx as u32, ny as u32, width);
                    gx += pixel * sobel_x[ky][kx] as f64;
                    gy += pixel * sobel_y[ky][kx] as f64;
                }
            }
        }
        
        (gx * gx + gy * gy).sqrt()
    }
    
    fn get_pixel_intensity(&self, data: &[u8], x: u32, y: u32, width: u32) -> f64 {
        let index = ((y * width + x) * 4) as usize;
        if index + 2 < data.len() {
            (data[index] as f64 + data[index + 1] as f64 + data[index + 2] as f64) / 3.0
        } else {
            0.0
        }
    }
}

impl AdvancedFilter for EdgeDetectionFilter {
    fn name(&self) -> &str {
        "EdgeDetectionFilter"
    }
    
    fn process(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        let edges = self.detect_edges(data, width, height);
        let mut result = Vec::new();
        
        for (i, &edge_strength) in edges.iter().enumerate() {
            let pixel_start = i * 4;
            if pixel_start + 3 < data.len() {
                let mut pixel = [
                    data[pixel_start],
                    data[pixel_start + 1],
                    data[pixel_start + 2],
                    data[pixel_start + 3],
                ];
                
                // 根据边缘强度调整像素值
                if edge_strength > self.sensitivity {
                    // 增强边缘
                    for j in 0..3 {
                        pixel[j] = (pixel[j] as f64 * 1.2).min(255.0) as u8;
                    }
                } else {
                    // 平滑非边缘区域
                    for j in 0..3 {
                        pixel[j] = (pixel[j] as f64 * 0.9).max(0.0) as u8;
                    }
                }
                
                result.extend_from_slice(&pixel);
            }
        }
        
        Ok(result)
    }
    
    fn get_compression_ratio(&self, data: &[u8]) -> f64 {
        // 边缘检测滤镜的压缩比计算
        let mut score = 0.0;
        for chunk in data.chunks_exact(4) {
            let intensity = (chunk[0] as f64 + chunk[1] as f64 + chunk[2] as f64) / 3.0;
            score += intensity;
        }
        score / (data.len() / 4) as f64
    }
    
    fn supports_parallel(&self) -> bool {
        true
    }
}

/// 噪声减少滤镜
pub struct NoiseReductionFilter {
    strength: f64,
    window_size: usize,
}

impl NoiseReductionFilter {
    pub fn new(strength: f64, window_size: usize) -> Self {
        Self {
            strength,
            window_size,
        }
    }
    
    fn apply_median_filter(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        let mut result = Vec::new();
        
        for y in 0..height {
            for x in 0..width {
                let pixel = self.get_median_pixel(data, x, y, width, height);
                result.extend_from_slice(&pixel);
            }
        }
        
        Ok(result)
    }
    
    fn get_median_pixel(&self, data: &[u8], x: u32, y: u32, width: u32, height: u32) -> [u8; 4] {
        let mut window = Vec::new();
        let half_window = self.window_size / 2;
        
        for dy in 0..self.window_size {
            for dx in 0..self.window_size {
                let nx = x as i32 + dx as i32 - half_window as i32;
                let ny = y as i32 + dy as i32 - half_window as i32;
                
                if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                    let index = ((ny as u32 * width + nx as u32) * 4) as usize;
                    if index + 3 < data.len() {
                        window.push([
                            data[index],
                            data[index + 1],
                            data[index + 2],
                            data[index + 3],
                        ]);
                    }
                }
            }
        }
        
        if window.is_empty() {
            return [0, 0, 0, 255];
        }
        
        // 计算中值
        let mut r_values: Vec<u8> = window.iter().map(|p| p[0]).collect();
        let mut g_values: Vec<u8> = window.iter().map(|p| p[1]).collect();
        let mut b_values: Vec<u8> = window.iter().map(|p| p[2]).collect();
        let mut a_values: Vec<u8> = window.iter().map(|p| p[3]).collect();
        
        r_values.sort();
        g_values.sort();
        b_values.sort();
        a_values.sort();
        
        let median_index = r_values.len() / 2;
        
        [
            r_values[median_index],
            g_values[median_index],
            b_values[median_index],
            a_values[median_index],
        ]
    }
}

impl AdvancedFilter for NoiseReductionFilter {
    fn name(&self) -> &str {
        "NoiseReductionFilter"
    }
    
    fn process(&self, data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
        self.apply_median_filter(data, width, height)
    }
    
    fn get_compression_ratio(&self, data: &[u8]) -> f64 {
        // 噪声减少滤镜的压缩比计算
        let mut score = 0.0;
        for chunk in data.chunks_exact(4) {
            let intensity = (chunk[0] as f64 + chunk[1] as f64 + chunk[2] as f64) / 3.0;
            score += intensity;
        }
        score / (data.len() / 4) as f64
    }
    
    fn supports_parallel(&self) -> bool {
        true
    }
}

/// 滤镜优化器
pub struct FilterOptimizer {
    performance_cache: std::collections::HashMap<String, f64>,
}

impl FilterOptimizer {
    pub fn new() -> Self {
        Self {
            performance_cache: std::collections::HashMap::new(),
        }
    }
    
    /// 优化滤镜选择
    pub fn optimize_filter_selection(&self, data: &[u8], width: u32, height: u32) -> Result<u8, String> {
        let mut best_filter = FILTER_NONE;
        let mut best_score = f64::MAX;
        
        for filter_type in [FILTER_NONE, FILTER_SUB, FILTER_UP, FILTER_AVERAGE, FILTER_PAETH] {
            let score = self.evaluate_filter(data, width, height, filter_type)?;
            if score < best_score {
                best_score = score;
                best_filter = filter_type;
            }
        }
        
        Ok(best_filter)
    }
    
    /// 评估滤镜性能
    fn evaluate_filter(&self, data: &[u8], width: u32, height: u32, filter_type: u8) -> Result<f64, String> {
        let cache_key = format!("{}_{}_{}_{}", width, height, filter_type, data.len());
        
        if let Some(&cached_score) = self.performance_cache.get(&cache_key) {
            return Ok(cached_score);
        }
        
        let score = self.calculate_filter_score(data, width, height, filter_type)?;
        
        // 缓存结果
        // self.performance_cache.insert(cache_key, score);
        
        Ok(score)
    }
    
    /// 计算滤镜分数
    fn calculate_filter_score(&self, data: &[u8], width: u32, height: u32, filter_type: u8) -> Result<f64, String> {
        let mut score = 0.0;
        let bytes_per_row = (width * 4) as usize;
        
        for y in 0..height {
            let row_start = (y * bytes_per_row) as usize;
            let row_end = row_start + bytes_per_row;
            
            if row_end > data.len() {
                return Err("Insufficient data for row".to_string());
            }
            
            let row_data = &data[row_start..row_end];
            let mut filtered_row = row_data.to_vec();
            
            // 应用滤镜
            self.apply_filter_to_row(&mut filtered_row, width as usize, y, data, bytes_per_row, filter_type)?;
            
            // 计算分数
            score += self.calculate_row_score(&filtered_row);
        }
        
        Ok(score)
    }
    
    fn apply_filter_to_row(&self, data: &mut [u8], width: usize, y: u32, full_data: &[u8], bytes_per_row: usize, filter_type: u8) -> Result<(), String> {
        match filter_type {
            FILTER_NONE => Ok(()),
            FILTER_SUB => {
                for i in 4..data.len() {
                    data[i] = data[i].wrapping_sub(data[i - 4]);
                }
                Ok(())
            }
            FILTER_UP => {
                if y > 0 {
                    let prev_row_start = ((y - 1) * bytes_per_row) as usize;
                    let prev_row = &full_data[prev_row_start..prev_row_start + bytes_per_row];
                    for i in 0..data.len() {
                        data[i] = data[i].wrapping_sub(prev_row[i]);
                    }
                }
                Ok(())
            }
            FILTER_AVERAGE => {
                for i in 0..data.len() {
                    let left = if i >= 4 { data[i - 4] } else { 0 };
                    let up = if y > 0 {
                        let prev_row_start = ((y - 1) * bytes_per_row) as usize;
                        if prev_row_start + i < full_data.len() {
                            full_data[prev_row_start + i]
                        } else {
                            0
                        }
                    } else {
                        0
                    };
                    data[i] = data[i].wrapping_sub((left + up) / 2);
                }
                Ok(())
            }
            FILTER_PAETH => {
                for i in 0..data.len() {
                    let left = if i >= 4 { data[i - 4] } else { 0 };
                    let up = if y > 0 {
                        let prev_row_start = ((y - 1) * bytes_per_row) as usize;
                        if prev_row_start + i < full_data.len() {
                            full_data[prev_row_start + i]
                        } else {
                            0
                        }
                    } else {
                        0
                    };
                    let up_left = if y > 0 && i >= 4 {
                        let prev_row_start = ((y - 1) * bytes_per_row) as usize;
                        if prev_row_start + i - 4 < full_data.len() {
                            full_data[prev_row_start + i - 4]
                        } else {
                            0
                        }
                    } else {
                        0
                    };
                    data[i] = data[i].wrapping_sub(self.paeth_predictor(left, up, up_left));
                }
                Ok(())
            }
            _ => Err("Unknown filter type".to_string()),
        }
    }
    
    fn paeth_predictor(&self, a: u8, b: u8, c: u8) -> u8 {
        let p = a as i16 + b as i16 - c as i16;
        let pa = (p - a as i16).abs();
        let pb = (p - b as i16).abs();
        let pc = (p - c as i16).abs();
        
        if pa <= pb && pa <= pc {
            a
        } else if pb <= pc {
            b
        } else {
            c
        }
    }
    
    fn calculate_row_score(&self, data: &[u8]) -> f64 {
        let mut score = 0.0;
        for chunk in data.chunks_exact(4) {
            let intensity = (chunk[0] as f64 + chunk[1] as f64 + chunk[2] as f64) / 3.0;
            score += intensity;
        }
        score / (data.len() / 4) as f64
    }
}
