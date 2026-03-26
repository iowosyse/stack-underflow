<template>
  <div class="d-flex flex-column vh-100 admin-bg" id="wrapper">
    <div class="d-flex flex-grow-1 overflow-hidden p-3 gap-3">

      <!-- Sidebar -->
      <div class="glass-admin p-4 d-flex flex-column justify-content-between flex-shrink-0" style="width: 280px;">
        <div>
          <div class="logo mb-5 text-accent">SYS_ADMIN</div>
          <nav class="list-group list-group-flush">
            <button @click="currentTab = 'dashboard'"
                    :class="['list-group-item text-start', { active: currentTab === 'dashboard' }]">
              Mis Tickets Asignados
            </button>
            <button @click="currentTab = 'usuarios'"
                    :class="['list-group-item text-start', { active: currentTab === 'usuarios' }]">
              Directorio y Tickets
            </button>
            <button @click="currentTab = 'disponibles'"
                    :class="['list-group-item text-start', { active: currentTab === 'disponibles' }]">
              Tickets sin Asignar
            </button>
            <button @click="currentTab = 'hechos'"
                    :class="['list-group-item text-start', { active: currentTab === 'hechos' }]">
              Historial de Hechos
            </button>
          </nav>
        </div>

        <!-- Theme Toggle -->
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

      <!-- Contenido principal -->
      <div class="glass-admin container-fluid p-5 d-flex flex-column overflow-auto w-100">

        <!-- ─ Dashboard: Mis Tickets Asignados ─ -->
        <div v-if="currentTab === 'dashboard'">
          <h2 class="h3 mb-4 fw-bold">Mis Tickets Asignados</h2>
          <div class="panel-inner p-4">
            <table class="table table-borderless align-middle mb-0">
              <thead>
                <tr>
                  <th class="ps-3">ID</th>
                  <th>Prioridad</th>
                  <th>Usuario</th>
                  <th>Asunto</th>
                  <th>Acción</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="t in myAssignedTickets" :key="t.id">
                  <td class="ps-3 col-id">{{ t.id }}</td>
                  <td><span :class="['badge', `badge-${t.priority}`]">{{ t.priority }}</span></td>
                  <td class="text-capitalize">{{ t.author }}</td>
                  <td>{{ t.subject }}</td>
                  <td>
                    <button @click="marcarComoHecho(t.id)"
                            class="btn btn-sm btn-info rounded-pill fw-bold px-3">
                      ✔ Hecho
                    </button>
                  </td>
                </tr>
                <tr v-if="myAssignedTickets.length === 0">
                  <td colspan="5" class="text-center py-5 opacity-55">
                    No tienes tickets asignados en cola.
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <!-- ─ Directorio de Usuarios ─ -->
        <div v-if="currentTab === 'usuarios'">
          <div class="d-flex justify-content-between align-items-center mb-4">
            <h2 class="h3 m-0 fw-bold">Directorio de Usuarios</h2>
            <RouterLink to="/admin/nuevo-usuario"
                        class="btn btn-info rounded-pill fw-bold px-4">
              + Nuevo Usuario
            </RouterLink>
          </div>

          <div class="row g-4">
            <div class="col-4">
              <div class="panel-inner p-2" style="max-height: 62vh; overflow-y: auto;">
                <div class="list-group list-group-flush">
                  <button v-for="u in store.users" :key="u.fullName"
                          @click="selectedUser = u"
                          :class="['list-group-item text-capitalize',
                                   { active: selectedUser?.fullName === u.fullName }]">
                    {{ u.fullName }}
                  </button>
                </div>
              </div>
            </div>

            <div class="col-8">
              <div v-if="selectedUser" class="panel-inner p-4">
                <h5 class="fw-bold mb-4 opacity-55 text-capitalize">
                  Historial de {{ selectedUser.fullName }}
                </h5>
                <table class="table table-borderless align-middle mb-0">
                  <thead>
                    <tr>
                      <th class="ps-3">ID</th>
                      <th>Asunto</th>
                      <th>Estado</th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr v-for="t in userTickets(selectedUser.fullName)" :key="t.id">
                      <td class="ps-3 col-id">{{ t.id }}</td>
                      <td>{{ t.subject }}</td>
                      <td>
                        <span :class="['badge', t.status === 'hecho' ? 'badge-hecho' : 'badge-open']">
                          {{ t.status }}
                        </span>
                      </td>
                    </tr>
                    <tr v-if="userTickets(selectedUser.fullName).length === 0">
                      <td colspan="3" class="text-center py-5 opacity-55">
                        Este usuario no ha levantado tickets.
                      </td>
                    </tr>
                  </tbody>
                </table>
              </div>
              <div v-else class="panel-inner p-4 d-flex align-items-center justify-content-center"
                   style="min-height: 200px;">
                <p class="opacity-55 mb-0" style="font-size: 0.875rem;">
                  Selecciona un usuario del directorio.
                </p>
              </div>
            </div>
          </div>
        </div>

        <!-- ─ Tickets Sin Asignar ─ -->
        <div v-if="currentTab === 'disponibles'">
          <div class="d-flex justify-content-between align-items-center mb-4">
            <h2 class="h3 m-0 fw-bold">
              Tickets Sin Asignar
              <span class="badge badge-open ms-2" style="font-size: 0.72rem; vertical-align: middle;">
                {{ unassignedTickets.length }}/10
              </span>
            </h2>
            <select class="form-select" style="max-width: 220px;" v-model="priorityFilter">
              <option value="todas">Todas las prioridades</option>
              <option value="urgente">Urgente</option>
              <option value="alta">Alta</option>
              <option value="moderada">Moderada</option>
              <option value="baja">Baja</option>
            </select>
          </div>

          <div class="panel-inner p-4">
            <table class="table table-borderless align-middle mb-0">
              <thead>
                <tr>
                  <th class="ps-3">ID</th>
                  <th>Prioridad</th>
                  <th>Autor</th>
                  <th>Asunto</th>
                  <th>Acción</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="t in filteredUnassigned" :key="t.id">
                  <td class="ps-3 col-id">{{ t.id }}</td>
                  <td><span :class="['badge', `badge-${t.priority}`]">{{ t.priority }}</span></td>
                  <td class="text-capitalize">{{ t.author }}</td>
                  <td>{{ t.subject }}</td>
                  <td>
                    <button @click="tomarTicket(t.id)"
                            class="btn btn-sm btn-outline-info rounded-pill px-3">
                      Tomar
                    </button>
                  </td>
                </tr>
                <tr v-if="filteredUnassigned.length === 0">
                  <td colspan="5" class="text-center py-5 opacity-55">
                    No hay tickets pendientes con ese filtro.
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

        <!-- ─ Historial de Hechos ─ -->
        <div v-if="currentTab === 'hechos'">
          <h2 class="h3 mb-4 fw-bold">Archivo de Tickets Resueltos</h2>
          <div class="panel-inner p-4">
            <table class="table table-borderless align-middle mb-0">
              <thead>
                <tr>
                  <th class="ps-3">ID</th>
                  <th>Autor</th>
                  <th>Asunto</th>
                  <th>Estado</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="t in store.closedTickets" :key="t.id">
                  <td class="ps-3 col-id">{{ t.id }}</td>
                  <td class="text-capitalize">{{ t.author }}</td>
                  <td>{{ t.subject }}</td>
                  <td><span class="badge badge-hecho">Resuelto</span></td>
                </tr>
                <tr v-if="store.closedTickets.length === 0">
                  <td colspan="4" class="text-center py-5 opacity-55">
                    Aún no hay tickets marcados como resueltos.
                  </td>
                </tr>
              </tbody>
            </table>
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

const router         = useRouter()
const currentTab     = ref('dashboard')
const selectedUser   = ref(null)
const priorityFilter = ref('todas')
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

const userTickets    = (name)  => [...store.tickets, ...store.closedTickets].filter(t => t.author === name)
const tomarTicket    = (id)    => store.acceptTicket(id)
const marcarComoHecho = (id)   => store.closeTicket(id)
const handleLogout   = ()      => { store.logout(); router.push('/') }
</script>
