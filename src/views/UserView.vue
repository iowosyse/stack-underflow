<template>
  <div class="user-theme d-flex flex-column vh-100" id="wrapper">
    <div class="d-flex flex-grow-1 overflow-hidden">
      <div class="sidebar border-end bg-light p-3 d-flex flex-column justify-content-between">
        <div>
          <div class="logo h4 fw-bold mb-4 px-2 text-primary-emphasis">stack-underflow</div>
          <div class="list-group list-group-flush bg-transparent">
            <RouterLink to="/user" class="list-group-item list-group-item-action bg-transparent active fw-bold">Mis Tickets</RouterLink>
            <RouterLink to="/knowledge-base" class="list-group-item list-group-item-action bg-transparent">Base de Conocimiento</RouterLink>
          </div>
        </div>
        <div class="px-2">
          <div class="small mb-2 text-capitalize">Perfil: <strong>{{ store.currentUser?.fullName }}</strong></div>
          <button @click="logout" class="btn btn-link text-danger text-decoration-none fw-bold p-0">Cerrar Sesion</button>
        </div>
      </div>

      <div class="container-fluid p-0 d-flex flex-column main-content overflow-auto">
        <div class="p-5 flex-grow-1">
          <h1 class="h3 mb-4 border-bottom pb-2">Mis Solicitudes</h1>

          <div class="row g-4 mb-4">
            <div class="col-md-8">
              <div class="card rounded-0 border-secondary-subtle shadow-sm">
                <div class="card-header bg-light border-bottom fw-bold rounded-0">Levantar Incidencia</div>
                <div class="card-body">
                  <form @submit.prevent="submitTicket">
                    <div class="mb-3">
                      <label class="form-label small text-muted">Asunto</label>
                      <input type="text" class="form-control rounded-0" v-model="subject">
                    </div>
                    <div class="row mb-3">
                      <div class="col">
                        <label class="form-label small text-muted">Categoría</label>
                        <select class="form-select rounded-0" v-model="category">
                          <option>Hardware</option><option>Software</option><option>Redes</option>
                        </select>
                      </div>
                      <div class="col">
                        <label class="form-label small text-muted">Prioridad</label>
                        <select class="form-select rounded-0" v-model="priority">
                          <option value="baja">Baja</option><option value="moderada">Moderada</option>
                          <option value="alta">Alta</option><option value="urgente">Urgente</option>
                        </select>
                      </div>
                    </div>
                    <div class="mb-3">
                      <label class="form-label small text-muted">Descripción</label>
                      <textarea class="form-control rounded-0" rows="3" v-model="description"></textarea>
                    </div>
                    <button type="submit" class="btn btn-primary rounded-0 px-4">Enviar Ticket</button>
                  </form>
                </div>
              </div>
            </div>

            <div class="col-md-4">
              <div class="card h-100 rounded-0 bg-light text-center shadow-sm d-flex justify-content-center">
                <div>
                  <h3 class="text-muted small fw-bold mb-2">Tickets Activos</h3>
                  <div class="display-1 fw-bold text-primary-emphasis">{{ activeTicketsCount }}</div>
                </div>
              </div>
            </div>
          </div>

          <div class="card rounded-0 shadow-sm">
            <div class="card-header bg-light fw-bold rounded-0">Historial</div>
            <div class="card-body p-0">
              <table class="table table-hover mb-0">
                <thead class="table-light">
                  <tr><th class="ps-4">ID</th><th>Asunto</th><th>Prioridad</th><th>Estado</th></tr>
                </thead>
                <tbody>
                  <tr v-for="t in myTickets" :key="t.id">
                    <td class="ps-4 font-monospace text-muted">{{ t.id }}</td>
                    <td>{{ t.subject }}</td><td>{{ t.priority }}</td>
                    <td><span class="badge bg-danger-subtle text-danger border">{{ t.status }}</span></td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { store } from '../store'

const router = useRouter()
const subject = ref('')
const category = ref('Hardware')
const priority = ref('baja')
const description = ref('')

const activeTicketsCount = computed(() => store.tickets.filter(t => t.author === store.currentUser?.fullName && t.status !== 'cerrado').length)
const myTickets = computed(() => store.tickets.filter(t => t.author === store.currentUser?.fullName))

const submitTicket = () => {
  if(!subject.value || !description.value) return alert("Llena todos los campos")
  store.addTicket(subject.value, category.value, priority.value, description.value)
  subject.value = ''
  description.value = ''
  alert("Ticket levantado con éxito")
}

const logout = () => {
  store.logout()
  router.push('/')
}
</script>