import { Request, Response, NextFunction } from 'express'
import bcrypt from 'bcryptjs'
import jwt from 'jsonwebtoken'
import { StatusCodes } from 'http-status-codes'
import { CustomError } from '../middleware/errorHandler'
import { AuthService } from '../services/AuthService'
import { EmailService } from '../services/EmailService'
import { logger } from '../utils/logger'

export class AuthController {
  private authService: AuthService
  private emailService: EmailService

  constructor() {
    this.authService = new AuthService()
    this.emailService = new EmailService()
  }

  // 用户注册
  public register = async (req: Request, res: Response, next: NextFunction) => {
    try {
      const { name, email, password } = req.body

      // 检查用户是否已存在
      const existingUser = await this.authService.findUserByEmail(email)
      if (existingUser) {
        throw new CustomError('用户已存在', StatusCodes.CONFLICT)
      }

      // 创建用户
      const user = await this.authService.createUser({
        name,
        email,
        password,
      })

      // 生成JWT令牌
      const token = jwt.sign(
        { id: user.id, email: user.email, role: user.role },
        process.env.JWT_SECRET!,
        { expiresIn: process.env.JWT_EXPIRES_IN || '7d' }
      )

      // 发送验证邮件
      await this.emailService.sendVerificationEmail(user.email, user.id)

      logger.info(`用户注册成功: ${email}`)

      res.status(StatusCodes.CREATED).json({
        success: true,
        message: '注册成功，请检查邮箱验证邮件',
        data: {
          user: {
            id: user.id,
            name: user.name,
            email: user.email,
            role: user.role,
            isVerified: user.isVerified,
          },
          token,
        },
      })
    } catch (error) {
      next(error)
    }
  }

  // 用户登录
  public login = async (req: Request, res: Response, next: NextFunction) => {
    try {
      const { email, password } = req.body

      // 查找用户
      const user = await this.authService.findUserByEmail(email)
      if (!user) {
        throw new CustomError('用户不存在', StatusCodes.UNAUTHORIZED)
      }

      // 验证密码
      const isPasswordValid = await bcrypt.compare(password, user.password)
      if (!isPasswordValid) {
        throw new CustomError('密码错误', StatusCodes.UNAUTHORIZED)
      }

      // 检查用户状态
      if (!user.isActive) {
        throw new CustomError('账户已被禁用', StatusCodes.FORBIDDEN)
      }

      // 生成JWT令牌
      const token = jwt.sign(
        { id: user.id, email: user.email, role: user.role },
        process.env.JWT_SECRET!,
        { expiresIn: process.env.JWT_EXPIRES_IN || '7d' }
      )

      // 更新最后登录时间
      await this.authService.updateLastLogin(user.id)

      logger.info(`用户登录成功: ${email}`)

      res.status(StatusCodes.OK).json({
        success: true,
        message: '登录成功',
        data: {
          user: {
            id: user.id,
            name: user.name,
            email: user.email,
            role: user.role,
            isVerified: user.isVerified,
            lastLoginAt: user.lastLoginAt,
          },
          token,
        },
      })
    } catch (error) {
      next(error)
    }
  }

  // 用户登出
  public logout = async (req: Request, res: Response, next: NextFunction) => {
    try {
      // 这里可以实现令牌黑名单机制
      logger.info(`用户登出: ${req.user?.email}`)

      res.status(StatusCodes.OK).json({
        success: true,
        message: '登出成功',
      })
    } catch (error) {
      next(error)
    }
  }

  // 刷新令牌
  public refreshToken = async (req: Request, res: Response, next: NextFunction) => {
    try {
      const { refreshToken } = req.body

      if (!refreshToken) {
        throw new CustomError('刷新令牌缺失', StatusCodes.BAD_REQUEST)
      }

      // 验证刷新令牌
      const decoded = jwt.verify(refreshToken, process.env.REFRESH_TOKEN_SECRET!) as any
      const user = await this.authService.findUserById(decoded.id)

      if (!user) {
        throw new CustomError('用户不存在', StatusCodes.UNAUTHORIZED)
      }

      // 生成新的访问令牌
      const newToken = jwt.sign(
        { id: user.id, email: user.email, role: user.role },
        process.env.JWT_SECRET!,
        { expiresIn: process.env.JWT_EXPIRES_IN || '7d' }
      )

      res.status(StatusCodes.OK).json({
        success: true,
        data: {
          token: newToken,
        },
      })
    } catch (error) {
      next(error)
    }
  }

  // 验证邮箱
  public verifyEmail = async (req: Request, res: Response, next: NextFunction) => {
    try {
      const { token } = req.body

      if (!token) {
        throw new CustomError('验证令牌缺失', StatusCodes.BAD_REQUEST)
      }

      // 验证邮箱
      await this.authService.verifyEmail(token)

      res.status(StatusCodes.OK).json({
        success: true,
        message: '邮箱验证成功',
      })
    } catch (error) {
      next(error)
    }
  }

  // 发送验证邮件
  public sendVerificationEmail = async (req: Request, res: Response, next: NextFunction) => {
    try {
      const userId = req.user!.id

      await this.emailService.sendVerificationEmail(req.user!.email, userId)

      res.status(StatusCodes.OK).json({
        success: true,
        message: '验证邮件已发送',
      })
    } catch (error) {
      next(error)
    }
  }

  // 忘记密码
  public forgotPassword = async (req: Request, res: Response, next: NextFunction) => {
    try {
      const { email } = req.body

      const user = await this.authService.findUserByEmail(email)
      if (!user) {
        throw new CustomError('用户不存在', StatusCodes.NOT_FOUND)
      }

      // 发送重置密码邮件
      await this.emailService.sendPasswordResetEmail(user.email, user.id)

      res.status(StatusCodes.OK).json({
        success: true,
        message: '重置密码邮件已发送',
      })
    } catch (error) {
      next(error)
    }
  }

  // 重置密码
  public resetPassword = async (req: Request, res: Response, next: NextFunction) => {
    try {
      const { token, password } = req.body

      if (!token || !password) {
        throw new CustomError('令牌和密码不能为空', StatusCodes.BAD_REQUEST)
      }

      await this.authService.resetPassword(token, password)

      res.status(StatusCodes.OK).json({
        success: true,
        message: '密码重置成功',
      })
    } catch (error) {
      next(error)
    }
  }

  // 修改密码
  public changePassword = async (req: Request, res: Response, next: NextFunction) => {
    try {
      const { currentPassword, newPassword } = req.body
      const userId = req.user!.id

      await this.authService.changePassword(userId, currentPassword, newPassword)

      res.status(StatusCodes.OK).json({
        success: true,
        message: '密码修改成功',
      })
    } catch (error) {
      next(error)
    }
  }

  // 获取当前用户信息
  public getCurrentUser = async (req: Request, res: Response, next: NextFunction) => {
    try {
      const userId = req.user!.id
      const user = await this.authService.findUserById(userId)

      if (!user) {
        throw new CustomError('用户不存在', StatusCodes.NOT_FOUND)
      }

      res.status(StatusCodes.OK).json({
        success: true,
        data: {
          user: {
            id: user.id,
            name: user.name,
            email: user.email,
            role: user.role,
            isVerified: user.isVerified,
            createdAt: user.createdAt,
            lastLoginAt: user.lastLoginAt,
          },
        },
      })
    } catch (error) {
      next(error)
    }
  }
}
