<template>
  <div class="d-flex flex-column vh-100 bg-light" id="wrapper">
    <div class="d-flex flex-grow-1 overflow-hidden">
      <div class="sidebar p-4 d-flex flex-column justify-content-between flex-shrink-0" style="width: 280px;">
        <div>
          <div class="logo h4 fw-bold mb-5 text-primary">stack-underflow</div>
          <div class="list-group list-group-flush bg-transparent">
            <RouterLink to="/user" class="list-group-item list-group-item-action bg-transparent active">Mis Tickets</RouterLink>
            <RouterLink to="/knowledge-base" class="list-group-item list-group-item-action bg-transparent">Base de Conocimiento</RouterLink>
          </div>
        </div>
        <div>
          <div class="small text-muted mb-2 text-capitalize">Perfil: <strong class="text-dark">{{ store.currentUser?.fullName }}</strong></div>
          <button @click="logout" class="btn btn-outline-danger w-100">Cerrar Sesión</button>
        </div>
      </div>

      <div class="container-fluid p-5 d-flex flex-column overflow-auto">
        <h2 class="h3 mb-4 fw-bold">Mis Solicitudes</h2>

        <div class="row g-4 mb-4">
          <div class="col-md-8">
            <div class="card p-4 h-100">
              <h5 class="fw-bold mb-4">Levantar Incidencia</h5>
              <form @submit.prevent="submitTicket">
                <div class="mb-3">
                  <label class="form-label text-muted small fw-bold">Asunto</label>
                  <input type="text" class="form-control" v-model="subject" placeholder="Ej. No enciende la computadora">
                </div>
                <div class="mb-3">
                  <label class="form-label text-muted small fw-bold">Categoría</label>
                  <select class="form-select" v-model="category">
                    <option>Hardware</option><option>Software</option><option>Redes</option>
                  </select>
                </div>
                <div class="mb-4">
                  <label class="form-label text-muted small fw-bold">Descripción detallada</label>
                  <textarea class="form-control" rows="3" v-model="description" placeholder="Describe el problema..."></textarea>
                </div>
                <button type="submit" class="btn btn-primary w-100 py-2">Enviar Ticket a Soporte</button>
              </form>
            </div>
          </div>

          <div class="col-md-4">
            <div class="card p-4 h-100 bg-primary text-white d-flex align-items-center justify-content-center text-center">
              <div>
                <h6 class="fw-bold mb-3 opacity-75 text-uppercase tracking-wide">Tickets en Proceso</h6>
                <div class="display-1 fw-bold">{{ activeTicketsCount }}</div>
              </div>
            </div>
          </div>
        </div>

        <div class="card p-3 mt-2">
          <h5 class="fw-bold px-3 pt-2 pb-3 mb-0 border-bottom">Historial de Reportes</h5>
          <div class="card-body p-0 mt-2">
            <table class="table table-borderless align-middle mb-0">
              <thead class="border-bottom">
                <tr><th class="ps-3 text-muted">ID</th><th class="text-muted">Asunto</th><th class="text-muted">Estado</th></tr>
              </thead>
              <tbody>
                <tr v-for="t in myTickets" :key="t.id" class="border-bottom">
                  <td class="ps-3 fw-bold">{{ t.id }}</td>
                  <td>{{ t.subject }}</td>
                  <td>
                    <span :class="['badge px-3 py-2 rounded-pill', t.status === 'hecho' ? 'bg-success bg-opacity-10 text-success' : 'bg-secondary bg-opacity-10 text-secondary']">
                      {{ t.status }}
                    </span>
                  </td>
                </tr>
                <tr v-if="myTickets.length === 0">
                    <td colspan="3" class="text-center py-5 text-muted">No has levantado tickets recientemente.</td>
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
import { ref, computed } from 'vue'
import { useRouter } from 'vue-router'
import { store } from '../store'

const router = useRouter()
const subject = ref('')
const category = ref('Hardware')
const description = ref('')

// Cuenta únicamente los activos de la tabla principal
const activeTicketsCount = computed(() => store.tickets.filter(t => t.author === store.currentUser?.fullName).length)

// Combina el arreglo activo con el cerrado para que el usuario pueda ver todo lo que ha pedido históricamente
const myTickets = computed(() => {
    return [...store.tickets, ...store.closedTickets].filter(t => t.author === store.currentUser?.fullName)
})

const submitTicket = () => {
  if(!subject.value || !description.value) return alert("Por favor, llena todos los campos.")
  store.addTicket(subject.value, category.value, description.value)
  subject.value = ''
  description.value = ''
}

const logout = () => {
  store.logout()
  router.push('/')
}
</script>