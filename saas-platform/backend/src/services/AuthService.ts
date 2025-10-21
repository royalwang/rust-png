import bcrypt from 'bcryptjs'
import jwt from 'jsonwebtoken'
import { v4 as uuidv4 } from 'uuid'
import { getDatabase } from '../utils/database'
import { CustomError } from '../middleware/errorHandler'
import { StatusCodes } from 'http-status-codes'
import { logger } from '../utils/logger'

export interface CreateUserData {
  name: string
  email: string
  password: string
}

export interface LoginCredentials {
  email: string
  password: string
}

export interface ChangePasswordData {
  currentPassword: string
  newPassword: string
}

export class AuthService {
  private db = getDatabase()

  async createUser(data: CreateUserData) {
    const { name, email, password } = data

    // 检查用户是否已存在
    const existingUser = await this.db.user.findUnique({
      where: { email }
    })

    if (existingUser) {
      throw new CustomError('用户已存在', StatusCodes.CONFLICT)
    }

    // 加密密码
    const hashedPassword = await bcrypt.hash(password, 12)

    // 生成邮箱验证令牌
    const emailVerificationToken = uuidv4()

    // 创建用户
    const user = await this.db.user.create({
      data: {
        name,
        email,
        password: hashedPassword,
        emailVerificationToken,
      },
      select: {
        id: true,
        name: true,
        email: true,
        role: true,
        isVerified: true,
        createdAt: true,
        updatedAt: true,
      }
    })

    // 创建免费订阅
    await this.db.subscription.create({
      data: {
        userId: user.id,
        plan: 'FREE',
        status: 'ACTIVE',
      }
    })

    logger.info(`用户创建成功: ${email}`)
    return user
  }

  async findUserByEmail(email: string) {
    return await this.db.user.findUnique({
      where: { email }
    })
  }

  async findUserById(id: string) {
    return await this.db.user.findUnique({
      where: { id },
      select: {
        id: true,
        name: true,
        email: true,
        role: true,
        isVerified: true,
        isActive: true,
        lastLoginAt: true,
        createdAt: true,
        updatedAt: true,
      }
    })
  }

  async updateLastLogin(userId: string) {
    await this.db.user.update({
      where: { id: userId },
      data: { lastLoginAt: new Date() }
    })
  }

  async verifyEmail(token: string) {
    const user = await this.db.user.findFirst({
      where: { emailVerificationToken: token }
    })

    if (!user) {
      throw new CustomError('无效的验证令牌', StatusCodes.BAD_REQUEST)
    }

    await this.db.user.update({
      where: { id: user.id },
      data: {
        isVerified: true,
        emailVerificationToken: null,
      }
    })

    logger.info(`邮箱验证成功: ${user.email}`)
  }

  async resetPassword(token: string, newPassword: string) {
    const user = await this.db.user.findFirst({
      where: {
        passwordResetToken: token,
        passwordResetExpires: {
          gt: new Date()
        }
      }
    })

    if (!user) {
      throw new CustomError('无效或过期的重置令牌', StatusCodes.BAD_REQUEST)
    }

    const hashedPassword = await bcrypt.hash(newPassword, 12)

    await this.db.user.update({
      where: { id: user.id },
      data: {
        password: hashedPassword,
        passwordResetToken: null,
        passwordResetExpires: null,
      }
    })

    logger.info(`密码重置成功: ${user.email}`)
  }

  async changePassword(userId: string, currentPassword: string, newPassword: string) {
    const user = await this.db.user.findUnique({
      where: { id: userId }
    })

    if (!user) {
      throw new CustomError('用户不存在', StatusCodes.NOT_FOUND)
    }

    const isCurrentPasswordValid = await bcrypt.compare(currentPassword, user.password)
    if (!isCurrentPasswordValid) {
      throw new CustomError('当前密码错误', StatusCodes.BAD_REQUEST)
    }

    const hashedPassword = await bcrypt.hash(newPassword, 12)

    await this.db.user.update({
      where: { id: userId },
      data: { password: hashedPassword }
    })

    logger.info(`密码修改成功: ${user.email}`)
  }

  async generatePasswordResetToken(email: string) {
    const user = await this.db.user.findUnique({
      where: { email }
    })

    if (!user) {
      throw new CustomError('用户不存在', StatusCodes.NOT_FOUND)
    }

    const resetToken = uuidv4()
    const resetExpires = new Date(Date.now() + 3600000) // 1小时后过期

    await this.db.user.update({
      where: { id: user.id },
      data: {
        passwordResetToken: resetToken,
        passwordResetExpires: resetExpires,
      }
    })

    return resetToken
  }

  async generateTokens(userId: string) {
    const accessToken = jwt.sign(
      { id: userId },
      process.env.JWT_SECRET!,
      { expiresIn: process.env.JWT_EXPIRES_IN || '7d' }
    )

    const refreshToken = jwt.sign(
      { id: userId },
      process.env.REFRESH_TOKEN_SECRET!,
      { expiresIn: process.env.REFRESH_TOKEN_EXPIRES_IN || '30d' }
    )

    return { accessToken, refreshToken }
  }

  async validateRefreshToken(token: string) {
    try {
      const decoded = jwt.verify(token, process.env.REFRESH_TOKEN_SECRET!) as any
      const user = await this.findUserById(decoded.id)
      
      if (!user) {
        throw new CustomError('用户不存在', StatusCodes.UNAUTHORIZED)
      }

      return user
    } catch (error) {
      throw new CustomError('无效的刷新令牌', StatusCodes.UNAUTHORIZED)
    }
  }
}
