<template>
  <div class="d-flex flex-column vh-100 admin-bg">
    <div class="d-flex flex-grow-1 overflow-hidden p-3 gap-3">

      <aside class="glass-admin d-flex flex-column flex-shrink-0" style="width: 256px;">
        <div class="sidebar-header">
          <div class="logo sidebar-logo text-accent">{{ panelNombre }}</div>
          <div class="sidebar-user-name text-capitalize">{{ nombreUsuario }}</div>
        </div>

        <nav class="sidebar-nav">
          <div class="nav-group-label">Navegación</div>
          <RouterLink :to="panelRuta" class="list-group-item active">
            <span class="nav-icon">←</span> Volver al Panel
          </RouterLink>
        </nav>

        <div class="sidebar-footer">
          <button @click="toggle" class="theme-toggle mb-3">
            <span>{{ isDark ? '☀' : '☾' }}</span><span>{{ isDark ? 'Modo Claro' : 'Modo Oscuro' }}</span>
          </button>
          <button @click="handleLogout" class="btn btn-outline-light w-100">Cerrar Sesión</button>
        </div>
      </aside>

      <main class="glass-admin d-flex flex-column overflow-auto w-100 content-main align-items-center">
        <div class="new-user-container mx-auto w-100" style="max-width: 650px;">
          <div class="d-flex justify-content-between align-items-center mb-4">
            <h2 class="h4 fw-bold mb-0">Crear Nuevo Usuario</h2>
            <RouterLink :to="panelRuta" class="btn btn-outline-light">Cancelar</RouterLink>
          </div>

          <div class="panel-inner p-5">
            <form @submit.prevent="crearUsuario">
              
              <div class="row g-3 mb-4">
                <div class="col-6">
                  <label class="form-label">Nombre(s)</label>
                  <input type="text" class="form-control" v-model="form.nombres" placeholder="Ej. Juan Carlos" required autofocus>
                </div>
                <div class="col-6">
                  <label class="form-label">Apellidos</label>
                  <input type="text" class="form-control" v-model="form.apellidos" placeholder="Ej. Pérez Gómez" required>
                </div>
              </div>

              <div class="mb-4 position-relative">
                <label class="form-label">Correo Electrónico Institucional</label>
                <input 
                  type="email" 
                  class="form-control" 
                  v-model="form.email" 
                  @input="validarCorreoEntrada"
                  placeholder="usuario@morelia.tecnm.mx" 
                  required
                >
                <div v-if="estadoCorreo !== 'idle'" :class="['mt-2 text-sm fw-bold transition-colors', claseColorCorreo]">
                  {{ mensajeCorreo }}
                </div>
              </div>

              <div class="row g-3 mb-5">
                <div class="col-6">
                  <label class="form-label">Contraseña Temporal</label>
                  <input type="password" class="form-control" v-model="form.password" placeholder="Contraseña inicial" required>
                </div>
                <div class="col-6">
                  <label class="form-label">Rol del Sistema</label>
                  <select class="form-select" v-model="form.rol">
                    <option value="cliente">Usuario Regular (Cliente)</option>
                    <option value="soporte">Técnico de Soporte</option>
                    <option v-if="rolActual === 'administrador'" value="administrador">Ejecutivo (Admin)</option>
                  </select>
                </div>
              </div>

              <button 
                type="submit" 
                class="btn btn-primary w-100 py-3 fw-bold rounded-pill"
                :disabled="estadoCorreo !== 'available'"
              >
                Guardar Usuario
              </button>
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

const form = ref({
  nombres: '',
  apellidos: '',
  email: '',
  password: '',
  rol: 'cliente'
})

onMounted(() => init())

const nombreUsuario = computed(() => localStorage.getItem('usuario_nombre') || 'Usuario')
const rolActual = computed(() => localStorage.getItem('rol'))

const panelRuta = computed(() => rolActual.value === 'administrador' ? '/ejecutivo' : '/soporte')
const panelNombre = computed(() => rolActual.value === 'administrador' ? 'SYS_ADMIN' : 'IT_SOPORTE')

// ── LÓGICA DE VALIDACIÓN AJAX + REGEX ──
const estadoCorreo = ref('idle') // 'idle' | 'invalid' | 'checking' | 'available' | 'taken'
let timeoutValidacion = null
const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/

const validarCorreoEntrada = () => {
  const emailActual = form.value.email.trim().toLowerCase()
  
  if (!emailActual) { estadoCorreo.value = 'idle'; return; }
  
  // 1. Verificación por Regex instantánea
  if (!emailRegex.test(emailActual)) { estadoCorreo.value = 'invalid'; return; }

  // 2. Verificación AJAX (Debounce de 500ms)
  estadoCorreo.value = 'checking'
  clearTimeout(timeoutValidacion)
  
  timeoutValidacion = setTimeout(async () => {
    try {
      const token = localStorage.getItem('token')
      const res = await fetch('/api/usuarios', { headers: { 'Authorization': `Bearer ${token}` } })
      
      if (res.ok) {
        const usuarios = await res.json()
        const existe = usuarios.some(u => (u.email || '').toLowerCase() === emailActual)
        estadoCorreo.value = existe ? 'taken' : 'available'
      }
    } catch (e) {
      console.error('Error AJAX:', e)
      estadoCorreo.value = 'idle'
    }
  }, 500)
}

const mensajeCorreo = computed(() => {
  const msgs = {
    invalid: '⚠ Formato de correo no válido',
    checking: '⏳ Verificando disponibilidad...',
    available: '✔ Correo disponible',
    taken: '✖ Este correo ya está registrado en el sistema'
  }
  return msgs[estadoCorreo.value] || ''
})

const claseColorCorreo = computed(() => {
  const classes = {
    invalid: 'msg-taken',
    checking: 'msg-checking',
    available: 'msg-available',
    taken: 'msg-taken'
  }
  return classes[estadoCorreo.value] || ''
})

const crearUsuario = async () => {
  if (estadoCorreo.value !== 'available') return;

  const nombreCompleto = `${form.value.nombres.trim()} ${form.value.apellidos.trim()}`;
  const passParaBackend = form.value.password.trim(); 
  const token = localStorage.getItem('token');
  
  try {
    const res = await fetch('/api/usuarios', {
      method: 'POST',
      headers: { 'Authorization': `Bearer ${token}`, 'Content-Type': 'application/json' },
      body: JSON.stringify({
        full_name: nombreCompleto,
        email: form.value.email.trim().toLowerCase(),
        role: form.value.rol,
        password: passParaBackend
      })
    });

    if (res.ok) {
      alert("Usuario creado exitosamente.");
      router.push(panelRuta.value);
    } else {
      const errorText = await res.text();
      alert(`Error: ${errorText}`);
    }
  } catch (e) {
    console.error('Error de red:', e);
  }
};

const handleLogout = async () => {
  const token = localStorage.getItem('token')
  if (token) { await fetch('/api/logout', { method: 'POST', headers: { 'Authorization': `Bearer ${token}` } }).catch(() => {}) }
  localStorage.clear()
  router.push('/')
}
</script>

<style scoped>
.transition-colors { transition: color 0.3s ease; font-size: 0.85rem; }
.msg-checking { color: #6c757d; }
.msg-available { color: #198754; }
.msg-taken { color: #dc3545; }
[data-theme="dark"] .msg-checking { color: #adb5bd; }
[data-theme="dark"] .msg-available { color: #81c784; }
[data-theme="dark"] .msg-taken { color: #e57373; }
</style>