import { Helmet } from 'react-helmet-async'
import { Link } from 'react-router-dom'
import { 
  ArrowRight, 
  Zap, 
  Shield, 
  Globe, 
  Smartphone, 
  BarChart3,
  Users,
  Star
} from 'lucide-react'

export function HomePage() {
  return (
    <>
      <Helmet>
        <title>Rust PNG SaaS Platform - 高性能图片处理平台</title>
        <meta name="description" content="基于Rust和WebAssembly的高性能图片处理SaaS平台，提供图片调整、裁剪、滤镜、压缩等功能" />
      </Helmet>

      <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100">
        {/* 导航栏 */}
        <nav className="bg-white/80 backdrop-blur-md border-b border-gray-200 sticky top-0 z-50">
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div className="flex justify-between items-center h-16">
              <div className="flex items-center">
                <div className="flex-shrink-0">
                  <h1 className="text-2xl font-bold gradient-text">Rust PNG SaaS</h1>
                </div>
              </div>
              <div className="flex items-center space-x-4">
                <Link to="/login" className="btn btn-ghost">
                  登录
                </Link>
                <Link to="/register" className="btn btn-primary">
                  注册
                </Link>
              </div>
            </div>
          </div>
        </nav>

        {/* 英雄区域 */}
        <section className="relative py-20 lg:py-32">
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div className="text-center">
              <h1 className="text-4xl md:text-6xl font-bold text-gray-900 mb-6">
                高性能图片处理
                <span className="block gradient-text">SaaS平台</span>
              </h1>
              <p className="text-xl text-gray-600 mb-8 max-w-3xl mx-auto">
                基于Rust和WebAssembly技术，提供比传统JavaScript快3-5倍的图片处理性能。
                支持调整大小、裁剪、滤镜、压缩等丰富功能。
              </p>
              <div className="flex flex-col sm:flex-row gap-4 justify-center">
                <Link to="/register" className="btn btn-primary btn-lg">
                  立即开始
                  <ArrowRight className="ml-2 h-5 w-5" />
                </Link>
                <Link to="/pricing" className="btn btn-outline btn-lg">
                  查看价格
                </Link>
              </div>
            </div>
          </div>
        </section>

        {/* 特性介绍 */}
        <section className="py-20 bg-white">
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div className="text-center mb-16">
              <h2 className="text-3xl md:text-4xl font-bold text-gray-900 mb-4">
                为什么选择我们？
              </h2>
              <p className="text-xl text-gray-600 max-w-2xl mx-auto">
                我们提供业界领先的图片处理技术，让您的图片处理更快、更高效。
              </p>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
              <div className="card p-6 text-center">
                <div className="flex justify-center mb-4">
                  <div className="p-3 bg-blue-100 rounded-full">
                    <Zap className="h-8 w-8 text-blue-600" />
                  </div>
                </div>
                <h3 className="text-xl font-semibold text-gray-900 mb-2">
                  极速处理
                </h3>
                <p className="text-gray-600">
                  基于Rust和WebAssembly技术，处理速度比传统方案快3-5倍。
                </p>
              </div>

              <div className="card p-6 text-center">
                <div className="flex justify-center mb-4">
                  <div className="p-3 bg-green-100 rounded-full">
                    <Shield className="h-8 w-8 text-green-600" />
                  </div>
                </div>
                <h3 className="text-xl font-semibold text-gray-900 mb-2">
                  安全可靠
                </h3>
                <p className="text-gray-600">
                  企业级安全标准，保护您的数据安全和隐私。
                </p>
              </div>

              <div className="card p-6 text-center">
                <div className="flex justify-center mb-4">
                  <div className="p-3 bg-purple-100 rounded-full">
                    <Globe className="h-8 w-8 text-purple-600" />
                  </div>
                </div>
                <h3 className="text-xl font-semibold text-gray-900 mb-2">
                  全球部署
                </h3>
                <p className="text-gray-600">
                  全球CDN加速，无论您在哪里都能享受快速服务。
                </p>
              </div>

              <div className="card p-6 text-center">
                <div className="flex justify-center mb-4">
                  <div className="p-3 bg-orange-100 rounded-full">
                    <Smartphone className="h-8 w-8 text-orange-600" />
                  </div>
                </div>
                <h3 className="text-xl font-semibold text-gray-900 mb-2">
                  响应式设计
                </h3>
                <p className="text-gray-600">
                  完美适配桌面和移动设备，随时随地处理图片。
                </p>
              </div>

              <div className="card p-6 text-center">
                <div className="flex justify-center mb-4">
                  <div className="p-3 bg-red-100 rounded-full">
                    <BarChart3 className="h-8 w-8 text-red-600" />
                  </div>
                </div>
                <h3 className="text-xl font-semibold text-gray-900 mb-2">
                  详细统计
                </h3>
                <p className="text-gray-600">
                  详细的使用统计和性能分析，帮助您优化工作流程。
                </p>
              </div>

              <div className="card p-6 text-center">
                <div className="flex justify-center mb-4">
                  <div className="p-3 bg-indigo-100 rounded-full">
                    <Users className="h-8 w-8 text-indigo-600" />
                  </div>
                </div>
                <h3 className="text-xl font-semibold text-gray-900 mb-2">
                  团队协作
                </h3>
                <p className="text-gray-600">
                  支持团队协作，共享处理模板和批量操作。
                </p>
              </div>
            </div>
          </div>
        </section>

        {/* 性能对比 */}
        <section className="py-20 bg-gray-50">
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div className="text-center mb-16">
              <h2 className="text-3xl md:text-4xl font-bold text-gray-900 mb-4">
                性能对比
              </h2>
              <p className="text-xl text-gray-600 max-w-2xl mx-auto">
                与传统JavaScript方案相比，我们的性能提升显著。
              </p>
            </div>

            <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
              <div className="card p-8">
                <h3 className="text-2xl font-semibold text-gray-900 mb-6">
                  处理速度对比
                </h3>
                <div className="space-y-4">
                  <div className="flex justify-between items-center">
                    <span className="text-gray-600">传统JavaScript</span>
                    <span className="text-gray-900 font-semibold">100%</span>
                  </div>
                  <div className="flex justify-between items-center">
                    <span className="text-gray-600">Rust PNG SaaS</span>
                    <span className="text-green-600 font-semibold">300%</span>
                  </div>
                </div>
                <div className="mt-6 bg-gray-200 rounded-full h-2">
                  <div className="bg-green-500 h-2 rounded-full" style={{ width: '75%' }}></div>
                </div>
              </div>

              <div className="card p-8">
                <h3 className="text-2xl font-semibold text-gray-900 mb-6">
                  内存使用对比
                </h3>
                <div className="space-y-4">
                  <div className="flex justify-between items-center">
                    <span className="text-gray-600">传统JavaScript</span>
                    <span className="text-gray-900 font-semibold">100%</span>
                  </div>
                  <div className="flex justify-between items-center">
                    <span className="text-gray-600">Rust PNG SaaS</span>
                    <span className="text-blue-600 font-semibold">60%</span>
                  </div>
                </div>
                <div className="mt-6 bg-gray-200 rounded-full h-2">
                  <div className="bg-blue-500 h-2 rounded-full" style={{ width: '40%' }}></div>
                </div>
              </div>
            </div>
          </div>
        </section>

        {/* 客户评价 */}
        <section className="py-20 bg-white">
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div className="text-center mb-16">
              <h2 className="text-3xl md:text-4xl font-bold text-gray-900 mb-4">
                客户评价
              </h2>
              <p className="text-xl text-gray-600 max-w-2xl mx-auto">
                来自全球用户的真实反馈
              </p>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
              <div className="card p-6">
                <div className="flex items-center mb-4">
                  <div className="flex text-yellow-400">
                    {[...Array(5)].map((_, i) => (
                      <Star key={i} className="h-5 w-5 fill-current" />
                    ))}
                  </div>
                </div>
                <p className="text-gray-600 mb-4">
                  "处理速度真的很快，比我们之前用的方案快了3倍以上。"
                </p>
                <div className="flex items-center">
                  <div className="w-10 h-10 bg-blue-500 rounded-full flex items-center justify-center text-white font-semibold">
                    A
                  </div>
                  <div className="ml-3">
                    <p className="font-semibold text-gray-900">Alex Chen</p>
                    <p className="text-sm text-gray-600">产品经理</p>
                  </div>
                </div>
              </div>

              <div className="card p-6">
                <div className="flex items-center mb-4">
                  <div className="flex text-yellow-400">
                    {[...Array(5)].map((_, i) => (
                      <Star key={i} className="h-5 w-5 fill-current" />
                    ))}
                  </div>
                </div>
                <p className="text-gray-600 mb-4">
                  "API设计很友好，集成到我们的系统中非常容易。"
                </p>
                <div className="flex items-center">
                  <div className="w-10 h-10 bg-green-500 rounded-full flex items-center justify-center text-white font-semibold">
                    S
                  </div>
                  <div className="ml-3">
                    <p className="font-semibold text-gray-900">Sarah Johnson</p>
                    <p className="text-sm text-gray-600">开发工程师</p>
                  </div>
                </div>
              </div>

              <div className="card p-6">
                <div className="flex items-center mb-4">
                  <div className="flex text-yellow-400">
                    {[...Array(5)].map((_, i) => (
                      <Star key={i} className="h-5 w-5 fill-current" />
                    ))}
                  </div>
                </div>
                <p className="text-gray-600 mb-4">
                  "批量处理功能很强大，大大提高了我们的工作效率。"
                </p>
                <div className="flex items-center">
                  <div className="w-10 h-10 bg-purple-500 rounded-full flex items-center justify-center text-white font-semibold">
                    M
                  </div>
                  <div className="ml-3">
                    <p className="font-semibold text-gray-900">Mike Wang</p>
                    <p className="text-sm text-gray-600">设计师</p>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </section>

        {/* CTA区域 */}
        <section className="py-20 bg-gradient-to-r from-blue-600 to-purple-600">
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
            <h2 className="text-3xl md:text-4xl font-bold text-white mb-4">
              准备开始了吗？
            </h2>
            <p className="text-xl text-blue-100 mb-8 max-w-2xl mx-auto">
              立即注册，享受高性能图片处理服务。免费试用，无需信用卡。
            </p>
            <div className="flex flex-col sm:flex-row gap-4 justify-center">
              <Link to="/register" className="btn bg-white text-blue-600 hover:bg-gray-100 btn-lg">
                免费开始
                <ArrowRight className="ml-2 h-5 w-5" />
              </Link>
              <Link to="/pricing" className="btn border-white text-white hover:bg-white hover:text-blue-600 btn-lg">
                查看价格
              </Link>
            </div>
          </div>
        </section>

        {/* 页脚 */}
        <footer className="bg-gray-900 text-white py-12">
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
            <div className="grid grid-cols-1 md:grid-cols-4 gap-8">
              <div>
                <h3 className="text-lg font-semibold mb-4">产品</h3>
                <ul className="space-y-2">
                  <li><Link to="/features" className="text-gray-400 hover:text-white">功能特性</Link></li>
                  <li><Link to="/pricing" className="text-gray-400 hover:text-white">价格方案</Link></li>
                  <li><Link to="/api" className="text-gray-400 hover:text-white">API文档</Link></li>
                </ul>
              </div>
              <div>
                <h3 className="text-lg font-semibold mb-4">支持</h3>
                <ul className="space-y-2">
                  <li><Link to="/help" className="text-gray-400 hover:text-white">帮助中心</Link></li>
                  <li><Link to="/contact" className="text-gray-400 hover:text-white">联系我们</Link></li>
                  <li><Link to="/status" className="text-gray-400 hover:text-white">服务状态</Link></li>
                </ul>
              </div>
              <div>
                <h3 className="text-lg font-semibold mb-4">公司</h3>
                <ul className="space-y-2">
                  <li><Link to="/about" className="text-gray-400 hover:text-white">关于我们</Link></li>
                  <li><Link to="/blog" className="text-gray-400 hover:text-white">博客</Link></li>
                  <li><Link to="/careers" className="text-gray-400 hover:text-white">招聘</Link></li>
                </ul>
              </div>
              <div>
                <h3 className="text-lg font-semibold mb-4">法律</h3>
                <ul className="space-y-2">
                  <li><Link to="/privacy" className="text-gray-400 hover:text-white">隐私政策</Link></li>
                  <li><Link to="/terms" className="text-gray-400 hover:text-white">服务条款</Link></li>
                  <li><Link to="/cookies" className="text-gray-400 hover:text-white">Cookie政策</Link></li>
                </ul>
              </div>
            </div>
            <div className="border-t border-gray-800 mt-8 pt-8 text-center">
              <p className="text-gray-400">
                © 2024 Rust PNG SaaS Platform. All rights reserved.
              </p>
            </div>
          </div>
        </footer>
      </div>
    </>
  )
}
