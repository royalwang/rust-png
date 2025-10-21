import { createClientComponentClient } from '@supabase/auth-helpers-nextjs'
import { createClient } from '@supabase/supabase-js'
import type { Database } from '@/types/supabase'

// 客户端组件使用的Supabase客户端
export const createSupabaseClient = () => {
  return createClientComponentClient<Database>()
}

// 服务端组件使用的Supabase客户端
export const createSupabaseServerClient = () => {
  return createClient<Database>(
    process.env.NEXT_PUBLIC_SUPABASE_URL!,
    process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY!
  )
}

// 管理员客户端（使用service role key）
export const createSupabaseAdminClient = () => {
  return createClient<Database>(
    process.env.NEXT_PUBLIC_SUPABASE_URL!,
    process.env.SUPABASE_SERVICE_ROLE_KEY!
  )
}

// 默认导出客户端
export const supabase = createSupabaseClient()
