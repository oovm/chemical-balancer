import { createApp } from 'vue'
import App from './App.vue'
import 'uno.css'
import 'katex/dist/katex.min.css'
import locates from './locates'

const app = createApp(App)
app.use(locates)
app.mount('#app')
