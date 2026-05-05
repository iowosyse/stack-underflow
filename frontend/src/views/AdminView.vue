<template>
  <div class="d-flex flex-column vh-100 admin-bg">
    <div class="d-flex flex-grow-1 overflow-hidden p-3 gap-3">

      <!-- ════════════════════════════════════
           SIDEBAR
           ════════════════════════════════════ -->
      <aside class="glass-admin d-flex flex-column flex-shrink-0" style="width: 256px;">

        <div class="sidebar-header">
          <div class="logo sidebar-logo text-accent">SYS_ADMIN</div>
          <div class="sidebar-user-name text-capitalize">{{ store.currentUser?.fullName }}</div>
        </div>

        <nav class="sidebar-nav">
          <div class="nav-group-label">Operaciones</div>
          <button @click="currentTab = 'dashboard'"
                  :class="['list-group-item', { active: currentTab === 'dashboard' }]">
            <span class="nav-icon">⊞</span>
            Panel Principal
            <span v-if="totalPending > 0" class="nav-badge ms-auto">{{ totalPending }}</span>
          </button>
          <button @click="currentTab = 'usuarios'"
                  :class="['list-group-item', { active: currentTab === 'usuarios' }]">
            <span class="nav-icon">◈</span>
            Directorio
          </button>

          <div class="nav-group-label mt-3">Registro</div>
          <button @click="currentTab = 'hechos'"
                  :class="['list-group-item', { active: currentTab === 'hechos' }]">
            <span class="nav-icon">✓</span>
            Tickets Resueltos
            <span v-if="store.closedTickets.length > 0" class="nav-badge nav-badge--dim ms-auto">
              {{ store.closedTickets.length }}
            </span>
          </button>
        </nav>

        <div class="sidebar-footer">
          <button @click="toggle" class="theme-toggle mb-3">
            <span>{{ isDark ? '☀' : '☾' }}</span>
            <span>{{ isDark ? 'Modo Claro' : 'Modo Oscuro' }}</span>
          </button>
          <RouterLink to="/admin/nuevo-usuario" class="btn btn-info w-100 mb-2 fw-bold">
            + Nuevo Usuario
          </RouterLink>
          <button @click="handleLogout" class="btn btn-outline-light w-100">Cerrar Sesión</button>
        </div>

      </aside>

      <!-- ════════════════════════════════════
           ÁREA DE CONTENIDO
           ════════════════════════════════════ -->
      <main class="glass-admin d-flex flex-column overflow-auto w-100 content-main">

        <!-- ─── PANEL PRINCIPAL ─── -->
        <section v-if="currentTab === 'dashboard'">

          <div class="row g-3 mb-4">
            <div class="col-4">
              <div class="metric-card">
                <div class="metric-value text-accent">{{ myAssignedTickets.length }}</div>
                <div class="metric-label">Asignados a mí</div>
              </div>
            </div>
            <div class="col-4">
              <div class="metric-card">
                <div class="metric-value metric-value--urgente">{{ urgentCount }}</div>
                <div class="metric-label">Urgentes en cola</div>
              </div>
            </div>
            <div class="col-4">
              <div class="metric-card">
                <div class="metric-value metric-value--dim">{{ unassignedTickets.length }}</div>
                <div class="metric-label">Sin asignar</div>
              </div>
            </div>
          </div>

          <div class="row g-3">

            <div class="col-7">
              <div class="panel-inner d-flex flex-column panel-scroll">
                <div class="panel-section-header">
                  <span>Mis Tickets</span>
                  <span class="panel-section-count">{{ myAssignedTickets.length }}</span>
                </div>
                <div class="panel-scroll-body">
                  <table class="table table-borderless align-middle mb-0">
                    <thead>
                      <tr>
                        <th class="ps-4">Prioridad</th>
                        <th>Usuario</th>
                        <th>Asunto</th>
                        <th></th>
                      </tr>
                    </thead>
                    <tbody>
                      <template v-for="t in myAssignedTickets" :key="t.id">
                        <tr @click="toggleTicket(t.id)" class="ticket-row">
                          <td class="ps-4"><span :class="['badge', `badge-${t.priority}`]">{{ t.priority }}</span></td>
                          <td class="td-sm text-capitalize">{{ t.author }}</td>
                          <td class="td-sm text-truncate" style="max-width: 160px;">{{ t.subject }}</td>
                          <td class="pe-3 text-end">
                            <button @click.stop="marcarComoHecho(t.id)" class="btn btn-sm btn-info fw-bold btn-xs">
                              ✔ Hecho
                            </button>
                          </td>
                        </tr>
                        <tr v-if="expandedTicket === t.id" class="expand-row">
                          <td colspan="4" class="p-0 border-0">
                            <div class="ticket-detail">
                              <div class="ticket-meta">
                                <span><strong class="text-accent">{{ t.category }}</strong></span>
                                <span>{{ fmtDate(t.createdAt) }}</span>
                                <span class="col-id">{{ t.id }}</span>
                              </div>
                              <p class="mb-0 opacity-75 ticket-body">{{ t.description }}</p>
                            </div>
                          </td>
                        </tr>
                      </template>
                      <tr v-if="myAssignedTickets.length === 0">
                        <td colspan="4" class="text-center py-5 opacity-55 td-sm">Sin tickets asignados.</td>
                      </tr>
                    </tbody>
                  </table>
                </div>
              </div>
            </div>

            <div class="col-5">
              <div class="panel-inner d-flex flex-column panel-scroll">
                <div class="panel-section-header">
                  <span>Cola de Entrada</span>
                  <div class="d-flex align-items-center gap-2">
                    <span class="panel-section-count">{{ unassignedTickets.length }}/10</span>
                    <select class="form-select form-select-xs" v-model="priorityFilter" @click.stop>
                      <option value="todas">Todas</option>
                      <option value="urgente">Urgente</option>
                      <option value="alta">Alta</option>
                      <option value="moderada">Moderada</option>
                      <option value="baja">Baja</option>
                    </select>
                  </div>
                </div>
                <div class="panel-scroll-body">
                  <table class="table table-borderless align-middle mb-0">
                    <thead>
                      <tr>
                        <th class="ps-4">Prior.</th>
                        <th>Solicitante</th>
                        <th></th>
                      </tr>
                    </thead>
                    <tbody>
                      <template v-for="t in filteredUnassigned" :key="t.id">
                        <tr @click="toggleTicket(t.id)" class="ticket-row">
                          <td class="ps-4"><span :class="['badge', `badge-${t.priority}`]">{{ t.priority }}</span></td>
                          <td class="td-sm">
                            <div class="text-capitalize">{{ t.author }}</div>
                            <div class="td-stack-sub">{{ t.subject }}</div>
                          </td>
                          <td class="pe-3 text-end">
                            <button @click.stop="tomarTicket(t.id)" class="btn btn-sm btn-outline-info btn-xs">
                              Tomar
                            </button>
                          </td>
                        </tr>
                        <tr v-if="expandedTicket === t.id" class="expand-row">
                          <td colspan="3" class="p-0 border-0">
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
                      <tr v-if="filteredUnassigned.length === 0">
                        <td colspan="3" class="text-center py-5 opacity-55 td-sm">Cola vacía.</td>
                      </tr>
                    </tbody>
                  </table>
                </div>
              </div>
            </div>

          </div>
        </section>

        <!-- ─── DIRECTORIO ─── -->
        <section v-if="currentTab === 'usuarios'" class="d-flex flex-column h-100">
          <h2 class="h4 fw-bold mb-4">Directorio de Usuarios</h2>

          <div class="row g-3 flex-grow-1" style="min-height: 0;">

            <div class="col-4 d-flex flex-column" style="min-height: 0;">
              <div class="panel-inner d-flex flex-column flex-grow-1" style="overflow: hidden;">
                <div class="panel-section-header">
                  <span>Usuarios</span>
                  <span class="panel-section-count">{{ store.users.length }}</span>
                </div>
                <div class="overflow-auto flex-grow-1 p-2">
                  <button
                    v-for="u in store.users" :key="u.fullName"
                    @click="selectedUser = u; expandedTicket = null"
                    :class="['user-card w-100', { 'user-card--active': selectedUser?.fullName === u.fullName }]">
                    <div class="user-avatar" :style="{ background: avatarColor(u.fullName) }">
                      {{ initials(u.fullName) }}
                    </div>
                    <div class="user-card-body">
                      <div class="user-card-name text-capitalize">{{ u.fullName }}</div>
                      <div class="user-card-sub">
                        {{ userTickets(u.fullName).length }} ticket{{ userTickets(u.fullName).length !== 1 ? 's' : '' }}
                      </div>
                    </div>
                    <span class="user-card-arrow">›</span>
                  </button>
                </div>
              </div>
            </div>

            <div class="col-8 d-flex flex-column" style="min-height: 0;">
              <div v-if="selectedUser" class="panel-inner d-flex flex-column flex-grow-1" style="overflow: hidden;">
                <div class="user-profile-bar">
                  <div class="user-avatar user-avatar--lg" :style="{ background: avatarColor(selectedUser.fullName) }">
                    {{ initials(selectedUser.fullName) }}
                  </div>
                  <div class="flex-grow-1 min-w-0">
                    <div class="user-detail-name text-capitalize">{{ selectedUser.fullName }}</div>
                    <div class="user-detail-role">
                      {{ selectedUser.role === 'admin' ? 'Administrador' : 'Usuario' }}
                    </div>
                  </div>
                  <div class="d-flex gap-4">
                    <div class="mini-stat">
                      <div class="mini-stat-val">{{ userTickets(selectedUser.fullName).length }}</div>
                      <div class="mini-stat-lbl">Total</div>
                    </div>
                    <div class="mini-stat">
                      <div class="mini-stat-val text-accent">{{ userOpenTickets(selectedUser.fullName) }}</div>
                      <div class="mini-stat-lbl">Abiertos</div>
                    </div>
                    <div class="mini-stat">
                      <div class="mini-stat-val val-hecho">{{ userClosedTickets(selectedUser.fullName) }}</div>
                      <div class="mini-stat-lbl">Resueltos</div>
                    </div>
                  </div>
                </div>

                <div class="overflow-auto flex-grow-1">
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
                      <template v-for="t in userTickets(selectedUser.fullName)" :key="t.id">
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
                              <div class="ticket-meta"><span>{{ fmtDate(t.createdAt) }}</span></div>
                              <p class="mb-0 opacity-75 ticket-body">{{ t.description }}</p>
                            </div>
                          </td>
                        </tr>
                      </template>
                      <tr v-if="userTickets(selectedUser.fullName).length === 0">
                        <td colspan="4" class="text-center py-5 opacity-55 td-sm">Sin tickets.</td>
                      </tr>
                    </tbody>
                  </table>
                </div>
              </div>

              <div v-else class="panel-inner flex-grow-1 panel-empty">
                <div class="panel-empty-icon">◈</div>
                <div class="panel-empty-text">Selecciona un usuario<br>para ver su historial.</div>
              </div>
            </div>

          </div>
        </section>

        <!-- ─── HISTORIAL ─── -->
        <section v-if="currentTab === 'hechos'">
          <div class="d-flex justify-content-between align-items-baseline mb-4">
            <h2 class="h4 fw-bold mb-0">Tickets Resueltos</h2>
            <span class="panel-section-count">{{ store.closedTickets.length }} registros</span>
          </div>
          <div class="panel-inner">
            <table class="table table-borderless align-middle mb-0">
              <thead>
                <tr>
                  <th class="ps-4">ID</th>
                  <th>Autor</th>
                  <th>Asunto</th>
                  <th>Categoría</th>
                  <th>Estado</th>
                </tr>
              </thead>
              <tbody>
                <template v-for="t in store.closedTickets" :key="t.id">
                  <tr @click="toggleTicket(t.id)" class="ticket-row">
                    <td class="ps-4 col-id">{{ t.id }}</td>
                    <td class="text-capitalize td-sm">{{ t.author }}</td>
                    <td class="td-sm">{{ t.subject }}</td>
                    <td><span class="cat-pill">{{ t.category }}</span></td>
                    <td><span class="badge badge-hecho">Resuelto</span></td>
                  </tr>
                  <tr v-if="expandedTicket === t.id" class="expand-row">
                    <td colspan="5" class="p-0 border-0">
                      <div class="ticket-detail">
                        <div class="ticket-meta"><span>{{ fmtDate(t.createdAt) }}</span></div>
                        <p class="mb-0 opacity-75 ticket-body">{{ t.description }}</p>
                      </div>
                    </td>
                  </tr>
                </template>
                <tr v-if="store.closedTickets.length === 0">
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
import { store } from '../store'
import { useTheme } from '../composables/useTheme'

