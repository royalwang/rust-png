import { Metadata } from 'next'
import { redirect } from 'next/navigation'
import { getCurrentUser } from '@/lib/supabase/server'
import { RegisterForm } from '@/components/auth/RegisterForm'

export const metadata: Metadata = {
  title: '注册',
  description: '注册Rust PNG SaaS平台账户',
}

export default async function RegisterPage() {
  const user = await getCurrentUser()
  
  // 如果用户已登录，重定向到仪表板
  if (user) {
    redirect('/dashboard')
  }

  return (
    <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800">
      <div className="max-w-md w-full space-y-8 p-8">
        <div className="text-center">
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
            创建账户
          </h1>
          <p className="mt-2 text-sm text-gray-600 dark:text-gray-400">
            开始使用Rust PNG SaaS平台
          </p>
        </div>
        
        <RegisterForm />
        
        <div className="text-center">
          <p className="text-sm text-gray-600 dark:text-gray-400">
            已有账户？{' '}
            <a
              href="/login"
              className="font-medium text-primary hover:text-primary/80 transition-colors"
            >
              立即登录
            </a>
          </p>
        </div>
      </div>
    </div>
  )
}
