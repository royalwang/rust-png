#!/bin/bash

# Rust PNG Library 发布脚本

set -e

echo "🚀 开始发布 Rust PNG Library 到 crates.io"

# 检查是否已登录
if ! cargo login --check > /dev/null 2>&1; then
    echo "❌ 请先登录 crates.io: cargo login"
    exit 1
fi

# 检查工作目录是否干净
if [ -n "$(git status --porcelain)" ]; then
    echo "❌ 工作目录不干净，请先提交所有更改"
    exit 1
fi

# 运行测试
echo "🧪 运行测试..."
cargo test

# 运行示例
echo "📝 运行示例..."
cargo run --bin basic_usage
cargo run --bin advanced_features
cargo run --bin wasm_optimization
cargo run --bin advanced_filters
cargo run --bin complete_application

# 检查文档
echo "📚 检查文档..."
cargo doc --no-deps

# 检查发布
echo "🔍 检查发布..."
cargo publish --dry-run

# 确认发布
echo "⚠️  准备发布到 crates.io"
read -p "确认发布? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "📦 发布到 crates.io..."
    cargo publish
    
    echo "✅ 发布成功!"
    echo "📖 文档: https://docs.rs/rust-png"
    echo "📦 包: https://crates.io/crates/rust-png"
else
    echo "❌ 取消发布"
    exit 1
fi
