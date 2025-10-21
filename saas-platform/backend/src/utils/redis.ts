import Redis from 'ioredis'
import { logger } from './logger'

let redis: Redis

export const connectRedis = async (): Promise<Redis> => {
  if (!redis) {
    redis = new Redis(process.env.REDIS_URL || 'redis://localhost:6379', {
      retryDelayOnFailover: 100,
      enableReadyCheck: false,
      maxRetriesPerRequest: null,
      lazyConnect: true,
    })

    // 监听连接事件
    redis.on('connect', () => {
      logger.info('Redis连接成功')
    })

    redis.on('error', (error) => {
      logger.error('Redis连接错误', error)
    })

    redis.on('close', () => {
      logger.warn('Redis连接已关闭')
    })

    redis.on('reconnecting', () => {
      logger.info('Redis重新连接中...')
    })

    try {
      await redis.ping()
      logger.info('Redis连接测试成功')
    } catch (error) {
      logger.error('Redis连接测试失败', error)
      throw error
    }
  }

  return redis
}

export const getRedis = (): Redis => {
  if (!redis) {
    throw new Error('Redis未初始化，请先调用 connectRedis()')
  }
  return redis
}

export const disconnectRedis = async (): Promise<void> => {
  if (redis) {
    await redis.quit()
    logger.info('Redis连接已断开')
  }
}

// Redis健康检查
export const checkRedisHealth = async (): Promise<boolean> => {
  try {
    const result = await redis.ping()
    return result === 'PONG'
  } catch (error) {
    logger.error('Redis健康检查失败', error)
    return false
  }
}

// 缓存工具函数
export const cache = {
  // 设置缓存
  async set(key: string, value: any, ttl?: number): Promise<void> {
    const redis = getRedis()
    const serializedValue = JSON.stringify(value)
    
    if (ttl) {
      await redis.setex(key, ttl, serializedValue)
    } else {
      await redis.set(key, serializedValue)
    }
  },

  // 获取缓存
  async get<T>(key: string): Promise<T | null> {
    const redis = getRedis()
    const value = await redis.get(key)
    
    if (!value) {
      return null
    }
    
    try {
      return JSON.parse(value) as T
    } catch (error) {
      logger.error('缓存反序列化失败', { key, error })
      return null
    }
  },

  // 删除缓存
  async del(key: string): Promise<void> {
    const redis = getRedis()
    await redis.del(key)
  },

  // 检查缓存是否存在
  async exists(key: string): Promise<boolean> {
    const redis = getRedis()
    const result = await redis.exists(key)
    return result === 1
  },

  // 设置过期时间
  async expire(key: string, ttl: number): Promise<void> {
    const redis = getRedis()
    await redis.expire(key, ttl)
  },

  // 获取剩余过期时间
  async ttl(key: string): Promise<number> {
    const redis = getRedis()
    return await redis.ttl(key)
  },
}

// 会话管理
export const session = {
  // 设置会话
  async set(userId: string, sessionData: any, ttl: number = 7 * 24 * 60 * 60): Promise<void> {
    const key = `session:${userId}`
    await cache.set(key, sessionData, ttl)
  },

  // 获取会话
  async get(userId: string): Promise<any> {
    const key = `session:${userId}`
    return await cache.get(key)
  },

  // 删除会话
  async del(userId: string): Promise<void> {
    const key = `session:${userId}`
    await cache.del(key)
  },

  // 更新会话
  async update(userId: string, sessionData: any, ttl?: number): Promise<void> {
    const key = `session:${userId}`
    await cache.set(key, sessionData, ttl)
  },
}

// 限流工具
export const rateLimit = {
  // 检查限流
  async check(key: string, limit: number, window: number): Promise<boolean> {
    const redis = getRedis()
    const current = await redis.incr(key)
    
    if (current === 1) {
      await redis.expire(key, window)
    }
    
    return current <= limit
  },

  // 获取剩余请求次数
  async getRemaining(key: string, limit: number): Promise<number> {
    const redis = getRedis()
    const current = await redis.get(key)
    const count = current ? parseInt(current) : 0
    return Math.max(0, limit - count)
  },
}
