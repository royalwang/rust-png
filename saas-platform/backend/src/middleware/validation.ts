import { Request, Response, NextFunction } from 'express'
import { body, param, query, validationResult } from 'express-validator'
import { StatusCodes } from 'http-status-codes'
import { CustomError } from './errorHandler'

export const handleValidationErrors = (req: Request, res: Response, next: NextFunction) => {
  const errors = validationResult(req)
  
  if (!errors.isEmpty()) {
    const errorMessages = errors.array().map(error => ({
      field: error.type === 'field' ? error.path : 'unknown',
      message: error.msg,
      value: error.type === 'field' ? error.value : undefined,
    }))

    throw new CustomError('验证失败', StatusCodes.BAD_REQUEST)
  }

  next()
}

// 用户注册验证
export const validateRegister = [
  body('name')
    .trim()
    .isLength({ min: 2, max: 50 })
    .withMessage('姓名长度必须在2-50个字符之间'),
  body('email')
    .isEmail()
    .normalizeEmail()
    .withMessage('请输入有效的邮箱地址'),
  body('password')
    .isLength({ min: 8, max: 128 })
    .withMessage('密码长度必须在8-128个字符之间')
    .matches(/^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]/)
    .withMessage('密码必须包含大小写字母、数字和特殊字符'),
  body('confirmPassword')
    .custom((value, { req }) => {
      if (value !== req.body.password) {
        throw new Error('确认密码不匹配')
      }
      return true
    }),
  body('acceptTerms')
    .equals('true')
    .withMessage('必须接受服务条款'),
  handleValidationErrors,
]

// 用户登录验证
export const validateLogin = [
  body('email')
    .isEmail()
    .normalizeEmail()
    .withMessage('请输入有效的邮箱地址'),
  body('password')
    .notEmpty()
    .withMessage('密码不能为空'),
  handleValidationErrors,
]

// 图片处理选项验证
export const validateProcessingOptions = [
  body('resize')
    .optional()
    .isObject()
    .withMessage('调整大小选项必须是对象'),
  body('resize.enabled')
    .optional()
    .isBoolean()
    .withMessage('调整大小启用状态必须是布尔值'),
  body('resize.width')
    .optional()
    .isInt({ min: 1, max: 10000 })
    .withMessage('宽度必须是1-10000之间的整数'),
  body('resize.height')
    .optional()
    .isInt({ min: 1, max: 10000 })
    .withMessage('高度必须是1-10000之间的整数'),
  body('crop')
    .optional()
    .isObject()
    .withMessage('裁剪选项必须是对象'),
  body('filters')
    .optional()
    .isObject()
    .withMessage('滤镜选项必须是对象'),
  body('compression')
    .optional()
    .isObject()
    .withMessage('压缩选项必须是对象'),
  body('compression.quality')
    .optional()
    .isInt({ min: 1, max: 100 })
    .withMessage('质量必须是1-100之间的整数'),
  body('compression.format')
    .optional()
    .isIn(['png', 'jpg', 'webp', 'avif'])
    .withMessage('格式必须是png、jpg、webp或avif之一'),
  handleValidationErrors,
]

// ID参数验证
export const validateId = [
  param('id')
    .isUUID()
    .withMessage('ID必须是有效的UUID'),
  handleValidationErrors,
]

// 分页查询验证
export const validatePagination = [
  query('page')
    .optional()
    .isInt({ min: 1 })
    .withMessage('页码必须是大于0的整数'),
  query('limit')
    .optional()
    .isInt({ min: 1, max: 100 })
    .withMessage('每页数量必须是1-100之间的整数'),
  handleValidationErrors,
]
