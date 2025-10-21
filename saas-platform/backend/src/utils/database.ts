import { PrismaClient } from '@prisma/client'
import { logger } from './logger'

let prisma: PrismaClient

export const connectDatabase = async (): Promise<PrismaClient> => {
  if (!prisma) {
    prisma = new PrismaClient({
      log: [
        { level: 'query', emit: 'event' },
        { level: 'error', emit: 'stdout' },
        { level: 'info', emit: 'stdout' },
        { level: 'warn', emit: 'stdout' },
      ],
    })

    // 监听查询事件
    prisma.$on('query', (e) => {
      if (process.env.NODE_ENV === 'development') {
        logger.debug('Database Query', {
          query: e.query,
          params: e.params,
          duration: `${e.duration}ms`,
        })
      }
    })

    // 监听错误事件
    prisma.$on('error', (e) => {
      logger.error('Database Error', {
        message: e.message,
        target: e.target,
      })
    })

    try {
      await prisma.$connect()
      logger.info('数据库连接成功')
    } catch (error) {
      logger.error('数据库连接失败', error)
      throw error
    }
  }

  return prisma
}

export const disconnectDatabase = async (): Promise<void> => {
  if (prisma) {
    await prisma.$disconnect()
    logger.info('数据库连接已断开')
  }
}

export const getDatabase = (): PrismaClient => {
  if (!prisma) {
    throw new Error('数据库未初始化，请先调用 connectDatabase()')
  }
  return prisma
}

// 数据库健康检查
export const checkDatabaseHealth = async (): Promise<boolean> => {
  try {
    await prisma.$queryRaw`SELECT 1`
    return true
  } catch (error) {
    logger.error('数据库健康检查失败', error)
    return false
  }
}

// 事务包装器
export const withTransaction = async <T>(
  callback: (tx: PrismaClient) => Promise<T>
): Promise<T> => {
  const db = getDatabase()
  return await db.$transaction(callback)
}

// 数据库清理（仅用于测试）
export const cleanupDatabase = async (): Promise<void> => {
  if (process.env.NODE_ENV === 'test') {
    const db = getDatabase()
    
    // 删除所有数据（按依赖关系顺序）
    await db.processingResult.deleteMany()
    await db.image.deleteMany()
    await db.user.deleteMany()
    
    logger.info('测试数据库已清理')
  }
}
