/**
 * 工具函数
 * 提供各种辅助功能
 */

import { 
  PNGMetadata, 
  ColorType, 
  EncodeOptions,
  PerformanceStats,
  PNGError
} from './types';
import { loadWASMModule } from './wasm-loader';

/**
 * 验证PNG数据
 */
export async function validatePNG(data: Uint8Array): Promise<boolean> {
  try {
    const wasmModule = await loadWASMModule();
    return wasmModule.validatePNG(data);
  } catch (error) {
    throw new PNGError(`PNG validation failed: ${error}`);
  }
}

/**
 * 获取PNG信息
 */
export async function getPNGInfo(data: Uint8Array): Promise<PNGMetadata | null> {
  try {
    const wasmModule = await loadWASMModule();
    return wasmModule.getPNGInfo(data);
  } catch (error) {
    throw new PNGError(`Failed to get PNG info: ${error}`);
  }
}

/**
 * 转换颜色类型
 */
export async function convertColorType(
  data: Uint8Array, 
  fromType: ColorType, 
  toType: ColorType
): Promise<Uint8Array> {
  try {
    const wasmModule = await loadWASMModule();
    return wasmModule.convertColorType(data, fromType, toType);
  } catch (error) {
    throw new PNGError(`Color type conversion failed: ${error}`);
  }
}

/**
 * 优化PNG
 */
export async function optimizePNG(
  data: Uint8Array, 
  options?: EncodeOptions
): Promise<Uint8Array> {
  try {
    const wasmModule = await loadWASMModule();
    return wasmModule.optimizePNG(data, options || {});
  } catch (error) {
    throw new PNGError(`PNG optimization failed: ${error}`);
  }
}

/**
 * 性能基准测试
 */
export async function benchmark(
  data: Uint8Array, 
  iterations: number = 100
): Promise<PerformanceStats> {
  try {
    const wasmModule = await loadWASMModule();
    return wasmModule.benchmark(data, iterations);
  } catch (error) {
    throw new PNGError(`Benchmark failed: ${error}`);
  }
}

/**
 * 获取内存使用情况
 */
export async function getMemoryUsage(): Promise<number> {
  try {
    const wasmModule = await loadWASMModule();
    return wasmModule.getMemoryUsage();
  } catch (error) {
    throw new PNGError(`Failed to get memory usage: ${error}`);
  }
}

/**
 * 清理缓存
 */
export async function clearCache(): Promise<void> {
  try {
    const wasmModule = await loadWASMModule();
    wasmModule.clearCache();
  } catch (error) {
    throw new PNGError(`Failed to clear cache: ${error}`);
  }
}

/**
 * 检查WebAssembly支持
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
 * 获取库版本信息
 */
export function getVersion(): string {
  return '0.1.0';
}

/**
 * 获取库信息
 */
export function getLibraryInfo(): {
  name: string;
  version: string;
  wasmSupported: boolean;
  features: string[];
} {
  return {
    name: 'rust-png-js',
    version: getVersion(),
    wasmSupported: isWASMSupported(),
    features: [
      'PNG parsing and encoding',
      'WebAssembly optimization',
      'Advanced filters',
      'Performance monitoring',
      'Memory optimization',
      'TypeScript support',
      '100% pngjs compatibility'
    ]
  };
}

/**
 * 预加载WebAssembly模块
 */
export async function preloadWASM(): Promise<void> {
  if (!isWASMSupported()) {
    throw new PNGError('WebAssembly is not supported in this environment');
  }
  
  try {
    await loadWASMModule();
  } catch (error) {
    throw new PNGError(`Failed to preload WASM: ${error}`);
  }
}

/**
 * 检查模块是否已加载
 */
export async function isWASMLoaded(): Promise<boolean> {
  try {
    const wasmModule = await loadWASMModule();
    return wasmModule !== null;
  } catch {
    return false;
  }
}
