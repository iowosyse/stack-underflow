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
              <div class="user-profile-name text-capitalize">
                {{ store.currentUser?.fullName || 'Usuario' }}
              </div>
              <div class="user-profile-meta">Base de Conocimiento</div>
            </div>
          </div>
          <button @click="toggle" class="theme-toggle mb-2">
            <span>{{ isDark ? '☀' : '☾' }}</span>
            <span>{{ isDark ? 'Modo Claro' : 'Modo Oscuro' }}</span>
          </button>
          <button @click="logout" class="btn btn-outline-danger w-100">
            Cerrar Sesión
          </button>
        </div>

      </aside>

      <!-- ════════════════════════════════════
           CONTENIDO
           ════════════════════════════════════ -->
      <main class="glass-user d-flex flex-column overflow-auto w-100 content-main">

        <div class="mb-4 pb-3 border-bottom">
          <h1 class="h3 m-0 fw-bold">Ayuda y Tutoriales</h1>
          <p class="opacity-55 mt-2 mb-0 td-sm">
            Resuelve dudas frecuentes sin esperar a un técnico.
          </p>
        </div>

        <!-- Redes e Internet -->
        <div class="panel-inner p-4 mb-3">
          <p class="kb-section-label mb-4">Redes e Internet</p>

          <div class="accordion-item-inner mb-3 p-3" @click="toggle_acc('ans1')">
            <div class="accordion-header fw-bold">
              &#9656; ¿Qué es una red y por qué no tengo sistema?
            </div>
            <div class="mt-3 opacity-55 accordion-content" v-show="openTab === 'ans1'">
              <p>Una red es un grupo de computadoras conectadas para compartir información.
                 Si "no tienes sistema", generalmente tu computadora perdió la conexión con
                 el servidor principal.</p>
              <p class="mb-0">
                <strong>Solución rápida:</strong> Revisa si puedes abrir páginas web normales.
                Si tampoco cargan, el problema es tu internet.
              </p>
            </div>
          </div>

          <div class="accordion-item-inner mb-3 p-3" @click="toggle_acc('ans2')">
            <div class="accordion-header fw-bold">
              &#9656; ¿Qué cable necesito para conectarme a internet?
            </div>
            <div class="mt-3 opacity-55 accordion-content" v-show="openTab === 'ans2'">
              <p class="mb-0">
                El cable estándar se llama <strong>Ethernet (RJ-45)</strong>. Es parecido
                al de teléfono fijo pero con la punta más gruesa. Generalmente es azul,
                gris o amarillo.
              </p>
            </div>
          </div>

          <div class="accordion-item-inner p-3" @click="toggle_acc('ans3')">
            <div class="accordion-header fw-bold">
              &#9656; Diferencia entre WiFi y Cable
            </div>
            <div class="mt-3 opacity-55 accordion-content" v-show="openTab === 'ans3'">
              <p class="mb-0">
                El <strong>Cable</strong> es más rápido y estable. El <strong>WiFi</strong>
                es inalámbrico pero puede fallar a cierta distancia del módem. Siempre que
                sea posible usa cable para evitar desconexiones.
              </p>
            </div>
          </div>
        </div>

        <!-- Tutoriales Comunes -->
        <div class="panel-inner p-4 mb-3">
          <p class="kb-section-label mb-4">Tutoriales Comunes</p>

          <div class="accordion-item-inner mb-3 p-3" @click="toggle_acc('ans4')">
            <div class="accordion-header fw-bold">
              &#9656; ¿Cómo instalar Google Chrome en una PC nueva?
            </div>
            <div class="mt-3 opacity-55 accordion-content" v-show="openTab === 'ans4'">
              <ol class="kb-list">
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
            <div class="mt-3 opacity-55 accordion-content" v-show="openTab === 'ans5'">
              <p>No necesitas crear un ticket para esto:</p>
              <ol class="kb-list">
                <li>En la pantalla de inicio busca <strong>"¿Olvidaste tu contraseña?"</strong>.</li>
                <li>Ingresa tu correo electrónico corporativo.</li>
                <li>Revisa tu bandeja de entrada y haz clic en el enlace seguro.</li>
              </ol>
            </div>
          </div>
        </div>

        <!-- Footer -->
        <div class="py-3 mt-auto text-center w-100 border-top opacity-55">
          <p class="m-0 kb-footer-text">&copy; 2026 stack-underflow System.</p>
        </div>

      </main>
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
const logout     = ()   => { store.logout(); router.push('/') }

const PALETTE = ['#2d6a4f','#1b4332','#40916c','#184e77','#1e6091','#6b3fa0','#553285','#8b2fc9','#7b2d8b','#9c4221']
const initials    = (n) => n.trim().split(' ').slice(0, 2).map(w => w[0]?.toUpperCase() ?? '').join('')
const avatarColor = (n) => { let h = 0; for (const c of n) h = (h * 31 + c.charCodeAt(0)) & 0xffff; return PALETTE[h % PALETTE.length] }
</script>
