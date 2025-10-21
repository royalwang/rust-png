import { Router } from 'express'
import { ImageController } from '../controllers/ImageController'
import { validateId, validatePagination } from '../middleware/validation'
import { authenticateToken } from '../middleware/auth'
import { upload } from '../middleware/upload'

const router = Router()
const imageController = new ImageController()

// 上传图片
router.post('/upload', authenticateToken, upload.single('image'), imageController.uploadImage)

// 批量上传图片
router.post('/upload/batch', authenticateToken, upload.array('images', 10), imageController.uploadBatch)

// 获取图片列表
router.get('/', authenticateToken, validatePagination, imageController.getImages)

// 获取图片详情
router.get('/:id', authenticateToken, validateId, imageController.getImage)

// 下载图片
router.get('/:id/download', authenticateToken, validateId, imageController.downloadImage)

// 删除图片
router.delete('/:id', authenticateToken, validateId, imageController.deleteImage)

// 批量删除图片
router.delete('/batch', authenticateToken, imageController.deleteBatch)

// 获取图片统计
router.get('/stats/overview', authenticateToken, imageController.getImageStats)

// 获取图片格式统计
router.get('/stats/formats', authenticateToken, imageController.getFormatStats)

// 获取图片大小统计
router.get('/stats/sizes', authenticateToken, imageController.getSizeStats)

// 搜索图片
router.get('/search', authenticateToken, imageController.searchImages)

// 获取图片标签
router.get('/:id/tags', authenticateToken, validateId, imageController.getImageTags)

// 更新图片标签
router.put('/:id/tags', authenticateToken, validateId, imageController.updateImageTags)

export { router as imageRoutes }
