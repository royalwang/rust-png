/**
 * Rust PNG JS - TypeScript类型定义
 * 完全兼容原始pngjs库的API
 */

// 基础类型定义
export interface PNGDimensions {
  width: number;
  height: number;
}

export interface PNGColorInfo {
  colorType: number;
  bitDepth: number;
  hasAlpha: boolean;
}

export interface PNGCompression {
  method: number;
  filter: number;
}

export interface PNGInterlace {
  method: number;
}

export interface PNGMetadata {
  dimensions: PNGDimensions;
  colorInfo: PNGColorInfo;
  compression: PNGCompression;
  interlace: PNGInterlace;
  gamma?: number;
  palette?: Uint8Array;
  transparentColor?: Uint16Array;
}

// 像素数据类型
export type PixelData = Uint8Array | Uint8ClampedArray;
export type PixelArray = [number, number, number, number]; // RGBA

// 解析选项
export interface ParseOptions {
  data?: boolean;
  skipRescale?: boolean;
  colorType?: number;
  bitDepth?: number;
}

// 编码选项
export interface EncodeOptions {
  deflateLevel?: number;
  deflateStrategy?: number;
  filterType?: number;
  colorType?: number;
  bitDepth?: number;
  palette?: Uint8Array;
  transparentColor?: Uint16Array;
  gamma?: number;
}

// 滤镜类型
export enum FilterType {
  None = 0,
  Sub = 1,
  Up = 2,
  Average = 3,
  Paeth = 4,
  Adaptive = 5
}

// 颜色类型
export enum ColorType {
  Grayscale = 0,
  RGB = 2,
  Palette = 3,
  GrayscaleAlpha = 4,
  RGBA = 6
}

// 位深度
export enum BitDepth {
  One = 1,
  Two = 2,
  Four = 4,
  Eight = 8,
  Sixteen = 16
}

// 交错方法
export enum InterlaceMethod {
  None = 0,
  Adam7 = 1
}

// 压缩方法
export enum CompressionMethod {
  Deflate = 0
}

// 滤镜方法
export enum FilterMethod {
  None = 0,
  Sub = 1,
  Up = 2,
  Average = 3,
  Paeth = 4,
  Adaptive = 5
}

// 错误类型
export class PNGError extends Error {
  constructor(message: string, public code?: string) {
    super(message);
    this.name = 'PNGError';
  }
}

// 回调函数类型
export type ParseCallback = (error: Error | null, png?: PNG) => void;
export type WriteCallback = (error: Error | null, buffer?: Uint8Array) => void;

// 性能统计
export interface PerformanceStats {
  parseTime: number;
  encodeTime: number;
  memoryUsage: number;
  compressionRatio: number;
}

// 高级滤镜选项
export interface AdvancedFilterOptions {
  enableAdaptiveFilter?: boolean;
  enableEdgeDetection?: boolean;
  enableNoiseReduction?: boolean;
  threshold?: number;
  sensitivity?: number;
  strength?: number;
}

// WebAssembly优化选项
export interface WASMOptimizationOptions {
  enableSIMD?: boolean;
  enableParallel?: boolean;
  enableCaching?: boolean;
  memoryPoolSize?: number;
  threadCount?: number;
}

// 主要PNG类接口
export interface PNGInterface {
  // 基本属性
  readonly width: number;
  readonly height: number;
  readonly data: PixelData;
  readonly gamma: number;
  readonly alpha: boolean;
  readonly readable: boolean;
  readonly writable: boolean;

  // 元数据访问
  getWidth(): number;
  getHeight(): number;
  getPixel(x: number, y: number): PixelArray;
  getRGBA8Array(): Uint8Array;
  getBitDepth(): number;
  getColorType(): number;
  getCompressionMethod(): number;
  getFilterMethod(): number;
  getInterlaceMethod(): number;
  getPalette(): Uint8Array | null;
  getMetadata(): PNGMetadata;

  // 图像操作
  bitblt(src: PNG, dst: PNG, srcX: number, srcY: number, width: number, height: number, deltaX: number, deltaY: number): void;
  adjustGamma(): void;
  isInterlaced(): boolean;
  getInterlacePasses(): number;
  getInterlaceStats(): any;

  // 解析和编码
  parse(data: Uint8Array, callback: ParseCallback): void;
  parseSync(data: Uint8Array): PNG;
  pack(): Uint8Array;
  writeFile(filename: string): void;
  toBuffer(): Uint8Array;

  // 高级功能
  setAdvancedFilters(options: AdvancedFilterOptions): void;
  setWASMOptimization(options: WASMOptimizationOptions): void;
  getPerformanceStats(): PerformanceStats;
}

// 同步PNG类接口
export interface PNGSyncInterface {
  read(data: Uint8Array): PNG;
  write(png: PNG, options?: EncodeOptions): Uint8Array;
}

// 语义PNG类接口
export interface SemanticPNGInterface extends PNGInterface {
  getSemanticMetadata(): any;
  setSemanticMetadata(metadata: any): void;
}

// 语义同步PNG类接口
export interface SemanticPNGSyncInterface extends PNGSyncInterface {
  readSemantic(data: Uint8Array): SemanticPNG;
  writeSemantic(png: SemanticPNG, options?: EncodeOptions): Uint8Array;
}

// 主要导出类型
export type PNG = PNGInterface;
export type PNGSync = PNGSyncInterface;
export type SemanticPNG = SemanticPNGInterface;
export type SemanticPNGSync = SemanticPNGSyncInterface;

// 模块导出
export interface RustPNGModule {
  PNG: new () => PNG;
  PNGSync: new () => PNGSync;
  SemanticPNG: new () => SemanticPNG;
  SemanticPNGSync: new () => SemanticPNGSync;
  
  // 工具函数
  validatePNG(data: Uint8Array): boolean;
  getPNGInfo(data: Uint8Array): PNGMetadata | null;
  convertColorType(data: Uint8Array, fromType: ColorType, toType: ColorType): Uint8Array;
  optimizePNG(data: Uint8Array, options?: EncodeOptions): Uint8Array;
  
  // 性能工具
  benchmark(data: Uint8Array, iterations?: number): PerformanceStats;
  getMemoryUsage(): number;
  clearCache(): void;
}
