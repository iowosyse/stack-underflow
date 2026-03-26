<template>
  <div class="d-flex flex-column vh-100 admin-bg" id="wrapper">
    <div class="d-flex flex-grow-1 overflow-hidden p-3 gap-3">

      <div class="glass-admin p-4 d-flex flex-column justify-content-between flex-shrink-0" style="width: 280px;">
        <div>
          <div class="logo mb-5 text-accent">SYS_ADMIN</div>
          <nav class="list-group list-group-flush">
            <RouterLink to="/admin" class="list-group-item active" style="text-decoration: none;">
              ← Volver al Panel
            </RouterLink>
          </nav>
        </div>

        <div>
          <button @click="toggle" class="theme-toggle mb-3">
            <span class="theme-toggle-icon">{{ isDark ? '☀' : '☾' }}</span>
            <span class="theme-toggle-label">{{ isDark ? 'Modo Claro' : 'Modo Oscuro' }}</span>
          </button>
          <div class="small mb-2 opacity-55 text-capitalize">
            Admin: {{ store.currentUser?.fullName }}
          </div>
          <button @click="handleLogout" class="btn btn-outline-light w-100 rounded-pill">
            Cerrar Sesión
          </button>
        </div>
      </div>

      <div class="glass-admin container-fluid p-5 d-flex flex-column overflow-auto w-100 align-items-center">
        <div class="w-100 mx-auto" style="max-width: 500px; margin-top: 8vh;">

          <div class="d-flex justify-content-between align-items-center mb-4">
            <h2 class="h3 m-0 fw-bold">Nuevo Usuario</h2>
            <RouterLink to="/admin" class="btn btn-outline-light rounded-pill" style="text-decoration: none;">
              Cancelar
            </RouterLink>
          </div>

          <div class="panel-inner p-5">
            <form @submit.prevent="crearUsuario">
              <div class="mb-5">
                <label class="form-label">Nombre Completo del Usuario</label>
                <input
                  type="text"
                  class="form-control form-control-lg"
                  v-model="newUserName"
                  placeholder="Ej. Juan Pérez Gómez"
                  autofocus
                >

                <div class="info-hint mt-4 p-3">
                  <p class="fw-bold mb-1 text-accent" style="font-size: 0.75rem; letter-spacing: 0.08em; text-transform: uppercase; font-family: 'Inter', sans-serif;">
                    Generación Automática
                  </p>
                  <p class="small mb-0 opacity-55" style="font-family: 'Inter', sans-serif;">
                    Usuario: primer nombre. &nbsp;Contraseña: apellidos.
                    <br>Todo en minúsculas y sin acentos.
                  </p>
                </div>
              </div>

              <div v-if="successMessage" class="text-accent text-center fw-bold small mb-3" style="font-family: 'Inter', sans-serif;">
                {{ successMessage }}
              </div>

              <button v-else type="submit" class="btn btn-primary w-100 py-3 fw-bold rounded-pill" :disabled="!isFormValid">
                Registrar Usuario
              </button>
            </form>
          </div>

        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { store } from '../store'
import { useTheme } from '../composables/useTheme'

const router      = useRouter()
const newUserName = ref('')
const successMessage = ref('')
const { isDark, toggle, init } = useTheme()

onMounted(() => init())

// Validación: El botón requiere que el nombre tenga al menos 3 caracteres
const isFormValid = computed(() => newUserName.value.trim().length >= 3)

const crearUsuario = () => {
  store.addUser(newUserName.value, 'user')
  successMessage.value = `Usuario registrado. Redirigiendo...`
  
  // Redirige al panel después de 1.5 segundos
  setTimeout(() => {
    router.push('/admin')
  }, 1500)
}

const handleLogout = () => { store.logout(); router.push('/') }
</script>