#!/bin/bash

# Rust PNG JS 发布脚本

set -e

echo "🚀 开始发布 Rust PNG JS 到 npm..."

# 检查是否已登录
if ! npm whoami &> /dev/null; then
    echo "❌ 请先登录 npm: npm login"
    exit 1
fi

# 检查工作目录是否干净
if [ -n "$(git status --porcelain)" ]; then
    echo "❌ 工作目录不干净，请先提交所有更改"
    exit 1
fi

# 运行测试
echo "🧪 运行测试..."
npm test

# 构建项目
echo "🔧 构建项目..."
npm run build

# 检查发布
echo "📦 检查发布..."
npm publish --dry-run

# 确认发布
echo "⚠️  准备发布到 npm"
read -p "确认发布? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "📦 发布到 npm..."
    npm publish
    
    echo "✅ 发布成功!"
    echo "📖 包: https://www.npmjs.com/package/rust-png-js"
else
    echo "❌ 取消发布"
    exit 1
fi
