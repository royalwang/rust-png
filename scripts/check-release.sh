#!/bin/bash

# Rust PNG Library 发布检查脚本

set -e

echo "🔍 开始检查发布准备..."

# 检查Cargo.toml
echo "📦 检查Cargo.toml..."
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Cargo.toml 不存在"
    exit 1
fi

# 检查必需字段
echo "📋 检查必需字段..."
cargo check --quiet

# 检查文档
echo "📚 检查文档..."
if [ ! -f "README.md" ]; then
    echo "❌ README.md 不存在"
    exit 1
fi

if [ ! -f "LICENSE" ]; then
    echo "❌ LICENSE 不存在"
    exit 1
fi

# 检查代码格式
echo "🎨 检查代码格式..."
cargo fmt --check

# 检查代码质量
echo "🔍 检查代码质量..."
cargo clippy -- -D warnings

# 运行测试
echo "🧪 运行测试..."
cargo test

# 检查文档生成
echo "📖 检查文档生成..."
cargo doc --no-deps

# 检查发布
echo "📦 检查发布..."
cargo publish --dry-run

# 检查示例
echo "📝 检查示例..."
if [ -d "examples" ]; then
    cd examples
    cargo check
    cd ..
fi

# 检查工作目录
echo "📁 检查工作目录..."
if [ -n "$(git status --porcelain)" ]; then
    echo "⚠️  工作目录有未提交的更改"
    git status --porcelain
else
    echo "✅ 工作目录干净"
fi

# 检查版本
echo "📊 检查版本信息..."
VERSION=$(grep '^version' Cargo.toml | cut -d'"' -f2)
echo "当前版本: $VERSION"

# 检查标签
echo "🏷️  检查Git标签..."
if git tag -l | grep -q "v$VERSION"; then
    echo "✅ 版本标签 v$VERSION 存在"
else
    echo "⚠️  版本标签 v$VERSION 不存在"
fi

echo "✅ 发布检查完成!"
echo ""
echo "📋 发布清单:"
echo "  ✅ Cargo.toml 配置正确"
echo "  ✅ 代码格式正确"
echo "  ✅ 代码质量检查通过"
echo "  ✅ 测试通过"
echo "  ✅ 文档生成成功"
echo "  ✅ 发布检查通过"
echo "  ✅ 示例代码检查通过"
echo ""
echo "🚀 可以发布到 crates.io!"
echo "运行: cargo publish"
