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
          <div class="nav-group-label">Operaciones</div>
          <button @click="currentTab = 'dashboard'" :class="['list-group-item', { active: currentTab === 'dashboard' }]">
            <span class="nav-icon">⊞</span> Panel Principal
            <span v-if="totalPendiente > 0" class="nav-badge ms-auto">{{ totalPendiente }}</span>
          </button>
          <button @click="currentTab = 'usuarios'" :class="['list-group-item', { active: currentTab === 'usuarios' }]">
            <span class="nav-icon">◈</span> Directorio
          </button>

          <div class="nav-group-label mt-3">Registro</div>
          <button @click="currentTab = 'hechos'" :class="['list-group-item', { active: currentTab === 'hechos' }]">
            <span class="nav-icon">✓</span> Tickets Resueltos
          </button>
        </nav>

        <div class="sidebar-footer">
          <button @click="toggle" class="theme-toggle mb-3">
            <span>{{ isDark ? '☀' : '☾' }}</span>
            <span>{{ isDark ? 'Modo Claro' : 'Modo Oscuro' }}</span>
          </button>
          <RouterLink to="/soporte/nuevo-usuario" class="btn btn-info w-100 mb-2 fw-bold">+ Nuevo Usuario</RouterLink>
          <button @click="handleLogout" class="btn btn-outline-light w-100">Cerrar Sesión</button>
        </div>
      </aside>

      <!-- CONTENIDO -->
      <main class="glass-admin d-flex flex-column overflow-auto w-100 content-main">

        <!-- PANEL PRINCIPAL -->
        <section v-if="currentTab === 'dashboard'">
          <div class="row g-3 mb-4">
            <div class="col-4">
              <div class="metric-card">
                <div class="metric-value text-accent">{{ misTicketsAsignados.length }}</div>
                <div class="metric-label">Asignados a mí</div>
              </div>
            </div>
            <div class="col-4">
              <div class="metric-card">
                <div class="metric-value metric-value--urgente">{{ urgentesEnCola }}</div>
                <div class="metric-label">Urgentes en cola</div>
              </div>
            </div>
            <div class="col-4">
              <div class="metric-card">
                <div class="metric-value metric-value--dim">{{ ticketsSinAsignar.length }}</div>
                <div class="metric-label">Sin asignar</div>
              </div>
            </div>
          </div>

          <div class="row g-3">
            <div class="col-7">
              <div class="panel-inner d-flex flex-column panel-scroll">
                <div class="panel-section-header"><span>Mis Tickets</span></div>
                <div class="panel-scroll-body">
                  <table class="table table-borderless align-middle mb-0">
                    <thead><tr><th class="ps-4">Prioridad</th><th>Solicitante</th><th>Asunto</th><th></th></tr></thead>
                    <tbody>
                      <template v-for="t in misTicketsAsignados" :key="t.id">
                        <tr @click="toggleTicket(t.id)" class="ticket-row">
                          <td class="ps-4"><span :class="['badge', `badge-${t.priority}`]">{{ t.priority }}</span></td>
                          <td class="td-sm text-capitalize">{{ t.author }}</td>
                          <td class="td-sm text-truncate" style="max-width: 160px;">{{ t.subject }}</td>
                          <td class="pe-3 text-end">
                            <button 
                              @click.stop="marcarComoHecho(t.id)" 
                              :disabled="procesando"
                              class="btn btn-sm btn-info fw-bold btn-xs"
                            >
                              {{ procesando ? '...' : '✔ Hecho' }}
                            </button>
                          </td>
                        </tr>
                      </template>
                      <tr v-if="misTicketsAsignados.length === 0"><td colspan="4" class="text-center py-5 opacity-55 td-sm">Sin tickets asignados.</td></tr>
                    </tbody>
                  </table>
                </div>
              </div>
            </div>

            <div class="col-5">
              <div class="panel-inner d-flex flex-column panel-scroll">
                <div class="panel-section-header">
                  <span>Cola de Entrada</span>
                  <select class="form-select form-select-xs w-auto" v-model="filtroPrioridad">
                    <option value="todas">Todas</option>
                    <option value="urgente">Urgente</option>
                  </select>
                </div>
                <div class="panel-scroll-body">
                  <table class="table table-borderless align-middle mb-0">
                    <tbody>
                      <template v-for="t in colaSinAsignarFiltrada" :key="t.id">
                        <tr class="ticket-row">
                          <td class="ps-4"><span :class="['badge', `badge-${t.priority}`]">{{ t.priority }}</span></td>
                          <td class="td-sm"><div class="text-capitalize">{{ t.author }}</div><div class="td-stack-sub">{{ t.subject }}</div></td>
                          <td class="pe-3 text-end">
                            <button 
                              @click.stop="tomarTicket(t.id)" 
                              :disabled="procesando"
                              class="btn btn-sm btn-outline-info btn-xs"
                            >
                              {{ procesando ? '...' : 'Tomar' }}
                            </button>
                          </td>
                        </tr>
                      </template>
                      <tr v-if="colaSinAsignarFiltrada.length === 0"><td colspan="3" class="text-center py-5 opacity-55 td-sm">Cola vacía.</td></tr>
                    </tbody>
                  </table>
                </div>
              </div>
            </div>
          </div>
        </section>

        <!-- DIRECTORIO -->
        <section v-if="currentTab === 'usuarios'" class="d-flex flex-column h-100">
          <h2 class="h4 fw-bold mb-4">Directorio de Usuarios</h2>
          <div class="row g-3 flex-grow-1" style="min-height: 0;">
            <div class="col-4 d-flex flex-column" style="min-height: 0;">
              <div class="panel-inner d-flex flex-column flex-grow-1" style="overflow: hidden;">
                <div class="panel-section-header">Usuarios</div>
                <div class="overflow-auto flex-grow-1 p-2">
                  <button v-for="u in listaUsuarios" :key="u.id" @click="usuarioSeleccionado = u; expandedTicket = null" :class="['user-card w-100', { 'user-card--active': usuarioSeleccionado?.id === u.id }]">
                    <div class="user-avatar" :style="{ background: avatarColor(u.full_name) }">{{ iniciales(u.full_name) }}</div>
                    <div class="user-card-body">
                      <!-- Corrección de color de texto -->
                      <div class="user-card-name text-capitalize">{{ u.full_name }}</div>
                    </div>
                    <span class="user-card-arrow">›</span>
                  </button>
                  <div v-if="listaUsuarios.length === 0" class="text-center p-4 opacity-55 small">Cargando directorio...</div>
                </div>
              </div>
            </div>

            <div class="col-8 d-flex flex-column" style="min-height: 0;">
              <div v-if="usuarioSeleccionado" class="panel-inner d-flex flex-column flex-grow-1" style="overflow: hidden;">
                <div class="user-profile-bar">
                  <div class="user-avatar user-avatar--lg" :style="{ background: avatarColor(usuarioSeleccionado.full_name) }">{{ iniciales(usuarioSeleccionado.full_name) }}</div>
                  <div class="flex-grow-1 min-w-0">
                    <div class="user-detail-name text-capitalize">{{ usuarioSeleccionado.full_name }}</div>
                    <div class="user-detail-role">{{ usuarioSeleccionado.role === 'administrador' ? 'Ejecutivo' : 'Usuario' }}</div>
                  </div>
                </div>
                <div class="overflow-auto flex-grow-1">
                  <table class="table table-borderless align-middle mb-0">
                    <thead><tr><th class="ps-4">ID</th><th>Asunto</th><th>Categoría</th><th>Estado</th></tr></thead>
                    <tbody>
                      <template v-for="t in ticketsDelUsuario(usuarioSeleccionado.full_name)" :key="t.id">
                        <tr class="ticket-row">
                          <td class="ps-4 col-id">{{ t.id }}</td>
                          <td class="td-sm text-truncate" style="max-width: 200px;">{{ t.subject }}</td>
                          <td><span class="cat-pill">{{ t.category }}</span></td>
                          <td><span :class="['badge', t.status === 'hecho' ? 'badge-hecho' : 'badge-open']">{{ t.status }}</span></td>
                        </tr>
                      </template>
                      <tr v-if="ticketsDelUsuario(usuarioSeleccionado.full_name).length === 0">
                        <td colspan="4" class="text-center py-5 opacity-55 td-sm">Sin tickets.</td>
                      </tr>
                    </tbody>
                  </table>
                </div>
              </div>
              <div v-else class="panel-inner flex-grow-1 panel-empty"><div class="panel-empty-icon">◈</div><div class="panel-empty-text">Selecciona un usuario<br>para ver su historial.</div></div>
            </div>
          </div>
        </section>

        <!-- HISTORIAL -->
        <section v-if="currentTab === 'hechos'">
          <div class="d-flex justify-content-between align-items-baseline mb-4"><h2 class="h4 fw-bold mb-0">Tickets Resueltos</h2></div>
          <div class="panel-inner">
            <table class="table table-borderless align-middle mb-0">
              <thead><tr><th class="ps-4">ID</th><th>Solicitante</th><th>Asunto</th><th>Categoría</th><th>Estado</th></tr></thead>
              <tbody>
                <template v-for="t in ticketsCerrados" :key="t.id">
                  <tr class="ticket-row">
                    <td class="ps-4 col-id">{{ t.id }}</td>
                    <td class="td-sm text-capitalize">{{ t.author }}</td>
                    <td class="td-sm text-truncate" style="max-width: 180px;">{{ t.subject }}</td>
                    <td><span class="cat-pill">{{ t.category }}</span></td>
                    <td><span class="badge badge-hecho">resuelto</span></td>
                  </tr>
                </template>
                <tr v-if="ticketsCerrados.length === 0">
                  <td colspan="5" class="text-center py-5 opacity-55 td-sm">Aún no hay tickets resueltos.</td>
                </tr>
              </tbody>
            </table>
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
const listaUsuarios   = ref([])

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
      listaUsuarios.value = datos; 
    } else if (respuesta.status === 401) {
      handleLogout(); // Si el token no sirve, fuera del sistema
    }
  } catch (error) {
    console.error("Error al conectar con el servidor:", error);
  }
};

