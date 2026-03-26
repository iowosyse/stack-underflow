<template>
  <div class="d-flex flex-column align-items-center justify-content-center vh-100 user-bg">

    <!-- Theme Toggle Flotante -->
    <button @click="toggle" class="theme-toggle theme-toggle--float">
      <span class="theme-toggle-icon">{{ isDark ? '☀' : '☾' }}</span>
      <span class="theme-toggle-label">{{ isDark ? 'Claro' : 'Oscuro' }}</span>
    </button>

    <div class="glass-user p-5 text-center" style="width: 100%; max-width: 420px;">

      <div class="mb-5">
        <h2 class="h4 fw-bold mb-2 text-accent">Recuperar Acceso</h2>
        <div style="height: 1px; background: linear-gradient(90deg, transparent, var(--accent), transparent); opacity: 0.30; margin: 0 auto; width: 60%;"></div>
        <p class="small mt-3 opacity-55">
          Ingresa tus datos para recibir un enlace de restablecimiento en tu correo corporativo.
        </p>
      </div>

      <form @submit.prevent="handleRecover" class="text-start">
        <div class="mb-3">
          <label class="form-label">ID de Empleado / Matrícula</label>
          <input type="text" class="form-control" v-model="employeeId" placeholder="Ej. EMP-1024">
        </div>
        <div class="mb-4">
          <label class="form-label">Correo Corporativo</label>
          <input type="email" class="form-control" v-model="email" placeholder="usuario@empresa.com">
        </div>
        <button type="submit" class="btn btn-primary w-100 py-2 rounded-pill mb-3">
          Enviar Enlace
        </button>
        <RouterLink to="/" class="btn btn-outline-dark w-100 py-2 rounded-pill">
          ← Volver al Login
        </RouterLink>
      </form>
    </div>

  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useTheme } from '../composables/useTheme'

const router     = useRouter()
const employeeId = ref('')
const email      = ref('')
const { isDark, toggle, init } = useTheme()

onMounted(() => init())

const handleRecover = () => {
  if (employeeId.value && email.value) {
    alert("Si los datos son correctos, recibirás un correo en los próximos minutos.")
    router.push('/')
  } else {
    alert("Por favor llena todos los campos.")
  }
}
</script>
