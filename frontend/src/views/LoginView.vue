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
          <label class="form-label">Usuario</label>
          <input type="text" class="form-control" v-model="username"
                 placeholder="primer nombre, sin acentos">
        </div>
        <div class="mb-4">
          <label class="form-label">Contraseña</label>
          <input type="password" class="form-control" v-model="password"
                 placeholder="apellidos, sin acentos">
        </div>
        <button type="submit" class="btn btn-dark w-100 py-2 rounded-pill mb-3">
          Ingresar al Sistema
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
import { store } from '../store'
import { useTheme } from '../composables/useTheme'

const router   = useRouter()
const username = ref('')
const password = ref('')
const { isDark, toggle, init } = useTheme()

onMounted(() => init())

const handleLogin = () => {
  const role = store.login(username.value, password.value)
  if      (role === 'admin') router.push('/admin')
  else if (role === 'user')  router.push('/user')
  else alert("Credenciales incorrectas. (Recuerda: minúsculas y sin acentos)")
}
</script>
