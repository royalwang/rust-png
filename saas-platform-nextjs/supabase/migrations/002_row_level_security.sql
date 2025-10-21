-- 启用行级安全策略
ALTER TABLE public.profiles ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.images ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.processing_results ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.processing_templates ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.usage_stats ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.api_keys ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.webhooks ENABLE ROW LEVEL SECURITY;

-- 用户配置文件策略
CREATE POLICY "Users can view own profile" ON public.profiles
    FOR SELECT USING (auth.uid() = id);

CREATE POLICY "Users can update own profile" ON public.profiles
    FOR UPDATE USING (auth.uid() = id);

CREATE POLICY "Users can insert own profile" ON public.profiles
    FOR INSERT WITH CHECK (auth.uid() = id);

-- 管理员可以查看所有配置文件
CREATE POLICY "Admins can view all profiles" ON public.profiles
    FOR SELECT USING (
        EXISTS (
            SELECT 1 FROM public.profiles
            WHERE id = auth.uid() AND role IN ('admin', 'super_admin')
        )
    );

-- 图片策略
CREATE POLICY "Users can view own images" ON public.images
    FOR SELECT USING (auth.uid() = user_id);

CREATE POLICY "Users can insert own images" ON public.images
    FOR INSERT WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Users can update own images" ON public.images
    FOR UPDATE USING (auth.uid() = user_id);

CREATE POLICY "Users can delete own images" ON public.images
    FOR DELETE USING (auth.uid() = user_id);

-- 公开图片可以被所有人查看
CREATE POLICY "Public images are viewable by everyone" ON public.images
    FOR SELECT USING (is_public = true);

-- 处理结果策略
CREATE POLICY "Users can view own processing results" ON public.processing_results
    FOR SELECT USING (auth.uid() = user_id);

CREATE POLICY "Users can insert own processing results" ON public.processing_results
    FOR INSERT WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Users can update own processing results" ON public.processing_results
    FOR UPDATE USING (auth.uid() = user_id);

CREATE POLICY "Users can delete own processing results" ON public.processing_results
    FOR DELETE USING (auth.uid() = user_id);

-- 处理模板策略
CREATE POLICY "Users can view own templates" ON public.processing_templates
    FOR SELECT USING (auth.uid() = user_id);

CREATE POLICY "Users can view public templates" ON public.processing_templates
    FOR SELECT USING (is_public = true);

CREATE POLICY "Users can insert own templates" ON public.processing_templates
    FOR INSERT WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Users can update own templates" ON public.processing_templates
    FOR UPDATE USING (auth.uid() = user_id);

CREATE POLICY "Users can delete own templates" ON public.processing_templates
    FOR DELETE USING (auth.uid() = user_id);

-- 使用统计策略
CREATE POLICY "Users can view own usage stats" ON public.usage_stats
    FOR SELECT USING (auth.uid() = user_id);

CREATE POLICY "Users can insert own usage stats" ON public.usage_stats
    FOR INSERT WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Users can update own usage stats" ON public.usage_stats
    FOR UPDATE USING (auth.uid() = user_id);

-- 管理员可以查看所有使用统计
CREATE POLICY "Admins can view all usage stats" ON public.usage_stats
    FOR SELECT USING (
        EXISTS (
            SELECT 1 FROM public.profiles
            WHERE id = auth.uid() AND role IN ('admin', 'super_admin')
        )
    );

-- API密钥策略
CREATE POLICY "Users can view own API keys" ON public.api_keys
    FOR SELECT USING (auth.uid() = user_id);

CREATE POLICY "Users can insert own API keys" ON public.api_keys
    FOR INSERT WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Users can update own API keys" ON public.api_keys
    FOR UPDATE USING (auth.uid() = user_id);

CREATE POLICY "Users can delete own API keys" ON public.api_keys
    FOR DELETE USING (auth.uid() = user_id);

-- Webhook策略
CREATE POLICY "Users can view own webhooks" ON public.webhooks
    FOR SELECT USING (auth.uid() = user_id);

CREATE POLICY "Users can insert own webhooks" ON public.webhooks
    FOR INSERT WITH CHECK (auth.uid() = user_id);

CREATE POLICY "Users can update own webhooks" ON public.webhooks
    FOR UPDATE USING (auth.uid() = user_id);

CREATE POLICY "Users can delete own webhooks" ON public.webhooks
    FOR DELETE USING (auth.uid() = user_id);

-- 创建用户配置文件触发器
CREATE OR REPLACE FUNCTION public.handle_new_user()
RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO public.profiles (id, email, full_name, avatar_url)
    VALUES (
        NEW.id,
        NEW.email,
        COALESCE(NEW.raw_user_meta_data->>'full_name', NEW.raw_user_meta_data->>'name'),
        NEW.raw_user_meta_data->>'avatar_url'
    );
    RETURN NEW;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- 创建新用户时自动创建配置文件
CREATE TRIGGER on_auth_user_created
    AFTER INSERT ON auth.users
    FOR EACH ROW EXECUTE FUNCTION public.handle_new_user();

-- 创建更新用户配置文件触发器
CREATE OR REPLACE FUNCTION public.handle_user_update()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE public.profiles
    SET
        email = NEW.email,
        full_name = COALESCE(NEW.raw_user_meta_data->>'full_name', NEW.raw_user_meta_data->>'name'),
        avatar_url = NEW.raw_user_meta_data->>'avatar_url',
        updated_at = NOW()
    WHERE id = NEW.id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql SECURITY DEFINER;

-- 更新用户时同步配置文件
CREATE TRIGGER on_auth_user_updated
    AFTER UPDATE ON auth.users
    FOR EACH ROW EXECUTE FUNCTION public.handle_user_update();
