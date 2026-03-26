<template>
  <div class="d-flex flex-column align-items-center justify-content-center vh-100 user-bg">

    <button @click="toggle" class="theme-toggle theme-toggle--float">
      <span class="theme-toggle-icon">{{ isDark ? '☀' : '☾' }}</span>
      <span class="theme-toggle-label">{{ isDark ? 'Claro' : 'Oscuro' }}</span>
    </button>

    <div class="glass-user p-5 text-center" style="width: 100%; max-width: 400px;">

      <div class="mb-5">
        <h1 class="logo mb-2 text-accent" style="font-size: 1.1rem;">stack-underflow</h1>
        <div style="height: 1px; background: linear-gradient(90deg, transparent, var(--accent), transparent); opacity: 0.30; margin: 0 auto; width: 80%;"></div>
        <p class="mt-3 opacity-55" style="font-size: 0.78rem; letter-spacing: 0.08em; text-transform: uppercase; font-family: 'Inter', sans-serif;">Sistema de Gestión de Tickets</p>
      </div>

      <form @submit.prevent="handleLogin" class="text-start">
        <div class="mb-3">
          <label class="form-label">Usuario</label>
          <input type="text" class="form-control" v-model="username" placeholder="primer nombre, sin acentos" @input="clearError">
        </div>
        <div class="mb-4">
          <label class="form-label">Contraseña</label>
          <input type="password" class="form-control" v-model="password" placeholder="apellidos, sin acentos" @input="clearError">
        </div>
        
        <div v-if="errorMessage" class="text-danger small fw-bold mb-3 text-center" style="font-family: 'Inter', sans-serif;">
          {{ errorMessage }}
        </div>

        <button type="submit" class="btn btn-dark w-100 py-2 rounded-pill mb-3" :disabled="!isFormValid">
          Ingresar al Sistema
        </button>
        <RouterLink to="/forgot-password" class="btn btn-outline-dark w-100 py-2 rounded-pill text-center d-block" style="text-decoration: none;">
          Olvidé mi Contraseña
        </RouterLink>
      </form>
    </div>

  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { store } from '../store'
import { useTheme } from '../composables/useTheme'

const router   = useRouter()
const username = ref('')
const password = ref('')
const errorMessage = ref('')
const { isDark, toggle, init } = useTheme()

onMounted(() => init())

// Validación en tiempo real
const isFormValid = computed(() => username.value.trim() !== '' && password.value.trim() !== '')

const clearError = () => errorMessage.value = ''

const handleLogin = () => {
  const role = store.login(username.value, password.value)
  if      (role === 'admin') router.push('/admin')
  else if (role === 'user')  router.push('/user')
  else errorMessage.value = "Credenciales incorrectas. Verifica tus datos."
}
</script>