'use client'

import { useState, useEffect } from 'react'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/Card'
import { Button } from '@/components/ui/Button'
import { Badge } from '@/components/ui/Badge'
import { Progress } from '@/components/ui/Progress'
import { FileUpload } from '@/components/ui/FileUpload'
import { ImagePreview } from '@/components/ui/ImagePreview'
import {
  Upload,
  Image as ImageIcon,
  Clock,
  CheckCircle,
  AlertCircle,
  BarChart3,
  Settings,
  Plus,
  Filter,
  Search,
  Grid,
  List,
} from 'lucide-react'
import { formatBytes, formatDate } from '@/lib/utils'
import Link from 'next/link'

interface ImageFile {
  id: string
  name: string
  size: number
  type: string
  url: string
  width: number
  height: number
  format: string
  createdAt: string
  status: 'pending' | 'processing' | 'completed' | 'failed'
}

interface ProcessingResult {
  id: string
  originalImage: ImageFile
  processedImage: ImageFile
  processingTime: number
  fileSizeReduction: number
  createdAt: string
}

export function DashboardPage() {
  const [uploadedFiles, setUploadedFiles] = useState<File[]>([])
  const [images, setImages] = useState<ImageFile[]>([])
  const [processingResults, setProcessingResults] = useState<ProcessingResult[]>([])
  const [viewMode, setViewMode] = useState<'grid' | 'list'>('grid')
  const [isProcessing, setIsProcessing] = useState(false)
  const [processingProgress, setProcessingProgress] = useState(0)

  // 模拟数据
  useEffect(() => {
    // 模拟图片数据
    setImages([
      {
        id: '1',
        name: 'sample1.jpg',
        size: 1024000,
        type: 'image/jpeg',
        url: '/api/placeholder/400/300',
        width: 1920,
        height: 1080,
        format: 'jpg',
        createdAt: new Date().toISOString(),
        status: 'completed',
      },
      {
        id: '2',
        name: 'sample2.png',
        size: 2048000,
        type: 'image/png',
        url: '/api/placeholder/400/300',
        width: 1200,
        height: 800,
        format: 'png',
        createdAt: new Date().toISOString(),
        status: 'processing',
      },
    ])

    // 模拟处理结果
    setProcessingResults([
      {
        id: '1',
        originalImage: {
          id: '1',
          name: 'original.jpg',
          size: 1024000,
          type: 'image/jpeg',
          url: '/api/placeholder/400/300',
          width: 1920,
          height: 1080,
          format: 'jpg',
          createdAt: new Date().toISOString(),
          status: 'completed',
        },
        processedImage: {
          id: '2',
          name: 'processed.jpg',
          size: 512000,
          type: 'image/jpeg',
          url: '/api/placeholder/400/300',
          width: 1920,
          height: 1080,
          format: 'jpg',
          createdAt: new Date().toISOString(),
          status: 'completed',
        },
        processingTime: 1500,
        fileSizeReduction: 50,
        createdAt: new Date().toISOString(),
      },
    ])
  }, [])

  const handleFileUpload = async (files: File[]) => {
    try {
      const formData = new FormData()
      files.forEach(file => {
        formData.append('files', file)
      })

      const response = await fetch('/api/upload', {
        method: 'POST',
        body: formData
      })

      if (!response.ok) {
        throw new Error('上传失败')
      }

      const result = await response.json()
      console.log('上传成功:', result)
      
      // 刷新数据
      window.location.reload()
    } catch (error) {
      console.error('上传失败:', error)
      throw error
    }
  }

  const handleFileSelect = (files: File[]) => {
    setUploadedFiles(prev => [...prev, ...files])
  }

  const handleRemoveFile = (index: number) => {
    setUploadedFiles(prev => prev.filter((_, i) => i !== index))
  }

  const handleStartProcessing = async () => {
    setIsProcessing(true)
    setProcessingProgress(0)

    // 模拟处理进度
    const interval = setInterval(() => {
      setProcessingProgress(prev => {
        if (prev >= 100) {
          clearInterval(interval)
          setIsProcessing(false)
          setUploadedFiles([])
          return 100
        }
        return prev + 10
      })
    }, 200)
  }

  const stats = [
    {
      title: '总图片数',
      value: images.length,
      icon: ImageIcon,
      color: 'text-blue-600',
      bgColor: 'bg-blue-100',
    },
    {
      title: '已处理',
      value: processingResults.length,
      icon: CheckCircle,
      color: 'text-green-600',
      bgColor: 'bg-green-100',
    },
    {
      title: '处理中',
      value: images.filter(img => img.status === 'processing').length,
      icon: Clock,
      color: 'text-yellow-600',
      bgColor: 'bg-yellow-100',
    },
    {
      title: '存储使用',
      value: '2.5 GB',
      icon: BarChart3,
      color: 'text-purple-600',
      bgColor: 'bg-purple-100',
    },
  ]

  return (
    <div className="space-y-6">
      {/* 欢迎区域 */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold">欢迎回来！</h1>
          <p className="text-muted-foreground">
            开始处理您的图片，体验高性能的图片处理服务
          </p>
        </div>
        <div className="flex items-center space-x-2">
          <Button variant="outline" size="sm">
            <Settings className="h-4 w-4 mr-2" />
            设置
          </Button>
          <Button size="sm">
            <Plus className="h-4 w-4 mr-2" />
            新建项目
          </Button>
        </div>
      </div>

      {/* 统计卡片 */}
      <div className="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-4">
        {stats.map((stat, index) => {
          const Icon = stat.icon
          return (
            <Card key={index}>
              <CardContent className="p-6">
                <div className="flex items-center">
                  <div className={`p-2 rounded-lg ${stat.bgColor}`}>
                    <Icon className={`h-6 w-6 ${stat.color}`} />
                  </div>
                  <div className="ml-4">
                    <p className="text-sm font-medium text-muted-foreground">
                      {stat.title}
                    </p>
                    <p className="text-2xl font-bold">{stat.value}</p>
                  </div>
                </div>
              </CardContent>
            </Card>
          )
        })}
      </div>

      {/* 文件上传区域 */}
      <Card>
        <CardHeader>
          <CardTitle>上传图片</CardTitle>
          <CardDescription>
            拖拽图片到下方区域，或点击选择文件开始处理
          </CardDescription>
        </CardHeader>
        <CardContent>
          <FileUpload
            onUpload={handleFileUpload}
            maxFiles={10}
            maxSize={50 * 1024 * 1024}
            className="mb-6"
          />

          {/* 上传的图片预览 */}
          {uploadedFiles.length > 0 && (
            <div className="space-y-4">
              <div className="flex items-center justify-between">
                <h3 className="text-lg font-semibold">已选择的图片</h3>
                <div className="flex items-center space-x-2">
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => setUploadedFiles([])}
                  >
                    清空
                  </Button>
                  <Button
                    size="sm"
                    onClick={handleStartProcessing}
                    disabled={isProcessing}
                  >
                    {isProcessing ? (
                      <>
                        <Clock className="mr-2 h-4 w-4 animate-spin" />
                        处理中...
                      </>
                    ) : (
                      <>
                        <Upload className="mr-2 h-4 w-4" />
                        开始处理
                      </>
                    )}
                  </Button>
                </div>
              </div>

              {isProcessing && (
                <div className="space-y-2">
                  <div className="flex items-center justify-between text-sm">
                    <span>处理进度</span>
                    <span>{processingProgress}%</span>
                  </div>
                  <Progress value={processingProgress} className="h-2" />
                </div>
              )}

              <div className="grid grid-cols-2 gap-4 sm:grid-cols-3 lg:grid-cols-4">
                {uploadedFiles.map((file, index) => (
                  <ImagePreview
                    key={index}
                    file={file}
                    onRemove={() => handleRemoveFile(index)}
                    className="w-full"
                  />
                ))}
              </div>
            </div>
          )}
        </CardContent>
      </Card>

      {/* 最近图片 */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <div>
              <CardTitle>最近图片</CardTitle>
              <CardDescription>
                查看您最近上传和处理的图片
              </CardDescription>
            </div>
            <div className="flex items-center space-x-2">
              <Button variant="outline" size="sm">
                <Filter className="h-4 w-4 mr-2" />
                筛选
              </Button>
              <Button variant="outline" size="sm">
                <Search className="h-4 w-4 mr-2" />
                搜索
              </Button>
              <div className="flex border rounded-md">
                <Button
                  variant={viewMode === 'grid' ? 'default' : 'ghost'}
                  size="sm"
                  onClick={() => setViewMode('grid')}
                >
                  <Grid className="h-4 w-4" />
                </Button>
                <Button
                  variant={viewMode === 'list' ? 'default' : 'ghost'}
                  size="sm"
                  onClick={() => setViewMode('list')}
                >
                  <List className="h-4 w-4" />
                </Button>
              </div>
            </div>
          </div>
        </CardHeader>
        <CardContent>
          {images.length === 0 ? (
            <div className="text-center py-12">
              <ImageIcon className="mx-auto h-12 w-12 text-muted-foreground" />
              <h3 className="mt-2 text-sm font-semibold">暂无图片</h3>
              <p className="mt-1 text-sm text-muted-foreground">
                开始上传您的第一张图片
              </p>
            </div>
          ) : (
            <div className={cn(
              'grid gap-4',
              viewMode === 'grid' 
                ? 'grid-cols-2 sm:grid-cols-3 lg:grid-cols-4' 
                : 'grid-cols-1'
            )}>
              {images.map((image) => (
                <Card key={image.id} className="overflow-hidden">
                  <div className="aspect-video bg-muted">
                    <img
                      src={image.url}
                      alt={image.name}
                      className="w-full h-full object-cover"
                    />
                  </div>
                  <CardContent className="p-4">
                    <div className="space-y-2">
                      <div className="flex items-center justify-between">
                        <h4 className="font-medium truncate">{image.name}</h4>
                        <Badge variant={
                          image.status === 'completed' ? 'default' :
                          image.status === 'processing' ? 'secondary' :
                          'destructive'
                        }>
                          {image.status === 'completed' ? '已完成' :
                           image.status === 'processing' ? '处理中' : '失败'}
                        </Badge>
                      </div>
                      <div className="flex items-center justify-between text-sm text-muted-foreground">
                        <span>{formatBytes(image.size)}</span>
                        <span>{image.width}×{image.height}</span>
                      </div>
                      <p className="text-xs text-muted-foreground">
                        {formatDate(image.createdAt)}
                      </p>
                      <div className="flex space-x-2 pt-2">
                        <Link href={`/process/${image.id}`}>
                          <Button size="sm" variant="outline" className="w-full">
                            <Settings className="w-4 h-4 mr-2" />
                            处理
                          </Button>
                        </Link>
                        <Button size="sm" variant="outline" className="w-full">
                          <Download className="w-4 h-4 mr-2" />
                          下载
                        </Button>
                      </div>
                    </div>
                  </CardContent>
                </Card>
              ))}
            </div>
          )}
        </CardContent>
      </Card>

      {/* 处理历史 */}
      <Card>
        <CardHeader>
          <CardTitle>处理历史</CardTitle>
          <CardDescription>
            查看您的图片处理历史和结果
          </CardDescription>
        </CardHeader>
        <CardContent>
          {processingResults.length === 0 ? (
            <div className="text-center py-12">
              <BarChart3 className="mx-auto h-12 w-12 text-muted-foreground" />
              <h3 className="mt-2 text-sm font-semibold">暂无处理历史</h3>
              <p className="mt-1 text-sm text-muted-foreground">
                开始处理图片以查看历史记录
              </p>
            </div>
          ) : (
            <div className="space-y-4">
              {processingResults.map((result) => (
                <Card key={result.id} className="p-4">
                  <div className="flex items-center space-x-4">
                    <div className="flex-shrink-0">
                      <img
                        src={result.originalImage.url}
                        alt={result.originalImage.name}
                        className="h-16 w-16 rounded-lg object-cover"
                      />
                    </div>
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center justify-between">
                        <h4 className="font-medium truncate">
                          {result.originalImage.name}
                        </h4>
                        <Badge variant="default">已完成</Badge>
                      </div>
                      <div className="mt-1 flex items-center space-x-4 text-sm text-muted-foreground">
                        <span>处理时间: {result.processingTime}ms</span>
                        <span>文件减少: {result.fileSizeReduction}%</span>
                        <span>{formatDate(result.createdAt)}</span>
                      </div>
                    </div>
                    <div className="flex-shrink-0">
                      <Button variant="outline" size="sm">
                        查看结果
                      </Button>
                    </div>
                  </div>
                </Card>
              ))}
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  )
}
