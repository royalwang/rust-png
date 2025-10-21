import { Metadata } from 'next'
import { redirect } from 'next/navigation'
import { getCurrentUser } from '@/lib/supabase/server'
import { DashboardLayout } from '@/components/layout/DashboardLayout'
import { DashboardPage } from '@/components/pages/DashboardPage'

export const metadata: Metadata = {
  title: '仪表板',
  description: 'Rust PNG SaaS平台用户仪表板',
}

export default async function Page() {
  const user = await getCurrentUser()
  
  // 如果用户未登录，重定向到登录页面
  if (!user) {
    redirect('/login')
  }

  return (
    <DashboardLayout>
      <DashboardPage />
    </DashboardLayout>
  )
}
