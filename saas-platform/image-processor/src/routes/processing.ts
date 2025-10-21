import { Router } from 'express'
import { ProcessingController } from '../controllers/ProcessingController'
import { upload } from '../middleware/upload'

const router = Router()
const processingController = new ProcessingController()

// 处理单张图片
router.post('/process', upload.single('image'), processingController.processImage)

// 批量处理图片
router.post('/process/batch', upload.array('images', 10), processingController.processBatch)

// 获取处理状态
router.get('/status/:id', processingController.getProcessingStatus)

// 获取处理结果
router.get('/result/:id', processingController.getProcessingResult)

// 下载处理结果
router.get('/result/:id/download', processingController.downloadResult)

// 删除处理结果
router.delete('/result/:id', processingController.deleteResult)

// 获取处理统计
router.get('/stats', processingController.getProcessingStats)

// 获取队列状态
router.get('/queue/status', processingController.getQueueStatus)

// 取消处理任务
router.post('/queue/cancel/:id', processingController.cancelTask)

// 重新处理
router.post('/reprocess/:id', processingController.reprocess)

export { router as processingRoutes }
