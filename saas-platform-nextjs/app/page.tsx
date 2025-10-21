import { Suspense } from 'react'
import { Metadata } from 'next'
import { redirect } from 'next/navigation'
import { getCurrentUser } from '@/lib/supabase/server'
import { HomePage } from '@/components/pages/HomePage'
import { LoadingSpinner } from '@/components/ui/LoadingSpinner'

export const metadata: Metadata = {
  title: '首页',
  description: '基于Rust PNG JS构建的高性能图片处理SaaS平台',
}

export default async function Page() {
  const user = await getCurrentUser()
  
  // 如果用户已登录，重定向到仪表板
  if (user) {
    redirect('/dashboard')
  }

  return (
    <Suspense fallback={<LoadingSpinner />}>
      <HomePage />
    </Suspense>
  )
}
