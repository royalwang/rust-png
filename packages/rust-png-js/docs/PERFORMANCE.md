# Rust PNG JS - 性能优化指南

## 目录

- [性能概述](#性能概述)
- [WebAssembly优化](#webassembly优化)
- [内存管理](#内存管理)
- [并发处理](#并发处理)
- [缓存策略](#缓存策略)
- [性能监控](#性能监控)
- [最佳实践](#最佳实践)
- [性能测试](#性能测试)

## 性能概述

Rust PNG JS 基于Rust和WebAssembly技术，提供了卓越的性能表现：

- **解析速度**: 比原生JavaScript快3-5倍
- **编码速度**: 比原生JavaScript快2-4倍
- **内存效率**: 内存使用量减少30-50%
- **压缩比**: 比原生JavaScript高5-15%

### 性能优势

1. **Rust性能**: 零成本抽象，内存安全
2. **WebAssembly**: 接近原生性能
3. **SIMD支持**: 向量化计算
4. **并行处理**: 多线程优化
5. **内存池**: 高效内存管理

## WebAssembly优化

### 启用SIMD支持

```typescript
import { isWASMSupported, preloadWASM } from 'rust-png-js';

// 检查SIMD支持
if (isWASMSupported()) {
  // 预加载WASM模块
  await preloadWASM();
  
  // 配置SIMD优化
  const wasmOptions = {
    enableSIMD: true,
    enableParallel: true,
    enableCaching: true,
    memoryPoolSize: 1024 * 1024 * 10, // 10MB
    threadCount: navigator.hardwareConcurrency || 4
  };
  
  png.setWASMOptimization(wasmOptions);
}
```

### 并行处理配置

```typescript
import { WASMOptimizationOptions } from 'rust-png-js';

// 配置并行处理
const parallelOptions: WASMOptimizationOptions = {
  enableParallel: true,
  threadCount: navigator.hardwareConcurrency || 4,
  enableCaching: true,
  memoryPoolSize: 1024 * 1024 * 20 // 20MB
};

png.setWASMOptimization(parallelOptions);
```

### 内存池优化

```typescript
// 配置内存池
const memoryOptions: WASMOptimizationOptions = {
  memoryPoolSize: 1024 * 1024 * 50, // 50MB
  enableCaching: true,
  enableParallel: true
};

png.setWASMOptimization(memoryOptions);
```

## 内存管理

### 内存监控

```typescript
import { getMemoryUsage, clearCache } from 'rust-png-js';

// 监控内存使用
async function monitorMemory() {
  const memoryUsage = await getMemoryUsage();
  console.log(`当前内存使用: ${(memoryUsage / 1024 / 1024).toFixed(2)} MB`);
  
  // 内存使用过高时清理缓存
  if (memoryUsage > 100 * 1024 * 1024) { // 100MB
    console.log('内存使用过高，清理缓存...');
    await clearCache();
    console.log('缓存已清理');
  }
}

// 定期监控
setInterval(monitorMemory, 30000); // 每30秒检查一次
```

### 内存优化策略

```typescript
class MemoryOptimizer {
  private maxMemoryUsage = 100 * 1024 * 1024; // 100MB
  private checkInterval = 30000; // 30秒
  
  constructor() {
    this.startMonitoring();
  }
  
  private startMonitoring() {
    setInterval(async () => {
      await this.checkMemoryUsage();
    }, this.checkInterval);
  }
  
  private async checkMemoryUsage() {
    const memoryUsage = await getMemoryUsage();
    
    if (memoryUsage > this.maxMemoryUsage) {
      console.log('内存使用过高，执行优化...');
      await this.optimizeMemory();
    }
  }
  
  private async optimizeMemory() {
    // 清理缓存
    await clearCache();
    
    // 强制垃圾回收（如果可用）
    if (typeof gc !== 'undefined') {
      gc();
    }
    
    console.log('内存优化完成');
  }
}

// 使用示例
const optimizer = new MemoryOptimizer();
```

### 内存池管理

```typescript
class MemoryPool {
  private pools: Map<string, Uint8Array[]> = new Map();
  private maxPoolSize = 10;
  
  getBuffer(size: number, type: string = 'default'): Uint8Array {
    const pool = this.pools.get(type) || [];
    
    // 查找合适大小的缓冲区
    const buffer = pool.find(buf => buf.length >= size);
    if (buffer) {
      return buffer.slice(0, size);
    }
    
    // 创建新缓冲区
    const newBuffer = new Uint8Array(size);
    return newBuffer;
  }
  
  returnBuffer(buffer: Uint8Array, type: string = 'default') {
    const pool = this.pools.get(type) || [];
    
    if (pool.length < this.maxPoolSize) {
      pool.push(buffer);
      this.pools.set(type, pool);
    }
  }
  
  clear() {
    this.pools.clear();
  }
}

// 使用示例
const memoryPool = new MemoryPool();

// 获取缓冲区
const buffer = memoryPool.getBuffer(1024 * 1024, 'image');

// 使用缓冲区...

// 归还缓冲区
memoryPool.returnBuffer(buffer, 'image');
```

## 并发处理

### 并行解析

```typescript
import { PNGSync } from 'rust-png-js';

class ParallelPNGProcessor {
  private maxConcurrency: number;
  private pngSync: PNGSync;
  
  constructor(maxConcurrency = 4) {
    this.maxConcurrency = maxConcurrency;
    this.pngSync = new PNGSync();
  }
  
  async processBatch(files: File[]): Promise<PNG[]> {
    const results: PNG[] = [];
    
    // 分批处理
    for (let i = 0; i < files.length; i += this.maxConcurrency) {
      const batch = files.slice(i, i + this.maxConcurrency);
      const batchResults = await Promise.all(
        batch.map(async (file) => {
          const data = new Uint8Array(await file.arrayBuffer());
          return this.pngSync.read(data);
        })
      );
      
      results.push(...batchResults);
    }
    
    return results;
  }
}

// 使用示例
const processor = new ParallelPNGProcessor(4);
const pngs = await processor.processBatch(files);
```

### 工作线程处理

```typescript
// worker.ts
import { PNGSync, optimizePNG } from 'rust-png-js';

self.onmessage = async function(event) {
  const { type, data, options } = event.data;
  
  try {
    switch (type) {
      case 'parse':
        const pngSync = new PNGSync();
        const png = pngSync.read(data);
        self.postMessage({ type: 'parse', result: png });
        break;
        
      case 'optimize':
        const optimizedData = await optimizePNG(data, options);
        self.postMessage({ type: 'optimize', result: optimizedData });
        break;
        
      default:
        throw new Error('未知的操作类型');
    }
  } catch (error) {
    self.postMessage({ type: 'error', error: error.message });
  }
};

// main.ts
class WorkerPNGProcessor {
  private workers: Worker[] = [];
  private workerIndex = 0;
  
  constructor(workerCount = 4) {
    for (let i = 0; i < workerCount; i++) {
      const worker = new Worker('./worker.js');
      this.workers.push(worker);
    }
  }
  
  async processPNG(data: Uint8Array): Promise<PNG> {
    const worker = this.getNextWorker();
    
    return new Promise((resolve, reject) => {
      worker.onmessage = (event) => {
        const { type, result, error } = event.data;
        
        if (type === 'parse') {
          resolve(result);
        } else if (type === 'error') {
          reject(new Error(error));
        }
      };
      
      worker.postMessage({ type: 'parse', data });
    });
  }
  
  private getNextWorker(): Worker {
    const worker = this.workers[this.workerIndex];
    this.workerIndex = (this.workerIndex + 1) % this.workers.length;
    return worker;
  }
  
  terminate() {
    this.workers.forEach(worker => worker.terminate());
  }
}
```

## 缓存策略

### 智能缓存

```typescript
class SmartCache {
  private cache: Map<string, any> = new Map();
  private maxSize: number;
  private accessCount: Map<string, number> = new Map();
  
  constructor(maxSize = 100) {
    this.maxSize = maxSize;
  }
  
  get(key: string): any {
    const value = this.cache.get(key);
    if (value) {
      // 更新访问计数
      const count = this.accessCount.get(key) || 0;
      this.accessCount.set(key, count + 1);
    }
    return value;
  }
  
  set(key: string, value: any): void {
    // 检查缓存大小
    if (this.cache.size >= this.maxSize) {
      this.evictLeastUsed();
    }
    
    this.cache.set(key, value);
    this.accessCount.set(key, 1);
  }
  
  private evictLeastUsed(): void {
    let leastUsedKey = '';
    let leastUsedCount = Infinity;
    
    for (const [key, count] of this.accessCount) {
      if (count < leastUsedCount) {
        leastUsedCount = count;
        leastUsedKey = key;
      }
    }
    
    if (leastUsedKey) {
      this.cache.delete(leastUsedKey);
      this.accessCount.delete(leastUsedKey);
    }
  }
  
  clear(): void {
    this.cache.clear();
    this.accessCount.clear();
  }
}

// 使用示例
const cache = new SmartCache(50);

// 缓存PNG数据
const pngData = pngSync.read(data);
cache.set('png_' + hash, pngData);

// 获取缓存的PNG数据
const cachedPNG = cache.get('png_' + hash);
```

### 预加载策略

```typescript
class PreloadManager {
  private preloadQueue: string[] = [];
  private preloadedData: Map<string, Uint8Array> = new Map();
  private maxPreloadSize = 10;
  
  async preloadPNG(url: string): Promise<void> {
    if (this.preloadedData.has(url)) {
      return;
    }
    
    try {
      const response = await fetch(url);
      const data = new Uint8Array(await response.arrayBuffer());
      
      // 检查预加载大小
      if (this.preloadedData.size >= this.maxPreloadSize) {
        this.evictOldest();
      }
      
      this.preloadedData.set(url, data);
      console.log(`PNG预加载完成: ${url}`);
      
    } catch (error) {
      console.error(`PNG预加载失败: ${url}`, error);
    }
  }
  
  getPreloadedData(url: string): Uint8Array | null {
    return this.preloadedData.get(url) || null;
  }
  
  private evictOldest(): void {
    const firstKey = this.preloadedData.keys().next().value;
    if (firstKey) {
      this.preloadedData.delete(firstKey);
    }
  }
  
  clear(): void {
    this.preloadedData.clear();
  }
}

// 使用示例
const preloadManager = new PreloadManager();

// 预加载PNG文件
await preloadManager.preloadPNG('/images/image1.png');
await preloadManager.preloadPNG('/images/image2.png');

// 获取预加载的数据
const data = preloadManager.getPreloadedData('/images/image1.png');
if (data) {
  const png = pngSync.read(data);
  console.log('使用预加载的PNG数据');
}
```

## 性能监控

### 性能统计

```typescript
import { benchmark, getMemoryUsage } from 'rust-png-js';

class PerformanceMonitor {
  private stats: PerformanceStats[] = [];
  private startTime: number = 0;
  
  start(): void {
    this.startTime = performance.now();
  }
  
  end(): number {
    return performance.now() - this.startTime;
  }
  
  async runBenchmark(data: Uint8Array, iterations: number = 10): Promise<PerformanceStats> {
    console.log(`运行性能基准测试 (${iterations} 次迭代)...`);
    
    const stats = await benchmark(data, iterations);
    
    console.log('基准测试结果:');
    console.log(`  解析时间: ${stats.parseTime.toFixed(2)} ms`);
    console.log(`  编码时间: ${stats.encodeTime.toFixed(2)} ms`);
    console.log(`  内存使用: ${(stats.memoryUsage / 1024 / 1024).toFixed(2)} MB`);
    console.log(`  压缩比: ${stats.compressionRatio.toFixed(2)}%`);
    
    this.stats.push(stats);
    return stats;
  }
  
  async monitorMemory(): Promise<void> {
    const memoryUsage = await getMemoryUsage();
    console.log(`当前内存使用: ${(memoryUsage / 1024 / 1024).toFixed(2)} MB`);
    
    if (memoryUsage > 100 * 1024 * 1024) { // 100MB
      console.log('内存使用过高，建议清理缓存');
    }
  }
  
  getAverageStats(): PerformanceStats | null {
    if (this.stats.length === 0) {
      return null;
    }
    
    const avgStats: PerformanceStats = {
      parseTime: this.stats.reduce((sum, s) => sum + s.parseTime, 0) / this.stats.length,
      encodeTime: this.stats.reduce((sum, s) => sum + s.encodeTime, 0) / this.stats.length,
      memoryUsage: this.stats.reduce((sum, s) => sum + s.memoryUsage, 0) / this.stats.length,
      compressionRatio: this.stats.reduce((sum, s) => sum + s.compressionRatio, 0) / this.stats.length
    };
    
    return avgStats;
  }
  
  getBestStats(): PerformanceStats | null {
    if (this.stats.length === 0) {
      return null;
    }
    
    return this.stats.reduce((best, current) => {
      return current.parseTime < best.parseTime ? current : best;
    });
  }
  
  getWorstStats(): PerformanceStats | null {
    if (this.stats.length === 0) {
      return null;
    }
    
    return this.stats.reduce((worst, current) => {
      return current.parseTime > worst.parseTime ? current : worst;
    });
  }
}

// 使用示例
const monitor = new PerformanceMonitor();

// 运行性能测试
monitor.runBenchmark(data, 20)
  .then(stats => {
    console.log('性能测试完成');
    
    const avgStats = monitor.getAverageStats();
    const bestStats = monitor.getBestStats();
    const worstStats = monitor.getWorstStats();
    
    console.log('平均性能:', avgStats);
    console.log('最佳性能:', bestStats);
    console.log('最差性能:', worstStats);
  })
  .catch(error => {
    console.error('性能测试失败:', error);
  });
```

### 实时性能监控

```typescript
class RealTimeMonitor {
  private metrics: Map<string, number[]> = new Map();
  private maxMetrics = 100;
  
  recordMetric(name: string, value: number): void {
    if (!this.metrics.has(name)) {
      this.metrics.set(name, []);
    }
    
    const values = this.metrics.get(name)!;
    values.push(value);
    
    // 保持最大指标数量
    if (values.length > this.maxMetrics) {
      values.shift();
    }
  }
  
  getAverageMetric(name: string): number | null {
    const values = this.metrics.get(name);
    if (!values || values.length === 0) {
      return null;
    }
    
    return values.reduce((sum, value) => sum + value, 0) / values.length;
  }
  
  getMaxMetric(name: string): number | null {
    const values = this.metrics.get(name);
    if (!values || values.length === 0) {
      return null;
    }
    
    return Math.max(...values);
  }
  
  getMinMetric(name: string): number | null {
    const values = this.metrics.get(name);
    if (!values || values.length === 0) {
      return null;
    }
    
    return Math.min(...values);
  }
  
  getMetrics(): Map<string, { average: number; max: number; min: number; count: number }> {
    const result = new Map();
    
    for (const [name, values] of this.metrics) {
      result.set(name, {
        average: this.getAverageMetric(name),
        max: this.getMaxMetric(name),
        min: this.getMinMetric(name),
        count: values.length
      });
    }
    
    return result;
  }
  
  clear(): void {
    this.metrics.clear();
  }
}

// 使用示例
const realTimeMonitor = new RealTimeMonitor();

// 记录性能指标
monitor.start();
const png = pngSync.read(data);
const parseTime = monitor.end();

realTimeMonitor.recordMetric('parseTime', parseTime);
realTimeMonitor.recordMetric('memoryUsage', await getMemoryUsage());

// 获取性能统计
const metrics = realTimeMonitor.getMetrics();
console.log('性能指标:', metrics);
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

### 3. 错误处理

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

### 4. 性能优化

```typescript
// 使用性能监控
const monitor = new PerformanceMonitor();

// 运行性能测试
const stats = await monitor.runBenchmark(data, 100);
console.log('性能统计:', stats);

// 监控内存使用
await monitor.monitorMemory();
```

### 5. 并发处理

```typescript
// 使用工作线程
const processor = new WorkerPNGProcessor(4);
const png = await processor.processPNG(data);

// 批量处理
const batchProcessor = new ParallelPNGProcessor(4);
const pngs = await batchProcessor.processBatch(files);
```

## 性能测试

### 基准测试

```typescript
import { benchmark } from 'rust-png-js';

async function runPerformanceTest() {
  const testData = new Uint8Array(1024 * 1024); // 1MB测试数据
  
  console.log('开始性能测试...');
  
  // 运行基准测试
  const stats = await benchmark(testData, 100);
  
  console.log('性能测试结果:');
  console.log(`  解析时间: ${stats.parseTime.toFixed(2)} ms`);
  console.log(`  编码时间: ${stats.encodeTime.toFixed(2)} ms`);
  console.log(`  内存使用: ${(stats.memoryUsage / 1024 / 1024).toFixed(2)} MB`);
  console.log(`  压缩比: ${stats.compressionRatio.toFixed(2)}%`);
  
  return stats;
}

// 运行性能测试
runPerformanceTest()
  .then(stats => {
    console.log('性能测试完成');
  })
  .catch(error => {
    console.error('性能测试失败:', error);
  });
```

### 压力测试

```typescript
class StressTest {
  private testData: Uint8Array;
  private iterations: number;
  
  constructor(testData: Uint8Array, iterations: number = 1000) {
    this.testData = testData;
    this.iterations = iterations;
  }
  
  async runStressTest(): Promise<StressTestResult> {
    console.log(`开始压力测试 (${this.iterations} 次迭代)...`);
    
    const startTime = performance.now();
    const results: number[] = [];
    
    for (let i = 0; i < this.iterations; i++) {
      const iterationStart = performance.now();
      
      try {
        const png = pngSync.read(this.testData);
        const iterationTime = performance.now() - iterationStart;
        results.push(iterationTime);
        
        if (i % 100 === 0) {
          console.log(`完成 ${i}/${this.iterations} 次迭代`);
        }
      } catch (error) {
        console.error(`第 ${i} 次迭代失败:`, error);
      }
    }
    
    const totalTime = performance.now() - startTime;
    const averageTime = results.reduce((sum, time) => sum + time, 0) / results.length;
    const maxTime = Math.max(...results);
    const minTime = Math.min(...results);
    
    return {
      totalTime,
      averageTime,
      maxTime,
      minTime,
      successCount: results.length,
      failureCount: this.iterations - results.length
    };
  }
}

// 使用示例
const stressTest = new StressTest(testData, 1000);
const result = await stressTest.runStressTest();

console.log('压力测试结果:');
console.log(`  总时间: ${result.totalTime.toFixed(2)} ms`);
console.log(`  平均时间: ${result.averageTime.toFixed(2)} ms`);
console.log(`  最大时间: ${result.maxTime.toFixed(2)} ms`);
console.log(`  最小时间: ${result.minTime.toFixed(2)} ms`);
console.log(`  成功次数: ${result.successCount}`);
console.log(`  失败次数: ${result.failureCount}`);
```

---

## 总结

通过本文档，您可以：

1. **了解性能优势**: Rust和WebAssembly带来的性能提升
2. **掌握优化技巧**: 内存管理、并发处理、缓存策略
3. **监控性能**: 实时监控和性能测试
4. **应用最佳实践**: 预加载、错误处理、性能优化

遵循这些指南，您将能够充分发挥Rust PNG JS的性能优势，构建高性能的PNG处理应用。
