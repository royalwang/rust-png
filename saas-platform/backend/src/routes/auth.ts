import { Router } from 'express'
import { AuthController } from '../controllers/AuthController'
import { validateLogin, validateRegister } from '../middleware/validation'
import { authenticateToken } from '../middleware/auth'

const router = Router()
const authController = new AuthController()

// 用户注册
router.post('/register', validateRegister, authController.register)

// 用户登录
router.post('/login', validateLogin, authController.login)

// 用户登出
router.post('/logout', authenticateToken, authController.logout)

// 刷新令牌
router.post('/refresh', authController.refreshToken)

// 验证邮箱
router.post('/verify-email', authController.verifyEmail)

// 发送验证邮件
router.post('/send-verification', authenticateToken, authController.sendVerificationEmail)

// 忘记密码
router.post('/forgot-password', authController.forgotPassword)

// 重置密码
router.post('/reset-password', authController.resetPassword)

// 修改密码
router.post('/change-password', authenticateToken, authController.changePassword)

// 获取当前用户信息
router.get('/me', authenticateToken, authController.getCurrentUser)

export { router as authRoutes }