const router         = useRouter()
const currentTab     = ref('dashboard')
const selectedUser   = ref(null)
const priorityFilter = ref('todas')
const expandedTicket = ref(null)
const { isDark, toggle, init } = useTheme()

onMounted(() => init())

const myAssignedTickets = computed(() =>
  store.tickets.filter(t => t.assignedTo === store.currentUser?.fullName && t.status !== 'hecho')
)
const unassignedTickets = computed(() =>
  store.tickets.filter(t => t.status === 'disponible')
)
const filteredUnassigned = computed(() => {
  const weight = { urgente: 4, alta: 3, moderada: 2, baja: 1 }
  let list = unassignedTickets.value
  if (priorityFilter.value !== 'todas')
    list = list.filter(t => t.priority === priorityFilter.value)
  return [...list].sort((a, b) =>
    weight[b.priority] !== weight[a.priority]
      ? weight[b.priority] - weight[a.priority]
      : a.createdAt - b.createdAt
  )
})

const urgentCount  = computed(() => unassignedTickets.value.filter(t => t.priority === 'urgente').length)
const totalPending = computed(() => myAssignedTickets.value.length + unassignedTickets.value.length)

const userTickets       = (n) => [...store.tickets, ...store.closedTickets].filter(t => t.author === n)
const userOpenTickets   = (n) => store.tickets.filter(t => t.author === n).length
const userClosedTickets = (n) => store.closedTickets.filter(t => t.author === n).length

const tomarTicket     = (id) => store.acceptTicket(id)
const marcarComoHecho = (id) => store.closeTicket(id)
const handleLogout    = ()   => { store.logout(); router.push('/') }
const toggleTicket    = (id) => { expandedTicket.value = expandedTicket.value === id ? null : id }
const fmtDate         = (ts) => new Date(ts).toLocaleString('es-MX', { dateStyle: 'short', timeStyle: 'short' })

const PALETTE = ['#2d6a4f','#1b4332','#40916c','#184e77','#1e6091','#6b3fa0','#553285','#8b2fc9','#7b2d8b','#9c4221']
const initials    = (n) => n.trim().split(' ').slice(0, 2).map(w => w[0].toUpperCase()).join('')
const avatarColor = (n) => { let h = 0; for (const c of n) h = (h * 31 + c.charCodeAt(0)) & 0xffff; return PALETTE[h % PALETTE.length] }
</script>
