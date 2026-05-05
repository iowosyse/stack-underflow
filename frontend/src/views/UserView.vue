<template>
  <div class="d-flex flex-column vh-100 user-bg">
    <div class="d-flex flex-grow-1 overflow-hidden p-3 gap-3">

      <!-- ════════════════════════════════════
           SIDEBAR
           ════════════════════════════════════ -->
      <aside class="glass-user d-flex flex-column flex-shrink-0" style="width: 256px;">

        <div class="sidebar-header">
          <div class="logo sidebar-logo text-accent">stack-underflow</div>
          <div class="sidebar-sub">Portal de Incidencias</div>
        </div>

        <nav class="sidebar-nav">
          <div class="nav-group-label">Menú</div>
          <RouterLink to="/user"           class="list-group-item">
            <span class="nav-icon">⊟</span> Mis Tickets
          </RouterLink>
          <RouterLink to="/knowledge-base" class="list-group-item">
            <span class="nav-icon">◎</span> Base de Conocimiento
          </RouterLink>
        </nav>

        <div class="sidebar-footer">
          <div class="user-profile-card mb-3">
            <div class="user-avatar" :style="{ background: avatarColor(store.currentUser?.fullName || '') }">
              {{ initials(store.currentUser?.fullName || '?') }}
            </div>
            <div class="min-w-0 flex-grow-1">
              <div class="user-profile-name text-capitalize">{{ store.currentUser?.fullName }}</div>
              <div class="user-profile-meta">
                {{ myTickets.length }} ticket{{ myTickets.length !== 1 ? 's' : '' }} registrados
              </div>
            </div>
          </div>
          <button @click="toggle" class="theme-toggle mb-2">
            <span>{{ isDark ? '☀' : '☾' }}</span>
            <span>{{ isDark ? 'Modo Claro' : 'Modo Oscuro' }}</span>
          </button>
          <button @click="logout" class="btn btn-outline-danger w-100">Cerrar Sesión</button>
        </div>

      </aside>

      <!-- ════════════════════════════════════
           ÁREA DE CONTENIDO
           ════════════════════════════════════ -->
      <main class="glass-user d-flex flex-column overflow-auto w-100 content-main">

        <h2 class="h4 fw-bold mb-4">Mis Solicitudes</h2>

        <div class="row g-3 mb-3">

          <!-- Formulario de ticket -->
          <div class="col-8">
            <div class="panel-inner p-4 h-100">
              <div class="panel-section-header-inline mb-3">Levantar Incidencia</div>
              <form @submit.prevent="submitTicket">
                <div class="mb-3">
                  <label class="form-label">Asunto</label>
                  <input type="text" class="form-control" v-model="subject"
                         placeholder="Ej. No enciende la computadora">
                </div>
                <div class="mb-3">
                  <label class="form-label">Categoría</label>
                  <select class="form-select" v-model="category">
                    <option>Hardware</option>
                    <option>Software</option>
                    <option>Redes</option>
                  </select>
                </div>
                <div class="mb-3">
                  <label class="form-label">Descripción detallada</label>
                  <textarea class="form-control" rows="3" v-model="description"
                            placeholder="Describe el problema con el mayor detalle posible…"></textarea>
                </div>

                <div v-if="ticketSuccess" class="success-toast mb-3">
                  <span class="text-accent fw-bold">✔</span>
                  Ticket enviado correctamente a soporte.
                </div>

                <button type="submit" class="btn btn-primary w-100 py-2" :disabled="!isFormValid">
                  Enviar Ticket
                </button>
              </form>
            </div>
          </div>

          <!-- Stats -->
          <div class="col-4 d-flex flex-column gap-3">
            <div class="stat-panel flex-grow-1 d-flex flex-column align-items-center justify-content-center text-center p-4">
              <div class="stat-label mb-2">En Proceso</div>
              <div class="stat-number">{{ activeTicketsCount }}</div>
            </div>
            <div class="stat-panel-secondary d-flex align-items-center gap-3 p-3">
              <div class="stat-secondary-val">{{ closedTicketsCount }}</div>
              <div>
                <div class="stat-secondary-label">Resueltos</div>
                <div class="stat-secondary-sub">en tu historial</div>
              </div>
            </div>
          </div>

        </div>

        <!-- Historial -->
        <div class="panel-inner">
          <div class="panel-section-header">
            <span>Historial de Reportes</span>
            <span class="panel-section-count panel-section-note">clic para expandir</span>
          </div>
          <table class="table table-borderless align-middle mb-0">
            <thead>
              <tr>
                <th class="ps-4">ID</th>
                <th>Asunto</th>
                <th>Categoría</th>
                <th>Estado</th>
              </tr>
            </thead>
            <tbody>
              <template v-for="t in myTickets" :key="t.id">
                <tr @click="toggleTicket(t.id)" class="ticket-row">
                  <td class="ps-4 col-id">{{ t.id }}</td>
                  <td class="td-sm">{{ t.subject }}</td>
                  <td><span class="cat-pill">{{ t.category }}</span></td>
                  <td>
                    <span :class="['badge', t.status === 'hecho' ? 'badge-hecho' : 'badge-open']">
                      {{ t.status }}
                    </span>
                  </td>
                </tr>
                <tr v-if="expandedTicket === t.id" class="expand-row">
                  <td colspan="4" class="p-0 border-0">
                    <div class="ticket-detail">
                      <div class="ticket-meta">
                        <span><strong class="text-accent">{{ t.category }}</strong></span>
                        <span>{{ fmtDate(t.createdAt) }}</span>
                      </div>
                      <p class="mb-0 opacity-75 ticket-body">{{ t.description }}</p>
                    </div>
                  </td>
                </tr>
              </template>
              <tr v-if="myTickets.length === 0">
                <td colspan="4" class="text-center py-5 opacity-55 td-sm">No has levantado tickets aún.</td>
              </tr>
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
import { store } from '../store'
import { useTheme } from '../composables/useTheme'

