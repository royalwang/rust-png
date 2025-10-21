import { Router } from 'express'
import { SubscriptionController } from '../controllers/SubscriptionController'
import { validateId } from '../middleware/validation'
import { authenticateToken } from '../middleware/auth'

const router = Router()
const subscriptionController = new SubscriptionController()

// 获取订阅计划
router.get('/plans', subscriptionController.getPlans)

// 获取用户订阅
router.get('/current', authenticateToken, subscriptionController.getCurrentSubscription)

// 创建订阅
router.post('/create', authenticateToken, subscriptionController.createSubscription)

// 更新订阅
router.put('/update', authenticateToken, subscriptionController.updateSubscription)

// 取消订阅
router.post('/cancel', authenticateToken, subscriptionController.cancelSubscription)

// 恢复订阅
router.post('/resume', authenticateToken, subscriptionController.resumeSubscription)

// 获取订阅历史
router.get('/history', authenticateToken, subscriptionController.getSubscriptionHistory)

// 获取发票
router.get('/invoices', authenticateToken, subscriptionController.getInvoices)

// 下载发票
router.get('/invoices/:id/download', authenticateToken, validateId, subscriptionController.downloadInvoice)

// 获取支付方式
router.get('/payment-methods', authenticateToken, subscriptionController.getPaymentMethods)

// 添加支付方式
router.post('/payment-methods', authenticateToken, subscriptionController.addPaymentMethod)

// 更新支付方式
router.put('/payment-methods/:id', authenticateToken, validateId, subscriptionController.updatePaymentMethod)

// 删除支付方式
router.delete('/payment-methods/:id', authenticateToken, validateId, subscriptionController.deletePaymentMethod)

// 设置默认支付方式
router.post('/payment-methods/:id/default', authenticateToken, validateId, subscriptionController.setDefaultPaymentMethod)

export { router as subscriptionRoutes }
