# Rust PNG JS - API参考文档

## 目录

- [主要类](#主要类)
- [工具函数](#工具函数)
- [类型定义](#类型定义)
- [枚举类型](#枚举类型)
- [错误处理](#错误处理)
- [性能监控](#性能监控)

## 主要类

### PNG类

异步PNG处理类，提供完整的PNG解析和编码功能。

#### 构造函数

```typescript
new PNG(): PNG
```

创建一个新的PNG实例。

#### 属性

```typescript
readonly width: number        // 图像宽度
readonly height: number       // 图像高度
readonly data: PixelData      // 像素数据
readonly gamma: number        // Gamma值
readonly alpha: boolean       // 是否有Alpha通道
readonly readable: boolean    // 是否可读
readonly writable: boolean    // 是否可写
```

#### 方法

##### 基本访问方法

```typescript
getWidth(): number
```
返回图像宽度。

```typescript
getHeight(): number
```
返回图像高度。

```typescript
getPixel(x: number, y: number): PixelArray
```
获取指定坐标的像素值，返回`[R, G, B, A]`数组。

**参数:**
- `x: number` - X坐标
- `y: number` - Y坐标

**返回值:** `PixelArray` - 像素值数组

**异常:** 如果坐标超出边界或数据不可用，抛出`PNGError`

```typescript
getRGBA8Array(): Uint8Array
```
获取完整的RGBA像素数据数组。

**返回值:** `Uint8Array` - RGBA数据数组

**异常:** 如果数据不可用，抛出`PNGError`

##### 元数据访问方法

```typescript
getBitDepth(): number
```
返回位深度。

```typescript
getColorType(): number
```
返回颜色类型。

```typescript
getCompressionMethod(): number
```
返回压缩方法。

```typescript
getFilterMethod(): number
```
返回滤镜方法。

```typescript
getInterlaceMethod(): number
```
返回交错方法。

```typescript
getPalette(): Uint8Array | null
```
返回调色板数据，如果没有调色板则返回`null`。

```typescript
getMetadata(): PNGMetadata
```
返回完整的PNG元数据。

**返回值:** `PNGMetadata` - 元数据对象

**异常:** 如果元数据不可用，抛出`PNGError`

##### 图像操作方法

```typescript
bitblt(
  src: PNG, 
  dst: PNG, 
  srcX: number, 
  srcY: number, 
  width: number, 
  height: number, 
  deltaX: number, 
  deltaY: number
): void
```
执行位块传输操作。

**参数:**
- `src: PNG` - 源PNG对象
- `dst: PNG` - 目标PNG对象
- `srcX: number` - 源X坐标
- `srcY: number` - 源Y坐标
- `width: number` - 传输宽度
- `height: number` - 传输高度
- `deltaX: number` - X偏移量
- `deltaY: number` - Y偏移量

```typescript
adjustGamma(): void
```
调整图像Gamma值。

```typescript
isInterlaced(): boolean
```
检查图像是否交错。

```typescript
getInterlacePasses(): number
```
获取交错通道数。

```typescript
getInterlaceStats(): any
```
获取交错统计信息。

##### 解析和编码方法

```typescript
parse(data: Uint8Array, callback: ParseCallback): void
```
异步解析PNG数据。

**参数:**
- `data: Uint8Array` - PNG数据
- `callback: ParseCallback` - 回调函数

**回调函数签名:**
```typescript
type ParseCallback = (error: Error | null, png?: PNG) => void;
```

```typescript
parseSync(data: Uint8Array): PNG
```
同步解析PNG数据。

**参数:**
- `data: Uint8Array` - PNG数据

**返回值:** `PNG` - 解析后的PNG对象

**异常:** 如果解析失败，抛出`PNGError`

```typescript
pack(): Uint8Array
```
将PNG对象打包为PNG数据。

**返回值:** `Uint8Array` - PNG数据

**异常:** 如果打包失败，抛出`PNGError`

```typescript
writeFile(filename: string): void
```
将PNG写入文件。

**参数:**
- `filename: string` - 文件名

**异常:** 如果写入失败，抛出`PNGError`

```typescript
toBuffer(): Uint8Array
```
将PNG转换为Buffer。

**返回值:** `Uint8Array` - PNG数据

##### 高级功能方法

```typescript
setAdvancedFilters(options: AdvancedFilterOptions): void
```
设置高级滤镜选项。

**参数:**
- `options: AdvancedFilterOptions` - 滤镜选项

```typescript
setWASMOptimization(options: WASMOptimizationOptions): void
```
设置WebAssembly优化选项。

**参数:**
- `options: WASMOptimizationOptions` - 优化选项

```typescript
getPerformanceStats(): PerformanceStats
```
获取性能统计信息。

**返回值:** `PerformanceStats` - 性能统计对象

### PNGSync类

同步PNG处理类，提供同步的PNG处理功能。

#### 构造函数

```typescript
new PNGSync(): PNGSync
```

创建一个新的同步PNG实例。

#### 方法

```typescript
read(data: Uint8Array): PNG
```
同步读取PNG数据。

**参数:**
- `data: Uint8Array` - PNG数据

**返回值:** `PNG` - 解析后的PNG对象

**异常:** 如果读取失败，抛出`PNGError`

```typescript
write(png: PNG, options?: EncodeOptions): Uint8Array
```
同步写入PNG数据。

**参数:**
- `png: PNG` - PNG对象
- `options?: EncodeOptions` - 编码选项

**返回值:** `Uint8Array` - PNG数据

**异常:** 如果写入失败，抛出`PNGError`

### SemanticPNG类

语义PNG处理类，继承自PNG类，提供语义元数据处理功能。

#### 构造函数

```typescript
new SemanticPNG(): SemanticPNG
```

创建一个新的语义PNG实例。

#### 方法

```typescript
getSemanticMetadata(): any
```
获取语义元数据。

**返回值:** `any` - 语义元数据

**异常:** 如果元数据不可用，抛出`PNGError`

```typescript
setSemanticMetadata(metadata: any): void
```
设置语义元数据。

**参数:**
- `metadata: any` - 语义元数据

**异常:** 如果设置失败，抛出`PNGError`

### SemanticPNGSync类

语义同步PNG处理类，提供语义化的同步PNG处理功能。

#### 构造函数

```typescript
new SemanticPNGSync(): SemanticPNGSync
```

创建一个新的语义同步PNG实例。

#### 方法

```typescript
readSemantic(data: Uint8Array): SemanticPNG
```
同步读取语义PNG数据。

**参数:**
- `data: Uint8Array` - PNG数据

**返回值:** `SemanticPNG` - 解析后的语义PNG对象

**异常:** 如果读取失败，抛出`PNGError`

```typescript
writeSemantic(png: SemanticPNG, options?: EncodeOptions): Uint8Array
```
同步写入语义PNG数据。

**参数:**
- `png: SemanticPNG` - 语义PNG对象
- `options?: EncodeOptions` - 编码选项

**返回值:** `Uint8Array` - PNG数据

**异常:** 如果写入失败，抛出`PNGError`

## 工具函数

### 验证函数

```typescript
validatePNG(data: Uint8Array): Promise<boolean>
```
验证PNG数据是否有效。

**参数:**
- `data: Uint8Array` - PNG数据

**返回值:** `Promise<boolean>` - 是否有效

**异常:** 如果验证失败，抛出`PNGError`

### 信息获取函数

```typescript
getPNGInfo(data: Uint8Array): Promise<PNGMetadata | null>
```
获取PNG信息。

**参数:**
- `data: Uint8Array` - PNG数据

**返回值:** `Promise<PNGMetadata | null>` - PNG元数据，如果无效则返回null

**异常:** 如果获取失败，抛出`PNGError`

### 转换函数

```typescript
convertColorType(
  data: Uint8Array, 
  fromType: ColorType, 
  toType: ColorType
): Promise<Uint8Array>
```
转换颜色类型。

**参数:**
- `data: Uint8Array` - PNG数据
- `fromType: ColorType` - 源颜色类型
- `toType: ColorType` - 目标颜色类型

**返回值:** `Promise<Uint8Array>` - 转换后的PNG数据

**异常:** 如果转换失败，抛出`PNGError`

### 优化函数

```typescript
optimizePNG(
  data: Uint8Array, 
  options?: EncodeOptions
): Promise<Uint8Array>
```
优化PNG数据。

**参数:**
- `data: Uint8Array` - PNG数据
- `options?: EncodeOptions` - 优化选项

**返回值:** `Promise<Uint8Array>` - 优化后的PNG数据

**异常:** 如果优化失败，抛出`PNGError`

### 性能函数

```typescript
benchmark(
  data: Uint8Array, 
  iterations?: number
): Promise<PerformanceStats>
```
运行性能基准测试。

**参数:**
- `data: Uint8Array` - PNG数据
- `iterations?: number` - 迭代次数，默认为100

**返回值:** `Promise<PerformanceStats>` - 性能统计

**异常:** 如果测试失败，抛出`PNGError`

### 内存管理函数

```typescript
getMemoryUsage(): Promise<number>
```
获取当前内存使用量。

**返回值:** `Promise<number>` - 内存使用量（字节）

```typescript
clearCache(): Promise<void>
```
清理缓存。

**异常:** 如果清理失败，抛出`PNGError`

### 环境检查函数

```typescript
isWASMSupported(): boolean
```
检查WebAssembly支持。

**返回值:** `boolean` - 是否支持WebAssembly

```typescript
preloadWASM(): Promise<void>
```
预加载WebAssembly模块。

**异常:** 如果预加载失败，抛出`PNGError`

```typescript
isWASMLoaded(): Promise<boolean>
```
检查WebAssembly模块是否已加载。

**返回值:** `Promise<boolean>` - 是否已加载

## 类型定义

### 基础类型

```typescript
interface PNGDimensions {
  width: number;
  height: number;
}

interface PNGColorInfo {
  colorType: number;
  bitDepth: number;
  hasAlpha: boolean;
}

interface PNGCompression {
  method: number;
  filter: number;
}

interface PNGInterlace {
  method: number;
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
```

### 像素数据类型

```typescript
type PixelData = Uint8Array | Uint8ClampedArray;
type PixelArray = [number, number, number, number]; // RGBA
```

### 选项类型

```typescript
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

interface AdvancedFilterOptions {
  enableAdaptiveFilter?: boolean;
  enableEdgeDetection?: boolean;
  enableNoiseReduction?: boolean;
  threshold?: number;
  sensitivity?: number;
  strength?: number;
}

interface WASMOptimizationOptions {
  enableSIMD?: boolean;
  enableParallel?: boolean;
  enableCaching?: boolean;
  memoryPoolSize?: number;
  threadCount?: number;
}
```

### 性能统计类型

```typescript
interface PerformanceStats {
  parseTime: number;
  encodeTime: number;
  memoryUsage: number;
  compressionRatio: number;
}
```

### 回调函数类型

```typescript
type ParseCallback = (error: Error | null, png?: PNG) => void;
type WriteCallback = (error: Error | null, buffer?: Uint8Array) => void;
```

## 枚举类型

### 滤镜类型

```typescript
enum FilterType {
  None = 0,
  Sub = 1,
  Up = 2,
  Average = 3,
  Paeth = 4,
  Adaptive = 5
}
```

### 颜色类型

```typescript
enum ColorType {
  Grayscale = 0,
  RGB = 2,
  Palette = 3,
  GrayscaleAlpha = 4,
  RGBA = 6
}
```

### 位深度

```typescript
enum BitDepth {
  One = 1,
  Two = 2,
  Four = 4,
  Eight = 8,
  Sixteen = 16
}
```

### 交错方法

```typescript
enum InterlaceMethod {
  None = 0,
  Adam7 = 1
}
```

### 压缩方法

```typescript
enum CompressionMethod {
  Deflate = 0
}
```

### 滤镜方法

```typescript
enum FilterMethod {
  None = 0,
  Sub = 1,
  Up = 2,
  Average = 3,
  Paeth = 4,
  Adaptive = 5
}
```

## 错误处理

### PNGError类

```typescript
class PNGError extends Error {
  constructor(message: string, public code?: string);
}
```

**属性:**
- `message: string` - 错误消息
- `code?: string` - 错误代码

**常见错误代码:**
- `INVALID_PNG` - 无效的PNG数据
- `PARSE_ERROR` - 解析错误
- `ENCODE_ERROR` - 编码错误
- `MEMORY_ERROR` - 内存错误
- `WASM_ERROR` - WebAssembly错误

### 错误处理示例

```typescript
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

## 性能监控

### 性能统计接口

```typescript
interface PerformanceStats {
  parseTime: number;        // 解析时间（毫秒）
  encodeTime: number;       // 编码时间（毫秒）
  memoryUsage: number;      // 内存使用量（字节）
  compressionRatio: number; // 压缩比（百分比）
}
```

### 性能监控示例

```typescript
// 获取性能统计
const stats = png.getPerformanceStats();
console.log('解析时间:', stats.parseTime, 'ms');
console.log('编码时间:', stats.encodeTime, 'ms');
console.log('内存使用:', stats.memoryUsage, 'bytes');
console.log('压缩比:', stats.compressionRatio, '%');

// 运行基准测试
const benchmarkStats = await benchmark(data, 100);
console.log('基准测试结果:', benchmarkStats);

// 监控内存使用
const memoryUsage = await getMemoryUsage();
console.log('当前内存使用:', memoryUsage / 1024 / 1024, 'MB');
```

---

## 总结

本文档提供了Rust PNG JS库的完整API参考。通过本文档，您可以：

1. 了解所有可用的类和方法
2. 掌握正确的API使用方法
3. 理解各种类型定义和选项
4. 学会错误处理和性能监控

如果您需要更多帮助，请查看项目的GitHub仓库或提交Issue。
