-- 启用必要的扩展
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- 创建用户配置文件表
CREATE TABLE IF NOT EXISTS public.profiles (
    id UUID REFERENCES auth.users(id) ON DELETE CASCADE PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    full_name TEXT,
    avatar_url TEXT,
    role user_role DEFAULT 'user' NOT NULL,
    subscription_plan subscription_plan DEFAULT 'free' NOT NULL,
    subscription_status subscription_status DEFAULT 'active' NOT NULL,
    subscription_id TEXT,
    stripe_customer_id TEXT,
    usage_images_processed INTEGER DEFAULT 0 NOT NULL,
    usage_storage_used BIGINT DEFAULT 0 NOT NULL,
    usage_api_calls INTEGER DEFAULT 0 NOT NULL,
    limits_max_images_per_month INTEGER DEFAULT 100 NOT NULL,
    limits_max_storage_gb INTEGER DEFAULT 1 NOT NULL,
    limits_max_api_calls_per_month INTEGER DEFAULT 1000 NOT NULL,
    is_active BOOLEAN DEFAULT true NOT NULL,
    email_verified BOOLEAN DEFAULT false NOT NULL,
    last_login_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

-- 创建图片表
CREATE TABLE IF NOT EXISTS public.images (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    user_id UUID REFERENCES public.profiles(id) ON DELETE CASCADE NOT NULL,
    name TEXT NOT NULL,
    original_name TEXT NOT NULL,
    size BIGINT NOT NULL,
    type TEXT NOT NULL,
    url TEXT NOT NULL,
    thumbnail_url TEXT,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    format TEXT NOT NULL,
    metadata JSONB,
    tags TEXT[] DEFAULT '{}',
    is_public BOOLEAN DEFAULT false NOT NULL,
    is_processed BOOLEAN DEFAULT false NOT NULL,
    processing_status processing_status DEFAULT 'pending' NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

-- 创建处理结果表
CREATE TABLE IF NOT EXISTS public.processing_results (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    user_id UUID REFERENCES public.profiles(id) ON DELETE CASCADE NOT NULL,
    original_image_id UUID REFERENCES public.images(id) ON DELETE CASCADE NOT NULL,
    processed_image_id UUID REFERENCES public.images(id) ON DELETE CASCADE NOT NULL,
    name TEXT NOT NULL,
    size BIGINT NOT NULL,
    type TEXT NOT NULL,
    url TEXT NOT NULL,
    width INTEGER NOT NULL,
    height INTEGER NOT NULL,
    format TEXT NOT NULL,
    options JSONB NOT NULL,
    processing_time INTEGER DEFAULT 0 NOT NULL,
    file_size_reduction DECIMAL(5,2) DEFAULT 0 NOT NULL,
    status processing_status DEFAULT 'pending' NOT NULL,
    error_message TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

-- 创建处理模板表
CREATE TABLE IF NOT EXISTS public.processing_templates (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    user_id UUID REFERENCES public.profiles(id) ON DELETE CASCADE NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    category TEXT,
    options JSONB NOT NULL,
    is_public BOOLEAN DEFAULT false NOT NULL,
    is_featured BOOLEAN DEFAULT false NOT NULL,
    usage_count INTEGER DEFAULT 0 NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

-- 创建使用统计表
CREATE TABLE IF NOT EXISTS public.usage_stats (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    user_id UUID REFERENCES public.profiles(id) ON DELETE CASCADE NOT NULL,
    date DATE NOT NULL,
    images_processed INTEGER DEFAULT 0 NOT NULL,
    storage_used BIGINT DEFAULT 0 NOT NULL,
    api_calls INTEGER DEFAULT 0 NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    UNIQUE(user_id, date)
);

-- 创建API密钥表
CREATE TABLE IF NOT EXISTS public.api_keys (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    user_id UUID REFERENCES public.profiles(id) ON DELETE CASCADE NOT NULL,
    name TEXT NOT NULL,
    key TEXT UNIQUE NOT NULL,
    permissions TEXT[] DEFAULT '{}' NOT NULL,
    is_active BOOLEAN DEFAULT true NOT NULL,
    last_used_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

-- 创建Webhook表
CREATE TABLE IF NOT EXISTS public.webhooks (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    user_id UUID REFERENCES public.profiles(id) ON DELETE CASCADE NOT NULL,
    url TEXT NOT NULL,
    events TEXT[] DEFAULT '{}' NOT NULL,
    secret TEXT NOT NULL,
    is_active BOOLEAN DEFAULT true NOT NULL,
    last_triggered_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
    updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
);

-- 创建枚举类型
DO $$ BEGIN
    CREATE TYPE user_role AS ENUM ('user', 'admin', 'super_admin');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE subscription_plan AS ENUM ('free', 'basic', 'pro', 'enterprise');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE subscription_status AS ENUM ('active', 'inactive', 'cancelled', 'past_due');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

DO $$ BEGIN
    CREATE TYPE processing_status AS ENUM ('pending', 'processing', 'completed', 'failed', 'cancelled');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_profiles_email ON public.profiles(email);
CREATE INDEX IF NOT EXISTS idx_profiles_stripe_customer_id ON public.profiles(stripe_customer_id);
CREATE INDEX IF NOT EXISTS idx_images_user_id ON public.images(user_id);
CREATE INDEX IF NOT EXISTS idx_images_created_at ON public.images(created_at);
CREATE INDEX IF NOT EXISTS idx_images_format ON public.images(format);
CREATE INDEX IF NOT EXISTS idx_images_is_public ON public.images(is_public);
CREATE INDEX IF NOT EXISTS idx_processing_results_user_id ON public.processing_results(user_id);
CREATE INDEX IF NOT EXISTS idx_processing_results_created_at ON public.processing_results(created_at);
CREATE INDEX IF NOT EXISTS idx_processing_results_status ON public.processing_results(status);
CREATE INDEX IF NOT EXISTS idx_processing_templates_user_id ON public.processing_templates(user_id);
CREATE INDEX IF NOT EXISTS idx_processing_templates_is_public ON public.processing_templates(is_public);
CREATE INDEX IF NOT EXISTS idx_processing_templates_is_featured ON public.processing_templates(is_featured);
CREATE INDEX IF NOT EXISTS idx_usage_stats_user_id ON public.usage_stats(user_id);
CREATE INDEX IF NOT EXISTS idx_usage_stats_date ON public.usage_stats(date);
CREATE INDEX IF NOT EXISTS idx_api_keys_user_id ON public.api_keys(user_id);
CREATE INDEX IF NOT EXISTS idx_api_keys_key ON public.api_keys(key);
CREATE INDEX IF NOT EXISTS idx_webhooks_user_id ON public.webhooks(user_id);

-- 创建更新时间触发器函数
CREATE OR REPLACE FUNCTION public.handle_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 为所有表添加更新时间触发器
CREATE TRIGGER handle_updated_at_profiles
    BEFORE UPDATE ON public.profiles
    FOR EACH ROW EXECUTE FUNCTION public.handle_updated_at();

CREATE TRIGGER handle_updated_at_images
    BEFORE UPDATE ON public.images
    FOR EACH ROW EXECUTE FUNCTION public.handle_updated_at();

CREATE TRIGGER handle_updated_at_processing_results
    BEFORE UPDATE ON public.processing_results
    FOR EACH ROW EXECUTE FUNCTION public.handle_updated_at();

CREATE TRIGGER handle_updated_at_processing_templates
    BEFORE UPDATE ON public.processing_templates
    FOR EACH ROW EXECUTE FUNCTION public.handle_updated_at();

CREATE TRIGGER handle_updated_at_usage_stats
    BEFORE UPDATE ON public.usage_stats
    FOR EACH ROW EXECUTE FUNCTION public.handle_updated_at();

CREATE TRIGGER handle_updated_at_api_keys
    BEFORE UPDATE ON public.api_keys
    FOR EACH ROW EXECUTE FUNCTION public.handle_updated_at();

CREATE TRIGGER handle_updated_at_webhooks
    BEFORE UPDATE ON public.webhooks
    FOR EACH ROW EXECUTE FUNCTION public.handle_updated_at();
