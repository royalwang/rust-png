import { Request, Response, NextFunction } from 'express'
import jwt from 'jsonwebtoken'
import { StatusCodes } from 'http-status-codes'
import { CustomError } from './errorHandler'

interface AuthRequest extends Request {
  user?: {
    id: string
    email: string
    role: string
  }
}

export const authenticateToken = (req: AuthRequest, res: Response, next: NextFunction) => {
  const authHeader = req.headers['authorization']
  const token = authHeader && authHeader.split(' ')[1] // Bearer TOKEN

  if (!token) {
    throw new CustomError('访问令牌缺失', StatusCodes.UNAUTHORIZED)
  }

  try {
    const decoded = jwt.verify(token, process.env.JWT_SECRET!) as any
    req.user = {
      id: decoded.id,
      email: decoded.email,
      role: decoded.role,
    }
    next()
  } catch (error) {
    throw new CustomError('无效的访问令牌', StatusCodes.UNAUTHORIZED)
  }
}

export const requireRole = (roles: string[]) => {
  return (req: AuthRequest, res: Response, next: NextFunction) => {
    if (!req.user) {
      throw new CustomError('用户未认证', StatusCodes.UNAUTHORIZED)
    }

    if (!roles.includes(req.user.role)) {
      throw new CustomError('权限不足', StatusCodes.FORBIDDEN)
    }

    next()
  }
}

export const optionalAuth = (req: AuthRequest, res: Response, next: NextFunction) => {
  const authHeader = req.headers['authorization']
  const token = authHeader && authHeader.split(' ')[1]

  if (!token) {
    return next()
  }

  try {
    const decoded = jwt.verify(token, process.env.JWT_SECRET!) as any
    req.user = {
      id: decoded.id,
      email: decoded.email,
      role: decoded.role,
    }
  } catch (error) {
    // 忽略可选认证的错误
  }

  next()
}
