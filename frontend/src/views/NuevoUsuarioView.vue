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

              <div class="mb-4">
                <label class="form-label">Correo Electrónico Institucional</label>
                <input type="email" class="form-control" v-model="form.email" placeholder="usuario@tecnm.mx" required>
              </div>

              <div class="row g-3 mb-5">
                <div class="col-6">
                  <label class="form-label">Contraseña Temporal</label>
                  <input type="password" class="form-control" v-model="form.password" placeholder="Contraseña inicial" required>
                  <div class="form-text mt-2 opacity-75" style="font-size: 0.8rem;">
                    *El sistema pondrá la primera letra en mayúscula automáticamente.
                  </div>
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

              <button type="submit" class="btn btn-primary w-100 py-3 fw-bold rounded-pill">Guardar Usuario</button>
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

// Variables reactivas del formulario
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

// Lógica de UI para que el menú se adapte a Ejecutivo o Soporte
const panelRuta = computed(() => rolActual.value === 'administrador' ? '/ejecutivo' : '/soporte')
const panelNombre = computed(() => rolActual.value === 'administrador' ? 'SYS_ADMIN' : 'IT_SOPORTE')

const crearUsuario = async () => {
  const nombreCompleto = `${form.value.nombres.trim()} ${form.value.apellidos.trim()}`;
  
  const passParaBackend = form.value.password.trim(); 

  const token = localStorage.getItem('token');
  
  try {
    const res = await fetch('/api/usuarios', {
      method: 'POST',
      headers: { 
        'Authorization': `Bearer ${token}`, 
        'Content-Type': 'application/json' 
      },
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

const handleLogout = () => { localStorage.clear(); router.push('/') }
</script>