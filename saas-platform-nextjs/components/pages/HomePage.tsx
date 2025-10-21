'use client'

import { useState } from 'react'
import { Button } from '@/components/ui/Button'
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/Card'
import { Badge } from '@/components/ui/Badge'
import { FileUpload } from '@/components/ui/FileUpload'
import { ImagePreview } from '@/components/ui/ImagePreview'
import { 
  Upload, 
  Zap, 
  Shield, 
  Globe, 
  ArrowRight, 
  CheckCircle,
  Star,
  Users,
  Image as ImageIcon,
  Clock
} from 'lucide-react'
import { cn } from '@/lib/utils'

export function HomePage() {
  const [uploadedFiles, setUploadedFiles] = useState<File[]>([])

  const handleFileSelect = (files: File[]) => {
    setUploadedFiles(prev => [...prev, ...files])
  }

  const handleRemoveFile = (index: number) => {
    setUploadedFiles(prev => prev.filter((_, i) => i !== index))
  }

  const features = [
    {
      icon: Zap,
      title: '高性能处理',
      description: '基于Rust和WebAssembly，处理速度提升10倍',
    },
    {
      icon: Shield,
      title: '安全可靠',
      description: '端到端加密，保护您的图片隐私',
    },
    {
      icon: Globe,
      title: '云端处理',
      description: '强大的云端处理能力，支持大规模批量处理',
    },
  ]

  const stats = [
    { label: '处理图片', value: '1M+', icon: ImageIcon },
    { label: '活跃用户', value: '10K+', icon: Users },
    { label: '处理时间', value: '<1s', icon: Clock },
    { label: '用户评分', value: '4.9/5', icon: Star },
  ]

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 via-white to-indigo-50 dark:from-gray-900 dark:via-gray-800 dark:to-gray-900">
      {/* 导航栏 */}
      <nav className="border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
        <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
          <div className="flex h-16 items-center justify-between">
            <div className="flex items-center">
              <h1 className="text-xl font-bold">Rust PNG SaaS</h1>
            </div>
            <div className="flex items-center space-x-4">
              <Button variant="ghost">登录</Button>
              <Button>开始使用</Button>
            </div>
          </div>
        </div>
      </nav>

      {/* 主要内容 */}
      <main>
        {/* 英雄区域 */}
        <section className="relative py-20 sm:py-32">
          <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
            <div className="text-center">
              <h1 className="text-4xl font-bold tracking-tight text-gray-900 dark:text-white sm:text-6xl">
                高性能图片处理
                <span className="block text-primary">SaaS平台</span>
              </h1>
              <p className="mx-auto mt-6 max-w-2xl text-lg leading-8 text-gray-600 dark:text-gray-300">
                基于Rust PNG JS构建，提供企业级图片处理服务。支持批量处理、实时预览、多种格式转换。
              </p>
              <div className="mt-10 flex items-center justify-center gap-x-6">
                <Button size="lg" className="text-lg px-8 py-4">
                  立即开始
                  <ArrowRight className="ml-2 h-5 w-5" />
                </Button>
                <Button variant="outline" size="lg" className="text-lg px-8 py-4">
                  查看演示
                </Button>
              </div>
            </div>
          </div>
        </section>

        {/* 统计信息 */}
        <section className="py-16 bg-muted/50">
          <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
            <div className="grid grid-cols-2 gap-8 sm:grid-cols-4">
              {stats.map((stat, index) => {
                const Icon = stat.icon
                return (
                  <div key={index} className="text-center">
                    <div className="flex justify-center">
                      <div className="rounded-full bg-primary/10 p-3">
                        <Icon className="h-6 w-6 text-primary" />
                      </div>
                    </div>
                    <p className="mt-2 text-3xl font-bold text-gray-900 dark:text-white">
                      {stat.value}
                    </p>
                    <p className="text-sm text-gray-600 dark:text-gray-400">
                      {stat.label}
                    </p>
                  </div>
                )
              })}
            </div>
          </div>
        </section>

        {/* 文件上传区域 */}
        <section className="py-16">
          <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
            <div className="text-center mb-12">
              <h2 className="text-3xl font-bold text-gray-900 dark:text-white">
                开始处理您的图片
              </h2>
              <p className="mt-4 text-lg text-gray-600 dark:text-gray-300">
                拖拽图片到下方区域，或点击选择文件
              </p>
            </div>

            <div className="max-w-2xl mx-auto">
              <FileUpload
                onFileSelect={handleFileSelect}
                maxFiles={10}
                maxSize={50 * 1024 * 1024}
                className="mb-8"
              />

              {/* 上传的图片预览 */}
              {uploadedFiles.length > 0 && (
                <div className="mt-8">
                  <h3 className="text-lg font-semibold mb-4">已选择的图片</h3>
                  <div className="grid grid-cols-2 gap-4 sm:grid-cols-3 lg:grid-cols-4">
                    {uploadedFiles.map((file, index) => (
                      <ImagePreview
                        key={index}
                        file={file}
                        onRemove={() => handleRemoveFile(index)}
                        className="w-full"
                      />
                    ))}
                  </div>
                  <div className="mt-6 flex justify-center">
                    <Button size="lg" className="px-8">
                      <Upload className="mr-2 h-5 w-5" />
                      开始处理
                    </Button>
                  </div>
                </div>
              )}
            </div>
          </div>
        </section>

        {/* 功能特性 */}
        <section className="py-16 bg-muted/50">
          <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
            <div className="text-center mb-12">
              <h2 className="text-3xl font-bold text-gray-900 dark:text-white">
                为什么选择我们
              </h2>
              <p className="mt-4 text-lg text-gray-600 dark:text-gray-300">
                基于最新技术栈，提供卓越的图片处理体验
              </p>
            </div>

            <div className="grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-3">
              {features.map((feature, index) => {
                const Icon = feature.icon
                return (
                  <Card key={index} className="text-center">
                    <CardHeader>
                      <div className="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-full bg-primary/10">
                        <Icon className="h-6 w-6 text-primary" />
                      </div>
                      <CardTitle>{feature.title}</CardTitle>
                    </CardHeader>
                    <CardContent>
                      <CardDescription>{feature.description}</CardDescription>
                    </CardContent>
                  </Card>
                )
              })}
            </div>
          </div>
        </section>

        {/* 定价方案 */}
        <section className="py-16">
          <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
            <div className="text-center mb-12">
              <h2 className="text-3xl font-bold text-gray-900 dark:text-white">
                选择适合您的方案
              </h2>
              <p className="mt-4 text-lg text-gray-600 dark:text-gray-300">
                灵活的定价方案，满足不同需求
              </p>
            </div>

            <div className="grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-4">
              {[
                {
                  name: '免费版',
                  price: '¥0',
                  description: '适合个人用户',
                  features: ['每月100张图片', '1GB存储空间', '基础处理功能'],
                  popular: false,
                },
                {
                  name: '基础版',
                  price: '¥29',
                  description: '适合小型团队',
                  features: ['每月1000张图片', '10GB存储空间', '高级处理功能', '优先支持'],
                  popular: true,
                },
                {
                  name: '专业版',
                  price: '¥99',
                  description: '适合企业用户',
                  features: ['每月10000张图片', '100GB存储空间', 'API访问', '自定义模板'],
                  popular: false,
                },
                {
                  name: '企业版',
                  price: '¥299',
                  description: '适合大型组织',
                  features: ['无限制图片', '1TB存储空间', '专属支持', '团队协作'],
                  popular: false,
                },
              ].map((plan, index) => (
                <Card key={index} className={cn(
                  'relative',
                  plan.popular && 'ring-2 ring-primary'
                )}>
                  {plan.popular && (
                    <div className="absolute -top-3 left-1/2 transform -translate-x-1/2">
                      <Badge className="bg-primary text-primary-foreground">
                        最受欢迎
                      </Badge>
                    </div>
                  )}
                  <CardHeader className="text-center">
                    <CardTitle>{plan.name}</CardTitle>
                    <div className="mt-4">
                      <span className="text-4xl font-bold">{plan.price}</span>
                      <span className="text-muted-foreground">/月</span>
                    </div>
                    <CardDescription>{plan.description}</CardDescription>
                  </CardHeader>
                  <CardContent>
                    <ul className="space-y-3">
                      {plan.features.map((feature, featureIndex) => (
                        <li key={featureIndex} className="flex items-center">
                          <CheckCircle className="h-4 w-4 text-green-500 mr-2" />
                          <span className="text-sm">{feature}</span>
                        </li>
                      ))}
                    </ul>
                    <Button 
                      className={cn(
                        'w-full mt-6',
                        plan.popular ? 'bg-primary' : 'bg-secondary'
                      )}
                      variant={plan.popular ? 'default' : 'secondary'}
                    >
                      选择方案
                    </Button>
                  </CardContent>
                </Card>
              ))}
            </div>
          </div>
        </section>
      </main>

      {/* 页脚 */}
      <footer className="border-t bg-background">
        <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 py-12">
          <div className="text-center">
            <h3 className="text-lg font-semibold">Rust PNG SaaS</h3>
            <p className="mt-2 text-sm text-muted-foreground">
              基于Rust PNG JS构建的高性能图片处理平台
            </p>
            <div className="mt-4 flex justify-center space-x-6">
              <Button variant="ghost" size="sm">隐私政策</Button>
              <Button variant="ghost" size="sm">服务条款</Button>
              <Button variant="ghost" size="sm">联系我们</Button>
            </div>
          </div>
        </div>
      </footer>
    </div>
  )
}
