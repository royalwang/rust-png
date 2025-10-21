# Rust PNG Library 发布指南

## 发布到 crates.io

### 准备工作

#### 1. 注册 crates.io 账户
```bash
# 访问 https://crates.io 注册账户
# 获取 API token
```

#### 2. 登录 crates.io
```bash
cargo login <your-api-token>
```

#### 3. 检查发布准备
```bash
# 运行发布检查脚本
./scripts/check-release.sh
```

### 发布步骤

#### 1. 更新版本号
```bash
# 在 Cargo.toml 中更新版本号
# 例如: version = "0.1.0"
```

#### 2. 提交更改
```bash
git add .
git commit -m "Release v0.1.0"
git tag v0.1.0
git push origin main
git push origin v0.1.0
```

#### 3. 发布到 crates.io
```bash
# 方法1: 使用发布脚本
./scripts/publish.sh

# 方法2: 手动发布
cargo publish
```

### 发布后验证

#### 1. 检查发布状态
```bash
# 检查包是否已发布
cargo search rust-png
```

#### 2. 测试安装
```bash
# 在新项目中测试安装
cargo new test-rust-png
cd test-rust-png
cargo add rust-png
```

#### 3. 检查文档
- 访问 https://docs.rs/rust-png
- 检查文档是否正常生成

### 发布检查清单

#### ✅ 代码质量
- [ ] 代码格式正确 (`cargo fmt`)
- [ ] 代码质量检查通过 (`cargo clippy`)
- [ ] 所有测试通过 (`cargo test`)
- [ ] 示例代码可运行

#### ✅ 文档完整
- [ ] README.md 完整
- [ ] API文档完整
- [ ] 示例代码完整
- [ ] 许可证文件存在

#### ✅ 元数据正确
- [ ] Cargo.toml 配置正确
- [ ] 版本号正确
- [ ] 作者信息正确
- [ ] 许可证信息正确
- [ ] 仓库链接正确

#### ✅ 发布准备
- [ ] 工作目录干净
- [ ] 所有更改已提交
- [ ] 版本标签已创建
- [ ] 发布检查通过

### 常见问题

#### Q: 发布失败怎么办？
A: 检查错误信息，常见问题：
- 版本号已存在
- 元数据不完整
- 依赖问题

#### Q: 如何更新版本？
A: 更新 Cargo.toml 中的版本号，然后重新发布

#### Q: 如何撤销发布？
A: crates.io 不允许撤销发布，只能发布新版本

### 发布后维护

#### 1. 监控下载量
- 访问 https://crates.io/crates/rust-png
- 查看下载统计

#### 2. 处理问题
- 监控 GitHub Issues
- 及时回复用户问题

#### 3. 更新文档
- 根据用户反馈更新文档
- 添加更多示例

### 版本管理

#### 语义化版本
- **主版本号**: 不兼容的API修改
- **次版本号**: 向下兼容的功能性新增
- **修订号**: 向下兼容的问题修正

#### 版本发布流程
1. 开发新功能
2. 更新版本号
3. 更新CHANGELOG.md
4. 提交更改
5. 创建标签
6. 发布到crates.io

### 发布脚本使用

#### 检查发布准备
```bash
./scripts/check-release.sh
```

#### 自动发布
```bash
./scripts/publish.sh
```

### 发布后推广

#### 1. 更新README
- 添加crates.io徽章
- 更新安装说明

#### 2. 社区推广
- 在Rust社区分享
- 写博客介绍
- 参与相关讨论

#### 3. 收集反馈
- 监控用户反馈
- 持续改进功能

---

**发布成功!** 🎉

现在你的Rust PNG Library已经发布到crates.io，全世界的Rust开发者都可以使用它了！
