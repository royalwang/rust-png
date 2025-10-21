# Rust PNG JS - 故障排除指南

## 目录

- [常见问题](#常见问题)
- [错误代码](#错误代码)
- [性能问题](#性能问题)
- [内存问题](#内存问题)
- [兼容性问题](#兼容性问题)
- [调试技巧](#调试技巧)
- [解决方案](#解决方案)

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
// 检查文件大小
if (data.length > 100 * 1024 * 1024) { // 100MB
  console.warn('文件较大，处理可能需要较长时间');
}

// 使用流式处理
async function processLargePNG(data: Uint8Array): Promise<PNG> {
  try {
    // 分块处理
    const chunkSize = 1024 * 1024; // 1MB
    const chunks = [];
    
    for (let i = 0; i < data.length; i += chunkSize) {
      const chunk = data.slice(i, i + chunkSize);
      chunks.push(chunk);
    }
    
    // 处理每个块
    const results = await Promise.all(
      chunks.map(chunk => pngSync.read(chunk))
    );
    
    return results[0]; // 返回第一个结果
  } catch (error) {
    console.error('处理大文件失败:', error);
    throw error;
  }
}
```

### Q: 如何优化内存使用？

```typescript
import { getMemoryUsage, clearCache } from 'rust-png-js';

// 监控内存使用
async function monitorMemory() {
  const memoryUsage = await getMemoryUsage();
  console.log(`当前内存使用: ${(memoryUsage / 1024 / 1024).toFixed(2)} MB`);
  
  if (memoryUsage > 100 * 1024 * 1024) { // 100MB
    console.log('内存使用过高，清理缓存...');
    await clearCache();
    console.log('缓存已清理');
  }
}

// 定期监控
setInterval(monitorMemory, 30000); // 每30秒检查一次
```

### Q: 如何处理错误？

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

### Q: 如何获取性能统计？

```typescript
import { benchmark } from 'rust-png-js';

const stats = await benchmark(data, 10);
console.log('解析时间:', stats.parseTime, 'ms');
console.log('内存使用:', stats.memoryUsage, 'bytes');
```

## 错误代码

### PNG错误

| 错误代码 | 描述 | 解决方案 |
|---------|------|----------|
| `INVALID_PNG` | 无效的PNG数据 | 检查文件格式，确保是有效的PNG文件 |
| `PARSE_ERROR` | 解析错误 | 检查PNG文件是否损坏，尝试重新下载 |
| `ENCODE_ERROR` | 编码错误 | 检查输入数据，确保格式正确 |
| `MEMORY_ERROR` | 内存错误 | 减少文件大小，或增加可用内存 |
| `WASM_ERROR` | WebAssembly错误 | 检查浏览器支持，更新到最新版本 |

### 示例错误处理

```typescript
import { PNGError } from 'rust-png-js';

try {
  const png = pngSync.read(data);
} catch (error) {
  if (error instanceof PNGError) {
    switch (error.code) {
      case 'INVALID_PNG':
        console.error('无效的PNG文件，请检查文件格式');
        break;
      case 'PARSE_ERROR':
        console.error('PNG解析失败，文件可能损坏');
        break;
      case 'ENCODE_ERROR':
        console.error('PNG编码失败，请检查输入数据');
        break;
      case 'MEMORY_ERROR':
        console.error('内存不足，请减少文件大小');
        break;
      case 'WASM_ERROR':
        console.error('WebAssembly错误，请更新浏览器');
        break;
      default:
        console.error('未知PNG错误:', error.message);
    }
  } else {
    console.error('未知错误:', error);
  }
}
```

## 性能问题

### 问题1: 解析速度慢

**症状**: PNG解析时间过长，用户体验差

**原因**:
- 文件过大
- 未启用WebAssembly优化
- 内存不足

**解决方案**:

```typescript
// 1. 启用WebAssembly优化
const wasmOptions = {
  enableSIMD: true,
  enableParallel: true,
  enableCaching: true,
  memoryPoolSize: 1024 * 1024 * 20, // 20MB
  threadCount: navigator.hardwareConcurrency || 4
};

png.setWASMOptimization(wasmOptions);

// 2. 使用并行处理
const processor = new ParallelPNGProcessor(4);
const pngs = await processor.processBatch(files);

// 3. 预加载WASM模块
await preloadWASM();
```

### 问题2: 内存使用过高

**症状**: 浏览器内存使用量持续增长，可能导致崩溃

**原因**:
- 未及时清理缓存
- 内存泄漏
- 文件过大

**解决方案**:

```typescript
// 1. 定期清理缓存
setInterval(async () => {
  const memoryUsage = await getMemoryUsage();
  if (memoryUsage > 100 * 1024 * 1024) { // 100MB
    await clearCache();
    console.log('内存使用过高，已清理缓存');
  }
}, 30000);

// 2. 使用内存池
const memoryPool = new MemoryPool();
const buffer = memoryPool.getBuffer(1024 * 1024, 'image');
// 使用缓冲区...
memoryPool.returnBuffer(buffer, 'image');

// 3. 限制文件大小
if (data.length > 50 * 1024 * 1024) { // 50MB
  throw new Error('文件过大，请选择较小的文件');
}
```

### 问题3: 编码速度慢

**症状**: PNG编码时间过长，影响用户体验

**原因**:
- 未使用优化选项
- 内存不足
- 并发处理不当

**解决方案**:

```typescript
// 1. 使用优化选项
const optimizedData = await optimizePNG(data, {
  deflateLevel: 9,
  filterType: 5, // 自适应滤镜
  colorType: 6, // RGBA
  bitDepth: 8
});

// 2. 使用工作线程
const worker = new Worker('./png-worker.js');
const result = await worker.processPNG(data);

// 3. 批量处理
const batchProcessor = new BatchPNGProcessor({
  maxConcurrency: 4,
  deflateLevel: 9,
  filterType: 5
});
```

## 内存问题

### 问题1: 内存泄漏

**症状**: 内存使用量持续增长，不释放

**原因**:
- 未清理事件监听器
- 未释放资源
- 循环引用

**解决方案**:

```typescript
class PNGProcessor {
  private cleanup: (() => void)[] = [];
  
  constructor() {
    // 注册清理函数
    this.cleanup.push(() => {
      // 清理事件监听器
      document.removeEventListener('beforeunload', this.handleBeforeUnload);
    });
  }
  
  private handleBeforeUnload = () => {
    // 清理资源
    this.cleanup.forEach(fn => fn());
  }
  
  destroy() {
    // 执行清理
    this.cleanup.forEach(fn => fn());
    this.cleanup = [];
  }
}

// 使用示例
const processor = new PNGProcessor();
// 使用处理器...
processor.destroy(); // 清理资源
```

### 问题2: 内存不足

**症状**: 处理大文件时出现内存不足错误

**原因**:
- 文件过大
- 内存池不足
- 并发处理过多

**解决方案**:

```typescript
// 1. 检查可用内存
async function checkAvailableMemory(): Promise<boolean> {
  const memoryUsage = await getMemoryUsage();
  const availableMemory = navigator.deviceMemory || 4; // GB
  const usedMemory = memoryUsage / 1024 / 1024 / 1024; // GB
  
  return usedMemory < availableMemory * 0.8; // 使用80%以下
}

// 2. 限制文件大小
const maxFileSize = 50 * 1024 * 1024; // 50MB
if (data.length > maxFileSize) {
  throw new Error('文件过大，请选择较小的文件');
}

// 3. 使用流式处理
async function processLargeFile(data: Uint8Array): Promise<PNG> {
  const chunkSize = 1024 * 1024; // 1MB
  const chunks = [];
  
  for (let i = 0; i < data.length; i += chunkSize) {
    const chunk = data.slice(i, i + chunkSize);
    chunks.push(chunk);
  }
  
  // 处理每个块
  const results = await Promise.all(
    chunks.map(chunk => pngSync.read(chunk))
  );
  
  return results[0];
}
```

## 兼容性问题

### 问题1: 浏览器不支持WebAssembly

**症状**: 在旧浏览器中无法使用

**解决方案**:

```typescript
// 检查WebAssembly支持
if (!isWASMSupported()) {
  console.warn('WebAssembly不支持，使用降级方案');
  
  // 使用JavaScript实现
  const fallbackPNG = new FallbackPNG();
  return fallbackPNG.read(data);
}
```

### 问题2: Node.js环境问题

**症状**: 在Node.js中无法使用

**解决方案**:

```typescript
// 检查环境
if (typeof window === 'undefined') {
  // Node.js环境
  const { PNG, PNGSync } = require('rust-png-js');
  const pngSync = new PNGSync();
  return pngSync.read(data);
} else {
  // 浏览器环境
  const pngSync = new PNGSync();
  return pngSync.read(data);
}
```

### 问题3: 模块加载问题

**症状**: 模块无法正确加载

**解决方案**:

```typescript
// 动态导入
async function loadPNGModule() {
  try {
    const { PNG, PNGSync } = await import('rust-png-js');
    return { PNG, PNGSync };
  } catch (error) {
    console.error('模块加载失败:', error);
    throw error;
  }
}

// 使用示例
const { PNG, PNGSync } = await loadPNGModule();
const pngSync = new PNGSync();
```

## 调试技巧

### 1. 启用调试日志

```typescript
// 设置调试级别
const DEBUG_LEVEL = 'verbose'; // 'none', 'error', 'warn', 'info', 'verbose'

// 调试日志函数
function debugLog(level: string, message: string, ...args: any[]) {
  const levels = ['none', 'error', 'warn', 'info', 'verbose'];
  const currentLevel = levels.indexOf(DEBUG_LEVEL);
  const messageLevel = levels.indexOf(level);
  
  if (messageLevel <= currentLevel) {
    console.log(`[${level.toUpperCase()}] ${message}`, ...args);
  }
}

// 使用示例
debugLog('info', '开始解析PNG', { size: data.length });
debugLog('verbose', 'PNG数据', data.slice(0, 100));
```

### 2. 性能监控

```typescript
class DebugMonitor {
  private startTime: number = 0;
  private metrics: Map<string, number[]> = new Map();
  
  start(operation: string): void {
    this.startTime = performance.now();
    debugLog('info', `开始操作: ${operation}`);
  }
  
  end(operation: string): number {
    const duration = performance.now() - this.startTime;
    debugLog('info', `完成操作: ${operation}`, { duration: `${duration.toFixed(2)}ms` });
    
    // 记录指标
    if (!this.metrics.has(operation)) {
      this.metrics.set(operation, []);
    }
    this.metrics.get(operation)!.push(duration);
    
    return duration;
  }
  
  getStats(): Map<string, { average: number; max: number; min: number; count: number }> {
    const stats = new Map();
    
    for (const [operation, durations] of this.metrics) {
      const average = durations.reduce((sum, d) => sum + d, 0) / durations.length;
      const max = Math.max(...durations);
      const min = Math.min(...durations);
      
      stats.set(operation, { average, max, min, count: durations.length });
    }
    
    return stats;
  }
}

// 使用示例
const monitor = new DebugMonitor();

monitor.start('parsePNG');
const png = pngSync.read(data);
monitor.end('parsePNG');

const stats = monitor.getStats();
console.log('性能统计:', stats);
```

### 3. 错误追踪

```typescript
class ErrorTracker {
  private errors: Error[] = [];
  
  trackError(error: Error, context?: any): void {
    this.errors.push(error);
    debugLog('error', '错误追踪', { error: error.message, context });
  }
  
  getErrors(): Error[] {
    return [...this.errors];
  }
  
  clearErrors(): void {
    this.errors = [];
  }
  
  getErrorStats(): { total: number; byType: Map<string, number> } {
    const byType = new Map();
    
    for (const error of this.errors) {
      const type = error.constructor.name;
      byType.set(type, (byType.get(type) || 0) + 1);
    }
    
    return { total: this.errors.length, byType };
  }
}

// 使用示例
const errorTracker = new ErrorTracker();

try {
  const png = pngSync.read(data);
} catch (error) {
  errorTracker.trackError(error, { dataSize: data.length });
  throw error;
}
```

## 解决方案

### 解决方案1: 完整的错误处理

```typescript
class RobustPNGProcessor {
  private errorTracker: ErrorTracker;
  private monitor: DebugMonitor;
  
  constructor() {
    this.errorTracker = new ErrorTracker();
    this.monitor = new DebugMonitor();
  }
  
  async processPNG(data: Uint8Array): Promise<PNG> {
    try {
      this.monitor.start('processPNG');
      
      // 验证数据
      const isValid = await validatePNG(data);
      if (!isValid) {
        throw new PNGError('无效的PNG数据', 'INVALID_PNG');
      }
      
      // 解析PNG
      const png = pngSync.read(data);
      
      this.monitor.end('processPNG');
      return png;
      
    } catch (error) {
      this.errorTracker.trackError(error, { dataSize: data.length });
      throw error;
    }
  }
  
  getDebugInfo(): any {
    return {
      errors: this.errorTracker.getErrorStats(),
      performance: this.monitor.getStats()
    };
  }
}
```

### 解决方案2: 内存优化

```typescript
class MemoryOptimizedPNGProcessor {
  private maxMemoryUsage = 100 * 1024 * 1024; // 100MB
  private checkInterval = 30000; // 30秒
  
  constructor() {
    this.startMemoryMonitoring();
  }
  
  private startMemoryMonitoring(): void {
    setInterval(async () => {
      await this.checkMemoryUsage();
    }, this.checkInterval);
  }
  
  private async checkMemoryUsage(): Promise<void> {
    const memoryUsage = await getMemoryUsage();
    
    if (memoryUsage > this.maxMemoryUsage) {
      console.log('内存使用过高，执行优化...');
      await this.optimizeMemory();
    }
  }
  
  private async optimizeMemory(): Promise<void> {
    // 清理缓存
    await clearCache();
    
    // 强制垃圾回收（如果可用）
    if (typeof gc !== 'undefined') {
      gc();
    }
    
    console.log('内存优化完成');
  }
  
  async processPNG(data: Uint8Array): Promise<PNG> {
    // 检查内存使用
    const memoryUsage = await getMemoryUsage();
    if (memoryUsage > this.maxMemoryUsage) {
      await this.optimizeMemory();
    }
    
    // 处理PNG
    return pngSync.read(data);
  }
}
```

### 解决方案3: 性能优化

```typescript
class PerformanceOptimizedPNGProcessor {
  private wasmOptions: WASMOptimizationOptions;
  private parallelProcessor: ParallelPNGProcessor;
  
  constructor() {
    this.wasmOptions = {
      enableSIMD: true,
      enableParallel: true,
      enableCaching: true,
      memoryPoolSize: 1024 * 1024 * 20, // 20MB
      threadCount: navigator.hardwareConcurrency || 4
    };
    
    this.parallelProcessor = new ParallelPNGProcessor(4);
  }
  
  async processPNG(data: Uint8Array): Promise<PNG> {
    // 设置WASM优化
    png.setWASMOptimization(this.wasmOptions);
    
    // 处理PNG
    return pngSync.read(data);
  }
  
  async processBatch(files: File[]): Promise<PNG[]> {
    // 使用并行处理
    return this.parallelProcessor.processBatch(files);
  }
}
```

---

## 总结

通过本文档，您可以：

1. **解决常见问题**: 了解常见问题的症状和解决方案
2. **处理错误**: 掌握错误代码和错误处理技巧
3. **优化性能**: 解决性能问题和内存问题
4. **调试应用**: 使用调试技巧和监控工具
5. **应用解决方案**: 使用完整的解决方案

遵循这些指南，您将能够快速解决Rust PNG JS使用中遇到的问题，构建稳定可靠的PNG处理应用。
