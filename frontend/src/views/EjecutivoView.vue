<template>
  <div class="d-flex flex-column vh-100 admin-bg">
    <div class="d-flex flex-grow-1 overflow-hidden p-3 gap-3">

      <!-- SIDEBAR -->
      <aside class="glass-admin d-flex flex-column flex-shrink-0" style="width: 256px;">
        <div class="sidebar-header">
          <div class="logo sidebar-logo text-accent">SYS_ADMIN</div>
          <div class="sidebar-user-name text-capitalize">{{ nombreEjecutivo }}</div>
        </div>

        <nav class="sidebar-nav">
          <div class="nav-group-label">Operaciones</div>
          <button @click="currentTab = 'todos-tickets'" :class="['list-group-item', { active: currentTab === 'todos-tickets' }]">
            <span class="nav-icon">⊞</span> Todos los Tickets
            <span v-if="todosLosTickets.length > 0" class="nav-badge ms-auto">{{ todosLosTickets.length }}</span>
          </button>
          <button @click="currentTab = 'usuarios'" :class="['list-group-item', { active: currentTab === 'usuarios' }]">
            <span class="nav-icon">◈</span> Gestión de Usuarios
          </button>
          <button @click="currentTab = 'nuevo-ticket'" :class="['list-group-item', { active: currentTab === 'nuevo-ticket' }]">
            <span class="nav-icon">＋</span> Nuevo Ticket
          </button>
        </nav>

        <div class="sidebar-footer">
          <button @click="toggle" class="theme-toggle mb-3">
            <span>{{ isDark ? '☀' : '☾' }}</span>
            <span>{{ isDark ? 'Modo Claro' : 'Modo Oscuro' }}</span>
          </button>
          <button @click="handleLogout" class="btn btn-outline-light w-100">Cerrar Sesión</button>
        </div>
      </aside>

      <!-- ÁREA DE CONTENIDO -->
      <main class="glass-admin d-flex flex-column overflow-auto w-100 content-main">

        <!-- TODOS LOS TICKETS -->
        <section v-if="currentTab === 'todos-tickets'">
          <div class="d-flex justify-content-between align-items-center mb-4">
            <h2 class="h4 fw-bold mb-0">Todos los Tickets</h2>
            <div class="d-flex gap-2 align-items-center">
              <select class="form-select form-select-sm" v-model="filtroEstado" style="width: 140px;">
                <option value="todos">Todos los estados</option>
                <option value="disponible">Disponible</option>
                <option value="asignado">Asignado</option>
                <option value="hecho">Resuelto</option>
              </select>
              <select class="form-select form-select-sm" v-model="filtroPrioridad" style="width: 140px;">
                <option value="todas">Todas las prioridades</option>
                <option value="urgente">Urgente</option>
                <option value="alta">Alta</option>
                <option value="moderada">Moderada</option>
                <option value="baja">Baja</option>
              </select>
            </div>
          </div>

          <!-- Métricas rápidas -->
          <div class="row g-3 mb-4">
            <div class="col-3">
              <div class="metric-card">
                <div class="metric-value text-accent">{{ todosLosTickets.length }}</div>
                <div class="metric-label">Total abiertos</div>
              </div>
            </div>
            <div class="col-3">
              <div class="metric-card">
                <div class="metric-value metric-value--urgente">{{ urgentesTotal }}</div>
                <div class="metric-label">Urgentes</div>
              </div>
            </div>
            <div class="col-3">
              <div class="metric-card">
                <div class="metric-value metric-value--dim">{{ sinAsignarTotal }}</div>
                <div class="metric-label">Sin asignar</div>
              </div>
            </div>
            <div class="col-3">
              <div class="metric-card">
                <div class="metric-value val-hecho">{{ ticketsCerrados.length }}</div>
                <div class="metric-label">Resueltos</div>
              </div>
            </div>
          </div>

          <!-- Tabla global de tickets -->
          <div class="panel-inner">
            <table class="table table-borderless align-middle mb-0">
              <thead>
                <tr>
                  <th class="ps-4">ID</th>
                  <th>Solicitante</th>
                  <th>Asunto</th>
                  <th>Categoría</th>
                  <th>Prioridad</th>
                  <th>Estado</th>
                  <th>Asignado a</th>
                </tr>
              </thead>
              <tbody>
                <template v-for="t in ticketsFiltrados" :key="t.id">
                  <tr @click="toggleTicket(t.id)" class="ticket-row">
                    <td class="ps-4 col-id">{{ t.id }}</td>
                    <td class="td-sm text-capitalize">{{ t.author }}</td>
                    <td class="td-sm text-truncate" style="max-width: 180px;">{{ t.subject }}</td>
                    <td><span class="cat-pill">{{ t.category }}</span></td>
                    <td><span :class="['badge', `badge-${t.priority}`]">{{ t.priority }}</span></td>
                    <td>
                      <span :class="['badge', t.status === 'hecho' ? 'badge-hecho' : 'badge-open']">
                        {{ t.status }}
                      </span>
                    </td>
                    <td class="td-sm text-capitalize">{{ t.assignedTo ?? '—' }}</td>
                  </tr>
                  <tr v-if="expandedTicket === t.id" class="expand-row">
                    <td colspan="7" class="p-0 border-0">
                      <div class="ticket-detail">
                        <div class="ticket-meta">
                          <span>{{ fmtDate(t.createdAt) }}</span>
                          <span><strong class="text-accent">{{ t.category }}</strong></span>
                        </div>
                        <p class="mb-0 opacity-75 ticket-body">{{ t.description }}</p>
                      </div>
                    </td>
                  </tr>
                </template>
                
                <!-- Tickets Cerrados -->
                <template v-if="filtroEstado === 'todos' || filtroEstado === 'hecho'">
                  <template v-for="t in ticketsCerrados" :key="'closed-'+t.id">
                    <tr @click="toggleTicket('closed-'+t.id)" class="ticket-row">
                      <td class="ps-4 col-id">{{ t.id }}</td>
                      <td class="td-sm text-capitalize">{{ t.author }}</td>
                      <td class="td-sm text-truncate" style="max-width: 180px;">{{ t.subject }}</td>
                      <td><span class="cat-pill">{{ t.category }}</span></td>
                      <td><span :class="['badge', `badge-${t.priority}`]">{{ t.priority }}</span></td>
                      <td><span class="badge badge-hecho">hecho</span></td>
                      <td class="td-sm text-capitalize">{{ t.assignedTo ?? '—' }}</td>
                    </tr>
                    <tr v-if="expandedTicket === 'closed-'+t.id" class="expand-row">
                      <td colspan="7" class="p-0 border-0">
                        <div class="ticket-detail">
                          <div class="ticket-meta"><span>{{ fmtDate(t.createdAt) }}</span></div>
                          <p class="mb-0 opacity-75 ticket-body">{{ t.description }}</p>
                        </div>
                      </td>
                    </tr>
                  </template>
                </template>

                <tr v-if="ticketsFiltrados.length === 0 && (filtroEstado === 'hecho' ? false : true)">
                  <td colspan="7" class="text-center py-5 opacity-55 td-sm">No hay tickets con estos filtros.</td>
                </tr>
              </tbody>
            </table>
          </div>
        </section>

        <!-- GESTIÓN DE USUARIOS (CRUD) -->
        <section v-if="currentTab === 'usuarios'" class="d-flex flex-column h-100">
          <div class="d-flex justify-content-between align-items-center mb-4">
            <h2 class="h4 fw-bold mb-0">Gestión de Usuarios</h2>
            <button @click="abrirFormularioCrear" class="btn btn-info fw-bold">+ Nuevo Usuario</button>
          </div>

          <div v-if="mostrarFormulario" class="panel-inner p-4 mb-4">
            <h3 class="h6 fw-bold mb-3">{{ modoEdicion ? 'Editar Usuario' : 'Crear Nuevo Usuario' }}</h3>
            <div class="row g-3">
              <div class="col-6">
                <label class="form-label">Nombre Completo</label>
                <input type="text" class="form-control" v-model="formNombre" placeholder="Ej. Juan Pérez Gómez">
              </div>
              <div class="col-6">
                <label class="form-label">Correo Corporativo</label>
                <input type="email" class="form-control" v-model="formEmail" placeholder="juan.perez@empresa.com">
              </div>
              <div class="col-4">
                <label class="form-label">Rol</label>
                <select class="form-select" v-model="formRol">
                  <option value="cliente">Usuario (cliente)</option>
                  <option value="soporte">Técnico de Soporte</option>
                  <option value="administrador">Ejecutivo (admin)</option>
                </select>
              </div>
              <div class="col-12 d-flex gap-2 justify-content-end">
                <button @click="cancelarFormulario" class="btn btn-outline-light">Cancelar</button>
                <button @click="guardarUsuario" class="btn btn-primary fw-bold">
                  {{ modoEdicion ? 'Guardar Cambios' : 'Registrar Usuario' }}
                </button>
              </div>
            </div>
          </div>

          <div v-if="usuarioAEliminar" class="modal-overlay" @click.self="usuarioAEliminar = null">
            <div class="modal-confirm glass-admin p-4">
              <h5 class="fw-bold mb-3 text-accent">Confirmar Eliminación</h5>
              <p class="mb-4 opacity-75">
                ¿Eliminar permanentemente a <strong class="text-capitalize">{{ usuarioAEliminar.fullName }}</strong>?
              </p>
              <div class="d-flex gap-3 justify-content-end">
                <button @click="usuarioAEliminar = null" class="btn btn-outline-light">Cancelar</button>
                <button @click="confirmarEliminar" class="btn btn-danger fw-bold">Eliminar</button>
              </div>
            </div>
          </div>

          <div class="panel-inner flex-grow-1">
            <table class="table table-borderless align-middle mb-0">
              <thead>
                <tr>
                  <th class="ps-4">Nombre</th>
                  <th>Rol</th>
                  <th>Tickets abiertos</th>
                  <th>Tickets resueltos</th>
                  <th class="text-end pe-4">Acciones</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="u in todosLosUsuarios" :key="u.id" class="ticket-row">
                  <td class="ps-4">
                    <div class="d-flex align-items-center gap-2">
                      <div class="user-avatar user-avatar--sm" :style="{ background: avatarColor(u.full_name) }">
                        {{ iniciales(u.full_name) }}
                      </div>
                      <span class="text-capitalize td-sm nombre-usuario-fix">{{ u.fullName }}</span>
                    </div>
                  </td>
                  <td>
                    <span :class="['cat-pill', u.role === 'administrador' ? 'cat-pill--admin' : '']">
                      {{ etiquetaRol(u.role) }}
                    </span>
                  </td>
                  <td class="td-sm">{{ ticketsAbiertosDe(u.fullName) }}</td>
                  <td class="td-sm">{{ ticketsCerradosDe(u.fullName) }}</td>
                  <td class="text-end pe-4">
                    <button @click="abrirFormularioEditar(u)" class="btn btn-sm btn-outline-info btn-xs me-2">Editar</button>
                    <button @click="pedirConfirmacionEliminar(u)" class="btn btn-sm btn-outline-danger btn-xs">Eliminar</button>
                  </td>
                </tr>
                <tr v-if="todosLosUsuarios.length === 0">
                  <td colspan="5" class="text-center py-5 opacity-55 td-sm">Cargando directorio de usuarios...</td>
                </tr>
              </tbody>
            </table>
          </div>
        </section>

        <!-- NUEVO TICKET -->
        <section v-if="currentTab === 'nuevo-ticket'" class="d-flex flex-column align-items-center">
          <div class="new-user-container mx-auto w-100" style="max-width: 640px;">
            <h2 class="h4 fw-bold mb-4">Levantar Ticket</h2>
            <div class="panel-inner p-5">
              <form @submit.prevent="enviarTicket">
                <div class="mb-3">
                  <label class="form-label">Asunto</label>
                  <input type="text" class="form-control" v-model="ticketAsunto" placeholder="Ej. Sin acceso al sistema">
                </div>
                <div class="mb-3">
                  <label class="form-label">Categoría</label>
                  <select class="form-select" v-model="ticketCategoria">
                    <option>Hardware</option>
                    <option>Software</option>
                    <option>Redes</option>
                  </select>
                </div>
                <div class="mb-4">
                  <label class="form-label">Descripción detallada</label>
                  <textarea class="form-control" rows="4" v-model="ticketDescripcion"></textarea>
                </div>
                <div v-if="ticketExitoso" class="success-toast mb-3"><span class="text-accent fw-bold">✔</span> Enviado correctamente.</div>
                <button type="submit" class="btn btn-primary w-100 py-3 fw-bold rounded-pill" :disabled="!ticketFormValido">Enviar Ticket</button>
              </form>
            </div>
          </div>
        </section>

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

