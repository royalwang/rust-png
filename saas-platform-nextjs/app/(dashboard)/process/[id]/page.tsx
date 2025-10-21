import { ImageProcessor } from '@/components/ui/ImageProcessor'
import { DashboardLayout } from '@/components/layout/DashboardLayout'
import { createServerSupabaseClient } from '@/lib/supabase/server'
import { notFound, redirect } from 'next/navigation'

interface ProcessPageProps {
  params: {
    id: string
  }
}

export default async function ProcessPage({ params }: ProcessPageProps) {
  const supabase = createServerSupabaseClient()
  
  // 验证用户身份
  const { data: { user }, error: authError } = await supabase.auth.getUser()
  if (authError || !user) {
    redirect('/login')
  }

  // 获取图片信息
  const { data: image, error: imageError } = await supabase
    .from('images')
    .select('*')
    .eq('id', params.id)
    .eq('user_id', user.id)
    .single()

  if (imageError || !image) {
    notFound()
  }

  return (
    <DashboardLayout>
      <div className="space-y-6">
        <div>
          <h1 className="text-3xl font-bold text-gray-900">图片处理</h1>
          <p className="text-gray-600 mt-1">
            选择处理选项并开始处理您的图片
          </p>
        </div>

        <ImageProcessor
          imageId={image.id}
          imageUrl={image.url}
          imageName={image.name}
          onProcessingComplete={(result) => {
            console.log('处理完成:', result)
          }}
        />
      </div>
    </DashboardLayout>
  )
}
