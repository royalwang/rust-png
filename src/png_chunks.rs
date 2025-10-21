//! PNG Chunk处理模块
//! 实现完整的PNG chunk解析和处理，匹配原始pngjs库的parser.js

use std::collections::HashMap;
use crate::constants::*;
use crate::crc::crc32;

/// PNG Chunk类型
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub enum ChunkType {
    IHDR,
    IEND,
    IDAT,
    PLTE,
    TRNS,
    GAMA,
    CHRM,
    SRGB,
    ICCP,
    TEXT,
    ZTXT,
    ITXT,
    Unknown(u32),
}

impl ChunkType {
    pub fn from_u32(value: u32) -> Self {
        match value {
            TYPE_IHDR => ChunkType::IHDR,
            TYPE_IEND => ChunkType::IEND,
            TYPE_IDAT => ChunkType::IDAT,
            TYPE_PLTE => ChunkType::PLTE,
            TYPE_tRNS => ChunkType::TRNS,
            TYPE_gAMA => ChunkType::GAMA,
            TYPE_cHRM => ChunkType::CHRM,
            TYPE_sRGB => ChunkType::SRGB,
            TYPE_iCCP => ChunkType::ICCP,
            TYPE_tEXt => ChunkType::TEXT,
            TYPE_zTXt => ChunkType::ZTXT,
            TYPE_iTXt => ChunkType::ITXT,
            _ => ChunkType::Unknown(value),
        }
    }
    
    pub fn to_u32(&self) -> u32 {
        match self {
            ChunkType::IHDR => TYPE_IHDR,
            ChunkType::IEND => TYPE_IEND,
            ChunkType::IDAT => TYPE_IDAT,
            ChunkType::PLTE => TYPE_PLTE,
            ChunkType::TRNS => TYPE_tRNS,
            ChunkType::GAMA => TYPE_gAMA,
            ChunkType::CHRM => TYPE_cHRM,
            ChunkType::SRGB => TYPE_sRGB,
            ChunkType::ICCP => TYPE_iCCP,
            ChunkType::TEXT => TYPE_tEXt,
            ChunkType::ZTXT => TYPE_zTXt,
            ChunkType::ITXT => TYPE_iTXt,
            ChunkType::Unknown(value) => *value,
        }
    }
}

/// PNG Chunk结构
#[derive(Debug, Clone)]
pub struct PNGChunk {
    pub length: u32,
    pub chunk_type: ChunkType,
    pub data: Vec<u8>,
    pub crc: u32,
}

impl PNGChunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let crc = Self::calculate_crc(&chunk_type, &data);
        Self {
            length: data.len() as u32,
            chunk_type,
            data,
            crc,
        }
    }
    
    pub fn calculate_crc(chunk_type: &ChunkType, data: &[u8]) -> u32 {
        let mut crc_data = Vec::new();
        crc_data.extend_from_slice(&chunk_type.to_u32().to_be_bytes());
        crc_data.extend_from_slice(data);
        crc32(&crc_data)
    }
    
    pub fn verify_crc(&self) -> bool {
        let calculated_crc = Self::calculate_crc(&self.chunk_type, &self.data);
        calculated_crc == self.crc
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.length.to_be_bytes());
        bytes.extend_from_slice(&self.chunk_type.to_u32().to_be_bytes());
        bytes.extend_from_slice(&self.data);
        bytes.extend_from_slice(&self.crc.to_be_bytes());
        bytes
    }
}

/// IHDR Chunk数据
#[derive(Debug, Clone)]
pub struct IHDRData {
    pub width: u32,
    pub height: u32,
    pub bit_depth: u8,
    pub color_type: u8,
    pub compression_method: u8,
    pub filter_method: u8,
    pub interlace_method: u8,
}

impl IHDRData {
    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        if data.len() != 13 {
            return Err("IHDR data must be 13 bytes".to_string());
        }
        
        Ok(Self {
            width: u32::from_be_bytes([data[0], data[1], data[2], data[3]]),
            height: u32::from_be_bytes([data[4], data[5], data[6], data[7]]),
            bit_depth: data[8],
            color_type: data[9],
            compression_method: data[10],
            filter_method: data[11],
            interlace_method: data[12],
        })
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.width.to_be_bytes());
        bytes.extend_from_slice(&self.height.to_be_bytes());
        bytes.push(self.bit_depth);
        bytes.push(self.color_type);
        bytes.push(self.compression_method);
        bytes.push(self.filter_method);
        bytes.push(self.interlace_method);
        bytes
    }
}

/// PLTE Chunk数据
#[derive(Debug, Clone)]
pub struct PLTEData {
    pub palette: Vec<[u8; 3]>, // RGB调色板
}

