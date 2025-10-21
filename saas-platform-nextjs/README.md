# SaaS 图片处理平台

基于 Next.js 14 和 Supabase 构建的现代化图片处理 SaaS 平台。

## 功能特性

- 🚀 **现代化技术栈**: Next.js 14 + Supabase + Tailwind CSS
- 📱 **响应式设计**: 支持桌面和移动设备
- 🔐 **用户认证**: 基于 Supabase Auth 的安全认证
- 📁 **文件上传**: 支持拖拽上传和批量处理
- 🖼️ **图片处理**: 多种处理选项（优化、调整尺寸、旋转、滤镜）
- 📊 **数据统计**: 实时处理统计和存储使用情况
- 🎨 **美观界面**: 基于 shadcn/ui 的现代化 UI 组件

## 技术栈

- **前端**: Next.js 14, React, TypeScript, Tailwind CSS
- **后端**: Supabase (PostgreSQL, Auth, Storage)
- **UI 组件**: shadcn/ui, Lucide React
- **状态管理**: React Context, Zustand
- **部署**: Vercel (推荐)

## 快速开始

### 1. 克隆项目

```bash
git clone <repository-url>
cd saas-platform-nextjs
```

### 2. 安装依赖

```bash
npm install
```

### 3. 配置环境变量

复制环境变量模板：

```bash
cp env.example .env.local
```

编辑 `.env.local` 文件，填入您的 Supabase 配置：

```env
NEXT_PUBLIC_SUPABASE_URL=your_supabase_url
NEXT_PUBLIC_SUPABASE_ANON_KEY=your_supabase_anon_key
```

### 4. 设置 Supabase

#### 4.1 创建 Supabase 项目

1. 访问 [Supabase](https://supabase.com)
2. 创建新项目
3. 获取项目 URL 和 API 密钥

#### 4.2 运行数据库迁移

```bash
# 安装 Supabase CLI
npm install -g supabase

# 初始化 Supabase 项目
supabase init

# 链接到您的 Supabase 项目
supabase link --project-ref your-project-ref

# 运行迁移
supabase db push
```

#### 4.3 设置存储桶

1. 在 Supabase 控制台中，进入 Storage
2. 创建名为 `images` 的存储桶
3. 设置适当的权限策略

### 5. 启动开发服务器

```bash
npm run dev
```

访问 [http://localhost:3000](http://localhost:3000) 查看应用。

## 项目结构

```
saas-platform-nextjs/
├── app/                    # Next.js App Router
│   ├── (auth)/            # 认证相关页面
│   ├── (dashboard)/       # 仪表板页面
│   ├── api/               # API 路由
│   └── globals.css        # 全局样式
├── components/            # React 组件
│   ├── ui/               # 基础 UI 组件
│   ├── auth/             # 认证组件
│   ├── layout/           # 布局组件
│   └── pages/           # 页面组件
├── lib/                  # 工具库
│   ├── supabase/         # Supabase 客户端
│   ├── auth/             # 认证逻辑
│   └── utils.ts          # 工具函数
├── supabase/             # Supabase 配置
│   ├── migrations/       # 数据库迁移
│   └── config.toml       # Supabase 配置
└── types/                # TypeScript 类型定义
```

## 主要功能

### 1. 用户认证

- 用户注册和登录
- 基于 Supabase Auth 的安全认证
- 自动会话管理

### 2. 文件上传

- 拖拽上传支持
- 批量文件上传
- 文件类型和大小验证
- 上传进度显示

### 3. 图片处理

- 图片优化压缩
- 尺寸调整
- 旋转和翻转
- 滤镜效果
- 批量处理

### 4. 数据管理

- 图片元数据存储
- 处理历史记录
- 用户存储统计
- 实时数据同步

## API 路由

### 文件上传

```typescript
POST /api/upload
Content-Type: multipart/form-data

// 请求体
files: File[]

// 响应
{
  success: boolean
  images: Image[]
}
```

### 图片处理

```typescript
POST /api/process
Content-Type: application/json

// 请求体
{
  imageId: string
  operations: ProcessingOperation[]
}

// 响应
{
  success: boolean
  taskId: string
  message: string
}
```

### 图片管理

```typescript
GET /api/images
// 查询参数
?page=1&limit=20&status=completed

// 响应
{
  images: Image[]
  pagination: PaginationInfo
}
```

## 部署

### 使用 Vercel 部署

1. 将代码推送到 GitHub
2. 在 Vercel 中导入项目
3. 配置环境变量
4. 部署

### 环境变量配置

在 Vercel 项目设置中配置以下环境变量：

```
NEXT_PUBLIC_SUPABASE_URL=your_supabase_url
NEXT_PUBLIC_SUPABASE_ANON_KEY=your_supabase_anon_key
```

## 开发指南

### 添加新的处理操作

1. 在 `components/ui/ImageProcessor.tsx` 中添加新的操作
2. 在 `app/api/process/route.ts` 中实现处理逻辑
3. 更新数据库模式（如需要）

### 自定义 UI 组件

1. 在 `components/ui/` 目录下创建新组件
2. 使用 Tailwind CSS 进行样式设计
3. 遵循 shadcn/ui 的设计规范

### 数据库迁移

1. 在 `supabase/migrations/` 目录下创建新的迁移文件
2. 运行 `supabase db push` 应用迁移
3. 更新 TypeScript 类型定义

## 贡献

1. Fork 项目
2. 创建功能分支
3. 提交更改
4. 推送到分支
5. 创建 Pull Request

## 许可证

MIT License

## 支持

如果您遇到问题或有任何建议，请：

1. 查看 [Issues](https://github.com/your-repo/issues)
2. 创建新的 Issue
3. 联系开发团队

---

**注意**: 这是一个演示项目，用于展示现代 SaaS 应用的开发模式。在生产环境中使用前，请确保：

- 实施适当的安全措施
- 配置生产级数据库
- 设置监控和日志记录
- 进行充分的测试