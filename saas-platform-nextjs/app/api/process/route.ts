import { NextRequest, NextResponse } from 'next/server'
import { createServerSupabaseClient } from '@/lib/supabase/server'

export async function POST(request: NextRequest) {
  try {
    const supabase = createServerSupabaseClient()
    
    // 验证用户身份
    const { data: { user }, error: authError } = await supabase.auth.getUser()
    if (authError || !user) {
      return NextResponse.json({ error: '未授权' }, { status: 401 })
    }

    const body = await request.json()
    const { imageId, operations } = body

    if (!imageId || !operations) {
      return NextResponse.json({ error: '缺少必要参数' }, { status: 400 })
    }

    // 获取图片信息
    const { data: image, error: imageError } = await supabase
      .from('images')
      .select('*')
      .eq('id', imageId)
      .eq('user_id', user.id)
      .single()

    if (imageError || !image) {
      return NextResponse.json({ error: '图片不存在' }, { status: 404 })
    }

    // 创建处理任务
    const { data: processingTask, error: taskError } = await supabase
      .from('processing_tasks')
      .insert({
        user_id: user.id,
        image_id: imageId,
        operations: operations,
        status: 'pending'
      })
      .select()
      .single()

    if (taskError) {
      return NextResponse.json({ error: '创建处理任务失败' }, { status: 500 })
    }

    // 这里应该调用实际的图片处理服务
    // 现在只是模拟处理
    setTimeout(async () => {
      try {
        // 模拟处理完成
        const processedImageUrl = `${image.url}?processed=${Date.now()}`
        
        // 更新处理任务状态
        await supabase
          .from('processing_tasks')
          .update({
            status: 'completed',
            result_url: processedImageUrl,
            completed_at: new Date().toISOString()
          })
          .eq('id', processingTask.id)

        // 创建处理结果记录
        await supabase
          .from('processing_results')
          .insert({
            user_id: user.id,
            original_image_id: imageId,
            processed_image_url: processedImageUrl,
            operations: operations,
            processing_time: 1500, // 模拟处理时间
            file_size_reduction: 25 // 模拟文件大小减少
          })
      } catch (error) {
        console.error('处理图片失败:', error)
        
        // 更新处理任务状态为失败
        await supabase
          .from('processing_tasks')
          .update({
            status: 'failed',
            error_message: error instanceof Error ? error.message : '处理失败'
          })
          .eq('id', processingTask.id)
      }
    }, 2000)

    return NextResponse.json({
      success: true,
      taskId: processingTask.id,
      message: '处理任务已创建'
    })

  } catch (error) {
    console.error('处理图片失败:', error)
    return NextResponse.json(
      { error: '服务器内部错误' },
      { status: 500 }
    )
  }
}

export async function GET(request: NextRequest) {
  try {
    const supabase = createServerSupabaseClient()
    
    // 验证用户身份
    const { data: { user }, error: authError } = await supabase.auth.getUser()
    if (authError || !user) {
      return NextResponse.json({ error: '未授权' }, { status: 401 })
    }

    const { searchParams } = new URL(request.url)
    const imageId = searchParams.get('imageId')
    const status = searchParams.get('status')

    let query = supabase
      .from('processing_tasks')
      .select(`
        *,
        images (
          id,
          name,
          url,
          size,
          width,
          height
        )
      `)
      .eq('user_id', user.id)

    if (imageId) {
      query = query.eq('image_id', imageId)
    }

    if (status) {
      query = query.eq('status', status)
    }

    const { data: tasks, error } = await query.order('created_at', { ascending: false })

    if (error) {
      return NextResponse.json({ error: '获取处理任务失败' }, { status: 500 })
    }

    return NextResponse.json({ tasks })

  } catch (error) {
    console.error('获取处理任务失败:', error)
    return NextResponse.json(
      { error: '服务器内部错误' },
      { status: 500 }
    )
  }
}