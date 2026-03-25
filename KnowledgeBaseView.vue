<template>
  <div class="user-theme d-flex flex-column vh-100" id="wrapper">
    <div class="d-flex flex-grow-1 overflow-hidden">
      <div class="sidebar border-end bg-light p-3 d-flex flex-column justify-content-between">
        <div>
          <div class="logo h4 fw-bold mb-4 px-2 text-primary-emphasis">stack-underflow</div>
          <div class="list-group list-group-flush bg-transparent">
            <RouterLink to="/user" class="list-group-item list-group-item-action bg-transparent">Mis Tickets</RouterLink>
            <RouterLink to="/knowledge-base" class="list-group-item list-group-item-action bg-transparent active fw-bold">Base de Conocimiento</RouterLink>
          </div>
        </div>
        <div class="px-2">
          <RouterLink to="/" class="text-danger text-decoration-none fw-bold">Cerrar Sesion</RouterLink>
        </div>
      </div>

      <div class="container-fluid p-0 d-flex flex-column main-content overflow-auto">
        <div class="p-5 flex-grow-1">
          <div class="mb-4 pb-2 border-bottom border-secondary-subtle">
            <h1 class="h3 m-0">Ayuda y Tutoriales</h1>
            <p class="text-muted mt-2">Resuelve tus dudas rápidas sin esperar a un técnico.</p>
          </div>

          <div class="row g-4">
            <div class="col-12">
              <div class="card rounded-0 border-secondary-subtle shadow-sm mb-4">
                <div class="card-header bg-light fw-bold rounded-0">Redes e Internet</div>
                <div class="card-body">
                  <div class="mb-3 p-2 border-bottom border-light-subtle">
                    <div class="fw-bold text-primary accordion-header" @click="toggle('ans1')">&#9656; ¿Qué es una red y por qué no tengo sistema?</div>
                    <div class="mt-2 text-secondary px-3 accordion-content" v-show="openTab === 'ans1'">
                      <p>Una red es un grupo de computadoras conectadas. Si no tienes sistema, perdiste conexión con el servidor.</p>
                    </div>
                  </div>
                  <div class="mb-3 p-2 border-bottom border-light-subtle">
                    <div class="fw-bold text-primary accordion-header" @click="toggle('ans2')">&#9656; Diferencia entre WiFi y Cable</div>
                    <div class="mt-2 text-secondary px-3 accordion-content" v-show="openTab === 'ans2'">
                      <p>El Cable (Ethernet) es más rápido y estable. El WiFi es inalámbrico y susceptible a interferencias.</p>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
const openTab = ref(null)
const toggle = (id) => { openTab.value = openTab.value === id ? null : id }
</script>

<style scoped>
.accordion-header { cursor: pointer; }
</style>