import { create } from 'zustand'
import { ImageFile, ProcessingOptions, ProcessingResult } from '@/types/image'

interface ImageState {
  // 当前处理的图片
  currentImage: ImageFile | null
  processingOptions: ProcessingOptions
  
  // 处理历史
  processingHistory: ProcessingResult[]
  
  // 状态
  isProcessing: boolean
  progress: number
  error: string | null
  
  // Actions
  setCurrentImage: (image: ImageFile | null) => void
  updateProcessingOptions: (options: Partial<ProcessingOptions>) => void
  startProcessing: () => void
  setProgress: (progress: number) => void
  completeProcessing: (result: ProcessingResult) => void
  setError: (error: string | null) => void
  addToHistory: (result: ProcessingResult) => void
  clearHistory: () => void
  removeFromHistory: (id: string) => void
}

export const useImageStore = create<ImageState>((set, get) => ({
  currentImage: null,
  processingOptions: {
    resize: {
      enabled: false,
      width: 0,
      height: 0,
      maintainAspectRatio: true,
    },
    crop: {
      enabled: false,
      x: 0,
      y: 0,
      width: 0,
      height: 0,
    },
    filters: {
      brightness: 0,
      contrast: 0,
      saturation: 0,
      hue: 0,
      blur: 0,
      sharpen: 0,
    },
    compression: {
      quality: 90,
      format: 'png',
      optimize: true,
    },
    watermark: {
      enabled: false,
      text: '',
      position: 'bottom-right',
      opacity: 0.5,
    },
  },
  processingHistory: [],
  isProcessing: false,
  progress: 0,
  error: null,

  setCurrentImage: (image) => {
    set({ currentImage: image, error: null })
  },

  updateProcessingOptions: (options) => {
    set((state) => ({
      processingOptions: {
        ...state.processingOptions,
        ...options,
      },
    }))
  },

  startProcessing: () => {
    set({ isProcessing: true, progress: 0, error: null })
  },

  setProgress: (progress) => {
    set({ progress: Math.min(100, Math.max(0, progress)) })
  },

  completeProcessing: (result) => {
    set({
      isProcessing: false,
      progress: 100,
      error: null,
    })
    
    // 添加到历史记录
    get().addToHistory(result)
  },

  setError: (error) => {
    set({ error, isProcessing: false, progress: 0 })
  },

  addToHistory: (result) => {
    set((state) => ({
      processingHistory: [result, ...state.processingHistory].slice(0, 50), // 保留最近50条记录
    }))
  },

  clearHistory: () => {
    set({ processingHistory: [] })
  },

  removeFromHistory: (id) => {
    set((state) => ({
      processingHistory: state.processingHistory.filter((item) => item.id !== id),
    }))
  },
}))
