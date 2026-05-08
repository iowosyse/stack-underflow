<template>
  <div class="d-flex flex-column vh-100 user-bg">
    <div class="d-flex flex-grow-1 overflow-hidden p-3 gap-3">

      <!-- SIDEBAR -->
      <aside class="glass-user d-flex flex-column flex-shrink-0" style="width: 256px;">
        <div class="sidebar-header">
          <div class="logo sidebar-logo text-accent">stack-underflow</div>
          <div class="sidebar-sub">Portal de Incidencias</div>
        </div>

        <nav class="sidebar-nav">
          <div class="nav-group-label">Menú</div>
          <RouterLink to="/user" class="list-group-item"><span class="nav-icon">⊟</span> Mis Tickets</RouterLink>
          <RouterLink to="/knowledge-base" class="list-group-item"><span class="nav-icon">◎</span> Base de Conocimiento</RouterLink>
        </nav>

        <div class="sidebar-footer">
          <div class="user-profile-card mb-3">
            <div class="user-avatar" :style="{ background: avatarColor(nombreUsuario) }">{{ iniciales(nombreUsuario) }}</div>
            <div class="min-w-0 flex-grow-1">
              <div class="user-profile-name text-capitalize profile-text-fix">{{ nombreUsuario }}</div>
              <div class="user-profile-meta">{{ misTickets.length }} tickets</div>
            </div>
          </div>
          <button @click="toggle" class="theme-toggle mb-2">
            <span>{{ isDark ? '☀' : '☾' }}</span><span>{{ isDark ? 'Modo Claro' : 'Modo Oscuro' }}</span>
          </button>
          <button @click="logout" class="btn btn-outline-danger w-100">Cerrar Sesión</button>
        </div>
      </aside>

      <!-- CONTENIDO -->
      <main class="glass-user d-flex flex-column overflow-auto w-100 content-main">
        <h2 class="h4 fw-bold mb-4">Mis Solicitudes</h2>

        <div class="row g-3 mb-3">
          <div class="col-8">
            <div class="panel-inner p-4 h-100">
              <div class="panel-section-header-inline mb-3">Levantar Incidencia</div>
              <form @submit.prevent="submitTicket">
                <div class="mb-3">
                  <label class="form-label">Asunto</label>
                  <input type="text" class="form-control" v-model="nuevoTicket.asunto" placeholder="Ej. No enciende la computadora">
                </div>
                <div class="mb-3">
                  <label class="form-label">Categoría</label>
                  <select class="form-select" v-model="nuevoTicket.categoria">
                    <option value="hardware">Soporte Técnico (Hardware)</option>
                    <option value="software">Soporte Técnico (Software)</option>
                    <option value="redes">Problemas de Red</option>
                    <option value="cuenta">Problemas con mi Cuenta</option>
                  </select>
                </div>
                <div class="mb-3">
                  <label class="form-label">Descripción detallada</label>
                  <textarea class="form-control" rows="3" v-model="nuevoTicket.descripcion"></textarea>
                </div>
                <div v-if="ticketSuccess" class="success-toast mb-3"><span class="text-accent fw-bold">✔</span> Ticket enviado.</div>
                <button type="submit" class="btn btn-primary w-100 py-2" :disabled="!isFormValid || enviando">
                  {{ enviando ? 'Enviando...' : 'Enviar Ticket' }}
                </button>
              </form>
            </div>
          </div>

          <div class="col-4 d-flex flex-column gap-3">
            <div class="stat-panel flex-grow-1 d-flex flex-column align-items-center justify-content-center text-center p-4">
              <div class="stat-label mb-2">En Proceso</div>
              <div class="stat-number">{{ ticketsActivos.length }}</div>
            </div>
            <div class="stat-panel-secondary d-flex align-items-center gap-3 p-3">
              <div class="stat-secondary-val">{{ ticketsCerrados.length }}</div>
              <div><div class="stat-secondary-label">Resueltos</div><div class="stat-secondary-sub">en tu historial</div></div>
            </div>
          </div>
        </div>

        <!-- Historial -->
        <div class="panel-inner">
          <div class="panel-section-header"><span>Historial de Reportes</span></div>
          <table class="table table-borderless align-middle mb-0">
            <thead><tr><th class="ps-4">ID</th><th>Asunto</th><th>Categoría</th><th>Estado</th></tr></thead>
            <tbody>
              <template v-for="t in misTickets" :key="t.id">
                <tr class="ticket-row">
                  <td class="ps-4 col-id">{{ t.id }}</td>
                  <td class="td-sm text-truncate" style="max-width: 220px;">{{ t.subject }}</td>
                  <td class="td-sm text-capitalize">{{ t.category }}</td>
                  <td>
                    <span :class="['badge', `badge-${t.status}`]">{{ t.status }}</span>
                  </td>
                </tr>
              </template>
              <tr v-if="misTickets.length === 0">
                <td colspan="4" class="text-center py-5 opacity-55 td-sm">No has levantado tickets aún.</td>
              </tr>
            </tbody>
          </table>
        </div>
      </main>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useTheme } from '../composables/useTheme'

