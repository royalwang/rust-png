/**
 * Jest测试设置
 */

// 模拟WebAssembly环境
global.WebAssembly = {
  instantiate: jest.fn(),
  compile: jest.fn(),
  validate: jest.fn(),
  Module: jest.fn(),
  Instance: jest.fn(),
  Memory: jest.fn(),
  Table: jest.fn(),
  CompileError: jest.fn(),
  RuntimeError: jest.fn(),
  LinkError: jest.fn(),
} as any;

// 模拟console方法
global.console = {
  ...console,
  log: jest.fn(),
  error: jest.fn(),
  warn: jest.fn(),
  info: jest.fn(),
};

// 模拟fetch（如果需要）
global.fetch = jest.fn();

// 设置测试超时
jest.setTimeout(10000);
