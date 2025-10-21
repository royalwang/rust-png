export type Json =
  | string
  | number
  | boolean
  | null
  | { [key: string]: Json | undefined }
  | Json[]

export type Database = {
  public: {
    Tables: {
      profiles: {
        Row: {
          id: string
          email: string
          full_name: string | null
          avatar_url: string | null
          role: 'user' | 'admin' | 'super_admin'
          subscription_plan: 'free' | 'basic' | 'pro' | 'enterprise'
          subscription_status: 'active' | 'inactive' | 'cancelled' | 'past_due'
          subscription_id: string | null
          stripe_customer_id: string | null
          usage_images_processed: number
          usage_storage_used: number
          usage_api_calls: number
          limits_max_images_per_month: number
          limits_max_storage_gb: number
          limits_max_api_calls_per_month: number
          is_active: boolean
          email_verified: boolean
          last_login_at: string | null
          created_at: string
          updated_at: string
        }
        Insert: {
          id: string
          email: string
          full_name?: string | null
          avatar_url?: string | null
          role?: 'user' | 'admin' | 'super_admin'
          subscription_plan?: 'free' | 'basic' | 'pro' | 'enterprise'
          subscription_status?: 'active' | 'inactive' | 'cancelled' | 'past_due'
          subscription_id?: string | null
          stripe_customer_id?: string | null
          usage_images_processed?: number
          usage_storage_used?: number
          usage_api_calls?: number
          limits_max_images_per_month?: number
          limits_max_storage_gb?: number
          limits_max_api_calls_per_month?: number
          is_active?: boolean
          email_verified?: boolean
          last_login_at?: string | null
          created_at?: string
          updated_at?: string
        }
        Update: {
          id?: string
          email?: string
          full_name?: string | null
          avatar_url?: string | null
          role?: 'user' | 'admin' | 'super_admin'
          subscription_plan?: 'free' | 'basic' | 'pro' | 'enterprise'
          subscription_status?: 'active' | 'inactive' | 'cancelled' | 'past_due'
          subscription_id?: string | null
          stripe_customer_id?: string | null
          usage_images_processed?: number
          usage_storage_used?: number
          usage_api_calls?: number
          limits_max_images_per_month?: number
          limits_max_storage_gb?: number
          limits_max_api_calls_per_month?: number
          is_active?: boolean
          email_verified?: boolean
          last_login_at?: string | null
          created_at?: string
          updated_at?: string
        }
        Relationships: []
      }
      images: {
        Row: {
          id: string
          user_id: string
          name: string
          original_name: string
          size: number
          type: string
          url: string
          thumbnail_url: string | null
          width: number
          height: number
          format: string
          metadata: Json | null
          tags: string[]
          is_public: boolean
          is_processed: boolean
          processing_status: 'pending' | 'processing' | 'completed' | 'failed'
          created_at: string
          updated_at: string
        }
        Insert: {
          id?: string
          user_id: string
          name: string
          original_name: string
          size: number
          type: string
          url: string
          thumbnail_url?: string | null
          width: number
          height: number
          format: string
          metadata?: Json | null
          tags?: string[]
          is_public?: boolean
          is_processed?: boolean
          processing_status?: 'pending' | 'processing' | 'completed' | 'failed'
          created_at?: string
          updated_at?: string
        }
        Update: {
          id?: string
          user_id?: string
          name?: string
          original_name?: string
          size?: number
          type?: string
          url?: string
          thumbnail_url?: string | null
          width?: number
          height?: number
          format?: string
          metadata?: Json | null
          tags?: string[]
          is_public?: boolean
          is_processed?: boolean
          processing_status?: 'pending' | 'processing' | 'completed' | 'failed'
          created_at?: string
          updated_at?: string
        }
        Relationships: [
          {
            foreignKeyName: "images_user_id_fkey"
            columns: ["user_id"]
            isOneToOne: false
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          }
        ]
      }
      processing_results: {
        Row: {
          id: string
          user_id: string
          original_image_id: string
          processed_image_id: string
          name: string
          size: number
          type: string
          url: string
          width: number
          height: number
          format: string
          options: Json
          processing_time: number
          file_size_reduction: number
          status: 'pending' | 'processing' | 'completed' | 'failed' | 'cancelled'
          error_message: string | null
          created_at: string
          updated_at: string
        }
        Insert: {
          id?: string
          user_id: string
          original_image_id: string
          processed_image_id: string
          name: string
          size: number
          type: string
          url: string
          width: number
          height: number
          format: string
          options: Json
          processing_time?: number
          file_size_reduction?: number
          status?: 'pending' | 'processing' | 'completed' | 'failed' | 'cancelled'
          error_message?: string | null
          created_at?: string
          updated_at?: string
        }
        Update: {
          id?: string
          user_id?: string
          original_image_id?: string
          processed_image_id?: string
          name?: string
          size?: number
          type?: string
          url?: string
          width?: number
          height?: number
          format?: string
          options?: Json
          processing_time?: number
          file_size_reduction?: number
          status?: 'pending' | 'processing' | 'completed' | 'failed' | 'cancelled'
          error_message?: string | null
          created_at?: string
          updated_at?: string
        }
        Relationships: [
          {
            foreignKeyName: "processing_results_user_id_fkey"
            columns: ["user_id"]
            isOneToOne: false
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "processing_results_original_image_id_fkey"
            columns: ["original_image_id"]
            isOneToOne: false
            referencedRelation: "images"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "processing_results_processed_image_id_fkey"
            columns: ["processed_image_id"]
            isOneToOne: false
            referencedRelation: "images"
            referencedColumns: ["id"]
          }
        ]
      }
      processing_templates: {
        Row: {
          id: string
          user_id: string
          name: string
          description: string | null
          category: string | null
          options: Json
          is_public: boolean
          is_featured: boolean
          usage_count: number
          created_at: string
          updated_at: string
        }
        Insert: {
          id?: string
          user_id: string
          name: string
          description?: string | null
          category?: string | null
          options: Json
          is_public?: boolean
          is_featured?: boolean
          usage_count?: number
          created_at?: string
          updated_at?: string
        }
        Update: {
          id?: string
          user_id?: string
          name?: string
          description?: string | null
          category?: string | null
          options?: Json
          is_public?: boolean
          is_featured?: boolean
          usage_count?: number
          created_at?: string
          updated_at?: string
        }
        Relationships: [
          {
            foreignKeyName: "processing_templates_user_id_fkey"
            columns: ["user_id"]
            isOneToOne: false
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          }
        ]
      }
      usage_stats: {
        Row: {
          id: string
          user_id: string
          date: string
          images_processed: number
          storage_used: number
          api_calls: number
          created_at: string
          updated_at: string
        }
        Insert: {
          id?: string
          user_id: string
          date: string
          images_processed?: number
          storage_used?: number
          api_calls?: number
          created_at?: string
          updated_at?: string
        }
        Update: {
          id?: string
          user_id?: string
          date?: string
          images_processed?: number
          storage_used?: number
          api_calls?: number
          created_at?: string
          updated_at?: string
        }
        Relationships: [
          {
            foreignKeyName: "usage_stats_user_id_fkey"
            columns: ["user_id"]
            isOneToOne: false
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          }
        ]
      }
      api_keys: {
        Row: {
          id: string
          user_id: string
          name: string
          key: string
          permissions: string[]
          is_active: boolean
          last_used_at: string | null
          expires_at: string | null
          created_at: string
          updated_at: string
        }
        Insert: {
          id?: string
          user_id: string
          name: string
          key: string
          permissions: string[]
          is_active?: boolean
          last_used_at?: string | null
          expires_at?: string | null
          created_at?: string
          updated_at?: string
        }
        Update: {
          id?: string
          user_id?: string
          name?: string
          key?: string
          permissions?: string[]
          is_active?: boolean
          last_used_at?: string | null
          expires_at?: string | null
          created_at?: string
          updated_at?: string
        }
        Relationships: [
          {
            foreignKeyName: "api_keys_user_id_fkey"
            columns: ["user_id"]
            isOneToOne: false
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          }
        ]
      }
      webhooks: {
        Row: {
          id: string
          user_id: string
          url: string
          events: string[]
          secret: string
          is_active: boolean
          last_triggered_at: string | null
          created_at: string
          updated_at: string
        }
        Insert: {
          id?: string
          user_id: string
          url: string
          events: string[]
          secret: string
          is_active?: boolean
          last_triggered_at?: string | null
          created_at?: string
          updated_at?: string
        }
        Update: {
          id?: string
          user_id?: string
          url?: string
          events?: string[]
          secret?: string
          is_active?: boolean
          last_triggered_at?: string | null
          created_at?: string
          updated_at?: string
        }
        Relationships: [
          {
            foreignKeyName: "webhooks_user_id_fkey"
            columns: ["user_id"]
            isOneToOne: false
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          }
        ]
      }
    }
    Views: {
      [_ in never]: never
    }
    Functions: {
      [_ in never]: never
    }
    Enums: {
      user_role: 'user' | 'admin' | 'super_admin'
      subscription_plan: 'free' | 'basic' | 'pro' | 'enterprise'
      subscription_status: 'active' | 'inactive' | 'cancelled' | 'past_due'
      processing_status: 'pending' | 'processing' | 'completed' | 'failed' | 'cancelled'
    }
    CompositeTypes: {
      [_ in never]: never
    }
  }
}
