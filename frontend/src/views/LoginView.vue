<template>
  <div class="d-flex flex-column align-items-center justify-content-center vh-100 user-bg">

    <button @click="toggle" class="theme-toggle theme-toggle--float">
      <span class="theme-toggle-icon">{{ isDark ? '☀' : '☾' }}</span>
      <span class="theme-toggle-label">{{ isDark ? 'Claro' : 'Oscuro' }}</span>
    </button>

    <div class="glass-user p-5 text-center auth-card">

      <div class="mb-5">
        <h1 class="logo logo-auth mb-2 text-accent">stack-underflow</h1>
        <div class="auth-divider"></div>
        <p class="auth-subtitle mt-3 mb-0">Sistema de Gestión de Tickets</p>
      </div>

      <form @submit.prevent="handleLogin" class="text-start">
        <div class="mb-3">
          <label class="form-label">Correo Electrónico</label>
          <input type="email" class="form-control" v-model="email"
                placeholder="usuario@morelia.tecnm.mx">
        </div>
        <div class="mb-4">
          <label class="form-label">Contraseña</label>
          <input type="password" class="form-control" v-model="password"
                 placeholder="contarseña">
        </div>
        <button type="submit" class="btn btn-dark w-100 py-2 rounded-pill mb-3" :disabled="cargando">
          {{ cargando ? 'Verificando...' : 'Ingresar al Sistema' }}
        </button>
        <RouterLink to="/forgot-password" class="btn btn-outline-dark w-100 py-2 rounded-pill">
          Olvidé mi Contraseña
        </RouterLink>
      </form>

    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useTheme } from '../composables/useTheme'

const router   = useRouter()
const email    = ref('') // Ahora usamos una variable de email directa
const password = ref('')
const cargando = ref(false)
const { isDark, toggle, init } = useTheme()

onMounted(() => init())

const handleLogin = async () => {
  if (!email.value || !password.value) {
    alert('Por favor ingresa tu correo y contraseña.')
    return
  }

  cargando.value = true

  try {
    const respuesta = await fetch('/api/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        // Enviamos el correo tal cual lo escribió el usuario
        email: email.value.trim().toLowerCase(),
        password: password.value.trim()
      })
    })

    if (respuesta.ok) {
      const datos = await respuesta.json()
      localStorage.setItem('token', datos.token)
      localStorage.setItem('rol', datos.rol)
      localStorage.setItem('usuario_id', datos.id)
      if (datos.nombre) localStorage.setItem('usuario_nombre', datos.nombre)

      // Redirección por roles
      if (datos.rol === 'administrador') router.push('/ejecutivo')
      else if (datos.rol === 'soporte') router.push('/soporte')
      else if (datos.rol === 'cliente') router.push('/user')

    } else if (respuesta.status === 401) {
      alert('Correo o contraseña incorrectos.')
    } else {
      alert('Error al conectar con el servidor.')
    }
  } catch (error) {
    console.error('Fallo de red:', error)
  } finally {
    cargando.value = false
  }
}
</script>