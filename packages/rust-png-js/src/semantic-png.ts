/**
 * 语义PNG类封装
 * 提供语义化的PNG处理API
 */

import { 
  SemanticPNGInterface, 
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
import { PNG } from './png';
import { loadWASMModule } from './wasm-loader';

export class SemanticPNG extends PNG implements SemanticPNGInterface {
  private wasmSemanticPNG: any;
  private _semanticMetadata: any = null;

  constructor() {
    super();
    this.initializeWASM();
  }

  private async initializeWASM(): Promise<void> {
    try {
      const wasmModule = await loadWASMModule();
      this.wasmSemanticPNG = new wasmModule.SemanticPNG();
    } catch (error) {
      throw new PNGError(`Failed to initialize WASM SemanticPNG: ${error}`);
    }
  }

  /**
   * 获取语义元数据
   */
  getSemanticMetadata(): any {
    if (!this._semanticMetadata) {
      throw new PNGError('Semantic metadata not available');
    }
    return this._semanticMetadata;
  }

  /**
   * 设置语义元数据
   */
  setSemanticMetadata(metadata: any): void {
    if (!this.wasmSemanticPNG) {
      throw new PNGError('WASM SemanticPNG not initialized');
    }

    try {
      this.wasmSemanticPNG.setSemanticMetadata(metadata);
      this._semanticMetadata = metadata;
    } catch (error) {
      throw new PNGError(`Failed to set semantic metadata: ${error}`);
    }
  }

  /**
   * 解析PNG数据（语义版本）
   */
  parse(data: Uint8Array, callback: ParseCallback): void {
    if (!this.wasmSemanticPNG) {
      callback(new PNGError('WASM SemanticPNG not initialized'));
      return;
    }

    try {
      this.wasmSemanticPNG.parse(data, (error: any, result: any) => {
        if (error) {
          callback(new PNGError(`Semantic parse failed: ${error}`));
          return;
        }

        // 更新内部状态
        (this as any)._width = result.width;
        (this as any)._height = result.height;
        (this as any)._data = result.data;
        (this as any)._metadata = result.metadata;
        (this as any)._gamma = result.gamma || 0;
        (this as any)._alpha = result.alpha || false;
        (this as any)._readable = true;
        this._semanticMetadata = result.semanticMetadata;

        callback(null, this);
      });
    } catch (error) {
      callback(new PNGError(`Semantic parse operation failed: ${error}`));
    }
  }

  /**
   * 同步解析PNG数据（语义版本）
   */
  parseSync(data: Uint8Array): SemanticPNG {
    if (!this.wasmSemanticPNG) {
      throw new PNGError('WASM SemanticPNG not initialized');
    }

    try {
      const result = this.wasmSemanticPNG.parseSync(data);
      
      // 更新内部状态
      (this as any)._width = result.width;
      (this as any)._height = result.height;
      (this as any)._data = result.data;
      (this as any)._metadata = result.metadata;
      (this as any)._gamma = result.gamma || 0;
      (this as any)._alpha = result.alpha || false;
      (this as any)._readable = true;
      this._semanticMetadata = result.semanticMetadata;

      return this;
    } catch (error) {
      throw new PNGError(`Semantic sync parse failed: ${error}`);
    }
  }

  /**
   * 打包PNG数据（语义版本）
   */
  pack(): Uint8Array {
    if (!this.wasmSemanticPNG) {
      throw new PNGError('WASM SemanticPNG not initialized');
    }

    try {
      return this.wasmSemanticPNG.pack();
    } catch (error) {
      throw new PNGError(`Semantic pack operation failed: ${error}`);
    }
  }
}
