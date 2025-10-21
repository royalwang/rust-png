import { AlertTriangle, RefreshCw } from 'lucide-react'

interface ErrorFallbackProps {
  error: Error
  resetErrorBoundary: () => void
}

export function ErrorFallback({ error, resetErrorBoundary }: ErrorFallbackProps) {
  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50">
      <div className="max-w-md w-full bg-white rounded-lg shadow-lg p-6 text-center">
        <div className="flex justify-center mb-4">
          <AlertTriangle className="h-12 w-12 text-red-500" />
        </div>
        
        <h1 className="text-2xl font-bold text-gray-900 mb-2">
          出现错误
        </h1>
        
        <p className="text-gray-600 mb-6">
          抱歉，应用程序遇到了一个错误。请尝试刷新页面或联系支持团队。
        </p>
        
        <div className="bg-red-50 border border-red-200 rounded-md p-4 mb-6">
          <p className="text-sm text-red-800 font-mono">
            {error.message}
          </p>
        </div>
        
        <div className="flex gap-3 justify-center">
          <button
            onClick={resetErrorBoundary}
            className="btn btn-primary flex items-center gap-2"
          >
            <RefreshCw className="h-4 w-4" />
            重试
          </button>
          
          <button
            onClick={() => window.location.href = '/'}
            className="btn btn-outline"
          >
            返回首页
          </button>
        </div>
      </div>
    </div>
  )
}
