/**
 * PNG类封装
 * 提供与原始pngjs库完全兼容的API
 */

import { 
  PNGInterface, 
  PNGMetadata, 
  PixelData, 
  PixelArray, 
  ParseOptions, 
  EncodeOptions,
  ParseCallback,
  PerformanceStats,
  AdvancedFilterOptions,
  WASMOptimizationOptions,
  PNGError
} from './types';
import { loadWASMModule } from './wasm-loader';

export class PNG implements PNGInterface {
  private wasmPNG: any;
  private _metadata: PNGMetadata | null = null;
  private _data: PixelData | null = null;
  private _width: number = 0;
  private _height: number = 0;
  private _gamma: number = 0;
  private _alpha: boolean = false;
  private _readable: boolean = false;
  private _writable: boolean = true;

  constructor() {
    this.initializeWASM();
  }

  private async initializeWASM(): Promise<void> {
    try {
      const wasmModule = await loadWASMModule();
      this.wasmPNG = new wasmModule.PNG();
    } catch (error) {
      throw new PNGError(`Failed to initialize WASM PNG: ${error}`);
    }
  }

  // 基本属性访问器
  get width(): number {
    return this._width;
  }

  get height(): number {
    return this._height;
  }

  get data(): PixelData {
    if (!this._data) {
      throw new PNGError('PNG data not loaded');
    }
    return this._data;
  }

  get gamma(): number {
    return this._gamma;
  }

  get alpha(): boolean {
    return this._alpha;
  }

  get readable(): boolean {
    return this._readable;
  }

  get writable(): boolean {
    return this._writable;
  }

  // 元数据访问方法
  getWidth(): number {
    return this._width;
  }

  getHeight(): number {
    return this._height;
  }

  getPixel(x: number, y: number): PixelArray {
    if (!this._data || !this._readable) {
      throw new PNGError('PNG data not available');
    }
    
    if (x < 0 || x >= this._width || y < 0 || y >= this._height) {
      throw new PNGError('Pixel coordinates out of bounds');
    }

    const pixelIndex = (y * this._width + x) * 4;
    return [
      this._data[pixelIndex],     // R
      this._data[pixelIndex + 1], // G
      this._data[pixelIndex + 2], // B
      this._data[pixelIndex + 3]  // A
    ];
  }

  getRGBA8Array(): Uint8Array {
    if (!this._data || !this._readable) {
      throw new PNGError('PNG data not available');
    }
    return new Uint8Array(this._data);
  }

  getBitDepth(): number {
    return this._metadata?.colorInfo.bitDepth || 8;
  }

  getColorType(): number {
    return this._metadata?.colorInfo.colorType || 6; // RGBA
  }

  getCompressionMethod(): number {
    return this._metadata?.compression.method || 0;
  }

  getFilterMethod(): number {
    return this._metadata?.compression.filter || 0;
  }

  getInterlaceMethod(): number {
    return this._metadata?.interlace.method || 0;
  }

  getPalette(): Uint8Array | null {
    return this._metadata?.palette || null;
  }

  getMetadata(): PNGMetadata {
    if (!this._metadata) {
      throw new PNGError('PNG metadata not available');
    }
    return this._metadata;
  }

  // 图像操作方法
  bitblt(
    src: PNG, 
    dst: PNG, 
    srcX: number, 
    srcY: number, 
    width: number, 
    height: number, 
    deltaX: number, 
    deltaY: number
  ): void {
    if (!this.wasmPNG) {
      throw new PNGError('WASM PNG not initialized');
    }

    try {
      this.wasmPNG.bitblt(src.wasmPNG, dst.wasmPNG, srcX, srcY, width, height, deltaX, deltaY);
    } catch (error) {
      throw new PNGError(`BitBlt operation failed: ${error}`);
    }
  }

