<template>
  <main>
    <router-view />
  </main>
</template>

<script lang="ts">
import "@/scss/base.scss";
import { defineComponent } from "vue";

import routes from "@/router";

export default defineComponent({
  name: "App",
  components: {},
  computed: {
    routesList: () => {
      return routes.getRoutes();
    },
  },
});
</script>
