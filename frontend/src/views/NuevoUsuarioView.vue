<template>
  <div class="d-flex flex-column vh-100 admin-bg">
    <div class="d-flex flex-grow-1 overflow-hidden p-3 gap-3">

      <!-- SIDEBAR -->
      <aside class="glass-admin d-flex flex-column flex-shrink-0" style="width: 256px;">
        <div class="sidebar-header">
          <div class="logo sidebar-logo text-accent">IT_SOPORTE</div>
          <div class="sidebar-user-name text-capitalize">{{ nombreUsuario }}</div>
        </div>

        <nav class="sidebar-nav">
          <div class="nav-group-label">Navegación</div>
          <RouterLink to="/soporte" class="list-group-item active"><span class="nav-icon">←</span> Volver al Panel</RouterLink>
        </nav>

        <div class="sidebar-footer">
          <button @click="toggle" class="theme-toggle mb-3">
            <span>{{ isDark ? '☀' : '☾' }}</span><span>{{ isDark ? 'Modo Claro' : 'Modo Oscuro' }}</span>
          </button>
          <button @click="handleLogout" class="btn btn-outline-light w-100">Cerrar Sesión</button>
        </div>
      </aside>

      <!-- CONTENIDO -->
      <main class="glass-admin d-flex flex-column overflow-auto w-100 content-main align-items-center">
        <div class="new-user-container mx-auto">
          <div class="d-flex justify-content-between align-items-center mb-4">
            <h2 class="h4 fw-bold mb-0">Nuevo Usuario</h2>
            <RouterLink to="/soporte" class="btn btn-outline-light">Cancelar</RouterLink>
          </div>

          <div class="panel-inner p-5">
            <form @submit.prevent="crearUsuario">
              <div class="mb-5">
                <label class="form-label">Nombre Completo del Usuario</label>
                <input type="text" class="form-control form-control-lg" v-model="newUserName" placeholder="Ej. Juan Pérez Gómez" autofocus>
                
                <div class="info-hint mt-4 p-3">
                  <p class="hint-label fw-bold mb-1">Generación Automática</p>
                  <p class="small mb-0 opacity-55">Usuario: primer nombre. &nbsp;Contraseña: apellidos.<br>Todo en minúsculas y sin acentos.</p>
                </div>
              </div>
              <button type="submit" class="btn btn-primary w-100 py-3 fw-bold rounded-pill">Registrar Usuario</button>
            </form>
          </div>
        </div>
      </main>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useTheme } from '../composables/useTheme'

const router = useRouter()
const { isDark, toggle, init } = useTheme()
const newUserName = ref('')

onMounted(() => init())

const nombreUsuario = computed(() => localStorage.getItem('usuario_nombre') || 'Técnico')

const crearUsuario = async () => {
  if (newUserName.value.trim()) {
    // TODO: POST /api/usuarios
    console.log("Registrar nuevo cliente por el técnico:", newUserName.value)
    alert(`Usuario registrado exitosamente.\nNombre: ${newUserName.value}\n(Pendiente de enviar al backend)`)
    router.push('/soporte')
  } else {
    alert('Ingresa un nombre válido.')
  }
}

const handleLogout = () => { localStorage.clear(); router.push('/') }
</script> 