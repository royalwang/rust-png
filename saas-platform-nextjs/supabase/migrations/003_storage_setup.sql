-- 创建存储桶
INSERT INTO storage.buckets (id, name, public, file_size_limit, allowed_mime_types)
VALUES (
  'images',
  'images',
  true,
  52428800, -- 50MB
  ARRAY['image/png', 'image/jpeg', 'image/jpg', 'image/gif', 'image/webp', 'image/avif']
);

-- 创建存储策略
CREATE POLICY "Users can upload their own images" ON storage.objects
  FOR INSERT WITH CHECK (
    bucket_id = 'images' AND
    auth.uid()::text = (storage.foldername(name))[1]
  );

CREATE POLICY "Users can view their own images" ON storage.objects
  FOR SELECT USING (
    bucket_id = 'images' AND
    auth.uid()::text = (storage.foldername(name))[1]
  );

CREATE POLICY "Users can update their own images" ON storage.objects
  FOR UPDATE USING (
    bucket_id = 'images' AND
    auth.uid()::text = (storage.foldername(name))[1]
  );

CREATE POLICY "Users can delete their own images" ON storage.objects
  FOR DELETE USING (
    bucket_id = 'images' AND
    auth.uid()::text = (storage.foldername(name))[1]
  );

-- 公开图片可以被所有人查看
CREATE POLICY "Public images are viewable by everyone" ON storage.objects
  FOR SELECT USING (
    bucket_id = 'images' AND
    EXISTS (
      SELECT 1 FROM public.images
      WHERE url LIKE '%' || name || '%'
      AND is_public = true
    )
  );

-- 创建存储函数
CREATE OR REPLACE FUNCTION storage.get_file_size(bucket_id text, file_path text)
RETURNS bigint
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
DECLARE
  file_size bigint;
BEGIN
  SELECT size INTO file_size
  FROM storage.objects
  WHERE storage.objects.bucket_id = storage.get_file_size.bucket_id
    AND storage.objects.name = storage.get_file_size.file_path;
  
  RETURN file_size;
END;
$$;

-- 创建存储清理函数
CREATE OR REPLACE FUNCTION storage.cleanup_orphaned_files()
RETURNS void
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
BEGIN
  -- 删除没有对应数据库记录的存储文件
  DELETE FROM storage.objects
  WHERE bucket_id = 'images'
    AND NOT EXISTS (
      SELECT 1 FROM public.images
      WHERE images.url LIKE '%' || storage.objects.name || '%'
    );
END;
$$;

-- 创建存储统计函数
CREATE OR REPLACE FUNCTION storage.get_user_storage_usage(user_id uuid)
RETURNS bigint
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
DECLARE
  total_size bigint;
BEGIN
  SELECT COALESCE(SUM(size), 0) INTO total_size
  FROM storage.objects
  WHERE bucket_id = 'images'
    AND (storage.foldername(name))[1] = user_id::text;
  
  RETURN total_size;
END;
$$;

-- 创建存储触发器
CREATE OR REPLACE FUNCTION storage.update_user_storage_usage()
RETURNS trigger
LANGUAGE plpgsql
SECURITY DEFINER
AS $$
DECLARE
  user_id uuid;
  file_size bigint;
BEGIN
  -- 获取用户ID
  user_id := (storage.foldername(NEW.name))[1]::uuid;
  
  -- 获取文件大小
  file_size := NEW.size;
  
  -- 更新用户存储使用量
  IF TG_OP = 'INSERT' THEN
    UPDATE public.profiles
    SET usage_storage_used = usage_storage_used + file_size
    WHERE id = user_id;
  ELSIF TG_OP = 'UPDATE' THEN
    UPDATE public.profiles
    SET usage_storage_used = usage_storage_used - OLD.size + NEW.size
    WHERE id = user_id;
  ELSIF TG_OP = 'DELETE' THEN
    UPDATE public.profiles
    SET usage_storage_used = usage_storage_used - OLD.size
    WHERE id = user_id;
  END IF;
  
  RETURN COALESCE(NEW, OLD);
END;
$$;

-- 创建存储触发器
CREATE TRIGGER storage_update_user_usage
  AFTER INSERT OR UPDATE OR DELETE ON storage.objects
  FOR EACH ROW
  EXECUTE FUNCTION storage.update_user_storage_usage();

-- 创建存储索引
CREATE INDEX IF NOT EXISTS storage_objects_bucket_id_idx ON storage.objects(bucket_id);
CREATE INDEX IF NOT EXISTS storage_objects_name_idx ON storage.objects(name);
CREATE INDEX IF NOT EXISTS storage_objects_created_at_idx ON storage.objects(created_at);

-- 创建存储视图
CREATE VIEW storage.user_files AS
SELECT 
  o.id,
  o.name,
  o.bucket_id,
  o.owner,
  o.created_at,
  o.updated_at,
  o.last_accessed_at,
  o.metadata,
  o.path_tokens,
  o.version,
  o.owner_id,
  o.size,
  (storage.foldername(o.name))[1] as user_id
FROM storage.objects o
WHERE o.bucket_id = 'images';

-- 创建存储统计视图
CREATE VIEW storage.user_storage_stats AS
SELECT 
  (storage.foldername(name))[1] as user_id,
  COUNT(*) as file_count,
  SUM(size) as total_size,
  AVG(size) as average_size,
  MAX(size) as largest_file,
  MIN(created_at) as first_upload,
  MAX(created_at) as last_upload
FROM storage.objects
WHERE bucket_id = 'images'
GROUP BY (storage.foldername(name))[1];
