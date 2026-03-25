import { reactive } from 'vue'

const normalize = (str) => str.toLowerCase().normalize("NFD").replace(/[\u0300-\u036f]/g, "").trim()

const parseUser = (fullName, role) => {
  const parts = fullName.trim().split(/\s+/)
  const username = normalize(parts[0]) 
  const password = normalize(parts.slice(1).join(' ')) 
  return { fullName, username, password, role }
}

const rawAdmins = ['Ruben Lara', 'Candido Ortega', 'Michael Villalon', 'Admin Admin']
const rawUsers = [
  'Ruben Barcenas', 'Dylan Raya', 'Diego Contreras', 'Alessandra Hernandez',
  'Jose Rodriguez', 'Jesus Barriga', 'Luis Lopez', 'Ghiulio Calderon',
  'Luis Roman', 'Alejandro Montejano', 'Jose Villegas', 'Luis Arguello',
  'Jose Razo', 'Oscar Arias', 'Carlos Reyna', 'Julio Araujo',
  'Braulio Paniagua', 'Abril Chavez', 'Julio Gutierrez', 'Jafet Santoyo',
  'Sebastian Valencia', 'Yazmin Garcia', 'Eduardo Lopez', 'Alfonso Fernandez',
  'Angel de jesus Lemus', 'iker solis', 'Root Root', 'Alberto Montoya'
]

export const store = reactive({
  currentUser: null,
  admins: rawAdmins.map(name => parseUser(name, 'admin')),
  users: rawUsers.map(name => parseUser(name, 'user')),
  tickets: [],

  login(user, pass) {
    const u = normalize(user)
    const p = normalize(pass)
    const admin = this.admins.find(x => x.username === u && x.password === p)
    if (admin) { this.currentUser = admin; return 'admin' }
    const regularUser = this.users.find(x => x.username === u && x.password === p)
    if (regularUser) { this.currentUser = regularUser; return 'user' }
    return null
  },

  logout() { this.currentUser = null },

  // La prioridad ahora se asigna internamente, no la pide como parámetro
  addTicket(subject, category, description) {
    const newTicket = {
      id: '#TK-' + (1000 + this.tickets.length + 1),
      subject, 
      category, 
      priority: 'baja', // <-- Prioridad automática por defecto
      description,
      status: 'disponible',
      author: this.currentUser.fullName,
      assignedTo: null,
      createdAt: new Date()
    }
    this.tickets.push(newTicket)
    this.checkAssignments()
  },

  addUser(fullName, role) {
    if (role === 'admin') this.admins.push(parseUser(fullName, 'admin'))
    else this.users.push(parseUser(fullName, 'user'))
  },

  checkAssignments() {
    const disponibles = this.tickets.filter(t => t.status === 'disponible')
    if (disponibles.length >= 10) {
      disponibles.sort((a, b) => a.createdAt - b.createdAt)
      const priorityMap = { 'baja': 1, 'moderada': 2, 'alta': 3, 'urgente': 4 }
      let bestAdmin = null;
      let maxDisponibilidad = -1; 

      this.admins.forEach(admin => {
        const attending = this.tickets.filter(t => t.assignedTo === admin.fullName && t.status !== 'cerrado')
        const cantidadTickets = attending.length;
        let disponibilidad = 0;
        
        if (cantidadTickets === 0) {
          disponibilidad = Infinity; 
        } else {
          const sumPriority = attending.reduce((acc, t) => acc + priorityMap[t.priority], 0)
          const avgPriority = sumPriority / cantidadTickets;
          disponibilidad = (cantidadTickets + avgPriority) / cantidadTickets;
        }

        if (disponibilidad > maxDisponibilidad) {
          maxDisponibilidad = disponibilidad;
          bestAdmin = admin;
        }
      })

      if (bestAdmin) {
        for(let i=0; i < 10; i++) {
            disponibles[i].assignedTo = bestAdmin.fullName;
            disponibles[i].status = 'asignado';
        }
      }
    }
  }
})