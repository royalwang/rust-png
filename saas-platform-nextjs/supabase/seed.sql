-- 插入默认管理员用户
INSERT INTO auth.users (
    id,
    instance_id,
    aud,
    role,
    email,
    encrypted_password,
    email_confirmed_at,
    recovery_sent_at,
    last_sign_in_at,
    raw_app_meta_data,
    raw_user_meta_data,
    created_at,
    updated_at,
    confirmation_token,
    email_change,
    email_change_token_new,
    recovery_token
) VALUES (
    '00000000-0000-0000-0000-000000000000',
    '00000000-0000-0000-0000-000000000000',
    'authenticated',
    'authenticated',
    'admin@rust-png-saas.com',
    crypt('admin123', gen_salt('bf')),
    NOW(),
    NOW(),
    NOW(),
    '{"provider": "email", "providers": ["email"]}',
    '{"full_name": "Admin User", "avatar_url": null}',
    NOW(),
    NOW(),
    '',
    '',
    '',
    ''
);

-- 插入管理员配置文件
INSERT INTO public.profiles (
    id,
    email,
    full_name,
    role,
    subscription_plan,
    subscription_status,
    limits_max_images_per_month,
    limits_max_storage_gb,
    limits_max_api_calls_per_month,
    is_active,
    email_verified
) VALUES (
    '00000000-0000-0000-0000-000000000000',
    'admin@rust-png-saas.com',
    'Admin User',
    'super_admin',
    'enterprise',
    'active',
    999999,
    999999,
    999999,
    true,
    true
);

-- 插入示例处理模板
INSERT INTO public.processing_templates (
    id,
    user_id,
    name,
    description,
    category,
    options,
    is_public,
    is_featured,
    usage_count
) VALUES (
    uuid_generate_v4(),
    '00000000-0000-0000-0000-000000000000',
    '社交媒体优化',
    '为社交媒体平台优化的图片处理模板',
    'social',
    '{
        "resize": {
            "enabled": true,
            "width": 1080,
            "height": 1080,
            "maintainAspectRatio": true
        },
        "compression": {
            "quality": 85,
            "format": "jpg",
            "optimize": true
        },
        "filters": {
            "brightness": 1.1,
            "contrast": 1.05,
            "saturation": 1.1
        }
    }',
    true,
    true,
    0
),
(
    uuid_generate_v4(),
    '00000000-0000-0000-0000-000000000000',
    '网页优化',
    '为网页显示优化的图片处理模板',
    'web',
    '{
        "resize": {
            "enabled": true,
            "width": 1920,
            "height": 1080,
            "maintainAspectRatio": true
        },
        "compression": {
            "quality": 90,
            "format": "webp",
            "optimize": true
        },
        "filters": {
            "brightness": 1.0,
            "contrast": 1.0,
            "saturation": 1.0
        }
    }',
    true,
    true,
    0
),
(
    uuid_generate_v4(),
    '00000000-0000-0000-0000-000000000000',
    '打印优化',
    '为打印输出优化的图片处理模板',
    'print',
    '{
        "resize": {
            "enabled": true,
            "width": 3000,
            "height": 3000,
            "maintainAspectRatio": true
        },
        "compression": {
            "quality": 100,
            "format": "png",
            "optimize": false
        },
        "filters": {
            "brightness": 1.0,
            "contrast": 1.0,
            "saturation": 1.0
        }
    }',
    true,
    true,
    0
);

-- 插入订阅计划配置
INSERT INTO public.subscription_plans (
    id,
    name,
    description,
    price_monthly,
    price_yearly,
    features,
    limits,
    is_active,
    created_at,
    updated_at
) VALUES (
    uuid_generate_v4(),
    'Free',
    '免费计划，适合个人用户',
    0,
    0,
    '["基础图片处理", "每月100张图片", "1GB存储空间", "基础支持"]',
    '{
        "max_images_per_month": 100,
        "max_storage_gb": 1,
        "max_api_calls_per_month": 1000,
        "max_file_size_mb": 10
    }',
    true,
    NOW(),
    NOW()
),
(
    uuid_generate_v4(),
    'Basic',
    '基础计划，适合小型团队',
    9.99,
    99.99,
    '["高级图片处理", "每月1000张图片", "10GB存储空间", "优先支持", "批量处理"]',
    '{
        "max_images_per_month": 1000,
        "max_storage_gb": 10,
        "max_api_calls_per_month": 10000,
        "max_file_size_mb": 50
    }',
    true,
    NOW(),
    NOW()
),
(
    uuid_generate_v4(),
    'Pro',
    '专业计划，适合企业用户',
    29.99,
    299.99,
    '["专业图片处理", "每月10000张图片", "100GB存储空间", "24/7支持", "API访问", "自定义模板"]',
    '{
        "max_images_per_month": 10000,
        "max_storage_gb": 100,
        "max_api_calls_per_month": 100000,
        "max_file_size_mb": 100
    }',
    true,
    NOW(),
    NOW()
),
(
    uuid_generate_v4(),
    'Enterprise',
    '企业计划，适合大型组织',
    99.99,
    999.99,
    '["企业级图片处理", "无限制图片", "1TB存储空间", "专属支持", "高级API", "团队协作", "自定义集成"]',
    '{
        "max_images_per_month": 999999,
        "max_storage_gb": 1000,
        "max_api_calls_per_month": 999999,
        "max_file_size_mb": 500
    }',
    true,
    NOW(),
    NOW()
);

-- 插入系统配置
INSERT INTO public.system_configs (
    key,
    value,
    description,
    created_at,
    updated_at
) VALUES (
    'app_name',
    'Rust PNG SaaS',
    '应用名称',
    NOW(),
    NOW()
),
(
    'app_version',
    '1.0.0',
    '应用版本',
    NOW(),
    NOW()
),
(
    'maintenance_mode',
    'false',
    '维护模式',
    NOW(),
    NOW()
),
(
    'max_file_size_mb',
    '50',
    '最大文件大小(MB)',
    NOW(),
    NOW()
),
(
    'allowed_file_types',
    'image/png,image/jpeg,image/jpg,image/gif,image/webp,image/avif',
    '允许的文件类型',
    NOW(),
    NOW()
),
(
    'default_processing_timeout',
    '300',
    '默认处理超时时间(秒)',
    NOW(),
    NOW()
);
