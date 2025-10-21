import { createServerComponentClient } from '@supabase/auth-helpers-nextjs'
import { cookies } from 'next/headers'
import type { Database } from '@/types/supabase'

// 服务端组件使用的Supabase客户端
export const createSupabaseServerClient = () => {
  const cookieStore = cookies()
  
  return createServerComponentClient<Database>({
    cookies: () => cookieStore,
  })
}

// 获取当前用户
export const getCurrentUser = async () => {
  const supabase = createSupabaseServerClient()
  
  try {
    const { data: { user }, error } = await supabase.auth.getUser()
    
    if (error) {
      console.error('Error getting user:', error)
      return null
    }
    
    return user
  } catch (error) {
    console.error('Error in getCurrentUser:', error)
    return null
  }
}

// 获取用户会话
export const getSession = async () => {
  const supabase = createSupabaseServerClient()
  
  try {
    const { data: { session }, error } = await supabase.auth.getSession()
    
    if (error) {
      console.error('Error getting session:', error)
      return null
    }
    
    return session
  } catch (error) {
    console.error('Error in getSession:', error)
    return null
  }
}

// 检查用户是否已认证
export const isAuthenticated = async () => {
  const user = await getCurrentUser()
  return !!user
}

// 获取用户角色
export const getUserRole = async () => {
  const user = await getCurrentUser()
  
  if (!user) {
    return null
  }
  
  // 从用户元数据中获取角色
  return user.user_metadata?.role || 'user'
}

// 检查用户是否有特定角色
export const hasRole = async (role: string) => {
  const userRole = await getUserRole()
  return userRole === role
}
