import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import hljs from 'highlight.js'
import 'highlight.js/styles/github.css'

const app = createApp(App)
app.use(router)

app.directive('highlight', function (el) {
    let blocks = el.querySelectorAll('pre code')
    blocks.forEach((block) => {
      hljs.highlightBlock(block)
    })
  })

app.mount('#app')