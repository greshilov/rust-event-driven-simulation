import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";

import "bootstrap/dist/css/bootstrap.min.css";
import "bootstrap";

import { set_panic_hook } from "red-simulation";

set_panic_hook();
const app = createApp(App);
app.use(router);

app.mount("#app");
