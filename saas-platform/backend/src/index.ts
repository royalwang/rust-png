import express from 'express'
import cors from 'cors'
import helmet from 'helmet'
import morgan from 'morgan'
import compression from 'compression'
import rateLimit from 'express-rate-limit'
import 'express-async-errors'
import { StatusCodes } from 'http-status-codes'

import { errorHandler } from './middleware/errorHandler'
import { notFoundHandler } from './middleware/notFoundHandler'
import { authRoutes } from './routes/auth'
import { userRoutes } from './routes/user'
import { imageRoutes } from './routes/image'
import { processingRoutes } from './routes/processing'
import { subscriptionRoutes } from './routes/subscription'
import { webhookRoutes } from './routes/webhook'
import { healthRoutes } from './routes/health'
import { logger } from './utils/logger'
import { connectDatabase } from './utils/database'
import { connectRedis } from './utils/redis'
import { startQueueWorkers } from './workers'

const app = express()
const PORT = process.env.PORT || 8000

// 安全中间件
app.use(helmet({
  contentSecurityPolicy: {
    directives: {
      defaultSrc: ["'self'"],
      styleSrc: ["'self'", "'unsafe-inline'"],
      scriptSrc: ["'self'"],
      imgSrc: ["'self'", "data:", "https:"],
    },
  },
}))

// CORS配置
app.use(cors({
  origin: process.env.FRONTEND_URL || 'http://localhost:3000',
  credentials: true,
  methods: ['GET', 'POST', 'PUT', 'DELETE', 'PATCH', 'OPTIONS'],
  allowedHeaders: ['Content-Type', 'Authorization', 'X-Requested-With'],
}))

// 请求日志
app.use(morgan('combined', {
  stream: {
    write: (message) => logger.info(message.trim()),
  },
}))

// 压缩响应
app.use(compression())

// 速率限制
const limiter = rateLimit({
  windowMs: 15 * 60 * 1000, // 15分钟
  max: 100, // 限制每个IP 15分钟内最多100个请求
  message: {
    error: '请求过于频繁，请稍后再试',
    code: 'RATE_LIMIT_EXCEEDED',
  },
  standardHeaders: true,
  legacyHeaders: false,
})

app.use('/api', limiter)

// 解析JSON
app.use(express.json({ limit: '10mb' }))
app.use(express.urlencoded({ extended: true, limit: '10mb' }))

// 健康检查
app.use('/health', healthRoutes)

// API路由
app.use('/api/auth', authRoutes)
app.use('/api/users', userRoutes)
app.use('/api/images', imageRoutes)
app.use('/api/processing', processingRoutes)
app.use('/api/subscriptions', subscriptionRoutes)
app.use('/api/webhooks', webhookRoutes)

// 404处理
app.use(notFoundHandler)

// 错误处理
app.use(errorHandler)

// 启动服务器
async function startServer() {
  try {
    // 连接数据库
    await connectDatabase()
    logger.info('数据库连接成功')

    // 连接Redis
    await connectRedis()
    logger.info('Redis连接成功')

    // 启动队列工作者
    await startQueueWorkers()
    logger.info('队列工作者启动成功')

    // 启动服务器
    app.listen(PORT, () => {
      logger.info(`服务器运行在端口 ${PORT}`)
      logger.info(`环境: ${process.env.NODE_ENV || 'development'}`)
    })
  } catch (error) {
    logger.error('服务器启动失败:', error)
    process.exit(1)
  }
}

// 优雅关闭
process.on('SIGTERM', () => {
  logger.info('收到SIGTERM信号，开始优雅关闭...')
  process.exit(0)
})

process.on('SIGINT', () => {
  logger.info('收到SIGINT信号，开始优雅关闭...')
  process.exit(0)
})

// 未捕获的异常处理
process.on('uncaughtException', (error) => {
  logger.error('未捕获的异常:', error)
  process.exit(1)
})

process.on('unhandledRejection', (reason, promise) => {
  logger.error('未处理的Promise拒绝:', reason, promise)
  process.exit(1)
})

startServer()
