<template>
  <div class="container">
    <Navbar :routes="routesList" />
    <router-view />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

import Navbar from "@/components/Navbar.vue";
import routes from "@/router";

export default defineComponent({
  name: "App",
  components: {
    Navbar,
  },
  computed: {
    routesList: () => {
      return routes.getRoutes();
    },
  },
});
</script>
<style>
body {
  font-size: 14px;
  font-weight: 400;
}
</style>
