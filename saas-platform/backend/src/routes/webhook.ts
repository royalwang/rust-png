import { Router } from 'express'
import { WebhookController } from '../controllers/WebhookController'

const router = Router()
const webhookController = new WebhookController()

// Stripe支付webhook
router.post('/stripe', webhookController.stripeWebhook)

// 支付成功webhook
router.post('/payment/success', webhookController.paymentSuccess)

// 支付失败webhook
router.post('/payment/failed', webhookController.paymentFailed)

// 订阅更新webhook
router.post('/subscription/updated', webhookController.subscriptionUpdated)

// 订阅取消webhook
router.post('/subscription/cancelled', webhookController.subscriptionCancelled)

export { router as webhookRoutes }
