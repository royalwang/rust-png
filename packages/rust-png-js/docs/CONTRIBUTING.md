# Rust PNG JS - 贡献指南

## 目录

- [贡献方式](#贡献方式)
- [开发环境](#开发环境)
- [代码规范](#代码规范)
- [测试指南](#测试指南)
- [文档贡献](#文档贡献)
- [发布流程](#发布流程)

## 贡献方式

### 报告问题

如果您发现了bug或有功能请求，请通过以下方式报告：

1. **GitHub Issues**: 在项目仓库中创建Issue
2. **Bug报告**: 使用bug报告模板
3. **功能请求**: 使用功能请求模板

### 提交代码

1. **Fork仓库**: 在GitHub上fork项目
2. **创建分支**: 创建功能分支
3. **提交代码**: 提交您的更改
4. **创建PR**: 创建Pull Request

### 贡献类型

- **Bug修复**: 修复已知问题
- **功能开发**: 添加新功能
- **性能优化**: 提升性能
- **文档改进**: 完善文档
- **测试覆盖**: 增加测试

## 开发环境

### 环境要求

- **Node.js**: 16.0.0 或更高版本
- **npm**: 8.0.0 或更高版本
- **Rust**: 1.70.0 或更高版本
- **wasm-pack**: 0.12.0 或更高版本

### 安装依赖

```bash
# 安装Node.js依赖
npm install

# 安装Rust依赖
cargo build

# 安装wasm-pack
cargo install wasm-pack
```

### 开发工具

```bash
# 安装开发工具
npm install -g typescript
npm install -g eslint
npm install -g prettier
npm install -g jest
```

### 项目结构

```
packages/rust-png-js/
├── src/                    # 源代码
│   ├── index.ts           # 主入口
│   ├── png.ts             # PNG类
│   ├── png-sync.ts        # 同步PNG类
│   ├── semantic-png.ts    # 语义PNG类
│   ├── semantic-png-sync.ts # 语义同步PNG类
│   ├── utils.ts           # 工具函数
│   ├── wasm-loader.ts     # WASM加载器
│   └── types.ts           # 类型定义
├── src/__tests__/         # 测试文件
├── docs/                  # 文档
├── examples/              # 示例
├── scripts/               # 脚本
├── package.json           # 包配置
├── tsconfig.json          # TypeScript配置
├── jest.config.js         # Jest配置
└── .eslintrc.js           # ESLint配置
```

## 代码规范

### TypeScript规范

```typescript
// 使用严格的TypeScript配置
interface PNGMetadata {
  dimensions: PNGDimensions;
  colorInfo: PNGColorInfo;
  compression: PNGCompression;
  interlace: PNGInterlace;
  gamma?: number;
  palette?: Uint8Array;
  transparentColor?: Uint16Array;
}

// 使用明确的类型注解
function processPNG(data: Uint8Array): Promise<PNG> {
  // 实现
}

// 使用泛型
class Cache<T> {
  private data: Map<string, T> = new Map();
  
  get(key: string): T | undefined {
    return this.data.get(key);
  }
  
  set(key: string, value: T): void {
    this.data.set(key, value);
  }
}
```

### 命名规范

```typescript
// 类名使用PascalCase
class PNGProcessor {
  // 方法名使用camelCase
  async processPNG(data: Uint8Array): Promise<PNG> {
    // 实现
  }
  
  // 私有方法使用下划线前缀
  private _validateData(data: Uint8Array): boolean {
    // 实现
  }
}

// 常量使用UPPER_SNAKE_CASE
const MAX_FILE_SIZE = 100 * 1024 * 1024; // 100MB
const DEFAULT_OPTIONS: EncodeOptions = {
  deflateLevel: 9,
  filterType: 5
};

// 接口使用PascalCase
interface EncodeOptions {
  deflateLevel?: number;
  filterType?: number;
  colorType?: number;
  bitDepth?: number;
}
```

### 注释规范

```typescript
/**
 * PNG处理器类
 * 提供PNG文件的解析、编码和优化功能
 */
class PNGProcessor {
  /**
   * 处理PNG数据
   * @param data PNG数据
   * @param options 处理选项
   * @returns 处理后的PNG对象
   * @throws {PNGError} 当PNG数据无效时
   */
  async processPNG(data: Uint8Array, options?: ProcessOptions): Promise<PNG> {
    // 实现
  }
  
  /**
   * 验证PNG数据
   * @param data 要验证的数据
   * @returns 是否为有效的PNG数据
   */
  private _validateData(data: Uint8Array): boolean {
    // 实现
  }
}
```

### 错误处理

```typescript
// 使用自定义错误类
class PNGError extends Error {
  constructor(message: string, public code?: string) {
    super(message);
    this.name = 'PNGError';
  }
}

// 错误处理示例
async function processPNG(data: Uint8Array): Promise<PNG> {
  try {
    // 验证数据
    if (!data || data.length === 0) {
      throw new PNGError('PNG数据为空', 'EMPTY_DATA');
    }
    
    // 处理PNG
    const png = pngSync.read(data);
    return png;
    
  } catch (error) {
    if (error instanceof PNGError) {
      throw error;
    } else {
      throw new PNGError(`处理PNG失败: ${error.message}`, 'PROCESS_ERROR');
    }
  }
}
```

## 测试指南

### 单元测试

```typescript
// src/__tests__/png.test.ts
import { PNG, PNGSync, validatePNG } from '../index';

describe('PNG', () => {
  let testData: Uint8Array;
  
  beforeEach(() => {
    // 设置测试数据
    testData = new Uint8Array(1024);
  });
  
  describe('基本功能', () => {
    test('应该能够解析PNG数据', () => {
      const pngSync = new PNGSync();
      const png = pngSync.read(testData);
      
      expect(png).toBeDefined();
      expect(png.width).toBeGreaterThan(0);
      expect(png.height).toBeGreaterThan(0);
    });
    
    test('应该能够获取像素值', () => {
      const pngSync = new PNGSync();
      const png = pngSync.read(testData);
      
      const pixel = png.getPixel(0, 0);
      expect(pixel).toHaveLength(4); // RGBA
      expect(pixel[0]).toBeGreaterThanOrEqual(0);
      expect(pixel[0]).toBeLessThanOrEqual(255);
    });
  });
  
  describe('错误处理', () => {
    test('应该处理无效的PNG数据', () => {
      const invalidData = new Uint8Array([1, 2, 3, 4]);
      
      expect(() => {
        const pngSync = new PNGSync();
        pngSync.read(invalidData);
      }).toThrow('无效的PNG数据');
    });
  });
});
```

### 集成测试

```typescript
// src/__tests__/integration.test.ts
import { PNG, PNGSync, optimizePNG } from '../index';

describe('集成测试', () => {
  test('应该能够完整处理PNG文件', async () => {
    // 读取测试文件
    const data = await fs.readFile('test/fixtures/sample.png');
    
    // 解析PNG
    const pngSync = new PNGSync();
    const png = pngSync.read(data);
    
    // 验证基本属性
    expect(png.width).toBeGreaterThan(0);
    expect(png.height).toBeGreaterThan(0);
    
    // 优化PNG
    const optimizedData = await optimizePNG(data, {
      deflateLevel: 9,
      filterType: 5
    });
    
    // 验证优化结果
    expect(optimizedData.length).toBeLessThanOrEqual(data.length);
  });
});
```

### 性能测试

```typescript
// src/__tests__/performance.test.ts
import { benchmark, getMemoryUsage } from '../index';

describe('性能测试', () => {
  test('应该能够运行性能基准测试', async () => {
    const testData = new Uint8Array(1024 * 1024); // 1MB
    
    const stats = await benchmark(testData, 10);
    
    expect(stats.parseTime).toBeGreaterThan(0);
    expect(stats.encodeTime).toBeGreaterThan(0);
    expect(stats.memoryUsage).toBeGreaterThan(0);
    expect(stats.compressionRatio).toBeGreaterThan(0);
  });
  
  test('应该能够监控内存使用', async () => {
    const memoryUsage = await getMemoryUsage();
    
    expect(memoryUsage).toBeGreaterThan(0);
    expect(typeof memoryUsage).toBe('number');
  });
});
```

### 运行测试

```bash
# 运行所有测试
npm test

# 运行特定测试
npm test -- --testNamePattern="PNG"

# 运行性能测试
npm test -- --testNamePattern="性能"

# 生成测试覆盖率报告
npm run test:coverage
```

## 文档贡献

### 文档结构

```
docs/
├── README.md              # 主文档
├── API.md                 # API参考
├── EXAMPLES.md            # 示例代码
├── PERFORMANCE.md         # 性能优化
├── TROUBLESHOOTING.md     # 故障排除
├── MIGRATION.md           # 迁移指南
└── CONTRIBUTING.md        # 贡献指南
```

### 文档规范

```markdown
# 标题使用一级标题

## 子标题使用二级标题

### 三级标题

#### 四级标题

**粗体文本**

*斜体文本*

`代码文本`

```typescript
// 代码块使用语言标识
const png = new PNG();
```

> 引用文本

- 列表项1
- 列表项2
  - 子列表项1
  - 子列表项2

1. 有序列表项1
2. 有序列表项2

[链接文本](链接地址)

![图片描述](图片地址)
```

### 示例代码

```typescript
// 基本使用示例
import { PNG, PNGSync } from 'rust-png-js';

const pngSync = new PNGSync();
const png = pngSync.read(data);

console.log('图像尺寸:', png.width, 'x', png.height);
console.log('像素值:', png.getPixel(0, 0));
```

### 文档更新

1. **更新现有文档**: 修改现有文档内容
2. **添加新文档**: 创建新的文档文件
3. **更新示例**: 更新示例代码
4. **检查链接**: 确保所有链接有效

## 发布流程

### 版本管理

```json
{
  "version": "1.0.0",
  "scripts": {
    "version:patch": "npm version patch",
    "version:minor": "npm version minor",
    "version:major": "npm version major"
  }
}
```

### 发布步骤

1. **更新版本号**
   ```bash
   npm version patch  # 补丁版本
   npm version minor  # 次要版本
   npm version major  # 主要版本
   ```

2. **构建项目**
   ```bash
   npm run build
   ```

3. **运行测试**
   ```bash
   npm test
   ```

4. **发布到npm**
   ```bash
   npm publish
   ```

### 发布检查清单

- [ ] 代码已通过所有测试
- [ ] 文档已更新
- [ ] 版本号已更新
- [ ] 变更日志已更新
- [ ] 构建成功
- [ ] 发布到npm成功

### 发布脚本

```bash
#!/bin/bash
# scripts/publish.sh

echo "开始发布流程..."

# 检查工作目录是否干净
if [ -n "$(git status --porcelain)" ]; then
  echo "错误: 工作目录不干净，请先提交所有更改"
  exit 1
fi

# 运行测试
echo "运行测试..."
npm test
if [ $? -ne 0 ]; then
  echo "错误: 测试失败"
  exit 1
fi

# 构建项目
echo "构建项目..."
npm run build
if [ $? -ne 0 ]; then
  echo "错误: 构建失败"
  exit 1
fi

# 发布到npm
echo "发布到npm..."
npm publish
if [ $? -ne 0 ]; then
  echo "错误: 发布失败"
  exit 1
fi

echo "发布完成!"
```

## 贡献检查清单

### 代码贡献

- [ ] 代码遵循项目规范
- [ ] 添加了适当的注释
- [ ] 处理了所有错误情况
- [ ] 添加了单元测试
- [ ] 更新了相关文档

### 功能贡献

- [ ] 功能设计合理
- [ ] API设计一致
- [ ] 性能影响评估
- [ ] 向后兼容性
- [ ] 用户文档更新

### 测试贡献

- [ ] 测试覆盖率高
- [ ] 测试用例完整
- [ ] 性能测试通过
- [ ] 集成测试通过
- [ ] 错误处理测试

### 文档贡献

- [ ] 文档结构清晰
- [ ] 示例代码正确
- [ ] 链接有效
- [ ] 语言规范
- [ ] 内容准确

---

## 总结

通过本文档，您可以：

1. **了解贡献方式**: 报告问题、提交代码、贡献类型
2. **设置开发环境**: 环境要求、依赖安装、项目结构
3. **遵循代码规范**: TypeScript规范、命名规范、注释规范
4. **编写测试**: 单元测试、集成测试、性能测试
5. **贡献文档**: 文档结构、规范、更新
6. **参与发布**: 版本管理、发布步骤、检查清单

遵循这些指南，您将能够为Rust PNG JS项目做出有价值的贡献。
