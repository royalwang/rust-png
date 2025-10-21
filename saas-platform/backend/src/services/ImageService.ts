import { getDatabase } from '../utils/database'
import { CustomError } from '../middleware/errorHandler'
import { StatusCodes } from 'http-status-codes'
import { logger } from '../utils/logger'
import { S3Service } from './S3Service'

export interface CreateImageData {
  name: string
  originalName: string
  size: number
  type: string
  url: string
  thumbnailUrl?: string
  width: number
  height: number
  format: string
  metadata?: any
  tags?: string[]
  isPublic?: boolean
}

export interface UpdateImageData {
  name?: string
  tags?: string[]
  isPublic?: boolean
}

export interface ImageFilters {
  search?: string
  format?: string
  tags?: string[]
  isPublic?: boolean
  dateFrom?: Date
  dateTo?: Date
}

export class ImageService {
  private db = getDatabase()
  private s3Service = new S3Service()

  async createImage(userId: string, data: CreateImageData) {
    const image = await this.db.image.create({
      data: {
        ...data,
        userId,
      }
    })

    // 更新用户存储使用量
    await this.updateUserStorageUsage(userId, data.size)

    logger.info(`图片创建成功: ${data.name} (用户: ${userId})`)
    return image
  }

  async getImageById(id: string, userId: string) {
    const image = await this.db.image.findFirst({
      where: {
        id,
        userId
      }
    })

    if (!image) {
      throw new CustomError('图片不存在', StatusCodes.NOT_FOUND)
    }

    return image
  }

  async getImages(userId: string, filters: ImageFilters = {}, pagination: { page: number; limit: number } = { page: 1, limit: 20 }) {
    const { search, format, tags, isPublic, dateFrom, dateTo } = filters
    const { page, limit } = pagination
    const skip = (page - 1) * limit

    const where: any = {
      userId
    }

    if (search) {
      where.OR = [
        { name: { contains: search, mode: 'insensitive' } },
        { originalName: { contains: search, mode: 'insensitive' } }
      ]
    }

    if (format) {
      where.format = format
    }

    if (tags && tags.length > 0) {
      where.tags = {
        hasSome: tags
      }
    }

    if (typeof isPublic === 'boolean') {
      where.isPublic = isPublic
    }

    if (dateFrom || dateTo) {
      where.createdAt = {}
      if (dateFrom) where.createdAt.gte = dateFrom
      if (dateTo) where.createdAt.lte = dateTo
    }

    const [images, total] = await Promise.all([
      this.db.image.findMany({
        where,
        skip,
        take: limit,
        orderBy: { createdAt: 'desc' }
      }),
      this.db.image.count({ where })
    ])

    return {
      images,
      pagination: {
        page,
        limit,
        total,
        pages: Math.ceil(total / limit)
      }
    }
  }

  async updateImage(id: string, userId: string, data: UpdateImageData) {
    const image = await this.getImageById(id, userId)

    const updatedImage = await this.db.image.update({
      where: { id },
      data: {
        ...data,
        updatedAt: new Date()
      }
    })

    logger.info(`图片更新成功: ${id} (用户: ${userId})`)
    return updatedImage
  }

  async deleteImage(id: string, userId: string) {
    const image = await this.getImageById(id, userId)

    // 删除S3文件
    try {
      await this.s3Service.deleteFile(image.url)
      if (image.thumbnailUrl) {
        await this.s3Service.deleteFile(image.thumbnailUrl)
      }
    } catch (error) {
      logger.warn(`S3文件删除失败: ${image.url}`, error)
    }

    // 删除数据库记录
    await this.db.image.delete({
      where: { id }
    })

    // 更新用户存储使用量
    await this.updateUserStorageUsage(userId, -image.size)

    logger.info(`图片删除成功: ${id} (用户: ${userId})`)
  }

  async deleteBatch(ids: string[], userId: string) {
    const images = await this.db.image.findMany({
      where: {
        id: { in: ids },
        userId
      }
    })

    // 删除S3文件
    for (const image of images) {
      try {
        await this.s3Service.deleteFile(image.url)
        if (image.thumbnailUrl) {
          await this.s3Service.deleteFile(image.thumbnailUrl)
        }
      } catch (error) {
        logger.warn(`S3文件删除失败: ${image.url}`, error)
      }
    }

    // 删除数据库记录
    await this.db.image.deleteMany({
      where: {
        id: { in: ids },
        userId
      }
    })

    // 更新用户存储使用量
    const totalSize = images.reduce((sum, image) => sum + image.size, 0)
    await this.updateUserStorageUsage(userId, -totalSize)

    logger.info(`批量删除图片成功: ${ids.length} 张 (用户: ${userId})`)
  }

  async getImageStats(userId: string) {
    const stats = await this.db.image.aggregate({
      where: { userId },
      _count: { id: true },
      _sum: { size: true }
    })

    const formatStats = await this.db.image.groupBy({
      by: ['format'],
      where: { userId },
      _count: { id: true }
    })

    const sizeStats = await this.db.image.groupBy({
      by: ['format'],
      where: { userId },
      _sum: { size: true }
    })

    return {
      totalImages: stats._count.id,
      totalSize: stats._sum.size || 0,
      formatStats: formatStats.map(f => ({
        format: f.format,
        count: f._count.id
      })),
      sizeStats: sizeStats.map(s => ({
        format: s.format,
        size: s._sum.size || 0
      }))
    }
  }

  async searchImages(userId: string, query: string, limit: number = 20) {
    const images = await this.db.image.findMany({
      where: {
        userId,
        OR: [
          { name: { contains: query, mode: 'insensitive' } },
          { originalName: { contains: query, mode: 'insensitive' } },
          { tags: { hasSome: [query] } }
        ]
      },
      take: limit,
      orderBy: { createdAt: 'desc' }
    })

    return images
  }

  async updateImageTags(id: string, userId: string, tags: string[]) {
    const image = await this.getImageById(id, userId)

    const updatedImage = await this.db.image.update({
      where: { id },
      data: {
        tags,
        updatedAt: new Date()
      }
    })

    logger.info(`图片标签更新成功: ${id} (用户: ${userId})`)
    return updatedImage
  }

  private async updateUserStorageUsage(userId: string, sizeChange: number) {
    // 更新用户存储使用量统计
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
        storageUsed: {
          increment: sizeChange
        }
      },
      create: {
        userId,
        date: today,
        storageUsed: Math.max(0, sizeChange)
      }
    })
  }
}
