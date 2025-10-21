import { PNG, PNGSync, optimizePNG } from 'rust-png-js'
import sharp from 'sharp'
import { v4 as uuidv4 } from 'uuid'
import { logger } from '../utils/logger'
import { ProcessingOptions, ProcessingResult } from '../types/processing'

export class ImageProcessor {
  private pngSync: PNGSync

  constructor() {
    this.pngSync = new PNGSync()
  }

  /**
   * 处理单张图片
   */
  async processImage(
    inputBuffer: Buffer,
    options: ProcessingOptions,
    originalFilename: string
  ): Promise<ProcessingResult> {
    const startTime = Date.now()
    const resultId = uuidv4()

    try {
      logger.info(`开始处理图片: ${originalFilename}`)

      // 获取图片信息
      const imageInfo = await sharp(inputBuffer).metadata()
      const originalSize = inputBuffer.length

      let processedBuffer: Buffer

      // 根据选项处理图片
      if (options.resize?.enabled) {
        processedBuffer = await this.resizeImage(inputBuffer, options.resize)
      } else if (options.crop?.enabled) {
        processedBuffer = await this.cropImage(inputBuffer, options.crop)
      } else if (options.filters) {
        processedBuffer = await this.applyFilters(inputBuffer, options.filters)
      } else {
        processedBuffer = inputBuffer
      }

      // 应用压缩
      if (options.compression) {
        processedBuffer = await this.compressImage(processedBuffer, options.compression)
      }

      // 应用水印
      if (options.watermark?.enabled) {
        processedBuffer = await this.addWatermark(processedBuffer, options.watermark)
      }

      const processingTime = Date.now() - startTime
      const fileSizeReduction = ((originalSize - processedBuffer.length) / originalSize) * 100

      const result: ProcessingResult = {
        id: resultId,
        originalImage: {
          id: uuidv4(),
          name: originalFilename,
          size: originalSize,
          type: 'image/png',
          url: '',
          width: imageInfo.width || 0,
          height: imageInfo.height || 0,
          format: imageInfo.format || 'png',
          createdAt: new Date(),
          updatedAt: new Date(),
        },
        processedImage: {
          id: uuidv4(),
          name: `processed_${originalFilename}`,
          size: processedBuffer.length,
          type: 'image/png',
          url: '',
          width: imageInfo.width || 0,
          height: imageInfo.height || 0,
          format: options.compression?.format || 'png',
          createdAt: new Date(),
          updatedAt: new Date(),
        },
        options,
        processingTime,
        fileSizeReduction,
        createdAt: new Date(),
      }

      logger.info(`图片处理完成: ${originalFilename}, 耗时: ${processingTime}ms, 压缩率: ${fileSizeReduction.toFixed(2)}%`)

      return result
    } catch (error) {
      logger.error(`图片处理失败: ${originalFilename}`, error)
      throw error
    }
  }

  /**
   * 调整图片大小
   */
  private async resizeImage(inputBuffer: Buffer, resizeOptions: any): Promise<Buffer> {
    const { width, height, maintainAspectRatio } = resizeOptions

    if (maintainAspectRatio) {
      return sharp(inputBuffer)
        .resize(width, height, { fit: 'inside', withoutEnlargement: true })
        .png()
        .toBuffer()
    } else {
      return sharp(inputBuffer)
        .resize(width, height)
        .png()
        .toBuffer()
    }
  }

  /**
   * 裁剪图片
   */
  private async cropImage(inputBuffer: Buffer, cropOptions: any): Promise<Buffer> {
    const { x, y, width, height } = cropOptions

    return sharp(inputBuffer)
      .extract({ left: x, top: y, width, height })
      .png()
      .toBuffer()
  }

