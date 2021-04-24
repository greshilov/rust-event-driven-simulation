import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";

import { library } from "@fortawesome/fontawesome-svg-core";
import {
  faPlay,
  faStop,
  faPause,
  faSync,
  faSignInAlt,
  faCircle,
} from "@fortawesome/free-solid-svg-icons";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";

import { set_panic_hook } from "red-simulation";

library.add(faCircle, faPlay, faStop, faPause, faSync, faSignInAlt);
set_panic_hook();

const app = createApp(App);

app.component("font-awesome-icon", FontAwesomeIcon);
app.use(router);

app.mount("#app");
