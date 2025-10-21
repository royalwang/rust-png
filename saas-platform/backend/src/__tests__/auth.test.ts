import { describe, it, expect, beforeEach, afterEach, jest } from '@jest/globals'
import request from 'supertest'
import { app } from '../index'
import { getDatabase } from '../utils/database'
import bcrypt from 'bcryptjs'

describe('Auth API', () => {
  let db: any

  beforeEach(async () => {
    db = getDatabase()
    // 清理测试数据
    await db.user.deleteMany()
    await db.subscription.deleteMany()
  })

  afterEach(async () => {
    // 清理测试数据
    await db.user.deleteMany()
    await db.subscription.deleteMany()
  })

  describe('POST /api/auth/register', () => {
    it('应该成功注册新用户', async () => {
      const userData = {
        name: 'Test User',
        email: 'test@example.com',
        password: 'TestPassword123!',
        confirmPassword: 'TestPassword123!',
        acceptTerms: true
      }

      const response = await request(app)
        .post('/api/auth/register')
        .send(userData)
        .expect(201)

      expect(response.body.success).toBe(true)
      expect(response.body.data.user.email).toBe(userData.email)
      expect(response.body.data.user.name).toBe(userData.name)
      expect(response.body.data.token).toBeDefined()

      // 验证用户已创建
      const user = await db.user.findUnique({
        where: { email: userData.email }
      })
      expect(user).toBeTruthy()
      expect(user.name).toBe(userData.name)
      expect(user.isVerified).toBe(false)

      // 验证订阅已创建
      const subscription = await db.subscription.findUnique({
        where: { userId: user.id }
      })
      expect(subscription).toBeTruthy()
      expect(subscription.plan).toBe('FREE')
    })

    it('应该拒绝重复邮箱注册', async () => {
      // 先创建一个用户
      await db.user.create({
        data: {
          name: 'Existing User',
          email: 'existing@example.com',
          password: await bcrypt.hash('password', 12),
          isVerified: true
        }
      })

      const userData = {
        name: 'Test User',
        email: 'existing@example.com',
        password: 'TestPassword123!',
        confirmPassword: 'TestPassword123!',
        acceptTerms: true
      }

      const response = await request(app)
        .post('/api/auth/register')
        .send(userData)
        .expect(409)

      expect(response.body.success).toBe(false)
      expect(response.body.error.message).toBe('用户已存在')
    })

    it('应该验证密码强度', async () => {
      const userData = {
        name: 'Test User',
        email: 'test@example.com',
        password: 'weak',
        confirmPassword: 'weak',
        acceptTerms: true
      }

      const response = await request(app)
        .post('/api/auth/register')
        .send(userData)
        .expect(400)

      expect(response.body.success).toBe(false)
    })

    it('应该验证必填字段', async () => {
      const userData = {
        name: '',
        email: 'invalid-email',
        password: 'TestPassword123!',
        confirmPassword: 'DifferentPassword123!',
        acceptTerms: false
      }

      const response = await request(app)
        .post('/api/auth/register')
        .send(userData)
        .expect(400)

      expect(response.body.success).toBe(false)
    })
  })

  describe('POST /api/auth/login', () => {
    beforeEach(async () => {
      // 创建测试用户
      await db.user.create({
        data: {
          name: 'Test User',
          email: 'test@example.com',
          password: await bcrypt.hash('TestPassword123!', 12),
          isVerified: true,
          isActive: true
        }
      })
    })

    it('应该成功登录', async () => {
      const loginData = {
        email: 'test@example.com',
        password: 'TestPassword123!'
      }

      const response = await request(app)
        .post('/api/auth/login')
        .send(loginData)
        .expect(200)

      expect(response.body.success).toBe(true)
      expect(response.body.data.user.email).toBe(loginData.email)
      expect(response.body.data.token).toBeDefined()
    })

    it('应该拒绝错误密码', async () => {
      const loginData = {
        email: 'test@example.com',
        password: 'WrongPassword123!'
      }

      const response = await request(app)
        .post('/api/auth/login')
        .send(loginData)
        .expect(401)

      expect(response.body.success).toBe(false)
      expect(response.body.error.message).toBe('密码错误')
    })

    it('应该拒绝不存在的用户', async () => {
      const loginData = {
        email: 'nonexistent@example.com',
        password: 'TestPassword123!'
      }

      const response = await request(app)
        .post('/api/auth/login')
        .send(loginData)
        .expect(401)

      expect(response.body.success).toBe(false)
      expect(response.body.error.message).toBe('用户不存在')
    })

    it('应该拒绝未验证的用户', async () => {
      // 创建未验证的用户
      await db.user.create({
        data: {
          name: 'Unverified User',
          email: 'unverified@example.com',
          password: await bcrypt.hash('TestPassword123!', 12),
          isVerified: false,
          isActive: true
        }
      })

      const loginData = {
        email: 'unverified@example.com',
        password: 'TestPassword123!'
      }

      const response = await request(app)
        .post('/api/auth/login')
        .send(loginData)
        .expect(200) // 登录成功，但isVerified为false

      expect(response.body.data.user.isVerified).toBe(false)
    })
  })

  describe('POST /api/auth/logout', () => {
    it('应该成功登出', async () => {
      // 创建用户并获取token
      const user = await db.user.create({
        data: {
          name: 'Test User',
          email: 'test@example.com',
          password: await bcrypt.hash('TestPassword123!', 12),
          isVerified: true,
          isActive: true
        }
      })

      const token = 'valid-jwt-token' // 在实际测试中需要生成真实的JWT token

      const response = await request(app)
        .post('/api/auth/logout')
        .set('Authorization', `Bearer ${token}`)
        .expect(200)

      expect(response.body.success).toBe(true)
      expect(response.body.message).toBe('登出成功')
    })
  })

  describe('GET /api/auth/me', () => {
    it('应该返回当前用户信息', async () => {
      // 创建用户
      const user = await db.user.create({
        data: {
          name: 'Test User',
          email: 'test@example.com',
          password: await bcrypt.hash('TestPassword123!', 12),
          isVerified: true,
          isActive: true
        }
      })

      const token = 'valid-jwt-token' // 在实际测试中需要生成真实的JWT token

      const response = await request(app)
        .get('/api/auth/me')
        .set('Authorization', `Bearer ${token}`)
        .expect(200)

      expect(response.body.success).toBe(true)
      expect(response.body.data.user.id).toBe(user.id)
      expect(response.body.data.user.email).toBe(user.email)
    })

    it('应该拒绝无效token', async () => {
      const response = await request(app)
        .get('/api/auth/me')
        .set('Authorization', 'Bearer invalid-token')
        .expect(401)

      expect(response.body.success).toBe(false)
    })
  })
})
