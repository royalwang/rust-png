import { getDatabase } from '../utils/database'
import { CustomError } from '../middleware/errorHandler'
import { StatusCodes } from 'http-status-codes'
import { logger } from '../utils/logger'
import { ProcessingOptions, ProcessingResult } from '../../shared/types/processing'

export interface CreateProcessingResultData {
  imageId: string
  originalImageId: string
  name: string
  size: number
  type: string
  url: string
  width: number
  height: number
  format: string
  options: ProcessingOptions
  processingTime: number
  fileSizeReduction: number
  status?: 'PENDING' | 'PROCESSING' | 'COMPLETED' | 'FAILED' | 'CANCELLED'
  errorMessage?: string
}

export interface ProcessingFilters {
  status?: string
  dateFrom?: Date
  dateTo?: Date
}

export class ProcessingService {
  private db = getDatabase()

  async createProcessingResult(userId: string, data: CreateProcessingResultData) {
    const result = await this.db.processingResult.create({
      data: {
        ...data,
        userId,
      }
    })

    // 更新用户处理统计
    await this.updateUserProcessingStats(userId, 1)

    logger.info(`处理结果创建成功: ${data.name} (用户: ${userId})`)
    return result
  }

  async getProcessingResult(id: string, userId: string) {
    const result = await this.db.processingResult.findFirst({
      where: {
        id,
        userId
      },
      include: {
        image: true,
        originalImage: true
      }
    })

    if (!result) {
      throw new CustomError('处理结果不存在', StatusCodes.NOT_FOUND)
    }

    return result
  }

  async getProcessingHistory(userId: string, filters: ProcessingFilters = {}, pagination: { page: number; limit: number } = { page: 1, limit: 20 }) {
    const { status, dateFrom, dateTo } = filters
    const { page, limit } = pagination
    const skip = (page - 1) * limit

    const where: any = {
      userId
    }

    if (status) {
      where.status = status
    }

    if (dateFrom || dateTo) {
      where.createdAt = {}
      if (dateFrom) where.createdAt.gte = dateFrom
      if (dateTo) where.createdAt.lte = dateTo
    }

    const [results, total] = await Promise.all([
      this.db.processingResult.findMany({
        where,
        skip,
        take: limit,
        orderBy: { createdAt: 'desc' },
        include: {
          image: true,
          originalImage: true
        }
      }),
      this.db.processingResult.count({ where })
    ])

    return {
      results,
      pagination: {
        page,
        limit,
        total,
        pages: Math.ceil(total / limit)
      }
    }
  }

  async deleteProcessingResult(id: string, userId: string) {
    const result = await this.getProcessingResult(id, userId)

    await this.db.processingResult.delete({
      where: { id }
    })

    logger.info(`处理结果删除成功: ${id} (用户: ${userId})`)
  }

  async getProcessingStats(userId: string) {
    const stats = await this.db.processingResult.aggregate({
      where: { userId },
      _count: { id: true },
      _avg: { 
        processingTime: true,
        fileSizeReduction: true
      }
    })

    const statusStats = await this.db.processingResult.groupBy({
      by: ['status'],
      where: { userId },
      _count: { id: true }
    })

    const formatStats = await this.db.processingResult.groupBy({
      by: ['format'],
      where: { userId },
      _count: { id: true }
    })

    // 获取最近30天的处理历史
    const thirtyDaysAgo = new Date()
    thirtyDaysAgo.setDate(thirtyDaysAgo.getDate() - 30)

    const dailyStats = await this.db.processingResult.groupBy({
      by: ['createdAt'],
      where: {
        userId,
        createdAt: {
          gte: thirtyDaysAgo
        }
      },
      _count: { id: true },
      orderBy: { createdAt: 'asc' }
    })

    return {
      totalProcessed: stats._count.id,
      averageProcessingTime: stats._avg.processingTime || 0,
      averageFileSizeReduction: stats._avg.fileSizeReduction || 0,
      statusStats: statusStats.map(s => ({
        status: s.status,
        count: s._count.id
      })),
      formatStats: formatStats.map(f => ({
        format: f.format,
        count: f._count.id
      })),
      dailyStats: dailyStats.map(d => ({
        date: d.createdAt,
        count: d._count.id
      }))
    }
  }

  async updateProcessingStatus(id: string, userId: string, status: string, errorMessage?: string) {
    const result = await this.getProcessingResult(id, userId)

    const updatedResult = await this.db.processingResult.update({
      where: { id },
      data: {
        status: status as any,
        errorMessage,
        updatedAt: new Date()
      }
    })

    logger.info(`处理状态更新成功: ${id} -> ${status} (用户: ${userId})`)
    return updatedResult
  }

  async getQueueStatus() {
    const pendingCount = await this.db.processingResult.count({
      where: { status: 'PENDING' }
    })

    const processingCount = await this.db.processingResult.count({
      where: { status: 'PROCESSING' }
    })

    const failedCount = await this.db.processingResult.count({
      where: { status: 'FAILED' }
    })

    return {
      pending: pendingCount,
      processing: processingCount,
      failed: failedCount,
      total: pendingCount + processingCount + failedCount
    }
  }

  async cancelTask(id: string, userId: string) {
    const result = await this.getProcessingResult(id, userId)

    if (result.status === 'COMPLETED') {
      throw new CustomError('已完成的任务无法取消', StatusCodes.BAD_REQUEST)
    }

    const updatedResult = await this.db.processingResult.update({
      where: { id },
      data: {
        status: 'CANCELLED',
        updatedAt: new Date()
      }
    })

    logger.info(`任务取消成功: ${id} (用户: ${userId})`)
    return updatedResult
  }

  async reprocess(id: string, userId: string, newOptions?: ProcessingOptions) {
    const result = await this.getProcessingResult(id, userId)

    if (result.status === 'PROCESSING') {
      throw new CustomError('正在处理的任务无法重新处理', StatusCodes.BAD_REQUEST)
    }

    const newResult = await this.db.processingResult.create({
      data: {
        userId,
        imageId: result.imageId,
        originalImageId: result.originalImageId,
        name: `Reprocessed_${result.name}`,
        size: result.size,
        type: result.type,
        url: result.url,
        width: result.width,
        height: result.height,
        format: result.format,
        options: newOptions || result.options,
        processingTime: 0,
        fileSizeReduction: 0,
        status: 'PENDING'
      }
    })

    logger.info(`重新处理任务创建成功: ${id} -> ${newResult.id} (用户: ${userId})`)
    return newResult
  }

  private async updateUserProcessingStats(userId: string, count: number) {
    // 更新用户处理统计
    const today = new Date()
    today.setHours(0, 0, 0, 0)

    await this.db.usageStats.upsert({
      where: {
        userId_date: {
          userId,
          date: today
        }
      },
      update: {
        imagesProcessed: {
          increment: count
        }
      },
      create: {
        userId,
        date: today,
        imagesProcessed: count
      }
    })
  }
}
