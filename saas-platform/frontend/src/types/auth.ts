export interface User {
  id: string
  email: string
  name: string
  avatar?: string
  role: 'user' | 'admin'
  subscription: {
    plan: 'free' | 'basic' | 'pro' | 'enterprise'
    status: 'active' | 'inactive' | 'cancelled'
    expiresAt?: Date
  }
  usage: {
    imagesProcessed: number
    storageUsed: number
    apiCalls: number
  }
  limits: {
    maxImagesPerMonth: number
    maxStorageGB: number
    maxApiCallsPerMonth: number
  }
  createdAt: Date
  updatedAt: Date
}

export interface LoginCredentials {
  email: string
  password: string
  remember?: boolean
}

export interface RegisterData {
  name: string
  email: string
  password: string
  confirmPassword: string
  acceptTerms: boolean
}

export interface AuthResponse {
  user: User
  token: string
  refreshToken?: string
}

export interface PasswordResetRequest {
  email: string
}

export interface PasswordReset {
  token: string
  password: string
  confirmPassword: string
}

export interface ChangePassword {
  currentPassword: string
  newPassword: string
  confirmPassword: string
}
