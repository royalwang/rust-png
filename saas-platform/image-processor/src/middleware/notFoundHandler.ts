import { Request, Response, NextFunction } from 'express'
import { StatusCodes } from 'http-status-codes'

export const notFoundHandler = (req: Request, res: Response, next: NextFunction) => {
  res.status(StatusCodes.NOT_FOUND).json({
    success: false,
    error: {
      message: `图片处理服务路由 ${req.method} ${req.originalUrl} 不存在`,
      statusCode: StatusCodes.NOT_FOUND,
    },
  })
}