  /**
   * 应用滤镜
   */
  private async applyFilters(inputBuffer: Buffer, filters: any): Promise<Buffer> {
    let sharpInstance = sharp(inputBuffer)

    if (filters.brightness !== 0) {
      sharpInstance = sharpInstance.modulate({
        brightness: 1 + filters.brightness / 100,
      })
    }

    if (filters.contrast !== 0) {
      sharpInstance = sharpInstance.modulate({
        contrast: 1 + filters.contrast / 100,
      })
    }

    if (filters.saturation !== 0) {
      sharpInstance = sharpInstance.modulate({
        saturation: 1 + filters.saturation / 100,
      })
    }

    if (filters.hue !== 0) {
      sharpInstance = sharpInstance.modulate({
        hue: filters.hue,
      })
    }

    if (filters.blur > 0) {
      sharpInstance = sharpInstance.blur(filters.blur)
    }

    if (filters.sharpen > 0) {
      sharpInstance = sharpInstance.sharpen(filters.sharpen)
    }

    return sharpInstance.png().toBuffer()
  }

  /**
   * 压缩图片
   */
  private async compressImage(inputBuffer: Buffer, compressionOptions: any): Promise<Buffer> {
    const { quality, format, optimize } = compressionOptions

    if (format === 'png') {
      // 使用Rust PNG JS进行PNG优化
      try {
        const optimizedData = await optimizePNG(new Uint8Array(inputBuffer), {
          deflateLevel: 9,
          filterType: 5,
        })
        return Buffer.from(optimizedData)
      } catch (error) {
        logger.warn('PNG优化失败，使用Sharp处理:', error)
        return sharp(inputBuffer)
          .png({ quality: Math.round(quality) })
          .toBuffer()
      }
    } else {
      // 使用Sharp处理其他格式
      const sharpInstance = sharp(inputBuffer)
      
      switch (format) {
        case 'jpg':
        case 'jpeg':
          return sharpInstance
            .jpeg({ quality: Math.round(quality), progressive: optimize })
            .toBuffer()
        case 'webp':
          return sharpInstance
            .webp({ quality: Math.round(quality) })
            .toBuffer()
        case 'avif':
          return sharpInstance
            .avif({ quality: Math.round(quality) })
            .toBuffer()
        default:
          return sharpInstance.png().toBuffer()
      }
    }
  }

  /**
   * 添加水印
   */
  private async addWatermark(inputBuffer: Buffer, watermarkOptions: any): Promise<Buffer> {
    const { text, position, opacity } = watermarkOptions

    // 创建水印文本
    const watermarkSvg = `
      <svg width="200" height="50" xmlns="http://www.w3.org/2000/svg">
        <text x="10" y="30" font-family="Arial" font-size="16" fill="rgba(255,255,255,${opacity})">
          ${text}
        </text>
      </svg>
    `

    const watermarkBuffer = Buffer.from(watermarkSvg)

    // 根据位置计算水印坐标
    const imageInfo = await sharp(inputBuffer).metadata()
    const { width, height } = imageInfo

    let left = 10
    let top = 10

    switch (position) {
      case 'top-right':
        left = (width || 0) - 210
        top = 10
        break
      case 'bottom-left':
        left = 10
        top = (height || 0) - 60
        break
      case 'bottom-right':
        left = (width || 0) - 210
        top = (height || 0) - 60
        break
      case 'center':
        left = ((width || 0) - 200) / 2
        top = ((height || 0) - 50) / 2
        break
    }

    return sharp(inputBuffer)
      .composite([
        {
          input: watermarkBuffer,
          left: Math.max(0, left),
          top: Math.max(0, top),
        },
      ])
      .png()
      .toBuffer()
  }

  /**
   * 批量处理图片
   */
  async processBatch(
    images: Array<{ buffer: Buffer; filename: string }>,
    options: ProcessingOptions
  ): Promise<ProcessingResult[]> {
    const results: ProcessingResult[] = []

    for (const image of images) {
      try {
        const result = await this.processImage(image.buffer, options, image.filename)
        results.push(result)
      } catch (error) {
        logger.error(`批量处理失败: ${image.filename}`, error)
        // 继续处理其他图片
      }
    }

    return results
  }
}
