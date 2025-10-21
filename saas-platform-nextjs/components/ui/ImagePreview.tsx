'use client'

import { useState } from 'react'
import { X, Download, Eye, Trash2, RotateCw } from 'lucide-react'
import { cn } from '@/lib/utils'
import { Button } from './Button'
import { Badge } from './Badge'
import { formatBytes } from '@/lib/utils'

interface ImagePreviewProps {
  file: File
  onRemove?: () => void
  onDownload?: () => void
  onRotate?: () => void
  className?: string
  showActions?: boolean
}

export function ImagePreview({
  file,
  onRemove,
  onDownload,
  onRotate,
  className,
  showActions = true,
}: ImagePreviewProps) {
  const [imageError, setImageError] = useState(false)
  const [imageLoaded, setImageLoaded] = useState(false)

  const handleImageLoad = () => {
    setImageLoaded(true)
  }

  const handleImageError = () => {
    setImageError(true)
  }

  return (
    <div className={cn('relative group', className)}>
      <div className="relative overflow-hidden rounded-lg border bg-card">
        {!imageError ? (
          <div className="relative">
            <img
              src={URL.createObjectURL(file)}
              alt={file.name}
              className={cn(
                'w-full h-48 object-cover transition-opacity duration-200',
                imageLoaded ? 'opacity-100' : 'opacity-0'
              )}
              onLoad={handleImageLoad}
              onError={handleImageError}
            />
            {!imageLoaded && (
              <div className="absolute inset-0 flex items-center justify-center bg-muted">
                <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary"></div>
              </div>
            )}
          </div>
        ) : (
          <div className="w-full h-48 flex items-center justify-center bg-muted">
            <div className="text-center">
              <div className="w-12 h-12 mx-auto mb-2 rounded-full bg-destructive/10 flex items-center justify-center">
                <X className="w-6 h-6 text-destructive" />
              </div>
              <p className="text-sm text-muted-foreground">图片加载失败</p>
            </div>
          </div>
        )}

        {/* 文件信息覆盖层 */}
        <div className="absolute inset-0 bg-black/50 opacity-0 group-hover:opacity-100 transition-opacity duration-200">
          <div className="absolute bottom-0 left-0 right-0 p-4 text-white">
            <div className="space-y-1">
              <p className="text-sm font-medium truncate">{file.name}</p>
              <div className="flex items-center space-x-2 text-xs">
                <Badge variant="secondary" className="text-xs">
                  {file.type.split('/')[1].toUpperCase()}
                </Badge>
                <span>{formatBytes(file.size)}</span>
              </div>
            </div>
          </div>
        </div>

        {/* 操作按钮 */}
        {showActions && (
          <div className="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity duration-200">
            <div className="flex space-x-1">
              {onDownload && (
                <Button
                  size="icon"
                  variant="secondary"
                  className="h-8 w-8"
                  onClick={onDownload}
                >
                  <Download className="h-4 w-4" />
                </Button>
              )}
              {onRotate && (
                <Button
                  size="icon"
                  variant="secondary"
                  className="h-8 w-8"
                  onClick={onRotate}
                >
                  <RotateCw className="h-4 w-4" />
                </Button>
              )}
              {onRemove && (
                <Button
                  size="icon"
                  variant="destructive"
                  className="h-8 w-8"
                  onClick={onRemove}
                >
                  <X className="h-4 w-4" />
                </Button>
              )}
            </div>
          </div>
        )}
      </div>

      {/* 文件详情 */}
      <div className="mt-2 space-y-1">
        <p className="text-sm font-medium truncate">{file.name}</p>
        <div className="flex items-center justify-between text-xs text-muted-foreground">
          <span>{formatBytes(file.size)}</span>
          <Badge variant="outline" className="text-xs">
            {file.type.split('/')[1].toUpperCase()}
          </Badge>
        </div>
      </div>
    </div>
  )
}
