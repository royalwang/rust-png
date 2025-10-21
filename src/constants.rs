//! PNG常量定义模块
//! 匹配原始pngjs库的constants.js

// PNG文件签名
pub const PNG_SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];

// Chunk类型常量
pub const TYPE_IHDR: u32 = 0x49484452;
pub const TYPE_IEND: u32 = 0x49454e44;
pub const TYPE_IDAT: u32 = 0x49444154;
pub const TYPE_PLTE: u32 = 0x504c5445;
pub const TYPE_tRNS: u32 = 0x74524e53;
pub const TYPE_gAMA: u32 = 0x67414d41;
pub const TYPE_cHRM: u32 = 0x6348524d;
pub const TYPE_sRGB: u32 = 0x73524742;
pub const TYPE_iCCP: u32 = 0x69434350;
pub const TYPE_tEXt: u32 = 0x74455874;
pub const TYPE_zTXt: u32 = 0x7a545874;
pub const TYPE_iTXt: u32 = 0x69545874;

// 颜色类型常量
pub const COLORTYPE_GRAYSCALE: u8 = 0;
pub const COLORTYPE_PALETTE: u8 = 1;
pub const COLORTYPE_COLOR: u8 = 2;
pub const COLORTYPE_ALPHA: u8 = 4;
pub const COLORTYPE_PALETTE_COLOR: u8 = 3;
pub const COLORTYPE_COLOR_ALPHA: u8 = 6;

// 颜色类型到每像素字节数的映射
pub const COLORTYPE_TO_BPP_MAP: [u8; 7] = [0, 1, 3, 1, 2, 0, 4];

// 滤镜类型常量
pub const FILTER_NONE: u8 = 0;
pub const FILTER_SUB: u8 = 1;
pub const FILTER_UP: u8 = 2;
pub const FILTER_AVERAGE: u8 = 3;
pub const FILTER_PAETH: u8 = 4;

// Gamma除法因子
pub const GAMMA_DIVISION: u32 = 100000;

// 交错通道定义 (Adam7)
pub const INTERLACE_PASSES: [[u8; 8]; 7] = [
    [0, 0, 0, 0, 0, 0, 0, 0], // pass 1
    [4, 0, 0, 0, 0, 0, 0, 0], // pass 2
    [0, 4, 0, 0, 0, 0, 0, 0], // pass 3
    [2, 6, 0, 0, 0, 0, 0, 0], // pass 4
    [0, 2, 4, 6, 0, 0, 0, 0], // pass 5
    [1, 3, 5, 7, 0, 0, 0, 0], // pass 6
    [0, 1, 2, 3, 4, 5, 6, 7], // pass 7
];

pub const INTERLACE_Y_PASSES: [[u8; 8]; 7] = [
    [0, 0, 0, 0, 0, 0, 0, 0], // pass 1
    [0, 0, 0, 0, 0, 0, 0, 0], // pass 2
    [4, 0, 0, 0, 0, 0, 0, 0], // pass 3
    [0, 4, 0, 0, 0, 0, 0, 0], // pass 4
    [2, 6, 0, 0, 0, 0, 0, 0], // pass 5
    [0, 2, 4, 6, 0, 0, 0, 0], // pass 6
    [1, 3, 5, 7, 0, 0, 0, 0], // pass 7
];

// 获取交错通道的宽度和高度
pub fn get_interlace_pass_size(width: u32, height: u32, pass: usize) -> (u32, u32) {
    if pass >= 7 {
        return (0, 0);
    }
    
    let x_pass = INTERLACE_PASSES[pass];
    let y_pass = INTERLACE_Y_PASSES[pass];
    
    let mut pass_width = 0;
    let mut pass_height = 0;
    
    for i in 0..8 {
        if x_pass[i] != 0 {
            pass_width += (width + (7 - x_pass[i] as u32)) / 8;
        }
        if y_pass[i] != 0 {
            pass_height += (height + (7 - y_pass[i] as u32)) / 8;
        }
    }
    
    (pass_width, pass_height)
}