// ── Estado Local (Reemplaza a store.js) ──────────────────────────────────────
const ticketsActivos  = ref([])
const ticketsCerrados = ref([])
const todosLosUsuarios = ref([])

const cargarUsuarios = async () => {
  const token = localStorage.getItem('token');
  
  try {
    const respuesta = await fetch('/api/usuarios', {
      method: 'GET',
      headers: {
        'Authorization': `Bearer ${token}`, // Enviamos la pulsera VIP
        'Content-Type': 'application/json'
      }
    });

    if (respuesta.ok) {
      const datos = await respuesta.json();
      // Si estás en SoporteView usa: listaUsuarios.value = datos
      // Si estás en EjecutivoView usa: todosLosUsuarios.value = datos
      todosLosUsuarios.value = datos; 
    } else if (respuesta.status === 401) {
      handleLogout(); // Si el token no sirve, fuera del sistema
    }
  } catch (error) {
    console.error("Error al conectar con el servidor:", error);
  }
};

onMounted(() => {
  init();
  cargarUsuarios();
});

const currentTab = ref('todos-tickets')
const expandedTicket = ref(null)
const filtroEstado = ref('todos')
const filtroPrioridad = ref('todas')

const mostrarFormulario = ref(false)
const modoEdicion = ref(false)
const usuarioEnEdicion = ref(null)
const formNombre = ref('')
const formEmail = ref('')
const formRol = ref('cliente') 
const usuarioAEliminar = ref(null)

