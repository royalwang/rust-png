#!/bin/bash

# Rust PNG JS 构建脚本

set -e

echo "🚀 开始构建 Rust PNG JS..."

# 检查依赖
echo "📦 检查依赖..."
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack 未安装，请先安装: cargo install wasm-pack"
    exit 1
fi

if ! command -v tsc &> /dev/null; then
    echo "❌ TypeScript 未安装，请先安装: npm install -g typescript"
    exit 1
fi

# 清理旧构建
echo "🧹 清理旧构建..."
rm -rf dist/
rm -rf dist/wasm/

# 构建WASM模块
echo "🔧 构建WASM模块..."
cd ../../
wasm-pack build --target web --out-dir packages/rust-png-js/dist/wasm --release
cd packages/rust-png-js/

# 构建TypeScript
echo "📝 构建TypeScript..."
tsc

# 复制WASM文件
echo "📋 复制WASM文件..."
cp dist/wasm/*.wasm dist/
cp dist/wasm/*.js dist/

# 创建包文件
echo "📦 创建包文件..."
mkdir -p dist/wasm
cp dist/wasm/* dist/wasm/

# 验证构建
echo "✅ 验证构建..."
if [ ! -f "dist/index.js" ]; then
    echo "❌ 构建失败: dist/index.js 不存在"
    exit 1
fi

if [ ! -f "dist/index.d.ts" ]; then
    echo "❌ 构建失败: dist/index.d.ts 不存在"
    exit 1
fi

echo "🎉 构建完成!"
echo ""
echo "📁 构建输出:"
echo "  - dist/index.js"
echo "  - dist/index.d.ts"
echo "  - dist/wasm/"
echo ""
echo "📦 可以发布到npm:"
echo "  npm publish"
