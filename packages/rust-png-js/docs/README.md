# Rust PNG JS - 完整使用文档

## 目录

- [快速开始](#快速开始)
- [安装指南](#安装指南)
- [基本使用](#基本使用)
- [高级功能](#高级功能)
- [API参考](#api参考)
- [性能优化](#性能优化)
- [错误处理](#错误处理)
- [最佳实践](#最佳实践)
- [示例代码](#示例代码)
- [常见问题](#常见问题)

## 快速开始

### 安装

```bash
npm install rust-png-js
```

### 基本使用

```typescript
import { PNG, PNGSync, validatePNG } from 'rust-png-js';

// 异步解析PNG
const png = new PNG();
png.parse(data, (error, result) => {
  if (error) {
    console.error('解析失败:', error);
    return;
  }
  console.log('图像尺寸:', result.width, 'x', result.height);
});

// 同步解析PNG
const pngSync = new PNGSync();
const result = pngSync.read(data);
console.log('图像尺寸:', result.width, 'x', result.height);
```

## 安装指南

### 环境要求

- **Node.js**: 16.0.0 或更高版本
- **浏览器**: Chrome 57+, Firefox 52+, Safari 11+, Edge 16+
- **TypeScript**: 4.0+ (可选，但推荐)

### 安装方式

#### npm
```bash
npm install rust-png-js
```

#### yarn
```bash
yarn add rust-png-js
```

#### pnpm
```bash
pnpm add rust-png-js
```

### 类型定义

TypeScript类型定义已包含在包中，无需额外安装`@types/`包。

## 基本使用

### 导入模块

```typescript
// 完整导入
import { PNG, PNGSync, SemanticPNG, SemanticPNGSync } from 'rust-png-js';

// 按需导入
import { PNG } from 'rust-png-js';

// 工具函数导入
import { validatePNG, getPNGInfo, optimizePNG } from 'rust-png-js';

// 类型导入
import type { PNGMetadata, PixelArray, EncodeOptions } from 'rust-png-js';
```

### 创建PNG实例

```typescript
// 异步PNG实例
const png = new PNG();

// 同步PNG实例
const pngSync = new PNGSync();

// 语义PNG实例
const semanticPng = new SemanticPNG();
```

### 解析PNG数据

#### 异步解析

```typescript
const png = new PNG();

// 使用回调
png.parse(data, (error, result) => {
  if (error) {
    console.error('解析失败:', error);
    return;
  }
  
  console.log('解析成功!');
  console.log('尺寸:', result.width, 'x', result.height);
  console.log('颜色类型:', result.getColorType());
  console.log('位深度:', result.getBitDepth());
});

// 使用Promise包装
const parsePNG = (data: Uint8Array): Promise<PNG> => {
  return new Promise((resolve, reject) => {
    png.parse(data, (error, result) => {
      if (error) {
        reject(error);
        return;
      }
      resolve(result);
    });
  });
};

// 使用async/await
try {
  const result = await parsePNG(data);
  console.log('解析成功:', result.width, 'x', result.height);
} catch (error) {
  console.error('解析失败:', error);
}
```

#### 同步解析

```typescript
const pngSync = new PNGSync();

try {
  const png = pngSync.read(data);
  console.log('解析成功!');
  console.log('尺寸:', png.width, 'x', png.height);
  console.log('颜色类型:', png.getColorType());
  console.log('位深度:', png.getBitDepth());
} catch (error) {
  console.error('解析失败:', error);
}
```

### 访问图像数据

```typescript
const png = pngSync.read(data);

// 基本属性
console.log('宽度:', png.width);
console.log('高度:', png.height);
console.log('像素数据:', png.data);
console.log('Gamma值:', png.gamma);
console.log('是否有Alpha:', png.alpha);

// 获取像素值
const pixel = png.getPixel(100, 50); // [R, G, B, A]
console.log('像素值:', pixel);

// 获取RGBA数组
const rgbaArray = png.getRGBA8Array();
console.log('RGBA数组长度:', rgbaArray.length);

// 获取元数据
const metadata = png.getMetadata();
console.log('元数据:', metadata);
```

### 编码PNG数据

```typescript
// 打包为PNG数据
const pngData = png.pack();
console.log('PNG数据长度:', pngData.length);

// 转换为Buffer (Node.js)
const buffer = png.toBuffer();

// 写入文件 (Node.js)
png.writeFile('output.png');
```

## 高级功能

### 语义PNG处理

```typescript
import { SemanticPNG, SemanticPNGSync } from 'rust-png-js';

// 创建语义PNG实例
const semanticPng = new SemanticPNG();

// 设置语义元数据
semanticPng.setSemanticMetadata({
  author: 'John Doe',
  description: 'My awesome image',
  tags: ['nature', 'landscape'],
  created: new Date(),
  location: {
    latitude: 40.7128,
    longitude: -74.0060
  }
});

// 解析PNG并保留语义信息
semanticPng.parse(data, (error, result) => {
  if (error) {
    console.error('解析失败:', error);
    return;
  }
  
  // 获取语义元数据
  const metadata = result.getSemanticMetadata();
  console.log('语义元数据:', metadata);
});
```

### 高级滤镜

```typescript
import { AdvancedFilterOptions } from 'rust-png-js';

// 配置高级滤镜
const filterOptions: AdvancedFilterOptions = {
  enableAdaptiveFilter: true,
  enableEdgeDetection: true,
  enableNoiseReduction: true,
  threshold: 0.5,
  sensitivity: 0.8,
  strength: 0.6
};

png.setAdvancedFilters(filterOptions);
```

### WebAssembly优化

```typescript
import { WASMOptimizationOptions } from 'rust-png-js';

// 配置WASM优化
const wasmOptions: WASMOptimizationOptions = {
  enableSIMD: true,
  enableParallel: true,
  enableCaching: true,
  memoryPoolSize: 1024 * 1024 * 10, // 10MB
  threadCount: 4
};

png.setWASMOptimization(wasmOptions);
```

### 性能监控

```typescript
// 获取性能统计
const stats = png.getPerformanceStats();
console.log('解析时间:', stats.parseTime, 'ms');
console.log('编码时间:', stats.encodeTime, 'ms');
console.log('内存使用:', stats.memoryUsage, 'bytes');
console.log('压缩比:', stats.compressionRatio, '%');

// 性能基准测试
import { benchmark } from 'rust-png-js';

const performanceStats = await benchmark(data, 100);
console.log('基准测试结果:', performanceStats);
```

## API参考

### 主要类

#### PNG类

```typescript
class PNG {
  // 属性
  readonly width: number;
  readonly height: number;
  readonly data: PixelData;
  readonly gamma: number;
  readonly alpha: boolean;
  readonly readable: boolean;
  readonly writable: boolean;

  // 方法
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
  bitblt(src: PNG, dst: PNG, srcX: number, srcY: number, 
         width: number, height: number, deltaX: number, deltaY: number): void;
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
```

#### PNGSync类

```typescript
class PNGSync {
  read(data: Uint8Array): PNG;
  write(png: PNG, options?: EncodeOptions): Uint8Array;
}
```

#### SemanticPNG类

```typescript
class SemanticPNG extends PNG {
  getSemanticMetadata(): any;
  setSemanticMetadata(metadata: any): void;
}
```

### 工具函数

```typescript
// 验证PNG数据
function validatePNG(data: Uint8Array): Promise<boolean>;

// 获取PNG信息
function getPNGInfo(data: Uint8Array): Promise<PNGMetadata | null>;

// 转换颜色类型
function convertColorType(
  data: Uint8Array, 
  fromType: ColorType, 
  toType: ColorType
): Promise<Uint8Array>;

// 优化PNG
function optimizePNG(
  data: Uint8Array, 
  options?: EncodeOptions
): Promise<Uint8Array>;

// 性能基准测试
function benchmark(
  data: Uint8Array, 
  iterations?: number
): Promise<PerformanceStats>;

// 内存管理
function getMemoryUsage(): Promise<number>;
function clearCache(): Promise<void>;

// 环境检查
function isWASMSupported(): boolean;
function preloadWASM(): Promise<void>;
function isWASMLoaded(): Promise<boolean>;
```

### 类型定义

```typescript
// 基础类型
interface PNGDimensions {
  width: number;
  height: number;
}

interface PNGColorInfo {
  colorType: number;
  bitDepth: number;
  hasAlpha: boolean;
}

interface PNGMetadata {
  dimensions: PNGDimensions;
  colorInfo: PNGColorInfo;
  compression: PNGCompression;
  interlace: PNGInterlace;
  gamma?: number;
  palette?: Uint8Array;
  transparentColor?: Uint16Array;
}

// 像素数据类型
type PixelData = Uint8Array | Uint8ClampedArray;
type PixelArray = [number, number, number, number]; // RGBA

// 选项类型
interface ParseOptions {
  data?: boolean;
  skipRescale?: boolean;
  colorType?: number;
  bitDepth?: number;
}

interface EncodeOptions {
  deflateLevel?: number;
  deflateStrategy?: number;
  filterType?: number;
  colorType?: number;
  bitDepth?: number;
  palette?: Uint8Array;
  transparentColor?: Uint16Array;
  gamma?: number;
}

// 枚举类型
enum FilterType {
  None = 0,
  Sub = 1,
  Up = 2,
  Average = 3,
  Paeth = 4,
  Adaptive = 5
}

enum ColorType {
  Grayscale = 0,
  RGB = 2,
  Palette = 3,
  GrayscaleAlpha = 4,
  RGBA = 6
}

enum BitDepth {
  One = 1,
  Two = 2,
  Four = 4,
  Eight = 8,
  Sixteen = 16
}
```

## 性能优化

### WebAssembly优化

```typescript
import { preloadWASM, isWASMSupported } from 'rust-png-js';

// 检查WASM支持
if (isWASMSupported()) {
  // 预加载WASM模块
  await preloadWASM();
  
  // 配置优化选项
  const wasmOptions: WASMOptimizationOptions = {
    enableSIMD: true,
    enableParallel: true,
    enableCaching: true,
    memoryPoolSize: 1024 * 1024 * 10, // 10MB
    threadCount: navigator.hardwareConcurrency || 4
  };
  
  png.setWASMOptimization(wasmOptions);
}
```

### 内存管理

```typescript
import { getMemoryUsage, clearCache } from 'rust-png-js';

// 监控内存使用
const memoryUsage = await getMemoryUsage();
console.log('当前内存使用:', memoryUsage / 1024 / 1024, 'MB');

// 清理缓存
await clearCache();
console.log('缓存已清理');
```

### 批量处理

```typescript
// 批量处理PNG文件
async function processBatch(files: File[]): Promise<PNG[]> {
  const results: PNG[] = [];
  
  for (const file of files) {
    const data = new Uint8Array(await file.arrayBuffer());
    const png = pngSync.read(data);
    results.push(png);
  }
  
  return results;
}

// 并行处理
async function processBatchParallel(files: File[]): Promise<PNG[]> {
  const promises = files.map(async (file) => {
    const data = new Uint8Array(await file.arrayBuffer());
    return pngSync.read(data);
  });
  
  return Promise.all(promises);
}
```

## 错误处理

### 错误类型

```typescript
import { PNGError } from 'rust-png-js';

try {
  const png = pngSync.read(data);
} catch (error) {
  if (error instanceof PNGError) {
    console.error('PNG错误:', error.message);
    console.error('错误代码:', error.code);
  } else {
    console.error('未知错误:', error);
  }
}
```

### 异步错误处理

```typescript
png.parse(data, (error, result) => {
  if (error) {
    if (error instanceof PNGError) {
      console.error('PNG解析错误:', error.message);
      console.error('错误代码:', error.code);
    } else {
      console.error('解析失败:', error);
    }
    return;
  }
  
  // 处理成功结果
  console.log('解析成功:', result.width, 'x', result.height);
});
```

### 错误恢复

```typescript
async function parseWithRetry(data: Uint8Array, maxRetries: number = 3): Promise<PNG> {
  for (let i = 0; i < maxRetries; i++) {
    try {
      return pngSync.read(data);
    } catch (error) {
      if (i === maxRetries - 1) {
        throw error;
      }
      
      console.warn(`解析失败，重试 ${i + 1}/${maxRetries}:`, error);
      await new Promise(resolve => setTimeout(resolve, 1000));
    }
  }
  
  throw new Error('解析失败，已达到最大重试次数');
}
```

## 最佳实践

### 1. 预加载WASM模块

```typescript
import { preloadWASM, isWASMSupported } from 'rust-png-js';

// 应用启动时预加载
async function initializeApp() {
  if (isWASMSupported()) {
    try {
      await preloadWASM();
      console.log('WASM模块预加载完成');
    } catch (error) {
      console.error('WASM模块预加载失败:', error);
    }
  }
}

// 在应用入口调用
initializeApp();
```

### 2. 内存管理

```typescript
// 定期清理缓存
setInterval(async () => {
  const memoryUsage = await getMemoryUsage();
  if (memoryUsage > 100 * 1024 * 1024) { // 100MB
    await clearCache();
    console.log('内存使用过高，已清理缓存');
  }
}, 30000); // 每30秒检查一次
```

### 3. 错误边界

```typescript
class PNGProcessor {
  async processPNG(data: Uint8Array): Promise<PNG> {
    try {
      // 验证数据
      const isValid = await validatePNG(data);
      if (!isValid) {
        throw new PNGError('无效的PNG数据', 'INVALID_PNG');
      }
      
      // 解析PNG
      return pngSync.read(data);
    } catch (error) {
      console.error('PNG处理失败:', error);
      throw error;
    }
  }
}
```

### 4. 性能监控

```typescript
class PerformanceMonitor {
  private startTime: number = 0;
  
  start() {
    this.startTime = performance.now();
  }
  
  end(): number {
    return performance.now() - this.startTime;
  }
  
  async benchmarkPNG(data: Uint8Array, iterations: number = 10): Promise<PerformanceStats> {
    const results: number[] = [];
    
    for (let i = 0; i < iterations; i++) {
      this.start();
      await pngSync.read(data);
      results.push(this.end());
    }
    
    const avgTime = results.reduce((a, b) => a + b, 0) / results.length;
    const minTime = Math.min(...results);
    const maxTime = Math.max(...results);
    
    return {
      parseTime: avgTime,
      encodeTime: 0,
      memoryUsage: await getMemoryUsage(),
      compressionRatio: 0
    };
  }
}
```

## 示例代码

### 浏览器示例

```typescript
// 文件上传处理
document.getElementById('fileInput')?.addEventListener('change', async (event) => {
  const file = (event.target as HTMLInputElement).files?.[0];
  if (!file) return;
  
  const data = new Uint8Array(await file.arrayBuffer());
  
  try {
    // 验证PNG
    const isValid = await validatePNG(data);
    if (!isValid) {
      throw new Error('无效的PNG文件');
    }
    
    // 解析PNG
    const png = pngSync.read(data);
    
    // 显示图像
    const canvas = document.getElementById('canvas') as HTMLCanvasElement;
    const ctx = canvas.getContext('2d')!;
    
    canvas.width = png.width;
    canvas.height = png.height;
    
    const imageData = ctx.createImageData(png.width, png.height);
    imageData.data.set(png.getRGBA8Array());
    ctx.putImageData(imageData, 0, 0);
    
    // 显示信息
    console.log('图像尺寸:', png.width, 'x', png.height);
    console.log('颜色类型:', png.getColorType());
    console.log('位深度:', png.getBitDepth());
    
  } catch (error) {
    console.error('处理失败:', error);
  }
});
```

### Node.js示例

```typescript
import { PNGSync, validatePNG, optimizePNG } from 'rust-png-js';
import { readFileSync, writeFileSync } from 'fs';
import { join } from 'path';

async function processPNGFile(inputPath: string, outputPath: string) {
  try {
    // 读取PNG文件
    const data = readFileSync(inputPath);
    
    // 验证PNG
    const isValid = await validatePNG(data);
    if (!isValid) {
      throw new Error('无效的PNG文件');
    }
    
    // 解析PNG
    const png = pngSync.read(data);
    console.log('图像尺寸:', png.width, 'x', png.height);
    console.log('颜色类型:', png.getColorType());
    console.log('位深度:', png.getBitDepth());
    
    // 优化PNG
    const optimizedData = await optimizePNG(data, {
      deflateLevel: 9,
      filterType: 5 // 自适应滤镜
    });
    
    // 保存优化后的文件
    writeFileSync(outputPath, optimizedData);
    
    const originalSize = data.length;
    const optimizedSize = optimizedData.length;
    const compressionRatio = ((originalSize - optimizedSize) / originalSize * 100).toFixed(2);
    
    console.log('优化完成!');
    console.log('原始大小:', originalSize, 'bytes');
    console.log('优化后大小:', optimizedSize, 'bytes');
    console.log('压缩比:', compressionRatio + '%');
    
  } catch (error) {
    console.error('处理失败:', error);
  }
}

// 使用示例
processPNGFile('input.png', 'output.png');
```

### 批量处理示例

```typescript
import { PNGSync, optimizePNG } from 'rust-png-js';
import { readdir, readFile, writeFile } from 'fs/promises';
import { join, extname } from 'path';

async function batchProcessPNG(inputDir: string, outputDir: string) {
  try {
    const files = await readdir(inputDir);
    const pngFiles = files.filter(file => extname(file).toLowerCase() === '.png');
    
    console.log(`找到 ${pngFiles.length} 个PNG文件`);
    
    const results = await Promise.all(
      pngFiles.map(async (file) => {
        const inputPath = join(inputDir, file);
        const outputPath = join(outputDir, file);
        
        try {
          // 读取文件
          const data = await readFile(inputPath);
          
          // 优化PNG
          const optimizedData = await optimizePNG(data, {
            deflateLevel: 9,
            filterType: 5
          });
          
          // 保存优化后的文件
          await writeFile(outputPath, optimizedData);
          
          const originalSize = data.length;
          const optimizedSize = optimizedData.length;
          const compressionRatio = ((originalSize - optimizedSize) / originalSize * 100).toFixed(2);
          
          return {
            file,
            originalSize,
            optimizedSize,
            compressionRatio: parseFloat(compressionRatio)
          };
          
        } catch (error) {
          console.error(`处理文件 ${file} 失败:`, error);
          return null;
        }
      })
    );
    
    // 统计结果
    const validResults = results.filter(r => r !== null);
    const totalOriginalSize = validResults.reduce((sum, r) => sum + r!.originalSize, 0);
    const totalOptimizedSize = validResults.reduce((sum, r) => sum + r!.optimizedSize, 0);
    const avgCompressionRatio = validResults.reduce((sum, r) => sum + r!.compressionRatio, 0) / validResults.length;
    
    console.log('批量处理完成!');
    console.log(`成功处理: ${validResults.length}/${pngFiles.length} 个文件`);
    console.log(`总原始大小: ${(totalOriginalSize / 1024 / 1024).toFixed(2)} MB`);
    console.log(`总优化后大小: ${(totalOptimizedSize / 1024 / 1024).toFixed(2)} MB`);
    console.log(`平均压缩比: ${avgCompressionRatio.toFixed(2)}%`);
    
  } catch (error) {
    console.error('批量处理失败:', error);
  }
}

// 使用示例
batchProcessPNG('./input', './output');
```

## 常见问题

### Q: 如何检查WebAssembly支持？

```typescript
import { isWASMSupported } from 'rust-png-js';

if (isWASMSupported()) {
  console.log('WebAssembly支持');
} else {
  console.log('WebAssembly不支持');
}
```

### Q: 如何处理大文件？

```typescript
// 使用流式处理
async function processLargePNG(data: Uint8Array): Promise<PNG> {
  // 检查文件大小
  if (data.length > 100 * 1024 * 1024) { // 100MB
    console.warn('文件较大，处理可能需要较长时间');
  }
  
  // 使用同步解析避免内存问题
  return pngSync.read(data);
}
```

### Q: 如何优化内存使用？

```typescript
// 定期清理缓存
setInterval(async () => {
  const memoryUsage = await getMemoryUsage();
  if (memoryUsage > 50 * 1024 * 1024) { // 50MB
    await clearCache();
  }
}, 60000); // 每分钟检查一次
```

### Q: 如何处理错误？

```typescript
try {
  const png = pngSync.read(data);
} catch (error) {
  if (error instanceof PNGError) {
    console.error('PNG错误:', error.message);
  } else {
    console.error('未知错误:', error);
  }
}
```

### Q: 如何获取性能统计？

```typescript
import { benchmark } from 'rust-png-js';

const stats = await benchmark(data, 10);
console.log('解析时间:', stats.parseTime, 'ms');
console.log('内存使用:', stats.memoryUsage, 'bytes');
```

---

## 总结

Rust PNG JS 提供了完整的TypeScript支持，让您能够在前端环境中享受Rust的高性能优势。通过本文档，您应该能够：

1. 快速上手使用库的基本功能
2. 了解高级功能和性能优化技巧
3. 掌握错误处理和最佳实践
4. 解决常见问题

如果您有任何问题或建议，请查看项目的GitHub仓库或提交Issue。
