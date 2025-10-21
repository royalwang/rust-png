import { Request, Response, NextFunction } from 'express'
import { StatusCodes } from 'http-status-codes'
import { logger } from '../utils/logger'

export interface AppError extends Error {
  statusCode?: number
  isOperational?: boolean
}

export class CustomError extends Error implements AppError {
  public statusCode: number
  public isOperational: boolean

  constructor(message: string, statusCode: number = StatusCodes.INTERNAL_SERVER_ERROR, isOperational: boolean = true) {
    super(message)
    this.statusCode = statusCode
    this.isOperational = isOperational

    Error.captureStackTrace(this, this.constructor)
  }
}

export const errorHandler = (
  error: AppError,
  req: Request,
  res: Response,
  next: NextFunction
) => {
  const { statusCode = StatusCodes.INTERNAL_SERVER_ERROR, message, isOperational = false } = error

  // 记录错误日志
  logger.error('Image Processor Error:', {
    error: {
      message,
      statusCode,
      stack: error.stack,
      isOperational,
    },
    request: {
      method: req.method,
      url: req.url,
      headers: req.headers,
      body: req.body,
    },
  })

  // 开发环境返回详细错误信息
  if (process.env.NODE_ENV === 'development') {
    return res.status(statusCode).json({
      success: false,
      error: {
        message,
        statusCode,
        stack: error.stack,
        isOperational,
      },
    })
  }

  // 生产环境返回简化错误信息
  if (isOperational) {
    return res.status(statusCode).json({
      success: false,
      error: {
        message,
        statusCode,
      },
    })
  }

  // 非操作错误，返回通用错误信息
  return res.status(StatusCodes.INTERNAL_SERVER_ERROR).json({
    success: false,
    error: {
      message: '图片处理服务内部错误',
      statusCode: StatusCodes.INTERNAL_SERVER_ERROR,
    },
  })
}
