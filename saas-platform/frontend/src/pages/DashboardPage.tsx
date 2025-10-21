import { useState, useEffect } from 'react'
import { Helmet } from 'react-helmet-async'
import { 
  Upload, 
  Image as ImageIcon, 
  Settings, 
  BarChart3, 
  Clock, 
  Download,
  Trash2,
  Edit,
  Eye,
  Filter,
  Zap
} from 'lucide-react'
import { useImageStore } from '../store/imageStore'
import { useAuthStore } from '../store/authStore'
import toast from 'react-hot-toast'

export function DashboardPage() {
  const [activeTab, setActiveTab] = useState<'images' | 'processing' | 'stats'>('images')
  const [selectedImages, setSelectedImages] = useState<string[]>([])
  const { user } = useAuthStore()
  const { 
    images, 
    processingResults, 
    stats, 
    isLoading, 
    fetchImages, 
    fetchProcessingResults, 
    fetchStats,
    deleteImage,
    deleteBatch
  } = useImageStore()

  useEffect(() => {
    fetchImages()
    fetchProcessingResults()
    fetchStats()
  }, [])

  const handleImageSelect = (imageId: string) => {
    setSelectedImages(prev => 
      prev.includes(imageId) 
        ? prev.filter(id => id !== imageId)
        : [...prev, imageId]
    )
  }

  const handleSelectAll = () => {
    if (selectedImages.length === images.length) {
      setSelectedImages([])
    } else {
      setSelectedImages(images.map(img => img.id))
    }
  }

  const handleDeleteSelected = async () => {
    if (selectedImages.length === 0) return

    try {
      if (selectedImages.length === 1) {
        await deleteImage(selectedImages[0])
        toast.success('图片删除成功')
      } else {
        await deleteBatch(selectedImages)
        toast.success(`批量删除 ${selectedImages.length} 张图片成功`)
      }
      setSelectedImages([])
      fetchImages()
    } catch (error) {
      toast.error('删除失败，请重试')
    }
  }

  const formatFileSize = (bytes: number) => {
    if (bytes === 0) return '0 Bytes'
    const k = 1024
    const sizes = ['Bytes', 'KB', 'MB', 'GB']
    const i = Math.floor(Math.log(bytes) / Math.log(k))
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
  }

  const formatDate = (date: string) => {
    return new Date(date).toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    })
  }

  return (
    <>
      <Helmet>
        <title>控制台 - Rust PNG SaaS Platform</title>
        <meta name="description" content="管理您的图片和处理任务" />
      </Helmet>

      <div className="min-h-screen bg-gray-50">
        {/* 顶部导航 */}
        <div className="bg-white border-b border-gray-200">
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div className="flex justify-between items-center h-16">
              <div className="flex items-center">
                <h1 className="text-2xl font-bold gradient-text">Rust PNG SaaS</h1>
              </div>
              <div className="flex items-center space-x-4">
                <div className="flex items-center space-x-2">
                  <div className="w-8 h-8 bg-primary-500 rounded-full flex items-center justify-center text-white text-sm font-semibold">
                    {user?.name?.charAt(0).toUpperCase()}
                  </div>
                  <span className="text-sm font-medium text-gray-700">{user?.name}</span>
                </div>
                <button className="btn btn-ghost btn-sm">设置</button>
              </div>
            </div>
          </div>
        </div>

        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
          {/* 欢迎区域 */}
          <div className="mb-8">
            <h2 className="text-3xl font-bold text-gray-900 mb-2">
              欢迎回来，{user?.name}！
            </h2>
            <p className="text-gray-600">
              管理您的图片和处理任务，享受高性能的图片处理服务。
            </p>
          </div>

          {/* 统计卡片 */}
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
            <div className="card p-6">
              <div className="flex items-center">
                <div className="p-3 bg-blue-100 rounded-full">
                  <ImageIcon className="h-6 w-6 text-blue-600" />
                </div>
                <div className="ml-4">
                  <p className="text-sm font-medium text-gray-600">总图片数</p>
                  <p className="text-2xl font-bold text-gray-900">{stats?.totalImages || 0}</p>
                </div>
              </div>
            </div>

            <div className="card p-6">
              <div className="flex items-center">
                <div className="p-3 bg-green-100 rounded-full">
                  <Zap className="h-6 w-6 text-green-600" />
                </div>
                <div className="ml-4">
                  <p className="text-sm font-medium text-gray-600">已处理</p>
                  <p className="text-2xl font-bold text-gray-900">{stats?.totalProcessed || 0}</p>
                </div>
              </div>
            </div>

            <div className="card p-6">
              <div className="flex items-center">
                <div className="p-3 bg-purple-100 rounded-full">
                  <BarChart3 className="h-6 w-6 text-purple-600" />
                </div>
                <div className="ml-4">
                  <p className="text-sm font-medium text-gray-600">存储使用</p>
                  <p className="text-2xl font-bold text-gray-900">
                    {formatFileSize(stats?.totalSize || 0)}
                  </p>
                </div>
              </div>
            </div>

            <div className="card p-6">
              <div className="flex items-center">
                <div className="p-3 bg-orange-100 rounded-full">
                  <Clock className="h-6 w-6 text-orange-600" />
                </div>
                <div className="ml-4">
                  <p className="text-sm font-medium text-gray-600">平均处理时间</p>
                  <p className="text-2xl font-bold text-gray-900">
                    {stats?.averageProcessingTime ? `${Math.round(stats.averageProcessingTime)}ms` : '0ms'}
                  </p>
                </div>
              </div>
            </div>
          </div>

          {/* 标签页 */}
          <div className="mb-6">
            <div className="border-b border-gray-200">
              <nav className="-mb-px flex space-x-8">
                <button
                  onClick={() => setActiveTab('images')}
                  className={`py-2 px-1 border-b-2 font-medium text-sm ${
                    activeTab === 'images'
                      ? 'border-primary-500 text-primary-600'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                  }`}
                >
                  我的图片
                </button>
                <button
                  onClick={() => setActiveTab('processing')}
                  className={`py-2 px-1 border-b-2 font-medium text-sm ${
                    activeTab === 'processing'
                      ? 'border-primary-500 text-primary-600'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                  }`}
                >
                  处理历史
                </button>
                <button
                  onClick={() => setActiveTab('stats')}
                  className={`py-2 px-1 border-b-2 font-medium text-sm ${
                    activeTab === 'stats'
                      ? 'border-primary-500 text-primary-600'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                  }`}
                >
                  统计报告
                </button>
              </nav>
            </div>
          </div>

          {/* 内容区域 */}
          {activeTab === 'images' && (
            <div className="space-y-6">
              {/* 操作栏 */}
              <div className="flex justify-between items-center">
                <div className="flex items-center space-x-4">
                  <button className="btn btn-primary">
                    <Upload className="h-4 w-4 mr-2" />
                    上传图片
                  </button>
                  {selectedImages.length > 0 && (
                    <button 
                      onClick={handleDeleteSelected}
                      className="btn btn-outline text-red-600 hover:bg-red-50"
                    >
                      <Trash2 className="h-4 w-4 mr-2" />
                      删除选中 ({selectedImages.length})
                    </button>
                  )}
                </div>
                <div className="flex items-center space-x-2">
                  <button className="btn btn-ghost btn-sm">
                    <Filter className="h-4 w-4 mr-1" />
                    筛选
                  </button>
                  <button className="btn btn-ghost btn-sm">
                    <Settings className="h-4 w-4 mr-1" />
                    设置
                  </button>
                </div>
              </div>

              {/* 图片网格 */}
              {isLoading ? (
                <div className="flex justify-center items-center py-12">
                  <div className="loading-spinner h-8 w-8" />
                </div>
              ) : images.length === 0 ? (
                <div className="text-center py-12">
                  <ImageIcon className="h-12 w-12 text-gray-400 mx-auto mb-4" />
                  <h3 className="text-lg font-medium text-gray-900 mb-2">还没有图片</h3>
                  <p className="text-gray-600 mb-4">上传您的第一张图片开始使用</p>
                  <button className="btn btn-primary">
                    <Upload className="h-4 w-4 mr-2" />
                    上传图片
                  </button>
                </div>
              ) : (
                <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-6 gap-4">
                  {images.map((image) => (
                    <div key={image.id} className="group relative">
                      <div className="aspect-square bg-gray-100 rounded-lg overflow-hidden">
                        <img
                          src={image.thumbnailUrl || image.url}
                          alt={image.name}
                          className="w-full h-full object-cover"
                        />
                        <div className="absolute inset-0 bg-black bg-opacity-0 group-hover:bg-opacity-50 transition-all duration-200 flex items-center justify-center">
                          <div className="opacity-0 group-hover:opacity-100 flex space-x-2">
                            <button className="btn btn-sm bg-white text-gray-900 hover:bg-gray-100">
                              <Eye className="h-4 w-4" />
                            </button>
                            <button className="btn btn-sm bg-white text-gray-900 hover:bg-gray-100">
                              <Edit className="h-4 w-4" />
                            </button>
                            <button className="btn btn-sm bg-white text-gray-900 hover:bg-gray-100">
                              <Download className="h-4 w-4" />
                            </button>
                          </div>
                        </div>
                      </div>
                      <div className="mt-2">
                        <p className="text-sm font-medium text-gray-900 truncate">{image.name}</p>
                        <p className="text-xs text-gray-500">
                          {formatFileSize(image.size)} • {image.format.toUpperCase()}
                        </p>
                      </div>
                      <input
                        type="checkbox"
                        checked={selectedImages.includes(image.id)}
                        onChange={() => handleImageSelect(image.id)}
                        className="absolute top-2 left-2 h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                      />
                    </div>
                  ))}
                </div>
              )}
            </div>
          )}

          {activeTab === 'processing' && (
            <div className="space-y-6">
              <div className="flex justify-between items-center">
                <h3 className="text-lg font-medium text-gray-900">处理历史</h3>
                <button className="btn btn-outline btn-sm">
                  <Filter className="h-4 w-4 mr-1" />
                  筛选
                </button>
              </div>

              {isLoading ? (
                <div className="flex justify-center items-center py-12">
                  <div className="loading-spinner h-8 w-8" />
                </div>
              ) : processingResults.length === 0 ? (
                <div className="text-center py-12">
                  <Clock className="h-12 w-12 text-gray-400 mx-auto mb-4" />
                  <h3 className="text-lg font-medium text-gray-900 mb-2">没有处理记录</h3>
                  <p className="text-gray-600">开始处理您的第一张图片</p>
                </div>
              ) : (
                <div className="space-y-4">
                  {processingResults.map((result) => (
                    <div key={result.id} className="card p-4">
                      <div className="flex items-center justify-between">
                        <div className="flex items-center space-x-4">
                          <div className="w-12 h-12 bg-gray-100 rounded-lg overflow-hidden">
                            <img
                              src={result.url}
                              alt={result.name}
                              className="w-full h-full object-cover"
                            />
                          </div>
                          <div>
                            <p className="font-medium text-gray-900">{result.name}</p>
                            <p className="text-sm text-gray-500">
                              {formatFileSize(result.size)} • {result.format.toUpperCase()}
                            </p>
                          </div>
                        </div>
                        <div className="flex items-center space-x-4">
                          <div className="text-right">
                            <p className="text-sm text-gray-500">处理时间</p>
                            <p className="font-medium">{result.processingTime}ms</p>
                          </div>
                          <div className="text-right">
                            <p className="text-sm text-gray-500">大小减少</p>
                            <p className="font-medium text-green-600">
                              {result.fileSizeReduction.toFixed(1)}%
                            </p>
                          </div>
                          <div className="flex space-x-2">
                            <button className="btn btn-ghost btn-sm">
                              <Eye className="h-4 w-4" />
                            </button>
                            <button className="btn btn-ghost btn-sm">
                              <Download className="h-4 w-4" />
                            </button>
                            <button className="btn btn-ghost btn-sm text-red-600">
                              <Trash2 className="h-4 w-4" />
                            </button>
                          </div>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          )}

          {activeTab === 'stats' && (
            <div className="space-y-6">
              <h3 className="text-lg font-medium text-gray-900">统计报告</h3>
              <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <div className="card p-6">
                  <h4 className="text-lg font-medium text-gray-900 mb-4">格式分布</h4>
                  <div className="space-y-3">
                    {stats?.formatStats?.map((format) => (
                      <div key={format.format} className="flex justify-between items-center">
                        <span className="text-sm text-gray-600">{format.format.toUpperCase()}</span>
                        <span className="font-medium">{format.count}</span>
                      </div>
                    ))}
                  </div>
                </div>
                <div className="card p-6">
                  <h4 className="text-lg font-medium text-gray-900 mb-4">处理统计</h4>
                  <div className="space-y-3">
                    <div className="flex justify-between items-center">
                      <span className="text-sm text-gray-600">总处理次数</span>
                      <span className="font-medium">{stats?.totalProcessed || 0}</span>
                    </div>
                    <div className="flex justify-between items-center">
                      <span className="text-sm text-gray-600">平均处理时间</span>
                      <span className="font-medium">
                        {stats?.averageProcessingTime ? `${Math.round(stats.averageProcessingTime)}ms` : '0ms'}
                      </span>
                    </div>
                    <div className="flex justify-between items-center">
                      <span className="text-sm text-gray-600">平均大小减少</span>
                      <span className="font-medium text-green-600">
                        {stats?.averageFileSizeReduction ? `${stats.averageFileSizeReduction.toFixed(1)}%` : '0%'}
                      </span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          )}
        </div>
      </div>
    </>
  )
}
