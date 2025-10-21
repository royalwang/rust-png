/**
 * 语义同步PNG类封装
 * 提供语义化的同步PNG处理API
 */

import { 
  SemanticPNGSyncInterface, 
  SemanticPNGInterface, 
  EncodeOptions,
  PNGError
} from './types';
import { SemanticPNG } from './semantic-png';
import { loadWASMModule } from './wasm-loader';

export class SemanticPNGSync implements SemanticPNGSyncInterface {
  private wasmSemanticPNGSync: any;

  constructor() {
    this.initializeWASM();
  }

  private async initializeWASM(): Promise<void> {
    try {
      const wasmModule = await loadWASMModule();
      this.wasmSemanticPNGSync = new wasmModule.SemanticPNGSync();
    } catch (error) {
      throw new PNGError(`Failed to initialize WASM SemanticPNGSync: ${error}`);
    }
  }

  /**
   * 同步读取语义PNG数据
   */
  readSemantic(data: Uint8Array): SemanticPNGInterface {
    if (!this.wasmSemanticPNGSync) {
      throw new PNGError('WASM SemanticPNGSync not initialized');
    }

    try {
      const result = this.wasmSemanticPNGSync.readSemantic(data);
      
      // 创建SemanticPNG实例并设置数据
      const png = new SemanticPNG();
      (png as any)._width = result.width;
      (png as any)._height = result.height;
      (png as any)._data = result.data;
      (png as any)._metadata = result.metadata;
      (png as any)._gamma = result.gamma || 0;
      (png as any)._alpha = result.alpha || false;
      (png as any)._readable = true;
      (png as any)._semanticMetadata = result.semanticMetadata;

      return png;
    } catch (error) {
      throw new PNGError(`Semantic sync read failed: ${error}`);
    }
  }

  /**
   * 同步写入语义PNG数据
   */
  writeSemantic(png: SemanticPNGInterface, options?: EncodeOptions): Uint8Array {
    if (!this.wasmSemanticPNGSync) {
      throw new PNGError('WASM SemanticPNGSync not initialized');
    }

    try {
      return this.wasmSemanticPNGSync.writeSemantic(png, options || {});
    } catch (error) {
      throw new PNGError(`Semantic sync write failed: ${error}`);
    }
  }
}
