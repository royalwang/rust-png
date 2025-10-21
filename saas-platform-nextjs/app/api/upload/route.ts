import { NextRequest, NextResponse } from 'next/server'
import { createSupabaseServerClient } from '@/lib/supabase/server'
import { getCurrentUser } from '@/lib/supabase/server'

export async function POST(request: NextRequest) {
  try {
    const user = await getCurrentUser()
    
    if (!user) {
      return NextResponse.json(
        { error: '未授权访问' },
        { status: 401 }
      )
    }

    const formData = await request.formData()
    const file = formData.get('file') as File
    
    if (!file) {
      return NextResponse.json(
        { error: '未找到文件' },
        { status: 400 }
      )
    }

    // 验证文件类型
    const allowedTypes = ['image/png', 'image/jpeg', 'image/jpg', 'image/gif', 'image/webp', 'image/avif']
    if (!allowedTypes.includes(file.type)) {
      return NextResponse.json(
        { error: '不支持的文件类型' },
        { status: 400 }
      )
    }

    // 验证文件大小 (50MB)
    const maxSize = 50 * 1024 * 1024
    if (file.size > maxSize) {
      return NextResponse.json(
        { error: '文件大小超过限制' },
        { status: 400 }
      )
    }

    const supabase = createSupabaseServerClient()
    
    // 生成唯一文件名
    const fileExt = file.name.split('.').pop()
    const fileName = `${user.id}/${Date.now()}.${fileExt}`
    
    // 上传文件到Supabase Storage
    const { data: uploadData, error: uploadError } = await supabase.storage
      .from('images')
      .upload(fileName, file, {
        contentType: file.type,
        upsert: false,
      })

    if (uploadError) {
      console.error('Upload error:', uploadError)
      return NextResponse.json(
        { error: '文件上传失败' },
        { status: 500 }
      )
    }

    // 获取文件URL
    const { data: urlData } = supabase.storage
      .from('images')
      .getPublicUrl(fileName)

    // 获取图片尺寸
    const imageBuffer = await file.arrayBuffer()
    const imageSize = await getImageDimensions(Buffer.from(imageBuffer))

    // 保存图片信息到数据库
    const { data: imageData, error: dbError } = await supabase
      .from('images')
      .insert({
        user_id: user.id,
        name: file.name,
        original_name: file.name,
        size: file.size,
        type: file.type,
        url: urlData.publicUrl,
        width: imageSize.width,
        height: imageSize.height,
        format: fileExt,
        processing_status: 'completed',
      })
      .select()
      .single()

    if (dbError) {
      console.error('Database error:', dbError)
      return NextResponse.json(
        { error: '保存图片信息失败' },
        { status: 500 }
      )
    }

    return NextResponse.json({
      success: true,
      data: {
        id: imageData.id,
        name: imageData.name,
        url: imageData.url,
        size: imageData.size,
        width: imageData.width,
        height: imageData.height,
        format: imageData.format,
      },
    })

  } catch (error) {
    console.error('Upload API error:', error)
    return NextResponse.json(
      { error: '服务器内部错误' },
      { status: 500 }
    )
  }
}

async function getImageDimensions(buffer: Buffer): Promise<{ width: number; height: number }> {
  // 这里应该使用实际的图片处理库来获取尺寸
  // 为了简化，返回默认值
  return { width: 1920, height: 1080 }
}
