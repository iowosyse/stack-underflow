<template>
  <div class="d-flex flex-column vh-100 bg-light" id="wrapper">
    <div class="d-flex flex-grow-1 overflow-hidden">
      <div class="sidebar p-4 d-flex flex-column justify-content-between flex-shrink-0" style="width: 280px;">
        <div>
          <div class="logo h4 fw-bold mb-5 text-primary">SYS_ADMIN</div>
          <div class="list-group list-group-flush bg-transparent">
            <RouterLink to="/admin" class="list-group-item list-group-item-action bg-transparent text-start active">
              ← Volver al Panel
            </RouterLink>
          </div>
        </div>
        <div>
          <div class="small text-muted mb-2 text-capitalize">Admin: {{ store.currentUser?.fullName }}</div>
          <button @click="handleLogout" class="btn btn-outline-danger w-100">Cerrar Sesión</button>
        </div>
      </div>

      <div class="container-fluid p-5 d-flex flex-column overflow-auto">
        
        <div class="w-100 mx-auto" style="max-width: 600px; margin-top: 8vh;">
            <div class="d-flex justify-content-between align-items-center mb-4">
                <h2 class="h3 m-0 fw-bold">Agregar Nuevo Usuario</h2>
                <RouterLink to="/admin" class="btn btn-outline-secondary">Cancelar</RouterLink>
            </div>

            <div class="card p-5">
                <form @submit.prevent="crearUsuario">
                    <div class="mb-4">
                        <label class="form-label text-muted small fw-bold">Nombre Completo del Usuario</label>
                        <input type="text" class="form-control form-control-lg" v-model="newUserName" placeholder="Ej. Juan Perez Gomez" autofocus>
                        
                        <div class="alert alert-info mt-4 pb-2 pt-3 border-0 bg-primary bg-opacity-10 text-primary rounded-3">
                            <h6 class="fw-bold"><small>ℹ️ Generación Automática de Credenciales</small></h6>
                            <p class="small mb-0 opacity-75">El sistema tomará el primer nombre como <strong>Usuario</strong> y los apellidos como <strong>Contraseña</strong> (todo en minúsculas y sin acentos).</p>
                        </div>
                    </div>
                    <button type="submit" class="btn btn-primary w-100 py-3 fw-bold fs-6">Registrar Usuario</button>
                </form>
            </div>
        </div>

      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { store } from '../store'

const router = useRouter()
const newUserName = ref('')

const crearUsuario = () => {
  if(newUserName.value.trim()) {
    store.addUser(newUserName.value, 'user')
    alert(`Usuario registrado exitosamente.\nNombre: ${newUserName.value}`)
    router.push('/admin') // Devuelve al admin a su panel automáticamente
  } else {
    alert("Por favor ingresa un nombre válido.")
  }
}

const handleLogout = () => {
  store.logout()
  router.push('/')
}
</script>