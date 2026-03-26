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
            Perfil: <strong>{{ store.currentUser?.fullName || 'Usuario' }}</strong>
          </div>
          <button @click="logout" class="btn btn-outline-danger w-100 rounded-pill">
            Cerrar Sesión
          </button>
        </div>
      </div>

      <!-- Contenido -->
      <div class="glass-user container-fluid p-5 d-flex flex-column overflow-auto w-100">

        <div class="mb-4 pb-3 border-bottom">
          <h1 class="h3 m-0 fw-bold">Ayuda y Tutoriales</h1>
          <p class="opacity-55 mt-2 mb-0" style="font-size: 0.875rem;">
            Resuelve dudas frecuentes sin esperar a un técnico.
          </p>
        </div>

        <!-- Sección: Redes e Internet -->
        <div class="panel-inner p-4 mb-4">
          <h5 class="fw-bold mb-4 text-accent" style="font-size: 0.78rem; letter-spacing: 0.08em; text-transform: uppercase; font-family: 'Inter', sans-serif;">
            Redes e Internet
          </h5>

          <div class="accordion-item-inner mb-3 p-3" @click="toggle_acc('ans1')">
            <div class="accordion-header fw-bold">
              &#9656; ¿Qué es una red y por qué no tengo sistema?
            </div>
            <div class="mt-3 opacity-55 accordion-content" v-show="openTab === 'ans1'" style="font-size: 0.9rem; line-height: 1.6;">
              <p>Una red es un grupo de computadoras conectadas para compartir información. Si "no tienes sistema", generalmente significa que tu computadora perdió la conexión con el servidor principal.</p>
              <p class="mb-0"><strong>Solución rápida:</strong> Revisa si puedes abrir páginas web normales. Si tampoco cargan, el problema es tu internet.</p>
            </div>
          </div>

          <div class="accordion-item-inner mb-3 p-3" @click="toggle_acc('ans2')">
            <div class="accordion-header fw-bold">
              &#9656; ¿Qué cable necesito para conectarme a internet?
            </div>
            <div class="mt-3 opacity-55 accordion-content" v-show="openTab === 'ans2'" style="font-size: 0.9rem; line-height: 1.6;">
              <p class="mb-0">El cable estándar se llama <strong>Ethernet (RJ-45)</strong>. Es parecido al de teléfono fijo pero con la punta más gruesa. Generalmente es azul, gris o amarillo.</p>
            </div>
          </div>

          <div class="accordion-item-inner p-3" @click="toggle_acc('ans3')">
            <div class="accordion-header fw-bold">
              &#9656; Diferencia entre WiFi y Cable
            </div>
            <div class="mt-3 opacity-55 accordion-content" v-show="openTab === 'ans3'" style="font-size: 0.9rem; line-height: 1.6;">
              <p class="mb-0">El <strong>Cable</strong> es más rápido y estable. El <strong>WiFi</strong> es inalámbrico pero puede fallar a cierta distancia del módem. Siempre que sea posible, usa cable para evitar desconexiones.</p>
            </div>
          </div>
        </div>

        <!-- Sección: Tutoriales Comunes -->
        <div class="panel-inner p-4 mb-4">
          <h5 class="fw-bold mb-4 text-accent" style="font-size: 0.78rem; letter-spacing: 0.08em; text-transform: uppercase; font-family: 'Inter', sans-serif;">
            Tutoriales Comunes
          </h5>

          <div class="accordion-item-inner mb-3 p-3" @click="toggle_acc('ans4')">
            <div class="accordion-header fw-bold">
              &#9656; ¿Cómo instalar Google Chrome en una PC nueva?
            </div>
            <div class="mt-3 opacity-55 accordion-content" v-show="openTab === 'ans4'" style="font-size: 0.9rem; line-height: 1.6;">
              <ol style="margin-left: 20px; margin-bottom: 0;">
                <li>Abre el navegador predeterminado (Edge o Safari).</li>
                <li>Escribe en la barra de direcciones: <code>google.com/chrome</code></li>
                <li>Haz clic en el botón <strong>"Descargar Chrome"</strong>.</li>
                <li>Espera a que termine la instalación automática.</li>
              </ol>
            </div>
          </div>

          <div class="accordion-item-inner p-3" @click="toggle_acc('ans5')">
            <div class="accordion-header fw-bold">
              &#9656; Olvidé mi contraseña, ¿cómo la restablezco?
            </div>
            <div class="mt-3 opacity-55 accordion-content" v-show="openTab === 'ans5'" style="font-size: 0.9rem; line-height: 1.6;">
              <p>No necesitas crear un ticket para esto:</p>
              <ol style="margin-left: 20px; margin-bottom: 0;">
                <li>En la pantalla de inicio busca <strong>"¿Olvidaste tu contraseña?"</strong>.</li>
                <li>Ingresa tu correo electrónico corporativo.</li>
                <li>Revisa tu bandeja de entrada y haz clic en el enlace seguro.</li>
              </ol>
            </div>
          </div>
        </div>

        <!-- Footer -->
        <div class="py-3 mt-auto text-center w-100 border-top opacity-55">
          <p class="m-0" style="font-size: 0.78rem; letter-spacing: 0.05em; font-family: 'Inter', sans-serif;">
            &copy; 2026 stack-underflow System.
          </p>
        </div>

      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { store } from '../store'
import { useTheme } from '../composables/useTheme'

const router  = useRouter()
const openTab = ref(null)
const { isDark, toggle, init } = useTheme()

onMounted(() => init())

const toggle_acc = (id) => { openTab.value = openTab.value === id ? null : id }
const logout     = () => { store.logout(); router.push('/') }
</script>

<style scoped>
.accordion-item-inner { cursor: pointer; }
</style>