  adjustGamma(): void {
    if (!this.wasmPNG) {
      throw new PNGError('WASM PNG not initialized');
    }

    try {
      this.wasmPNG.adjustGamma();
    } catch (error) {
      throw new PNGError(`Gamma adjustment failed: ${error}`);
    }
  }

  isInterlaced(): boolean {
    return this.getInterlaceMethod() === 1; // Adam7
  }

  getInterlacePasses(): number {
    if (!this.wasmPNG) {
      throw new PNGError('WASM PNG not initialized');
    }

    try {
      return this.wasmPNG.getInterlacePasses();
    } catch (error) {
      throw new PNGError(`Failed to get interlace passes: ${error}`);
    }
  }

  getInterlaceStats(): any {
    if (!this.wasmPNG) {
      throw new PNGError('WASM PNG not initialized');
    }

    try {
      return this.wasmPNG.getInterlaceStats();
    } catch (error) {
      throw new PNGError(`Failed to get interlace stats: ${error}`);
    }
  }

  // 解析和编码方法
  parse(data: Uint8Array, callback: ParseCallback): void {
    if (!this.wasmPNG) {
      callback(new PNGError('WASM PNG not initialized'));
      return;
    }

    try {
      this.wasmPNG.parse(data, (error: any, result: any) => {
        if (error) {
          callback(new PNGError(`Parse failed: ${error}`));
          return;
        }

        // 更新内部状态
        this._width = result.width;
        this._height = result.height;
        this._data = result.data;
        this._metadata = result.metadata;
        this._gamma = result.gamma || 0;
        this._alpha = result.alpha || false;
        this._readable = true;

        callback(null, this);
      });
    } catch (error) {
      callback(new PNGError(`Parse operation failed: ${error}`));
    }
  }

  parseSync(data: Uint8Array): PNG {
    if (!this.wasmPNG) {
      throw new PNGError('WASM PNG not initialized');
    }

    try {
      const result = this.wasmPNG.parseSync(data);
      
      // 更新内部状态
      this._width = result.width;
      this._height = result.height;
      this._data = result.data;
      this._metadata = result.metadata;
      this._gamma = result.gamma || 0;
      this._alpha = result.alpha || false;
      this._readable = true;

      return this;
    } catch (error) {
      throw new PNGError(`Sync parse failed: ${error}`);
    }
  }

  pack(): Uint8Array {
    if (!this.wasmPNG) {
      throw new PNGError('WASM PNG not initialized');
    }

    try {
      return this.wasmPNG.pack();
    } catch (error) {
      throw new PNGError(`Pack operation failed: ${error}`);
    }
  }

  writeFile(filename: string): void {
    if (!this.wasmPNG) {
      throw new PNGError('WASM PNG not initialized');
    }

    try {
      this.wasmPNG.writeFile(filename);
    } catch (error) {
      throw new PNGError(`Write file failed: ${error}`);
    }
  }

  toBuffer(): Uint8Array {
    return this.pack();
  }

  // 高级功能方法
  setAdvancedFilters(options: AdvancedFilterOptions): void {
    if (!this.wasmPNG) {
      throw new PNGError('WASM PNG not initialized');
    }

    try {
      this.wasmPNG.setAdvancedFilters(options);
    } catch (error) {
      throw new PNGError(`Failed to set advanced filters: ${error}`);
    }
  }

  setWASMOptimization(options: WASMOptimizationOptions): void {
    if (!this.wasmPNG) {
      throw new PNGError('WASM PNG not initialized');
    }

    try {
      this.wasmPNG.setWASMOptimization(options);
    } catch (error) {
      throw new PNGError(`Failed to set WASM optimization: ${error}`);
    }
  }

  getPerformanceStats(): PerformanceStats {
    if (!this.wasmPNG) {
      throw new PNGError('WASM PNG not initialized');
    }

    try {
      return this.wasmPNG.getPerformanceStats();
    } catch (error) {
      throw new PNGError(`Failed to get performance stats: ${error}`);
    }
  }
}
