import { createRouter, createWebHistory } from 'vue-router'

// Importación de tus vistas (¡Asegúrate de haber borrado AdminView.vue!)
import LoginView from '../views/LoginView.vue'
import ForgotPasswordView from '../views/ForgotPasswordView.vue'
import UserView from '../views/UserView.vue'
import KnowledgeBaseView from '../views/KnowledgeBaseView.vue'
import SoporteView from '../views/SoporteView.vue'
import EjecutivoView from '../views/EjecutivoView.vue'
import NuevoUsuarioView from '../views/NuevoUsuarioView.vue'
import RegistroView from '../views/RegistroView.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { 
      path: '/', 
      name: 'login', 
      component: LoginView 
    },
    {
      path: '/forgot-password',
      name: 'forgot-password',
      component: ForgotPasswordView
    },
    {
      path: '/registro',
      name: 'registro',
      component: RegistroView
    },
    { 
      path: '/user', 
      name: 'user', 
      component: UserView, 
      meta: { requiresAuth: true, allowedRole: 'cliente' } 
    },
    { 
      path: '/knowledge-base', 
      name: 'knowledge-base', 
      component: KnowledgeBaseView, 
      meta: { requiresAuth: true, allowedRole: 'cliente' } 
    },
    { 
      path: '/soporte', 
      name: 'soporte', 
      component: SoporteView, 
      meta: { requiresAuth: true, allowedRole: 'soporte' } 
    },
    { 
      path: '/ejecutivo', 
      name: 'ejecutivo', 
      component: EjecutivoView, 
      meta: { requiresAuth: true, allowedRole: 'administrador' } 
    },
    { 
      path: '/soporte/nuevo-usuario', 
      name: 'nuevo-usuario', 
      component: NuevoUsuarioView, 
      meta: { requiresAuth: true, allowedRoles: ['soporte', 'ejecutivo']} 
    }
  ]
})

// GUARDIÁN DE NAVEGACIÓN (Modernizado para Vue Router v4 sin 'next()')
router.beforeEach((to, from) => {
  const token = localStorage.getItem('token')
  const rolUsuario = localStorage.getItem('rol') 

  // Regla 1: Si requiere login y no hay token, lo mandamos al inicio
  if (to.meta.requiresAuth && !token) {
    return '/'
  }

  // Regla 2: Si ya inició sesión e intenta ir al Login o Registro, lo regresamos a su panel
  if ((to.path === '/' || to.path === '/registro') && token) {
    if (rolUsuario === 'administrador') return '/ejecutivo'
    if (rolUsuario === 'soporte') return '/soporte'
    if (rolUsuario === 'cliente') return '/user'
  }

  // Regla 3: Si intenta entrar a una vista que no es de su rol
  if (to.meta.allowedRole && to.meta.allowedRole !== rolUsuario) {
    alert('Acceso denegado: Tu nivel de credenciales no permite ver esta área.')
    
    // Lo forzamos a regresar a su carril
    if (rolUsuario === 'administrador') return '/ejecutivo'
    if (rolUsuario === 'soporte') return '/soporte'
    if (rolUsuario === 'cliente') return '/user'
    
    // Por seguridad, si el rol está corrupto, lo saca
    return '/'
  }

  // Regla 4: Todo en orden, puede pasar
  return true
})

export default router