const cargarTickets = async () => {
  const token = localStorage.getItem('token');
  const headers = {
    'Authorization': `Bearer ${token}`,
    'Content-Type': 'application/json'
  };
  try {
    const [resActivos, resCerrados] = await Promise.all([
      fetch('/api/tickets',          { headers }),
      fetch('/api/tickets/cerrados', { headers }),
    ]);
    if (resActivos.status === 401 || resCerrados.status === 401) { handleLogout(); return; }
    if (resActivos.ok)  ticketsActivos.value  = await resActivos.json();
    if (resCerrados.ok) ticketsCerrados.value = await resCerrados.json();
  } catch (error) {
    console.error('Error al cargar tickets:', error);
  }
};

onMounted(() => {
  init();
  cargarUsuarios();
  cargarTickets();
});

const currentTab = ref('dashboard')
const usuarioSeleccionado = ref(null)
const filtroPrioridad = ref('todas')
const expandedTicket = ref(null)

const nombreUsuario = computed(() => localStorage.getItem('usuario_nombre') || 'Técnico')

const misTicketsAsignados = computed(() => ticketsActivos.value.filter(t => t.assignedTo === nombreUsuario.value))
const ticketsSinAsignar = computed(() => ticketsActivos.value.filter(t => t.status === 'disponible'))
const colaSinAsignarFiltrada = computed(() => {
  let lista = ticketsSinAsignar.value
  if (filtroPrioridad.value !== 'todas') lista = lista.filter(t => t.priority === filtroPrioridad.value)
  return lista
})