const router         = useRouter()
const subject        = ref('')
const category       = ref('Hardware')
const description    = ref('')
const ticketSuccess  = ref(false)
const expandedTicket = ref(null)
const { isDark, toggle, init } = useTheme()

onMounted(() => init())

const activeTicketsCount = computed(() =>
  store.tickets.filter(t => t.author === store.currentUser?.fullName).length
)
const closedTicketsCount = computed(() =>
  store.closedTickets.filter(t => t.author === store.currentUser?.fullName).length
)
const myTickets = computed(() =>
  [...store.tickets, ...store.closedTickets].filter(t => t.author === store.currentUser?.fullName)
)
const isFormValid = computed(() => subject.value.trim() !== '' && description.value.trim() !== '')

const submitTicket = () => {
  store.addTicket(subject.value, category.value, description.value)
  subject.value = ''; description.value = ''
  ticketSuccess.value = true
  setTimeout(() => ticketSuccess.value = false, 3500)
}
const toggleTicket = (id) => { expandedTicket.value = expandedTicket.value === id ? null : id }
const logout       = ()   => { store.logout(); router.push('/') }
const fmtDate      = (ts) => new Date(ts).toLocaleString('es-MX', { dateStyle: 'short', timeStyle: 'short' })

const PALETTE = ['#2d6a4f','#1b4332','#40916c','#184e77','#1e6091','#6b3fa0','#553285','#8b2fc9','#7b2d8b','#9c4221']
const initials    = (n) => n.trim().split(' ').slice(0, 2).map(w => w[0]?.toUpperCase() ?? '').join('')
const avatarColor = (n) => { let h = 0; for (const c of n) h = (h * 31 + c.charCodeAt(0)) & 0xffff; return PALETTE[h % PALETTE.length] }
</script>
