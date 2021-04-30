import { createRouter, createWebHashHistory, RouteRecordRaw } from "vue-router";
import Main from "@/views/Main.vue";

function loadView(view: any) {
  return () =>
    import(/* webpackChunkName: "view-[request]" */ `@/views/${view}.vue`);
}

const routes: Array<RouteRecordRaw> = [
  {
    path: "/",
    name: "main",
    component: Main,
  },
  {
    path: "/editor",
    name: "editor",
    component: loadView("Editor"),
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;
