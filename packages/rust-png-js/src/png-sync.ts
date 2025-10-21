/**
 * 同步PNG类封装
 * 提供同步的PNG处理API
 */

import { 
  PNGSyncInterface, 
  PNGInterface, 
  EncodeOptions,
  PNGError
} from './types';
import { PNG } from './png';
import { loadWASMModule } from './wasm-loader';

export class PNGSync implements PNGSyncInterface {
  private wasmPNGSync: any;

  constructor() {
    this.initializeWASM();
  }

  private async initializeWASM(): Promise<void> {
    try {
      const wasmModule = await loadWASMModule();
      this.wasmPNGSync = new wasmModule.PNGSync();
    } catch (error) {
      throw new PNGError(`Failed to initialize WASM PNGSync: ${error}`);
    }
  }

  /**
   * 同步读取PNG数据
   */
  read(data: Uint8Array): PNGInterface {
    if (!this.wasmPNGSync) {
      throw new PNGError('WASM PNGSync not initialized');
    }

    try {
      const result = this.wasmPNGSync.read(data);
      
      // 创建PNG实例并设置数据
      const png = new PNG();
      (png as any)._width = result.width;
      (png as any)._height = result.height;
      (png as any)._data = result.data;
      (png as any)._metadata = result.metadata;
      (png as any)._gamma = result.gamma || 0;
      (png as any)._alpha = result.alpha || false;
      (png as any)._readable = true;

      return png;
    } catch (error) {
      throw new PNGError(`Sync read failed: ${error}`);
    }
  }

  /**
   * 同步写入PNG数据
   */
  write(png: PNGInterface, options?: EncodeOptions): Uint8Array {
    if (!this.wasmPNGSync) {
      throw new PNGError('WASM PNGSync not initialized');
    }

    try {
      return this.wasmPNGSync.write(png, options || {});
    } catch (error) {
      throw new PNGError(`Sync write failed: ${error}`);
    }
  }
}
