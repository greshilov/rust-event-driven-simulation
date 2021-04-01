<template>
  <div class="row">
    <div class="nav-col col-md-8">
      <nav class="navbar navbar-expand-lg navbar-light bg-light rounded">
        <div class="container-fluid">
          <ul class="navbar-nav me-auto mb-2 mb-lg-0">
            <li v-for="route in routes" :key="route.name" class="nav-item">
              <router-link class="nav-link" :to="route.path">{{
                route.name
              }}</router-link>
            </li>
          </ul>
        </div>
      </nav>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import { RouteRecordRaw } from "vue-router";

export default defineComponent({
  name: "Navbar",
  props: {
    routes: Object as PropType<Array<RouteRecordRaw>>,
  },
});
</script>

<style scoped>
.nav-col,
.navbar-nav {
  margin: 0 auto;
}

.rounded {
  border-radius: 0.25rem !important;
}
</style>
