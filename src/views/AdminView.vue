<template>
  <div class="d-flex flex-column vh-100 bg-light" id="wrapper">
    <div class="d-flex flex-grow-1 overflow-hidden">
      <div class="sidebar p-4 d-flex flex-column justify-content-between" style="width: 280px;">
        <div>
          <div class="logo h4 fw-bold mb-5 text-primary">SYS_ADMIN</div>
          <div class="list-group list-group-flush bg-transparent">
            <button @click="currentTab = 'dashboard'" :class="['list-group-item list-group-item-action bg-transparent text-start', { 'active': currentTab === 'dashboard' }]">Mis Tickets Asignados</button>
            <button @click="currentTab = 'usuarios'" :class="['list-group-item list-group-item-action bg-transparent text-start', { 'active': currentTab === 'usuarios' }]">Directorio y Tickets</button>
            <button @click="currentTab = 'disponibles'" :class="['list-group-item list-group-item-action bg-transparent text-start', { 'active': currentTab === 'disponibles' }]">Tickets sin Asignar</button>
          </div>
        </div>
        <div>
          <div class="small text-muted mb-2 text-capitalize">Admin: {{ store.currentUser?.fullName }}</div>
          <button @click="handleLogout" class="btn btn-outline-danger w-100">Cerrar Sesión</button>
        </div>
      </div>

      <div class="container-fluid p-5 d-flex flex-column overflow-auto">

        <div v-if="currentTab === 'dashboard'">
          <h2 class="h3 mb-4 fw-bold">Mis Tickets Asignados</h2>
          <div class="card bg-white p-3">
            <div class="card-body p-0">
              <table class="table table-borderless align-middle mb-0">
                <thead class="border-bottom">
                  <tr><th class="ps-3 text-muted">ID</th><th class="text-muted">Prioridad</th><th class="text-muted">Usuario</th><th class="text-muted">Asunto</th><th class="text-muted">Acción</th></tr>
                </thead>
                <tbody>
                  <tr v-for="t in myAssignedTickets" :key="t.id" class="border-bottom">
                    <td class="ps-3 fw-bold">{{ t.id }}</td>
                    <td><span class="badge bg-danger bg-opacity-10 text-danger px-3 py-2 rounded-pill">{{ t.priority }}</span></td>
                    <td class="text-capitalize">{{ t.author }}</td>
                    <td>{{ t.subject }}</td>
                    <td><button @click="cerrarTicket(t)" class="btn btn-sm btn-dark px-3 rounded-pill">Resolver</button></td>
                  </tr>
                  <tr v-if="myAssignedTickets.length === 0">
                    <td colspan="5" class="text-center py-5 text-muted">No tienes tickets asignados en cola. ¡Buen trabajo!</td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>

        <div v-if="currentTab === 'usuarios'">
          <div class="d-flex justify-content-between align-items-center mb-4">
            <h2 class="h3 m-0 fw-bold">Directorio de Usuarios</h2>
            <RouterLink to="/admin/nuevo-usuario" class="btn btn-primary shadow-sm">+ Nuevo Usuario</RouterLink>
          </div>
          <div class="row g-4">
            <div class="col-4">
              <div class="card p-2">
                <div class="list-group list-group-flush" style="max-height: 60vh; overflow-y: auto;">
                  <button v-for="u in store.users" :key="u.fullName" @click="selectedUser = u" :class="['list-group-item list-group-item-action text-capitalize', { 'active': selectedUser?.fullName === u.fullName }]">
                    {{ u.fullName }}
                  </button>
                </div>
              </div>
            </div>
            <div class="col-8">
              <div v-if="selectedUser" class="card p-3">
                <div class="card-header bg-transparent border-0 pt-2 pb-3 fw-bold text-capitalize fs-5">Historial de {{ selectedUser.fullName }}</div>
                <div class="card-body p-0">
                   <table class="table table-borderless align-middle mb-0">
                      <thead class="border-bottom"><tr><th class="ps-3 text-muted">ID</th><th class="text-muted">Asunto</th><th class="text-muted">Estado</th></tr></thead>
                      <tbody>
                        <tr v-for="t in userTickets(selectedUser.fullName)" :key="t.id" class="border-bottom">
                          <td class="ps-3">{{ t.id }}</td><td>{{ t.subject }}</td>
                          <td><span class="badge bg-secondary bg-opacity-10 text-secondary px-3 py-2 rounded-pill">{{ t.status }}</span></td>
                        </tr>
                        <tr v-if="userTickets(selectedUser.fullName).length === 0">
                          <td colspan="3" class="text-center py-5 text-muted">Este usuario no ha levantado tickets.</td>
                        </tr>
                      </tbody>
                   </table>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div v-if="currentTab === 'disponibles'">
          <div class="d-flex justify-content-between align-items-center mb-4">
            <h2 class="h3 m-0 fw-bold">Tickets Sin Asignar ({{ unassignedTickets.length }}/10)</h2>
            <select class="form-select w-25 shadow-sm" v-model="priorityFilter">
              <option value="todas">Todas las prioridades</option>
              <option value="urgente">Urgente</option>
              <option value="alta">Alta</option>
              <option value="moderada">Moderada</option>
              <option value="baja">Baja</option>
            </select>
          </div>
          
          <div class="card p-3">
            <table class="table table-borderless align-middle mb-0">
              <thead class="border-bottom">
                <tr><th class="ps-3 text-muted">ID</th><th class="text-muted">Prioridad</th><th class="text-muted">Autor</th><th class="text-muted">Asunto</th><th class="text-muted">Fecha</th></tr>
              </thead>
              <tbody>
                <tr v-for="t in filteredUnassigned" :key="t.id" class="border-bottom">
                  <td class="ps-3 fw-bold">{{ t.id }}</td>
                  <td><span class="badge bg-secondary px-3 py-2 rounded-pill">{{ t.priority }}</span></td>
                  <td class="text-capitalize">{{ t.author }}</td>
                  <td>{{ t.subject }}</td>
                  <td class="small text-muted">{{ t.createdAt.toLocaleTimeString() }}</td>
                </tr>
                <tr v-if="filteredUnassigned.length === 0">
                   <td colspan="5" class="text-center py-5 text-muted">No hay tickets pendientes actualmente.</td>
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
const currentTab = ref('dashboard')
const selectedUser = ref(null)
const priorityFilter = ref('todas')

const myAssignedTickets = computed(() => store.tickets.filter(t => t.assignedTo === store.currentUser?.fullName && t.status !== 'cerrado'))
const unassignedTickets = computed(() => store.tickets.filter(t => t.status === 'disponible'))

const filteredUnassigned = computed(() => {
  let list = unassignedTickets.value
  if (priorityFilter.value !== 'todas') list = list.filter(t => t.priority === priorityFilter.value)
  return list.sort((a, b) => a.createdAt - b.createdAt)
})

const userTickets = (authorName) => store.tickets.filter(t => t.author === authorName)
const cerrarTicket = (ticket) => { ticket.status = 'cerrado' }

const handleLogout = () => {
  store.logout()
  router.push('/')
}
</script>