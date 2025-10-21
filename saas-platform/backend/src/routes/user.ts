import { Router } from 'express'
import { UserController } from '../controllers/UserController'
import { validateId, validatePagination } from '../middleware/validation'
import { authenticateToken, requireRole } from '../middleware/auth'

const router = Router()
const userController = new UserController()

// 获取用户列表 (管理员)
router.get('/', authenticateToken, requireRole(['admin']), validatePagination, userController.getUsers)

// 获取用户详情
router.get('/:id', authenticateToken, validateId, userController.getUser)

// 更新用户信息
router.put('/:id', authenticateToken, validateId, userController.updateUser)

// 删除用户 (管理员)
router.delete('/:id', authenticateToken, requireRole(['admin']), validateId, userController.deleteUser)

// 更新用户头像
router.post('/:id/avatar', authenticateToken, validateId, userController.updateAvatar)

// 获取用户统计
router.get('/:id/stats', authenticateToken, validateId, userController.getUserStats)

// 获取用户图片
router.get('/:id/images', authenticateToken, validateId, validatePagination, userController.getUserImages)

// 获取用户订阅信息
router.get('/:id/subscription', authenticateToken, validateId, userController.getUserSubscription)

// 更新用户订阅
router.put('/:id/subscription', authenticateToken, validateId, userController.updateUserSubscription)

export { router as userRoutes }