impl PLTEData {
    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        if data.len() % 3 != 0 {
            return Err("PLTE data length must be multiple of 3".to_string());
        }
        
        let mut palette = Vec::new();
        for chunk in data.chunks_exact(3) {
            palette.push([chunk[0], chunk[1], chunk[2]]);
        }
        
        Ok(Self { palette })
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for &[r, g, b] in &self.palette {
            bytes.push(r);
            bytes.push(g);
            bytes.push(b);
        }
        bytes
    }
}

/// tRNS Chunk数据
#[derive(Debug, Clone)]
pub enum TRNSData {
    Grayscale { value: u16 },
    RGB { r: u16, g: u16, b: u16 },
    Palette { alpha: Vec<u8> },
}

impl TRNSData {
    pub fn from_bytes(data: &[u8], color_type: u8) -> Result<Self, String> {
        match color_type {
            COLORTYPE_GRAYSCALE => {
                if data.len() != 2 {
                    return Err("Grayscale tRNS must be 2 bytes".to_string());
                }
                Ok(TRNSData::Grayscale {
                    value: u16::from_be_bytes([data[0], data[1]]),
                })
            }
            COLORTYPE_COLOR => {
                if data.len() != 6 {
                    return Err("RGB tRNS must be 6 bytes".to_string());
                }
                Ok(TRNSData::RGB {
                    r: u16::from_be_bytes([data[0], data[1]]),
                    g: u16::from_be_bytes([data[2], data[3]]),
                    b: u16::from_be_bytes([data[4], data[5]]),
                })
            }
            COLORTYPE_PALETTE_COLOR => {
                Ok(TRNSData::Palette {
                    alpha: data.to_vec(),
                })
            }
            _ => Err("Invalid color type for tRNS".to_string()),
        }
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            TRNSData::Grayscale { value } => value.to_be_bytes().to_vec(),
            TRNSData::RGB { r, g, b } => {
                let mut bytes = Vec::new();
                bytes.extend_from_slice(&r.to_be_bytes());
                bytes.extend_from_slice(&g.to_be_bytes());
                bytes.extend_from_slice(&b.to_be_bytes());
                bytes
            }
            TRNSData::Palette { alpha } => alpha.clone(),
        }
    }
}

/// gAMA Chunk数据
#[derive(Debug, Clone)]
pub struct GAMAData {
    pub gamma: u32,
}

impl GAMAData {
    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        if data.len() != 4 {
            return Err("gAMA data must be 4 bytes".to_string());
        }
        
        Ok(Self {
            gamma: u32::from_be_bytes([data[0], data[1], data[2], data[3]]),
        })
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        self.gamma.to_be_bytes().to_vec()
    }
    
    pub fn get_gamma_value(&self) -> f64 {
        self.gamma as f64 / 100000.0
    }
}

/// cHRM Chunk数据
#[derive(Debug, Clone)]
pub struct CHRMData {
    pub white_point_x: u32,
    pub white_point_y: u32,
    pub red_x: u32,
    pub red_y: u32,
    pub green_x: u32,
    pub green_y: u32,
    pub blue_x: u32,
    pub blue_y: u32,
}

impl CHRMData {
    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        if data.len() != 32 {
            return Err("cHRM data must be 32 bytes".to_string());
        }
        
        Ok(Self {
            white_point_x: u32::from_be_bytes([data[0], data[1], data[2], data[3]]),
            white_point_y: u32::from_be_bytes([data[4], data[5], data[6], data[7]]),
            red_x: u32::from_be_bytes([data[8], data[9], data[10], data[11]]),
            red_y: u32::from_be_bytes([data[12], data[13], data[14], data[15]]),
            green_x: u32::from_be_bytes([data[16], data[17], data[18], data[19]]),
            green_y: u32::from_be_bytes([data[20], data[21], data[22], data[23]]),
            blue_x: u32::from_be_bytes([data[24], data[25], data[26], data[27]]),
            blue_y: u32::from_be_bytes([data[28], data[29], data[30], data[31]]),
        })
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.white_point_x.to_be_bytes());
        bytes.extend_from_slice(&self.white_point_y.to_be_bytes());
        bytes.extend_from_slice(&self.red_x.to_be_bytes());
        bytes.extend_from_slice(&self.red_y.to_be_bytes());
        bytes.extend_from_slice(&self.green_x.to_be_bytes());
        bytes.extend_from_slice(&self.green_y.to_be_bytes());
        bytes.extend_from_slice(&self.blue_x.to_be_bytes());
        bytes.extend_from_slice(&self.blue_y.to_be_bytes());
        bytes
    }
}

