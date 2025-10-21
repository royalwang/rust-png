/**
 * WebAssembly加载器
 * 负责加载和初始化Rust编译的WASM模块
 */

import { RustPNGModule } from './types';

// WASM模块缓存
let wasmModule: RustPNGModule | null = null;
let wasmModulePromise: Promise<RustPNGModule> | null = null;

/**
 * 加载WebAssembly模块
 */
export async function loadWASMModule(): Promise<RustPNGModule> {
  if (wasmModule) {
    return wasmModule;
  }

  if (wasmModulePromise) {
    return wasmModulePromise;
  }

  wasmModulePromise = loadWASMModuleInternal();
  wasmModule = await wasmModulePromise;
  return wasmModule;
}

/**
 * 内部加载函数
 */
async function loadWASMModuleInternal(): Promise<RustPNGModule> {
  try {
    // 动态导入WASM模块
    const wasmInit = await import('../dist/wasm/rust_png.js');
    
    // 初始化WASM模块
    await wasmInit.default();
    
    // 返回模块接口
    return {
      PNG: wasmInit.PNG,
      PNGSync: wasmInit.PNGSync,
      SemanticPNG: wasmInit.SemanticPNG,
      SemanticPNGSync: wasmInit.SemanticPNGSync,
      
      // 工具函数
      validatePNG: wasmInit.validatePNG,
      getPNGInfo: wasmInit.getPNGInfo,
      convertColorType: wasmInit.convertColorType,
      optimizePNG: wasmInit.optimizePNG,
      
      // 性能工具
      benchmark: wasmInit.benchmark,
      getMemoryUsage: wasmInit.getMemoryUsage,
      clearCache: wasmInit.clearCache,
    };
  } catch (error) {
    throw new Error(`Failed to load WASM module: ${error}`);
  }
}

/**
 * 检查WASM支持
 */
export function isWASMSupported(): boolean {
  try {
    return typeof WebAssembly === 'object' && 
           typeof WebAssembly.instantiate === 'function';
  } catch {
    return false;
  }

}

/**
 * 获取WASM模块信息
 */
export function getWASMInfo(): { supported: boolean; loaded: boolean } {
  return {
    supported: isWASMSupported(),
    loaded: wasmModule !== null
  };
}

/**
 * 清理WASM模块缓存
 */
export function clearWASMCache(): void {
  wasmModule = null;
  wasmModulePromise = null;
}

/**
 * 预加载WASM模块
 */
export async function preloadWASM(): Promise<void> {
  if (!isWASMSupported()) {
    throw new Error('WebAssembly is not supported in this environment');
  }
  
  await loadWASMModule();
}

/**
 * 检查模块是否已加载
 */
export function isWASMLoaded(): boolean {
  return wasmModule !== null;
}
