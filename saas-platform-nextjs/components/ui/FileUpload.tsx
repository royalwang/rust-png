'use client'

import { useCallback, useState, useRef } from 'react'
import { Upload, X, FileImage, AlertCircle, CheckCircle } from 'lucide-react'
import { cn } from '@/lib/utils'
import { Button } from './Button'
import { Progress } from './Progress'
import { Alert, AlertDescription } from './Alert'

interface FileUploadProps {
  onUpload: (files: File[]) => Promise<void>
  maxFiles?: number
  maxSize?: number // in bytes
  acceptedTypes?: string[]
  disabled?: boolean
  className?: string
}

interface UploadProgress {
  file: File
  progress: number
  status: 'pending' | 'uploading' | 'success' | 'error'
  error?: string
}

export function FileUpload({
  onUpload,
  maxFiles = 10,
  maxSize = 50 * 1024 * 1024, // 50MB
  acceptedTypes = ['image/png', 'image/jpeg', 'image/jpg', 'image/gif', 'image/webp', 'image/avif'],
  disabled = false,
  className,
}: FileUploadProps) {
  const [isDragOver, setIsDragOver] = useState(false)
  const [uploadProgress, setUploadProgress] = useState<UploadProgress[]>([])
  const [isUploading, setIsUploading] = useState(false)
  const fileInputRef = useRef<HTMLInputElement>(null)

  const validateFile = useCallback((file: File): string | null => {
    if (file.size > maxSize) {
      return `文件大小不能超过 ${Math.round(maxSize / 1024 / 1024)}MB`
    }
    
    if (acceptedTypes.length > 0 && !acceptedTypes.includes(file.type)) {
      return '不支持的文件类型'
    }
    
    return null
  }, [maxSize, acceptedTypes])

  const handleFiles = useCallback(async (files: FileList) => {
    const fileArray = Array.from(files)
    
    if (fileArray.length > maxFiles) {
      alert(`最多只能上传 ${maxFiles} 个文件`)
      return
    }

    // 验证文件
    const validFiles: File[] = []
    const errors: string[] = []

    fileArray.forEach(file => {
      const error = validateFile(file)
      if (error) {
        errors.push(`${file.name}: ${error}`)
      } else {
        validFiles.push(file)
      }
    })

    if (errors.length > 0) {
      alert(errors.join('\n'))
    }

    if (validFiles.length === 0) return

    // 初始化上传进度
    const progress: UploadProgress[] = validFiles.map(file => ({
      file,
      progress: 0,
      status: 'pending'
    }))
    setUploadProgress(progress)

    try {
      setIsUploading(true)
      await onUpload(validFiles)
      
      // 更新状态为成功
      setUploadProgress(prev => prev.map(p => ({ ...p, status: 'success' as const })))
    } catch (error) {
      // 更新状态为错误
      setUploadProgress(prev => prev.map(p => ({ 
        ...p, 
        status: 'error' as const, 
        error: error instanceof Error ? error.message : '上传失败'
      })))
    } finally {
      setIsUploading(false)
    }
  }, [maxFiles, validateFile, onUpload])

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault()
    if (!disabled) {
      setIsDragOver(true)
    }
  }, [disabled])

  const handleDragLeave = useCallback((e: React.DragEvent) => {
    e.preventDefault()
    setIsDragOver(false)
  }, [])

  const handleDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault()
    setIsDragOver(false)
    
    if (disabled) return
    
    const files = e.dataTransfer.files
    if (files.length > 0) {
      handleFiles(files)
    }
  }, [disabled, handleFiles])

  const handleFileInput = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files
    if (files && files.length > 0) {
      handleFiles(files)
    }
  }, [handleFiles])

  const handleClick = useCallback(() => {
    if (!disabled && fileInputRef.current) {
      fileInputRef.current.click()
    }
  }, [disabled])

  const removeFile = useCallback((index: number) => {
    setUploadProgress(prev => prev.filter((_, i) => i !== index))
  }, [])

  const clearAll = useCallback(() => {
    setUploadProgress([])
  }, [])

  const getStatusIcon = (status: UploadProgress['status']) => {
    switch (status) {
      case 'success':
        return <CheckCircle className="w-4 h-4 text-green-500" />
      case 'error':
        return <AlertCircle className="w-4 h-4 text-red-500" />
      case 'uploading':
        return <div className="w-4 h-4 border-2 border-blue-500 border-t-transparent rounded-full animate-spin" />
      default:
        return <FileImage className="w-4 h-4 text-gray-400" />
    }
  }

  const getStatusText = (status: UploadProgress['status']) => {
    switch (status) {
      case 'success':
        return '上传成功'
      case 'error':
        return '上传失败'
      case 'uploading':
        return '上传中...'
      default:
        return '等待上传'
    }
  }

  return (
    <div className={cn('w-full', className)}>
      {/* 拖拽上传区域 */}
      <div
        className={cn(
          'relative border-2 border-dashed rounded-lg p-8 text-center transition-colors',
          isDragOver ? 'border-blue-500 bg-blue-50' : 'border-gray-300',
          disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer hover:border-gray-400'
        )}
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        onDrop={handleDrop}
        onClick={handleClick}
      >
        <input
          ref={fileInputRef}
          type="file"
          accept={acceptedTypes.join(',')}
          multiple={maxFiles > 1}
          onChange={handleFileInput}
          className="hidden"
          disabled={disabled}
        />
        
        <div className="flex flex-col items-center space-y-4">
          <Upload className={`w-12 h-12 ${isDragOver ? 'text-blue-500' : 'text-gray-400'}`} />
          
          <div>
            <p className="text-lg font-medium text-gray-900">
              {isDragOver ? '释放文件开始上传' : '拖拽文件到此处或点击选择文件'}
            </p>
            <p className="text-sm text-gray-500 mt-1">
              支持 {acceptedTypes.join(', ')} 格式，单个文件最大 {Math.round(maxSize / 1024 / 1024)}MB
              {maxFiles > 1 && `，最多 ${maxFiles} 个文件`}
            </p>
          </div>
          
          <Button
            type="button"
            variant="outline"
            disabled={disabled}
            className="mt-4"
          >
            选择文件
          </Button>
        </div>
      </div>

      {/* 上传进度 */}
      {uploadProgress.length > 0 && (
        <div className="mt-6 space-y-4">
          <div className="flex items-center justify-between">
            <h3 className="text-lg font-medium">上传进度</h3>
            <Button
              type="button"
              variant="outline"
              size="sm"
              onClick={clearAll}
              disabled={isUploading}
            >
              清空
            </Button>
          </div>
          
          <div className="space-y-3">
            {uploadProgress.map((progress, index) => (
              <div key={index} className="border rounded-lg p-4">
                <div className="flex items-center justify-between mb-2">
                  <div className="flex items-center space-x-2">
                    {getStatusIcon(progress.status)}
                    <span className="font-medium text-sm">{progress.file.name}</span>
                    <span className="text-xs text-gray-500">
                      ({Math.round(progress.file.size / 1024)}KB)
                    </span>
                  </div>
                  
                  <div className="flex items-center space-x-2">
                    <span className="text-sm text-gray-600">
                      {getStatusText(progress.status)}
                    </span>
                    <Button
                      type="button"
                      variant="ghost"
                      size="sm"
                      onClick={() => removeFile(index)}
                      disabled={isUploading}
                    >
                      <X className="w-4 h-4" />
                    </Button>
                  </div>
                </div>
                
                {progress.status === 'uploading' && (
                  <Progress value={progress.progress} className="h-2" />
                )}
                
                {progress.status === 'error' && progress.error && (
                  <Alert variant="destructive" className="mt-2">
                    <AlertCircle className="h-4 w-4" />
                    <AlertDescription>{progress.error}</AlertDescription>
                  </Alert>
                )}
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  )
}
