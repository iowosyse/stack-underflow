<template>
  <div class="d-flex flex-column vh-100 user-bg" id="wrapper">
    <div class="d-flex flex-grow-1 overflow-hidden p-3 gap-3">

      <!-- Sidebar -->
      <div class="glass-user p-4 d-flex flex-column justify-content-between flex-shrink-0" style="width: 280px;">
        <div>
          <div class="logo mb-5 text-accent">stack-underflow</div>
          <nav class="list-group list-group-flush">
            <RouterLink to="/user"           class="list-group-item">Mis Tickets</RouterLink>
            <RouterLink to="/knowledge-base" class="list-group-item">Base de Conocimiento</RouterLink>
          </nav>
        </div>

        <!-- Theme Toggle -->
        <div>
          <button @click="toggle" class="theme-toggle mb-3">
            <span class="theme-toggle-icon">{{ isDark ? '☀' : '☾' }}</span>
            <span class="theme-toggle-label">{{ isDark ? 'Modo Claro' : 'Modo Oscuro' }}</span>
          </button>
          <div class="small mb-2 opacity-55 text-capitalize">
            Perfil: <strong>{{ store.currentUser?.fullName }}</strong>
          </div>
          <button @click="logout" class="btn btn-outline-danger w-100 rounded-pill">
            Cerrar Sesión
          </button>
        </div>
      </div>

      <!-- Contenido principal -->
      <div class="glass-user container-fluid p-5 d-flex flex-column overflow-auto w-100">

        <h2 class="h3 mb-4 fw-bold">Mis Solicitudes</h2>

        <div class="row g-4 mb-4">

          <!-- Formulario de ticket -->
          <div class="col-md-8">
            <div class="panel-inner p-4 h-100">
              <h5 class="fw-bold mb-4 opacity-55">Levantar Incidencia</h5>
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
                <div class="mb-4">
                  <label class="form-label">Descripción detallada</label>
                  <textarea class="form-control" rows="3" v-model="description"
                            placeholder="Describe el problema con el mayor detalle posible…"></textarea>
                </div>
                <button type="submit" class="btn btn-primary w-100 py-2 rounded-pill">
                  Enviar Ticket
                </button>
              </form>
            </div>
          </div>

          <!-- Contador de tickets -->
          <div class="col-md-4">
            <div class="stat-panel p-4 h-100 d-flex align-items-center justify-content-center text-center">
              <div>
                <p class="stat-label mb-3">Tickets en Proceso</p>
                <div class="stat-number">{{ activeTicketsCount }}</div>
              </div>
            </div>
          </div>

        </div>

        <!-- Historial -->
        <div class="panel-inner p-4">
          <h5 class="fw-bold mb-3 opacity-55">Historial de Reportes</h5>
          <table class="table table-borderless align-middle mb-0">
            <thead>
              <tr>
                <th class="ps-3">ID</th>
                <th>Asunto</th>
                <th>Estado</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="t in myTickets" :key="t.id">
                <td class="ps-3 col-id">{{ t.id }}</td>
                <td>{{ t.subject }}</td>
                <td>
                  <span :class="['badge', t.status === 'hecho' ? 'badge-hecho' : 'badge-open']">
                    {{ t.status }}
                  </span>
                </td>
              </tr>
              <tr v-if="myTickets.length === 0">
                <td colspan="3" class="text-center py-5 opacity-55">
                  No has levantado tickets recientemente.
                </td>
              </tr>
            </tbody>
          </table>
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
const subject     = ref('')
const category    = ref('Hardware')
const description = ref('')
const { isDark, toggle, init } = useTheme()

onMounted(() => init())

const activeTicketsCount = computed(() =>
  store.tickets.filter(t => t.author === store.currentUser?.fullName).length
)
const myTickets = computed(() =>
  [...store.tickets, ...store.closedTickets].filter(t => t.author === store.currentUser?.fullName)
)

const submitTicket = () => {
  if (!subject.value || !description.value) return alert("Llena todos los campos.")
  store.addTicket(subject.value, category.value, description.value)
  subject.value     = ''
  description.value = ''
}

const logout = () => { store.logout(); router.push('/') }
</script>