const urgentesEnCola = computed(() => ticketsSinAsignar.value.filter(t => t.priority === 'urgente').length)
const totalPendiente = computed(() => misTicketsAsignados.value.length + ticketsSinAsignar.value.length)

const ticketsDelUsuario = (n) => [...ticketsActivos.value, ...ticketsCerrados.value].filter(t => t.author === n)

// Variable para bloquear clics dobles durante la carga
const procesando = ref(false);

const tomarTicket = async (id) => {
  if (procesando.value) return;
  procesando.value = true;
  
  const token = localStorage.getItem('token');
  try {
    const res = await fetch(`/api/tickets/${id}/asignar`, {
      method: 'PUT',
      headers: { 
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json'
      }
    });

    if (res.ok) {
      // Recargamos los tickets para que aparezca en "Mis Tickets"
      await cargarTickets();
    } else {
      const errorText = await res.text();
      alert(`No se pudo tomar el ticket: ${errorText}`);
    }
  } catch (e) {
    console.error('Error de conexión:', e);
  } finally {
    procesando.value = false;
  }
};

const marcarComoHecho = async (id) => {
  if (procesando.value) return;
  procesando.value = true;

  const token = localStorage.getItem('token');
  try {
    const res = await fetch(`/api/tickets/${id}/cerrar`, {
      method: 'PUT',
      headers: { 
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json'
      }
    });

    if (res.ok) {
      // Recargamos para que el ticket se mueva a la pestaña de "Resueltos"
      await cargarTickets();
    } else {
      const errorText = await res.text();
      alert(`Error al cerrar: ${errorText}`);
    }
  } catch (e) {
    console.error('Error de conexión:', e);
  } finally {
    procesando.value = false;
  }
};

const handleLogout = () => { localStorage.clear(); router.push('/') }
const toggleTicket = (id) => { expandedTicket.value = expandedTicket.value === id ? null : id }
const fmtDate = (ts) => new Date(ts).toLocaleString('es-MX', { dateStyle: 'short', timeStyle: 'short' })

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
/* Resetea los botones list para que tomen el color de fuente correcto en modo oscuro */
.user-card { background: transparent; border: 1px solid rgba(128,128,128,0.2); text-align: left; padding: 0.5rem; display: flex; align-items: center; gap: 10px; color: inherit; }
.user-card-name { color: #212529; font-weight: 500; }
[data-theme="dark"] .user-card-name { color: #f8f9fa; }
</style>