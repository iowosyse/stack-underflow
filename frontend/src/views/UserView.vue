<template>
  <div class="d-flex flex-column vh-100 user-bg">
    <div class="d-flex flex-grow-1 overflow-hidden p-3 gap-3">

      <!-- SIDEBAR -->
      <aside class="glass-user d-flex flex-column flex-shrink-0" style="width: 256px;">
        <div class="sidebar-header">
          <div class="logo sidebar-logo text-accent">stack-underflow</div>
          <div class="sidebar-sub">Portal de Incidencias</div>
        </div>

        <nav class="sidebar-nav">
          <div class="nav-group-label">Menú</div>
          <RouterLink to="/user" class="list-group-item"><span class="nav-icon">⊟</span> Mis Tickets</RouterLink>
          <RouterLink to="/knowledge-base" class="list-group-item"><span class="nav-icon">◎</span> Base de Conocimiento</RouterLink>
        </nav>

        <div class="sidebar-footer">
          <div class="user-profile-card mb-3">
            <div class="user-avatar" :style="{ background: avatarColor(nombreUsuario) }">{{ iniciales(nombreUsuario) }}</div>
            <div class="min-w-0 flex-grow-1">
              <div class="user-profile-name text-capitalize profile-text-fix">{{ nombreUsuario }}</div>
              <div class="user-profile-meta">{{ misTickets.length }} tickets</div>
            </div>
          </div>
          <button @click="toggle" class="theme-toggle mb-2">
            <span>{{ isDark ? '☀' : '☾' }}</span><span>{{ isDark ? 'Modo Claro' : 'Modo Oscuro' }}</span>
          </button>
          <button @click="logout" class="btn btn-outline-danger w-100">Cerrar Sesión</button>
        </div>
      </aside>

      <!-- CONTENIDO -->
      <main class="glass-user d-flex flex-column overflow-auto w-100 content-main">
        <h2 class="h4 fw-bold mb-4">Mis Solicitudes</h2>

        <div class="row g-3 mb-3">
          <div class="col-8">
            <div class="panel-inner p-4 h-100">
              <div class="panel-section-header-inline mb-3">Levantar Incidencia</div>
              <form @submit.prevent="submitTicket">
                <div class="mb-3">
                  <label class="form-label">Asunto</label>
                  <input type="text" class="form-control" v-model="subject" placeholder="Ej. No enciende la computadora">
                </div>
                <div class="mb-3">
                  <label class="form-label">Categoría</label>
                  <select class="form-select" v-model="category">
                    <option>Hardware</option><option>Software</option><option>Redes</option>
                  </select>
                </div>
                <div class="mb-3">
                  <label class="form-label">Descripción detallada</label>
                  <textarea class="form-control" rows="3" v-model="description"></textarea>
                </div>
                <div v-if="ticketSuccess" class="success-toast mb-3"><span class="text-accent fw-bold">✔</span> Ticket enviado.</div>
                <button type="submit" class="btn btn-primary w-100 py-2" :disabled="!isFormValid">Enviar Ticket</button>
              </form>
            </div>
          </div>

          <div class="col-4 d-flex flex-column gap-3">
            <div class="stat-panel flex-grow-1 d-flex flex-column align-items-center justify-content-center text-center p-4">
              <div class="stat-label mb-2">En Proceso</div>
              <div class="stat-number">{{ ticketsActivos.length }}</div>
            </div>
            <div class="stat-panel-secondary d-flex align-items-center gap-3 p-3">
              <div class="stat-secondary-val">{{ ticketsCerrados.length }}</div>
              <div><div class="stat-secondary-label">Resueltos</div><div class="stat-secondary-sub">en tu historial</div></div>
            </div>
          </div>
        </div>

        <!-- Historial -->
        <div class="panel-inner">
          <div class="panel-section-header"><span>Historial de Reportes</span></div>
          <table class="table table-borderless align-middle mb-0">
            <thead><tr><th class="ps-4">ID</th><th>Asunto</th><th>Categoría</th><th>Estado</th></tr></thead>
            <tbody>
              <tr v-if="misTickets.length === 0"><td colspan="4" class="text-center py-5 opacity-55 td-sm">No has levantado tickets aún.</td></tr>
            </tbody>
          </table>
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

// ── Estado Local ─────────────────────────────────────────────────────────────
const ticketsActivos = ref([])
const ticketsCerrados = ref([])

onMounted(() => {
  init()
  // TODO: GET /api/tickets?usuario_id=...
})

const subject = ref(''); const category = ref('Hardware'); const description = ref('')
const ticketSuccess = ref(false)

const nombreUsuario = computed(() => localStorage.getItem('usuario_nombre') || 'Usuario')
const misTickets = computed(() => [...ticketsActivos.value, ...ticketsCerrados.value])
const isFormValid = computed(() => subject.value.trim() !== '' && description.value.trim() !== '')

// ── Acciones ─────────────────────────────────────────────────────────────────
const submitTicket = async () => {
  // TODO: POST /api/tickets
  console.log("Levantar ticket", subject.value)
  subject.value = ''; description.value = ''
  ticketSuccess.value = true
  setTimeout(() => ticketSuccess.value = false, 3500)
}

const logout = () => { localStorage.clear(); router.push('/') }
const iniciales = (n) => n.trim().split(' ').slice(0, 2).map(w => w[0]?.toUpperCase() || '').join('')
const avatarColor = (n) => { 
  const pLight = ['#4db6ac','#7986cb','#9575cd','#e57373','#f06292','#64b5f6','#4dd0e1','#81c784','#dce775','#ffb74d']
  const pDark  = ['#00695c','#283593','#4527a0','#c62828','#ad1457','#1565c0','#00838f','#2e7d32','#9e9d24','#ef6c00']
  const palette = isDark.value ? pDark : pLight
  let h = 0; for (const c of n) h = (h * 31 + c.charCodeAt(0)) & 0xffff; return palette[h % palette.length] 
}
</script>
<style scoped>
.profile-text-fix { color: inherit; }
[data-theme="dark"] .profile-text-fix { color: #f8f9fa; }
</style>