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
  constructor(options?: PngOptions);
  
  // 属性（兼容原始pngjs）
  readonly width: number;
  readonly height: number;
  readonly bitDepth: number;
  readonly colorType: number;
  readonly compressionMethod: number;
  readonly filterMethod: number;
  readonly interlaceMethod: number;
  data?: Uint8ClampedArray; // 像素数据
  gamma: number;
  alpha: boolean;
  readable: boolean;
  writable: boolean;
  
  // 基本方法
  getWidth(): number;
  getHeight(): number;
  getBitDepth(): number;
  getColorType(): number;
  getCompressionMethod(): number;
  getFilterMethod(): number;
  getInterlaceMethod(): number;
  getPalette(): Uint8Array | null;
  
  // 像素操作
  getPixel(x: number, y: number): PngPixel;
  setPixel(x: number, y: number, r: number, g: number, b: number, a: number): void;
  getRGBA8Array(): Uint8ClampedArray;
  
  // 原始pngjs的核心方法
  parse(data: ArrayBuffer | Uint8Array, callback?: () => void): Promise<void>;
  pack(): Uint8Array;
  writeFile(filename: string): Promise<void>;
  toBuffer(): Uint8Array;
  
  // 图像操作
  bitblt(dst: PNG, srcX: number, srcY: number, width: number, height: number, deltaX: number, deltaY: number): void;
  adjustGamma(): void;
  getTransColor(): Uint8Array | null;
}

// 同步API（兼容原始pngjs）
export declare class PNGSync {
  static read(buffer: ArrayBuffer | Uint8Array, options?: PngOptions): PNG;
  static write(png: PNG, options?: PngOptions): Uint8Array;
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