/// sRGB Chunk数据
#[derive(Debug, Clone)]
pub struct SRGBData {
    pub rendering_intent: u8,
}

impl SRGBData {
    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        if data.len() != 1 {
            return Err("sRGB data must be 1 byte".to_string());
        }
        
        Ok(Self {
            rendering_intent: data[0],
        })
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        vec![self.rendering_intent]
    }
}

/// tEXt Chunk数据
#[derive(Debug, Clone)]
pub struct TEXTData {
    pub keyword: String,
    pub text: String,
}

impl TEXTData {
    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        let null_pos = data.iter().position(|&b| b == 0).ok_or("No null terminator found")?;
        let keyword = String::from_utf8(data[..null_pos].to_vec())
            .map_err(|_| "Invalid keyword encoding")?;
        let text = String::from_utf8(data[null_pos + 1..].to_vec())
            .map_err(|_| "Invalid text encoding")?;
        
        Ok(Self { keyword, text })
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.keyword.as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(self.text.as_bytes());
        bytes
    }
}

/// zTXt Chunk数据
#[derive(Debug, Clone)]
pub struct ZTXTData {
    pub keyword: String,
    pub compression_method: u8,
    pub compressed_text: Vec<u8>,
}

impl ZTXTData {
    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        let null_pos = data.iter().position(|&b| b == 0).ok_or("No null terminator found")?;
        let keyword = String::from_utf8(data[..null_pos].to_vec())
            .map_err(|_| "Invalid keyword encoding")?;
        
        if null_pos + 1 >= data.len() {
            return Err("Insufficient data for zTXt".to_string());
        }
        
        let compression_method = data[null_pos + 1];
        let compressed_text = data[null_pos + 2..].to_vec();
        
        Ok(Self {
            keyword,
            compression_method,
            compressed_text,
        })
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.keyword.as_bytes());
        bytes.push(0);
        bytes.push(self.compression_method);
        bytes.extend_from_slice(&self.compressed_text);
        bytes
    }
}

/// iTXt Chunk数据
#[derive(Debug, Clone)]
pub struct ITXTData {
    pub keyword: String,
    pub compression_flag: u8,
    pub compression_method: u8,
    pub language_tag: String,
    pub translated_keyword: String,
    pub text: String,
}

impl ITXTData {
    pub fn from_bytes(data: &[u8]) -> Result<Self, String> {
        let null_pos = data.iter().position(|&b| b == 0).ok_or("No null terminator found")?;
        let keyword = String::from_utf8(data[..null_pos].to_vec())
            .map_err(|_| "Invalid keyword encoding")?;
        
        if null_pos + 3 >= data.len() {
            return Err("Insufficient data for iTXt".to_string());
        }
        
        let compression_flag = data[null_pos + 1];
        let compression_method = data[null_pos + 2];
        
        let mut offset = null_pos + 3;
        
        // 解析语言标签
        let lang_null_pos = data[offset..].iter().position(|&b| b == 0)
            .ok_or("No language tag terminator found")?;
        let language_tag = String::from_utf8(data[offset..offset + lang_null_pos].to_vec())
            .map_err(|_| "Invalid language tag encoding")?;
        offset += lang_null_pos + 1;
        
        // 解析翻译关键字
        let trans_null_pos = data[offset..].iter().position(|&b| b == 0)
            .ok_or("No translated keyword terminator found")?;
        let translated_keyword = String::from_utf8(data[offset..offset + trans_null_pos].to_vec())
            .map_err(|_| "Invalid translated keyword encoding")?;
        offset += trans_null_pos + 1;
        
        // 剩余数据为文本
        let text = if compression_flag == 0 {
            String::from_utf8(data[offset..].to_vec())
                .map_err(|_| "Invalid text encoding")?
        } else {
            // 这里需要解压缩，简化处理
            String::from_utf8(data[offset..].to_vec())
                .map_err(|_| "Invalid compressed text encoding")?
        };
        
        Ok(Self {
            keyword,
            compression_flag,
            compression_method,
            language_tag,
            translated_keyword,
            text,
        })
    }
    
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.keyword.as_bytes());
        bytes.push(0);
        bytes.push(self.compression_flag);
        bytes.push(self.compression_method);
        bytes.extend_from_slice(self.language_tag.as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(self.translated_keyword.as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(self.text.as_bytes());
        bytes
    }
}

