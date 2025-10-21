/**
 * TypeScript definitions for rust-png WASM module
 * Compatible with pngjs library interface
 */

export interface PngOptions {
  data?: boolean;
}

export interface PngInfo {
  width: number;
  height: number;
  bitDepth: number;
  colorType: number;
  compressionMethod: number;
  filterMethod: number;
  interlaceMethod: number;
  palette?: Uint8Array;
}

export interface PngPixel {
  0: number; // Red
  1: number; // Green
  2: number; // Blue
  3: number; // Alpha
}

// 主要PNG类（兼容原始pngjs API）
export declare class PNG {
  constructor(data: ArrayBuffer | Uint8Array, options?: PngOptions);
  
  // 属性（兼容原始pngjs）
  readonly width: number;
  readonly height: number;
  readonly bitDepth: number;
  readonly colorType: number;
  readonly compressionMethod: number;
  readonly filterMethod: number;
  readonly interlaceMethod: number;
  data?: Uint8ClampedArray; // 像素数据
  
  // 方法（兼容原始pngjs）
  getWidth(): number;
  getHeight(): number;
  getPixel(x: number, y: number): PngPixel;
  getRGBA8Array(): Uint8ClampedArray;
  getBitDepth(): number;
  getColorType(): number;
  getCompressionMethod(): number;
  getFilterMethod(): number;
  getInterlaceMethod(): number;
  getPalette(): Uint8Array | null;
  
  // 原始pngjs的parse方法
  parse(data: ArrayBuffer | Uint8Array, callback?: () => void): Promise<void>;
}

// 兼容性别名
export declare class PngDecoder extends PNG {}

export declare function createPngDecoder(
  data: ArrayBuffer | Uint8Array, 
  options?: PngOptions
): PngDecoder;

// WASM module exports
export interface WasmExports {
  memory: WebAssembly.Memory;
  create_png_decoder: (data_ptr: number, data_len: number, options_ptr: number) => number;
  PngDecoder: {
    new: (data_ptr: number, data_len: number, options_ptr: number) => PngDecoder;
    get_width: (ptr: number) => number;
    get_height: (ptr: number) => number;
    get_pixel: (ptr: number, x: number, y: number) => number;
    get_rgba8_array: (ptr: number) => number;
    get_bit_depth: (ptr: number) => number;
    get_color_type: (ptr: number) => number;
    get_compression_method: (ptr: number) => number;
    get_filter_method: (ptr: number) => number;
    get_interlace_method: (ptr: number) => number;
    get_palette: (ptr: number) => number;
  };
}

declare const wasm: WasmExports;
export default wasm;
