import { describe, it, expect, beforeEach, afterEach } from '@jest/globals'
import request from 'supertest'
import { app } from '../index'
import { getDatabase } from '../utils/database'
import bcrypt from 'bcryptjs'

describe('Image API', () => {
  let db: any
  let authToken: string
  let userId: string

  beforeEach(async () => {
    db = getDatabase()
    
    // 清理测试数据
    await db.processingResult.deleteMany()
    await db.image.deleteMany()
    await db.user.deleteMany()
    await db.subscription.deleteMany()

    // 创建测试用户
    const user = await db.user.create({
      data: {
        name: 'Test User',
        email: 'test@example.com',
        password: await bcrypt.hash('TestPassword123!', 12),
        isVerified: true,
        isActive: true
      }
    })

    userId = user.id
    authToken = 'valid-jwt-token' // 在实际测试中需要生成真实的JWT token

    // 创建订阅
    await db.subscription.create({
      data: {
        userId: user.id,
        plan: 'FREE',
        status: 'ACTIVE'
      }
    })
  })

  afterEach(async () => {
    // 清理测试数据
    await db.processingResult.deleteMany()
    await db.image.deleteMany()
    await db.user.deleteMany()
    await db.subscription.deleteMany()
  })

  describe('POST /api/images/upload', () => {
    it('应该成功上传图片', async () => {
      const imageBuffer = Buffer.from('fake-image-data')
      
      const response = await request(app)
        .post('/api/images/upload')
        .set('Authorization', `Bearer ${authToken}`)
        .attach('image', imageBuffer, 'test-image.jpg')
        .expect(201)

      expect(response.body.success).toBe(true)
      expect(response.body.data.image).toBeDefined()
      expect(response.body.data.image.name).toBe('test-image.jpg')
      expect(response.body.data.image.userId).toBe(userId)

      // 验证图片已保存到数据库
      const image = await db.image.findFirst({
        where: { userId }
      })
      expect(image).toBeTruthy()
      expect(image.name).toBe('test-image.jpg')
    })

    it('应该拒绝未认证用户上传', async () => {
      const imageBuffer = Buffer.from('fake-image-data')
      
      const response = await request(app)
        .post('/api/images/upload')
        .attach('image', imageBuffer, 'test-image.jpg')
        .expect(401)

      expect(response.body.success).toBe(false)
    })

    it('应该验证文件类型', async () => {
      const textBuffer = Buffer.from('not-an-image')
      
      const response = await request(app)
        .post('/api/images/upload')
        .set('Authorization', `Bearer ${authToken}`)
        .attach('image', textBuffer, 'test.txt')
        .expect(400)

      expect(response.body.success).toBe(false)
    })
  })

  describe('GET /api/images', () => {
    beforeEach(async () => {
      // 创建测试图片
      await db.image.createMany({
        data: [
          {
            userId,
            name: 'image1.jpg',
            originalName: 'image1.jpg',
            size: 1024,
            type: 'image/jpeg',
            url: 'https://example.com/image1.jpg',
            width: 800,
            height: 600,
            format: 'jpg'
          },
          {
            userId,
            name: 'image2.png',
            originalName: 'image2.png',
            size: 2048,
            type: 'image/png',
            url: 'https://example.com/image2.png',
            width: 1200,
            height: 800,
            format: 'png'
          }
        ]
      })
    })

    it('应该返回用户的图片列表', async () => {
      const response = await request(app)
        .get('/api/images')
        .set('Authorization', `Bearer ${authToken}`)
        .expect(200)

      expect(response.body.success).toBe(true)
      expect(response.body.data.images).toHaveLength(2)
      expect(response.body.data.pagination.total).toBe(2)
    })

    it('应该支持分页', async () => {
      const response = await request(app)
        .get('/api/images?page=1&limit=1')
        .set('Authorization', `Bearer ${authToken}`)
        .expect(200)

      expect(response.body.success).toBe(true)
      expect(response.body.data.images).toHaveLength(1)
      expect(response.body.data.pagination.page).toBe(1)
      expect(response.body.data.pagination.limit).toBe(1)
    })

    it('应该支持搜索', async () => {
      const response = await request(app)
        .get('/api/images?search=image1')
        .set('Authorization', `Bearer ${authToken}`)
        .expect(200)

      expect(response.body.success).toBe(true)
      expect(response.body.data.images).toHaveLength(1)
      expect(response.body.data.images[0].name).toBe('image1.jpg')
    })

    it('应该支持格式筛选', async () => {
      const response = await request(app)
        .get('/api/images?format=png')
        .set('Authorization', `Bearer ${authToken}`)
        .expect(200)

      expect(response.body.success).toBe(true)
      expect(response.body.data.images).toHaveLength(1)
      expect(response.body.data.images[0].format).toBe('png')
    })
  })

  describe('GET /api/images/:id', () => {
    let imageId: string

    beforeEach(async () => {
      const image = await db.image.create({
        data: {
          userId,
          name: 'test-image.jpg',
          originalName: 'test-image.jpg',
          size: 1024,
          type: 'image/jpeg',
          url: 'https://example.com/test-image.jpg',
          width: 800,
          height: 600,
          format: 'jpg'
        }
      })
      imageId = image.id
    })

    it('应该返回图片详情', async () => {
      const response = await request(app)
        .get(`/api/images/${imageId}`)
        .set('Authorization', `Bearer ${authToken}`)
        .expect(200)

      expect(response.body.success).toBe(true)
      expect(response.body.data.image.id).toBe(imageId)
      expect(response.body.data.image.name).toBe('test-image.jpg')
    })

    it('应该拒绝访问其他用户的图片', async () => {
      // 创建另一个用户
      const otherUser = await db.user.create({
        data: {
          name: 'Other User',
          email: 'other@example.com',
          password: await bcrypt.hash('TestPassword123!', 12),
          isVerified: true,
          isActive: true
        }
      })

      const otherImage = await db.image.create({
        data: {
          userId: otherUser.id,
          name: 'other-image.jpg',
          originalName: 'other-image.jpg',
          size: 1024,
          type: 'image/jpeg',
          url: 'https://example.com/other-image.jpg',
          width: 800,
          height: 600,
          format: 'jpg'
        }
      })

      const response = await request(app)
        .get(`/api/images/${otherImage.id}`)
        .set('Authorization', `Bearer ${authToken}`)
        .expect(404)

      expect(response.body.success).toBe(false)
    })

    it('应该返回404对于不存在的图片', async () => {
      const response = await request(app)
        .get('/api/images/non-existent-id')
        .set('Authorization', `Bearer ${authToken}`)
        .expect(404)

      expect(response.body.success).toBe(false)
    })
  })

  describe('DELETE /api/images/:id', () => {
    let imageId: string

    beforeEach(async () => {
      const image = await db.image.create({
        data: {
          userId,
          name: 'test-image.jpg',
          originalName: 'test-image.jpg',
          size: 1024,
          type: 'image/jpeg',
          url: 'https://example.com/test-image.jpg',
          width: 800,
          height: 600,
          format: 'jpg'
        }
      })
      imageId = image.id
    })

    it('应该成功删除图片', async () => {
      const response = await request(app)
        .delete(`/api/images/${imageId}`)
        .set('Authorization', `Bearer ${authToken}`)
        .expect(200)

      expect(response.body.success).toBe(true)
      expect(response.body.message).toBe('图片删除成功')

      // 验证图片已从数据库删除
      const image = await db.image.findUnique({
        where: { id: imageId }
      })
      expect(image).toBeNull()
    })

    it('应该拒绝删除其他用户的图片', async () => {
      // 创建另一个用户
      const otherUser = await db.user.create({
        data: {
          name: 'Other User',
          email: 'other@example.com',
          password: await bcrypt.hash('TestPassword123!', 12),
          isVerified: true,
          isActive: true
        }
      })

      const otherImage = await db.image.create({
        data: {
          userId: otherUser.id,
          name: 'other-image.jpg',
          originalName: 'other-image.jpg',
          size: 1024,
          type: 'image/jpeg',
          url: 'https://example.com/other-image.jpg',
          width: 800,
          height: 600,
          format: 'jpg'
        }
      })

      const response = await request(app)
        .delete(`/api/images/${otherImage.id}`)
        .set('Authorization', `Bearer ${authToken}`)
        .expect(404)

      expect(response.body.success).toBe(false)
    })
  })

  describe('DELETE /api/images/batch', () => {
    let imageIds: string[]

    beforeEach(async () => {
      const images = await db.image.createMany({
        data: [
          {
            userId,
            name: 'image1.jpg',
            originalName: 'image1.jpg',
            size: 1024,
            type: 'image/jpeg',
            url: 'https://example.com/image1.jpg',
            width: 800,
            height: 600,
            format: 'jpg'
          },
          {
            userId,
            name: 'image2.jpg',
            originalName: 'image2.jpg',
            size: 2048,
            type: 'image/jpeg',
            url: 'https://example.com/image2.jpg',
            width: 1200,
            height: 800,
            format: 'jpg'
          }
        ]
      })
      imageIds = images.map(img => img.id)
    })

    it('应该成功批量删除图片', async () => {
      const response = await request(app)
        .delete('/api/images/batch')
        .set('Authorization', `Bearer ${authToken}`)
        .send({ ids: imageIds })
        .expect(200)

      expect(response.body.success).toBe(true)
      expect(response.body.message).toBe(`批量删除图片成功: ${imageIds.length} 张`)

      // 验证图片已从数据库删除
      const images = await db.image.findMany({
        where: { id: { in: imageIds } }
      })
      expect(images).toHaveLength(0)
    })
  })

  describe('GET /api/images/stats/overview', () => {
    beforeEach(async () => {
      // 创建测试图片
      await db.image.createMany({
        data: [
          {
            userId,
            name: 'image1.jpg',
            originalName: 'image1.jpg',
            size: 1024,
            type: 'image/jpeg',
            url: 'https://example.com/image1.jpg',
            width: 800,
            height: 600,
            format: 'jpg'
          },
          {
            userId,
            name: 'image2.png',
            originalName: 'image2.png',
            size: 2048,
            type: 'image/png',
            url: 'https://example.com/image2.png',
            width: 1200,
            height: 800,
            format: 'png'
          }
        ]
      })
    })

    it('应该返回图片统计', async () => {
      const response = await request(app)
        .get('/api/images/stats/overview')
        .set('Authorization', `Bearer ${authToken}`)
        .expect(200)

      expect(response.body.success).toBe(true)
      expect(response.body.data.totalImages).toBe(2)
      expect(response.body.data.totalSize).toBe(3072) // 1024 + 2048
      expect(response.body.data.formatStats).toHaveLength(2)
    })
  })
})