const ticketAsunto = ref('')
const ticketCategoria = ref('Hardware')
const ticketDescripcion = ref('')
const ticketExitoso = ref(false)

const nombreEjecutivo = computed(() => localStorage.getItem('usuario_nombre') || 'Ejecutivo')
const todosLosTickets = computed(() => ticketsActivos.value)

const ticketsFiltrados = computed(() => {
  let lista = ticketsActivos.value
  if (filtroEstado.value !== 'todos' && filtroEstado.value !== 'hecho')
    lista = lista.filter(t => t.status === filtroEstado.value)
  if (filtroPrioridad.value !== 'todas')
    lista = lista.filter(t => t.priority === filtroPrioridad.value)
  return lista
})

const urgentesTotal = computed(() => ticketsActivos.value.filter(t => t.priority === 'urgente').length)
const sinAsignarTotal = computed(() => ticketsActivos.value.filter(t => t.status === 'disponible').length)

const ticketsAbiertosDe = (n) => ticketsActivos.value.filter(t => t.author === n).length
const ticketsCerradosDe = (n) => ticketsCerrados.value.filter(t => t.author === n).length

// ── CRUD Acciones (Endpoints Pendientes) ───────────────────────────────────
const abrirFormularioCrear = () => {
  modoEdicion.value = false; usuarioEnEdicion.value = null
  formNombre.value = ''; formEmail.value = ''; formRol.value = 'cliente'
  mostrarFormulario.value = true
}

