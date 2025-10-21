import { Router, Request, Response } from 'express'
import { StatusCodes } from 'http-status-codes'
import { connectRedis } from '../utils/redis'

const router = Router()

// 健康检查
router.get('/', async (req: Request, res: Response) => {
  try {
    // 检查Redis连接
    await connectRedis()
    
    res.status(StatusCodes.OK).json({
      status: 'healthy',
      timestamp: new Date().toISOString(),
      uptime: process.uptime(),
      environment: process.env.NODE_ENV || 'development',
      service: 'image-processor',
    })
  } catch (error) {
    res.status(StatusCodes.SERVICE_UNAVAILABLE).json({
      status: 'unhealthy',
      timestamp: new Date().toISOString(),
      error: error instanceof Error ? error.message : 'Unknown error',
    })
  }
})

// 详细健康检查
router.get('/detailed', async (req: Request, res: Response) => {
  const checks = {
    redis: { status: 'unknown', responseTime: 0 },
    memory: { status: 'unknown', usage: 0 },
    disk: { status: 'unknown', usage: 0 },
  }

  try {
    // 检查Redis
    const redisStart = Date.now()
    await connectRedis()
    checks.redis = {
      status: 'healthy',
      responseTime: Date.now() - redisStart,
    }
  } catch (error) {
    checks.redis = {
      status: 'unhealthy',
      responseTime: 0,
    }
  }

  // 检查内存使用
  const memoryUsage = process.memoryUsage()
  checks.memory = {
    status: memoryUsage.heapUsed / memoryUsage.heapTotal > 0.9 ? 'warning' : 'healthy',
    usage: memoryUsage.heapUsed / memoryUsage.heapTotal,
  }

  // 检查磁盘使用
  const fs = require('fs')
  try {
    const stats = fs.statSync('.')
    checks.disk = {
      status: 'healthy',
      usage: 0, // 简化实现
    }
  } catch (error) {
    checks.disk = {
      status: 'unhealthy',
      usage: 0,
    }
  }

  const allHealthy = Object.values(checks).every(check => check.status === 'healthy')
  
  res.status(allHealthy ? StatusCodes.OK : StatusCodes.SERVICE_UNAVAILABLE).json({
    status: allHealthy ? 'healthy' : 'unhealthy',
    timestamp: new Date().toISOString(),
    checks,
  })
})

export { router as healthRoutes }
