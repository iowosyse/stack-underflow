import { createRouter, createWebHistory } from 'vue-router'
import LoginView from '../views/LoginView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    { path: '/', name: 'login', component: LoginView },
    { path: '/forgot-password', name: 'forgot-password', component: () => import('../views/ForgotPasswordView.vue') },
    { path: '/admin', name: 'admin', component: () => import('../views/AdminView.vue') },
    // NUEVA RUTA AQUÍ:
    { path: '/admin/nuevo-usuario', name: 'new-user', component: () => import('../views/NewUserView.vue') },
    { path: '/user', name: 'user', component: () => import('../views/UserView.vue') },
    { path: '/knowledge-base', name: 'knowledge-base', component: () => import('../views/KnowledgeBaseView.vue') }
  ]
})
export default router