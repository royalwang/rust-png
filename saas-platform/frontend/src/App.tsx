import { Routes, Route } from 'react-router-dom'
import { Helmet } from 'react-helmet-async'
import { useAuthStore } from './store/authStore'

// 页面组件
import { HomePage } from './pages/HomePage'
import { LoginPage } from './pages/LoginPage'
import { RegisterPage } from './pages/RegisterPage'
import { DashboardPage } from './pages/DashboardPage'
import { EditorPage } from './pages/EditorPage'
import { GalleryPage } from './pages/GalleryPage'
import { SettingsPage } from './pages/SettingsPage'
import { PricingPage } from './pages/PricingPage'
import { AboutPage } from './pages/AboutPage'
import { ContactPage } from './pages/ContactPage'
import { NotFoundPage } from './pages/NotFoundPage'

// 布局组件
import { Layout } from './components/Layout'
import { ProtectedRoute } from './components/ProtectedRoute'
import { LoadingSpinner } from './components/LoadingSpinner'

function App() {
  const { isAuthenticated, isLoading } = useAuthStore()

  if (isLoading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <LoadingSpinner size="lg" />
      </div>
    )
  }

  return (
    <>
      <Helmet>
        <title>Rust PNG SaaS Platform</title>
        <meta name="description" content="基于Rust PNG JS构建的高性能图片处理SaaS平台" />
      </Helmet>
      
      <Routes>
        {/* 公开路由 */}
        <Route path="/" element={<Layout />}>
          <Route index element={<HomePage />} />
          <Route path="login" element={<LoginPage />} />
          <Route path="register" element={<RegisterPage />} />
          <Route path="pricing" element={<PricingPage />} />
          <Route path="about" element={<AboutPage />} />
          <Route path="contact" element={<ContactPage />} />
        </Route>

        {/* 受保护的路由 */}
        <Route
          path="/dashboard"
          element={
            <ProtectedRoute>
              <Layout />
            </ProtectedRoute>
          }
        >
          <Route index element={<DashboardPage />} />
          <Route path="editor" element={<EditorPage />} />
          <Route path="gallery" element={<GalleryPage />} />
          <Route path="settings" element={<SettingsPage />} />
        </Route>

        {/* 404页面 */}
        <Route path="*" element={<NotFoundPage />} />
      </Routes>
    </>
  )
}

export default App