const abrirFormularioEditar = (u) => {
  modoEdicion.value = true; usuarioEnEdicion.value = u
  formNombre.value = u.fullName; formEmail.value = u.email ?? ''; formRol.value = u.role
  mostrarFormulario.value = true
}

const cancelarFormulario = () => { mostrarFormulario.value = false; usuarioEnEdicion.value = null }

const guardarUsuario = async () => {
  if (!formNombre.value.trim()) return
  if (modoEdicion.value) {
    // TODO: PUT /api/usuarios/:id
    console.log("Actualizar usuario", usuarioEnEdicion.value.id, formNombre.value)
  } else {
    // TODO: POST /api/usuarios
    console.log("Crear usuario", formNombre.value, formRol.value)
  }
  cancelarFormulario()
}

const pedirConfirmacionEliminar = (u) => { usuarioAEliminar.value = u }

const confirmarEliminar = async () => {
  if (!usuarioAEliminar.value) return
  // TODO: DELETE /api/usuarios/:id
  console.log("Eliminar usuario", usuarioAEliminar.value.id)
  usuarioAEliminar.value = null
}

const ticketFormValido = computed(() => ticketAsunto.value.trim() !== '' && ticketDescripcion.value.trim() !== '')

const enviarTicket = async () => {
  // TODO: POST /api/tickets
  console.log("Nuevo ticket de Admin:", ticketAsunto.value)
  ticketAsunto.value = ''; ticketDescripcion.value = ''
  ticketExitoso.value = true
  setTimeout(() => (ticketExitoso.value = false), 3500)
}

