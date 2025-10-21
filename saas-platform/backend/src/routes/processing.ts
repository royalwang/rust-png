import { Router } from 'express'
import { ProcessingController } from '../controllers/ProcessingController'
import { validateId, validateProcessingOptions, validatePagination } from '../middleware/validation'
import { authenticateToken } from '../middleware/auth'

const router = Router()
const processingController = new ProcessingController()

// 处理单张图片
router.post('/process', authenticateToken, validateProcessingOptions, processingController.processImage)

// 批量处理图片
router.post('/process/batch', authenticateToken, processingController.processBatch)

// 获取处理历史
router.get('/history', authenticateToken, validatePagination, processingController.getProcessingHistory)

// 获取处理结果
router.get('/result/:id', authenticateToken, validateId, processingController.getProcessingResult)

// 下载处理结果
router.get('/result/:id/download', authenticateToken, validateId, processingController.downloadResult)

// 删除处理结果
router.delete('/result/:id', authenticateToken, validateId, processingController.deleteResult)

// 获取处理统计
router.get('/stats', authenticateToken, processingController.getProcessingStats)

// 获取处理队列状态
router.get('/queue/status', authenticateToken, processingController.getQueueStatus)

// 取消处理任务
router.post('/queue/cancel/:id', authenticateToken, validateId, processingController.cancelTask)

// 重新处理
router.post('/reprocess/:id', authenticateToken, validateId, processingController.reprocess)

// 获取处理模板
router.get('/templates', authenticateToken, processingController.getTemplates)

// 创建处理模板
router.post('/templates', authenticateToken, processingController.createTemplate)

// 更新处理模板
router.put('/templates/:id', authenticateToken, validateId, processingController.updateTemplate)

// 删除处理模板
router.delete('/templates/:id', authenticateToken, validateId, processingController.deleteTemplate)

export { router as processingRoutes }
