/**
 * Rust PNG JS - 主入口文件
 * 高性能的Rust PNG处理库，完全兼容原始pngjs库的API
 */

// 导出主要类
export { PNG } from './png';
export { PNGSync } from './png-sync';
export { SemanticPNG } from './semantic-png';
export { SemanticPNGSync } from './semantic-png-sync';

// 导出工具函数
export {
  validatePNG,
  getPNGInfo,
  convertColorType,
  optimizePNG,
  benchmark,
  getMemoryUsage,
  clearCache,
  isWASMSupported,
  getVersion,
  getLibraryInfo,
  preloadWASM,
  isWASMLoaded
} from './utils';

// 导出WebAssembly加载器
export { loadWASMModule, getWASMInfo, clearWASMCache, preloadWASM as preloadWASMModule } from './wasm-loader';

// 导出所有类型
export * from './types';

// 默认导出
export default {
  PNG,
  PNGSync,
  SemanticPNG,
  SemanticPNGSync,
  validatePNG,
  getPNGInfo,
  convertColorType,
  optimizePNG,
  benchmark,
  getMemoryUsage,
  clearCache,
  isWASMSupported,
  getVersion,
  getLibraryInfo,
  preloadWASM,
  isWASMLoaded
};
