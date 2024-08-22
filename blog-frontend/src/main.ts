import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import About from "./pages/About.vue";
import Home from "./pages/Home.vue";
import { createMemoryHistory, createRouter } from "vue-router";

const routes = [
  { path: "/", component: Home },
  { path: "/about", component: About },
];

const router = createRouter({
  history: createMemoryHistory(),
  routes,
});

createApp(App).use(router).mount("#app");