const handleLogout = () => { localStorage.clear(); router.push('/') }
const toggleTicket = (id) => { expandedTicket.value = expandedTicket.value === id ? null : id }
const fmtDate = (ts) => new Date(ts).toLocaleString('es-MX', { dateStyle: 'short', timeStyle: 'short' })
const etiquetaRol = (r) => ({ administrador: 'Ejecutivo', soporte: 'Soporte IT', cliente: 'Usuario' }[r] ?? r)

// ── Paletas Inteligentes ───────────────────────────────────────────────────
const iniciales = (n) => n.trim().split(' ').slice(0, 2).map(w => w[0]?.toUpperCase() || '').join('')
const avatarColor = (n) => { 
  const pLight = ['#4db6ac','#7986cb','#9575cd','#e57373','#f06292','#64b5f6','#4dd0e1','#81c784','#dce775','#ffb74d']
  const pDark  = ['#00695c','#283593','#4527a0','#c62828','#ad1457','#1565c0','#00838f','#2e7d32','#9e9d24','#ef6c00']
  const palette = isDark.value ? pDark : pLight
  let h = 0; for (const c of n) h = (h * 31 + c.charCodeAt(0)) & 0xffff; return palette[h % palette.length] 
}
</script>

<style scoped>
.modal-overlay { position: fixed; inset: 0; background: rgba(0,0,0,0.55); display: flex; align-items: center; justify-content: center; z-index: 999; }
.modal-confirm { max-width: 420px; width: 100%; border-radius: 16px; }
.cat-pill--admin { background: rgba(139, 47, 201, 0.15); color: #c084fc; }
.user-avatar--sm { width: 30px; height: 30px; min-width: 30px; font-size: 0.7rem; border-radius: 8px; display: flex; align-items: center; justify-content: center; color: #fff; font-weight: 700; }
/* Corrección de color de texto en modo oscuro */
.nombre-usuario-fix { color: inherit; }
[data-theme="dark"] .nombre-usuario-fix { color: #f8f9fa; }
</style>