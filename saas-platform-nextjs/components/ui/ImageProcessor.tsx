'use client'

import { useState } from 'react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from './Card'
import { Button } from './Button'
import { Badge } from './Badge'
import { Progress } from './Progress'
import { Alert } from './Alert'
import { 
  Settings, 
  RotateCw, 
  Crop, 
  Palette, 
  Zap, 
  Download,
  Eye,
  Clock,
  CheckCircle,
  AlertCircle
} from 'lucide-react'

interface ProcessingOperation {
  id: string
  name: string
  description: string
  icon: React.ComponentType<{ className?: string }>
  enabled: boolean
  parameters?: Record<string, any>
}

interface ImageProcessorProps {
  imageId: string
  imageUrl: string
  imageName: string
  onProcessingComplete?: (result: any) => void
  className?: string
}

export function ImageProcessor({
  imageId,
  imageUrl,
  imageName,
  onProcessingComplete,
  className = ''
}: ImageProcessorProps) {
  const [selectedOperations, setSelectedOperations] = useState<string[]>([])
  const [isProcessing, setIsProcessing] = useState(false)
  const [processingProgress, setProcessingProgress] = useState(0)
  const [processingStatus, setProcessingStatus] = useState<'idle' | 'processing' | 'completed' | 'error'>('idle')
  const [processingResult, setProcessingResult] = useState<any>(null)
  const [error, setError] = useState<string | null>(null)

  const operations: ProcessingOperation[] = [
    {
      id: 'optimize',
      name: '优化压缩',
      description: '自动优化图片大小和质量',
      icon: Zap,
      enabled: true
    },
    {
      id: 'resize',
      name: '调整尺寸',
      description: '调整图片尺寸',
      icon: Crop,
      enabled: true,
      parameters: {
        width: 1920,
        height: 1080,
        maintainAspectRatio: true
      }
    },
    {
      id: 'rotate',
      name: '旋转',
      description: '旋转图片角度',
      icon: RotateCw,
      enabled: true,
      parameters: {
        angle: 90
      }
    },
    {
      id: 'filter',
      name: '滤镜效果',
      description: '应用各种滤镜效果',
      icon: Palette,
      enabled: true,
      parameters: {
        filter: 'none'
      }
    }
  ]

  const handleOperationToggle = (operationId: string) => {
    setSelectedOperations(prev => 
      prev.includes(operationId)
        ? prev.filter(id => id !== operationId)
        : [...prev, operationId]
    )
  }

  const handleStartProcessing = async () => {
    if (selectedOperations.length === 0) {
      setError('请选择至少一个处理操作')
      return
    }

    setIsProcessing(true)
    setProcessingProgress(0)
    setProcessingStatus('processing')
    setError(null)

    try {
      // 模拟处理进度
      const progressInterval = setInterval(() => {
        setProcessingProgress(prev => {
          if (prev >= 90) {
            clearInterval(progressInterval)
            return 90
          }
          return prev + 10
        })
      }, 200)

      // 调用处理API
      const response = await fetch('/api/process', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          imageId,
          operations: selectedOperations.map(id => {
            const operation = operations.find(op => op.id === id)
            return {
              id,
              parameters: operation?.parameters || {}
            }
          })
        })
      })

      if (!response.ok) {
        throw new Error('处理失败')
      }

      const result = await response.json()
      
      clearInterval(progressInterval)
      setProcessingProgress(100)
      setProcessingStatus('completed')
      setProcessingResult(result)
      
      if (onProcessingComplete) {
        onProcessingComplete(result)
      }

    } catch (error) {
      setProcessingStatus('error')
      setError(error instanceof Error ? error.message : '处理失败')
    } finally {
      setIsProcessing(false)
    }
  }

  const getStatusIcon = () => {
    switch (processingStatus) {
      case 'completed':
        return <CheckCircle className="w-5 h-5 text-green-500" />
      case 'error':
        return <AlertCircle className="w-5 h-5 text-red-500" />
      case 'processing':
        return <Clock className="w-5 h-5 text-blue-500 animate-spin" />
      default:
        return <Settings className="w-5 h-5 text-gray-500" />
    }
  }

  const getStatusText = () => {
    switch (processingStatus) {
      case 'completed':
        return '处理完成'
      case 'error':
        return '处理失败'
      case 'processing':
        return '处理中...'
      default:
        return '准备处理'
    }
  }

  return (
    <div className={`space-y-6 ${className}`}>
      {/* 图片预览 */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center space-x-2">
            <Eye className="w-5 h-5" />
            <span>图片预览</span>
          </CardTitle>
          <CardDescription>{imageName}</CardDescription>
        </CardHeader>
        <CardContent>
          <div className="aspect-video bg-gray-100 rounded-lg overflow-hidden">
            <img
              src={imageUrl}
              alt={imageName}
              className="w-full h-full object-cover"
            />
          </div>
        </CardContent>
      </Card>

      {/* 处理选项 */}
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center space-x-2">
            <Settings className="w-5 h-5" />
            <span>处理选项</span>
          </CardTitle>
          <CardDescription>
            选择您想要应用的图片处理操作
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {operations.map((operation) => {
              const Icon = operation.icon
              const isSelected = selectedOperations.includes(operation.id)
              
              return (
                <div
                  key={operation.id}
                  className={`
                    p-4 border rounded-lg cursor-pointer transition-all
                    ${isSelected 
                      ? 'border-blue-500 bg-blue-50' 
                      : 'border-gray-200 hover:border-gray-300'
                    }
                    ${!operation.enabled ? 'opacity-50 cursor-not-allowed' : ''}
                  `}
                  onClick={() => operation.enabled && handleOperationToggle(operation.id)}
                >
                  <div className="flex items-center space-x-3">
                    <div className={`
                      p-2 rounded-lg
                      ${isSelected ? 'bg-blue-100' : 'bg-gray-100'}
                    `}>
                      <Icon className={`
                        w-5 h-5
                        ${isSelected ? 'text-blue-600' : 'text-gray-600'}
                      `} />
                    </div>
                    <div className="flex-1">
                      <h3 className="font-medium text-gray-900">
                        {operation.name}
                      </h3>
                      <p className="text-sm text-gray-600">
                        {operation.description}
                      </p>
                    </div>
                    {isSelected && (
                      <CheckCircle className="w-5 h-5 text-blue-500" />
                    )}
                  </div>
                </div>
              )
            })}
          </div>
        </CardContent>
      </Card>

      {/* 处理状态 */}
      {processingStatus !== 'idle' && (
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center space-x-2">
              {getStatusIcon()}
              <span>处理状态</span>
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <span className="font-medium">{getStatusText()}</span>
                <Badge variant={
                  processingStatus === 'completed' ? 'success' :
                  processingStatus === 'error' ? 'destructive' :
                  'secondary'
                }>
                  {processingStatus === 'completed' ? '完成' :
                   processingStatus === 'error' ? '失败' :
                   processingStatus === 'processing' ? '处理中' : '准备'}
                </Badge>
              </div>
              
              {processingStatus === 'processing' && (
                <div className="space-y-2">
                  <div className="flex items-center justify-between text-sm">
                    <span>处理进度</span>
                    <span>{processingProgress}%</span>
                  </div>
                  <Progress value={processingProgress} className="h-2" />
                </div>
              )}

              {error && (
                <Alert variant="destructive">
                  <AlertCircle className="h-4 w-4" />
                  <span>{error}</span>
                </Alert>
              )}

              {processingStatus === 'completed' && processingResult && (
                <div className="space-y-4">
                  <div className="p-4 bg-green-50 border border-green-200 rounded-lg">
                    <div className="flex items-center space-x-2 text-green-800">
                      <CheckCircle className="w-5 h-5" />
                      <span className="font-medium">处理完成！</span>
                    </div>
                    <p className="text-sm text-green-700 mt-1">
                      您的图片已成功处理，可以下载或查看结果。
                    </p>
                  </div>
                  
                  <div className="flex space-x-2">
                    <Button className="flex items-center space-x-2">
                      <Download className="w-4 h-4" />
                      <span>下载结果</span>
                    </Button>
                    <Button variant="outline" className="flex items-center space-x-2">
                      <Eye className="w-4 h-4" />
                      <span>查看结果</span>
                    </Button>
                  </div>
                </div>
              )}
            </div>
          </CardContent>
        </Card>
      )}

      {/* 操作按钮 */}
      <div className="flex justify-end space-x-2">
        <Button
          variant="outline"
          onClick={() => {
            setSelectedOperations([])
            setProcessingStatus('idle')
            setProcessingResult(null)
            setError(null)
          }}
        >
          重置
        </Button>
        <Button
          onClick={handleStartProcessing}
          disabled={isProcessing || selectedOperations.length === 0}
          className="flex items-center space-x-2"
        >
          {isProcessing ? (
            <>
              <Clock className="w-4 h-4 animate-spin" />
              <span>处理中...</span>
            </>
          ) : (
            <>
              <Settings className="w-4 h-4" />
              <span>开始处理</span>
            </>
          )}
        </Button>
      </div>
    </div>
  )
}
