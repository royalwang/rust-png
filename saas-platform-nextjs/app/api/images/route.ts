import { NextRequest, NextResponse } from 'next/server'
import { createSupabaseServerClient } from '@/lib/supabase/server'
import { getCurrentUser } from '@/lib/supabase/server'

export async function GET(request: NextRequest) {
  try {
    const user = await getCurrentUser()
    
    if (!user) {
      return NextResponse.json(
        { error: '未授权访问' },
        { status: 401 }
      )
    }

    const { searchParams } = new URL(request.url)
    const page = parseInt(searchParams.get('page') || '1')
    const limit = parseInt(searchParams.get('limit') || '20')
    const search = searchParams.get('search') || ''
    const format = searchParams.get('format') || ''
    const status = searchParams.get('status') || ''

    const supabase = createSupabaseServerClient()
    
    let query = supabase
      .from('images')
      .select('*')
      .eq('user_id', user.id)
      .order('created_at', { ascending: false })

    // 应用筛选条件
    if (search) {
      query = query.or(`name.ilike.%${search}%,original_name.ilike.%${search}%`)
    }
    
    if (format) {
      query = query.eq('format', format)
    }
    
    if (status) {
      query = query.eq('processing_status', status)
    }

    // 分页
    const from = (page - 1) * limit
    const to = from + limit - 1
    
    query = query.range(from, to)

    const { data: images, error } = await query

    if (error) {
      console.error('Database error:', error)
      return NextResponse.json(
        { error: '获取图片列表失败' },
        { status: 500 }
      )
    }

    // 获取总数
    const { count } = await supabase
      .from('images')
      .select('*', { count: 'exact', head: true })
      .eq('user_id', user.id)

    return NextResponse.json({
      success: true,
      data: {
        images: images || [],
        pagination: {
          page,
          limit,
          total: count || 0,
          pages: Math.ceil((count || 0) / limit),
        },
      },
    })

  } catch (error) {
    console.error('Images API error:', error)
    return NextResponse.json(
      { error: '服务器内部错误' },
      { status: 500 }
    )
  }
}

export async function DELETE(request: NextRequest) {
  try {
    const user = await getCurrentUser()
    
    if (!user) {
      return NextResponse.json(
        { error: '未授权访问' },
        { status: 401 }
      )
    }

    const { searchParams } = new URL(request.url)
    const imageId = searchParams.get('id')
    
    if (!imageId) {
      return NextResponse.json(
        { error: '缺少图片ID' },
        { status: 400 }
      )
    }

    const supabase = createSupabaseServerClient()

    // 获取图片信息
    const { data: image, error: fetchError } = await supabase
      .from('images')
      .select('*')
      .eq('id', imageId)
      .eq('user_id', user.id)
      .single()

    if (fetchError || !image) {
      return NextResponse.json(
        { error: '图片不存在' },
        { status: 404 }
      )
    }

    // 从存储中删除文件
    const fileName = image.url.split('/').pop()
    if (fileName) {
      const { error: storageError } = await supabase.storage
        .from('images')
        .remove([`${user.id}/${fileName}`])

      if (storageError) {
        console.error('Storage deletion error:', storageError)
      }
    }

    // 从数据库中删除记录
    const { error: dbError } = await supabase
      .from('images')
      .delete()
      .eq('id', imageId)
      .eq('user_id', user.id)

    if (dbError) {
      console.error('Database deletion error:', dbError)
      return NextResponse.json(
        { error: '删除图片失败' },
        { status: 500 }
      )
    }

    return NextResponse.json({
      success: true,
      message: '图片删除成功',
    })

  } catch (error) {
    console.error('Delete image API error:', error)
    return NextResponse.json(
      { error: '服务器内部错误' },
      { status: 500 }
    )
  }
}