/// PNG Chunk解析器
pub struct PNGChunkParser {
    pub chunks: HashMap<ChunkType, Vec<PNGChunk>>,
    pub ihdr: Option<IHDRData>,
    pub palette: Option<PLTEData>,
    pub transparency: Option<TRNSData>,
    pub gamma: Option<GAMAData>,
    pub chroma: Option<CHRMData>,
    pub srgb: Option<SRGBData>,
    pub text_chunks: Vec<TEXTData>,
    pub ztxt_chunks: Vec<ZTXTData>,
    pub itxt_chunks: Vec<ITXTData>,
}

impl PNGChunkParser {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            ihdr: None,
            palette: None,
            transparency: None,
            gamma: None,
            chroma: None,
            srgb: None,
            text_chunks: Vec::new(),
            ztxt_chunks: Vec::new(),
            itxt_chunks: Vec::new(),
        }
    }
    
    /// 解析PNG数据
    pub fn parse(&mut self, data: &[u8]) -> Result<(), String> {
        let mut offset = 0;
        
        // 检查PNG签名
        if data.len() < PNG_SIGNATURE.len() {
            return Err("Insufficient data for PNG signature".to_string());
        }
        
        if &data[offset..offset + PNG_SIGNATURE.len()] != &PNG_SIGNATURE {
            return Err("Invalid PNG signature".to_string());
        }
        offset += PNG_SIGNATURE.len();
        
        // 解析chunks
        while offset < data.len() {
            if offset + 8 > data.len() {
                return Err("Insufficient data for chunk header".to_string());
            }
            
            let length = u32::from_be_bytes([
                data[offset], data[offset + 1], data[offset + 2], data[offset + 3]
            ]);
            let chunk_type = u32::from_be_bytes([
                data[offset + 4], data[offset + 5], data[offset + 6], data[offset + 7]
            ]);
            
            offset += 8;
            
            if offset + length as usize + 4 > data.len() {
                return Err("Insufficient data for chunk".to_string());
            }
            
            let chunk_data = data[offset..offset + length as usize].to_vec();
            offset += length as usize;
            
            let crc = u32::from_be_bytes([
                data[offset], data[offset + 1], data[offset + 2], data[offset + 3]
            ]);
            offset += 4;
            
            let chunk = PNGChunk {
                length,
                chunk_type: ChunkType::from_u32(chunk_type),
                data: chunk_data,
                crc,
            };
            
            // 验证CRC
            if !chunk.verify_crc() {
                return Err(format!("Invalid CRC for chunk {:?}", chunk.chunk_type));
            }
            
            // 处理chunk
            self.process_chunk(chunk)?;
        }
        
        Ok(())
    }
    
    /// 处理chunk
    fn process_chunk(&mut self, chunk: PNGChunk) -> Result<(), String> {
        match chunk.chunk_type {
            ChunkType::IHDR => {
                self.ihdr = Some(IHDRData::from_bytes(&chunk.data)?);
            }
            ChunkType::PLTE => {
                self.palette = Some(PLTEData::from_bytes(&chunk.data)?);
            }
            ChunkType::TRNS => {
                if let Some(ref ihdr) = self.ihdr {
                    self.transparency = Some(TRNSData::from_bytes(&chunk.data, ihdr.color_type)?);
                }
            }
            ChunkType::GAMA => {
                self.gamma = Some(GAMAData::from_bytes(&chunk.data)?);
            }
            ChunkType::CHRM => {
                self.chroma = Some(CHRMData::from_bytes(&chunk.data)?);
            }
            ChunkType::SRGB => {
                self.srgb = Some(SRGBData::from_bytes(&chunk.data)?);
            }
            ChunkType::TEXT => {
                self.text_chunks.push(TEXTData::from_bytes(&chunk.data)?);
            }
            ChunkType::ZTXT => {
                self.ztxt_chunks.push(ZTXTData::from_bytes(&chunk.data)?);
            }
            ChunkType::ITXT => {
                self.itxt_chunks.push(ITXTData::from_bytes(&chunk.data)?);
            }
            _ => {}
        }
        
        // 存储chunk
        self.chunks.entry(chunk.chunk_type.clone()).or_insert_with(Vec::new).push(chunk);
        
        Ok(())
    }
    
    /// 获取特定类型的chunks
    pub fn get_chunks(&self, chunk_type: &ChunkType) -> Option<&Vec<PNGChunk>> {
        self.chunks.get(chunk_type)
    }
    
    /// 检查是否包含特定chunk
    pub fn has_chunk(&self, chunk_type: &ChunkType) -> bool {
        self.chunks.contains_key(chunk_type)
    }
    
    /// 获取所有chunk类型
    pub fn get_chunk_types(&self) -> Vec<ChunkType> {
        self.chunks.keys().cloned().collect()
    }
}
