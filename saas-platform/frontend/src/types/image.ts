export interface ImageFile {
  id: string
  name: string
  size: number
  type: string
  url: string
  thumbnail?: string
  width: number
  height: number
  format: string
  createdAt: Date
  updatedAt: Date
}

export interface ProcessingOptions {
  resize: {
    enabled: boolean
    width: number
    height: number
    maintainAspectRatio: boolean
  }
  crop: {
    enabled: boolean
    x: number
    y: number
    width: number
    height: number
  }
  filters: {
    brightness: number
    contrast: number
    saturation: number
    hue: number
    blur: number
    sharpen: number
  }
  compression: {
    quality: number
    format: 'png' | 'jpg' | 'webp' | 'avif'
    optimize: boolean
  }
  watermark: {
    enabled: boolean
    text: string
    position: 'top-left' | 'top-right' | 'bottom-left' | 'bottom-right' | 'center'
    opacity: number
  }
}

export interface ProcessingResult {
  id: string
  originalImage: ImageFile
  processedImage: ImageFile
  options: ProcessingOptions
  processingTime: number
  fileSizeReduction: number
  createdAt: Date
}

export interface BatchProcessingJob {
  id: string
  name: string
  images: ImageFile[]
  options: ProcessingOptions
  status: 'pending' | 'processing' | 'completed' | 'failed'
  progress: number
  results: ProcessingResult[]
  createdAt: Date
  completedAt?: Date
}

export interface ImageStats {
  totalImages: number
  totalSize: number
  averageProcessingTime: number
  mostUsedFormats: Array<{
    format: string
    count: number
  }>
  processingHistory: Array<{
    date: string
    count: number
  }>
}

export interface ImageFilter {
  id: string
  name: string
  description: string
  category: 'basic' | 'artistic' | 'vintage' | 'modern'
  preview: string
  settings: Record<string, any>
}

export interface ImageTemplate {
  id: string
  name: string
  description: string
  category: 'social' | 'web' | 'print' | 'mobile'
  dimensions: {
    width: number
    height: number
  }
  preview: string
  settings: ProcessingOptions
}