const router = useRouter()
const { isDark, toggle, init } = useTheme()

// ── Estado Local ─────────────────────────────────────────────────────────────
const ticketsActivos = ref([])
const ticketsCerrados = ref([])

const cargarTickets = async () => {
  const token = localStorage.getItem('token');
  const headers = {
    'Authorization': `Bearer ${token}`,
    'Content-Type': 'application/json'
  };

  try {
    const [resActivos, resCerrados] = await Promise.all([
      fetch('/api/tickets',          { headers }),
      fetch('/api/tickets/cerrados', { headers }),
    ]);

    if (resActivos.status === 401 || resCerrados.status === 401) {
      logout();
      return;
    }

    if (resActivos.ok)   ticketsActivos.value  = await resActivos.json();
    if (resCerrados.ok)  ticketsCerrados.value = await resCerrados.json();
  } catch (error) {
    console.error('Error al cargar tickets:', error);
  }
};

onMounted(() => {
  init()
  cargarTickets()
})

// ── Estado Local ─────────────────────────────────────────────────────────────
const nuevoTicket = ref({
  asunto: '',
  categoria: 'hardware',
  descripcion: ''
});

const enviando = ref(false); // Nuestro interruptor de bloqueo
const ticketSuccess = ref(false);

const nombreUsuario = computed(() => localStorage.getItem('usuario_nombre') || 'Usuario');
const misTickets = computed(() => [...ticketsActivos.value, ...ticketsCerrados.value]);

// Arreglamos la validación para que apunte al objeto correcto
const isFormValid = computed(() => nuevoTicket.value.asunto.trim() !== '' && nuevoTicket.value.descripcion.trim() !== '');

// ── Acciones ─────────────────────────────────────────────────────────────────
const submitTicket = async () => {
  enviando.value = true; // Bloquea el botón
  ticketSuccess.value = false; // Oculta el mensaje de éxito por si había uno previo
  const token = localStorage.getItem('token');
  
  try {
    const respuesta = await fetch('/api/tickets', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${token}` 
      },
      body: JSON.stringify({
        asunto: nuevoTicket.value.asunto,
        categoria: nuevoTicket.value.categoria,
        descripcion: nuevoTicket.value.descripcion
      })
    });

    if (respuesta.ok) {
      ticketSuccess.value = true;
      nuevoTicket.value = { asunto: '', categoria: 'hardware', descripcion: '' };
      await cargarTickets(); // Refleja el ticket nuevo en la lista inmediatamente
      setTimeout(() => { ticketSuccess.value = false }, 3000);
    } else {
      const error = await respuesta.text();
      console.error("Error del servidor:", error);
      alert("Error al enviar el ticket. Revisa la consola.");
    }
  } catch (err) {
    console.error("Error de conexión:", err);
    alert("Error de conexión con el servidor.");
  } finally {
    enviando.value = false; // Desbloquea el botón siempre, falle o sea éxito
  }
};

const logout = () => { localStorage.clear(); router.push('/') }
const iniciales = (n) => n.trim().split(' ').slice(0, 2).map(w => w[0]?.toUpperCase() || '').join('')
const avatarColor = (n) => { 
  const pLight = ['#4db6ac','#7986cb','#9575cd','#e57373','#f06292','#64b5f6','#4dd0e1','#81c784','#dce775','#ffb74d']
  const pDark  = ['#00695c','#283593','#4527a0','#c62828','#ad1457','#1565c0','#00838f','#2e7d32','#9e9d24','#ef6c00']
  const palette = isDark.value ? pDark : pLight
  let h = 0; for (const c of n) h = (h * 31 + c.charCodeAt(0)) & 0xffff; return palette[h % palette.length] 
}
</script>


<style scoped>
.profile-text-fix { color: inherit; }
[data-theme="dark"] .profile-text-fix { color: #f8f9fa; }
</style>