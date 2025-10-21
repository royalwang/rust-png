import AWS from 'aws-sdk'
import { v4 as uuidv4 } from 'uuid'
import { logger } from '../utils/logger'
import { CustomError } from '../middleware/errorHandler'
import { StatusCodes } from 'http-status-codes'

export interface UploadOptions {
  bucket?: string
  key?: string
  contentType?: string
  metadata?: Record<string, string>
  acl?: string
}

export interface UploadResult {
  url: string
  key: string
  bucket: string
  etag: string
}

export class S3Service {
  private s3: AWS.S3
  private bucket: string

  constructor() {
    this.s3 = new AWS.S3({
      accessKeyId: process.env.AWS_ACCESS_KEY_ID,
      secretAccessKey: process.env.AWS_SECRET_ACCESS_KEY,
      region: process.env.AWS_REGION || 'us-east-1'
    })
    this.bucket = process.env.AWS_S3_BUCKET || 'rust-png-saas'
  }

  async uploadFile(
    buffer: Buffer,
    options: UploadOptions = {}
  ): Promise<UploadResult> {
    try {
      const {
        bucket = this.bucket,
        key = this.generateKey(),
        contentType = 'application/octet-stream',
        metadata = {},
        acl = 'private'
      } = options

      const params: AWS.S3.PutObjectRequest = {
        Bucket: bucket,
        Key: key,
        Body: buffer,
        ContentType: contentType,
        Metadata: metadata,
        ACL: acl as any
      }

      const result = await this.s3.upload(params).promise()

      logger.info(`文件上传成功: ${key}`)

      return {
        url: result.Location,
        key: result.Key,
        bucket: result.Bucket,
        etag: result.ETag
      }
    } catch (error) {
      logger.error('S3文件上传失败', error)
      throw new CustomError('文件上传失败', StatusCodes.INTERNAL_SERVER_ERROR)
    }
  }

  async uploadImage(
    buffer: Buffer,
    originalName: string,
    userId: string,
    isPublic: boolean = false
  ): Promise<UploadResult> {
    const extension = this.getFileExtension(originalName)
    const key = `images/${userId}/${uuidv4()}${extension}`
    const contentType = this.getContentType(extension)

    return await this.uploadFile(buffer, {
      key,
      contentType,
      acl: isPublic ? 'public-read' : 'private',
      metadata: {
        userId,
        originalName,
        uploadedAt: new Date().toISOString()
      }
    })
  }

  async uploadThumbnail(
    buffer: Buffer,
    originalKey: string,
    userId: string
  ): Promise<UploadResult> {
    const thumbnailKey = originalKey.replace('/images/', '/thumbnails/')
    const contentType = 'image/jpeg'

    return await this.uploadFile(buffer, {
      key: thumbnailKey,
      contentType,
      acl: 'private',
      metadata: {
        userId,
        type: 'thumbnail',
        uploadedAt: new Date().toISOString()
      }
    })
  }

  async deleteFile(url: string): Promise<void> {
    try {
      const key = this.extractKeyFromUrl(url)
      if (!key) {
        throw new Error('无法从URL中提取文件键')
      }

      await this.s3.deleteObject({
        Bucket: this.bucket,
        Key: key
      }).promise()

      logger.info(`文件删除成功: ${key}`)
    } catch (error) {
      logger.error('S3文件删除失败', error)
      throw new CustomError('文件删除失败', StatusCodes.INTERNAL_SERVER_ERROR)
    }
  }

  async getSignedUrl(key: string, expiresIn: number = 3600): Promise<string> {
    try {
      const params = {
        Bucket: this.bucket,
        Key: key,
        Expires: expiresIn
      }

      const url = await this.s3.getSignedUrlPromise('getObject', params)
      return url
    } catch (error) {
      logger.error('生成签名URL失败', error)
      throw new CustomError('生成签名URL失败', StatusCodes.INTERNAL_SERVER_ERROR)
    }
  }

  async getFileMetadata(key: string) {
    try {
      const result = await this.s3.headObject({
        Bucket: this.bucket,
        Key: key
      }).promise()

      return {
        size: result.ContentLength,
        contentType: result.ContentType,
        lastModified: result.LastModified,
        metadata: result.Metadata
      }
    } catch (error) {
      logger.error('获取文件元数据失败', error)
      throw new CustomError('获取文件元数据失败', StatusCodes.INTERNAL_SERVER_ERROR)
    }
  }

  async copyFile(sourceKey: string, destinationKey: string): Promise<void> {
    try {
      await this.s3.copyObject({
        Bucket: this.bucket,
        CopySource: `${this.bucket}/${sourceKey}`,
        Key: destinationKey
      }).promise()

      logger.info(`文件复制成功: ${sourceKey} -> ${destinationKey}`)
    } catch (error) {
      logger.error('文件复制失败', error)
      throw new CustomError('文件复制失败', StatusCodes.INTERNAL_SERVER_ERROR)
    }
  }

  async listFiles(prefix: string, maxKeys: number = 1000) {
    try {
      const result = await this.s3.listObjectsV2({
        Bucket: this.bucket,
        Prefix: prefix,
        MaxKeys: maxKeys
      }).promise()

      return result.Contents || []
    } catch (error) {
      logger.error('列出文件失败', error)
      throw new CustomError('列出文件失败', StatusCodes.INTERNAL_SERVER_ERROR)
    }
  }

  async deleteMultipleFiles(keys: string[]): Promise<void> {
    try {
      const objects = keys.map(key => ({ Key: key }))
      
      await this.s3.deleteObjects({
        Bucket: this.bucket,
        Delete: {
          Objects: objects
        }
      }).promise()

      logger.info(`批量删除文件成功: ${keys.length} 个文件`)
    } catch (error) {
      logger.error('批量删除文件失败', error)
      throw new CustomError('批量删除文件失败', StatusCodes.INTERNAL_SERVER_ERROR)
    }
  }

  private generateKey(): string {
    return `${uuidv4()}-${Date.now()}`
  }

  private getFileExtension(filename: string): string {
    const lastDot = filename.lastIndexOf('.')
    return lastDot !== -1 ? filename.substring(lastDot) : ''
  }

  private getContentType(extension: string): string {
    const contentTypes: Record<string, string> = {
      '.jpg': 'image/jpeg',
      '.jpeg': 'image/jpeg',
      '.png': 'image/png',
      '.gif': 'image/gif',
      '.webp': 'image/webp',
      '.avif': 'image/avif',
      '.svg': 'image/svg+xml'
    }

    return contentTypes[extension.toLowerCase()] || 'application/octet-stream'
  }

  private extractKeyFromUrl(url: string): string | null {
    try {
      const urlObj = new URL(url)
      const pathParts = urlObj.pathname.split('/')
      return pathParts.slice(1).join('/') // 移除开头的斜杠
    } catch (error) {
      return null
    }
  }
}
