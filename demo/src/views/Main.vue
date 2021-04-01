<template>
  <div class="home">
    <Simulation :width="400" :height="400" />
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

import Simulation from "@/components/Simulation.vue";

export default defineComponent({
  name: "Main",
  components: {
    Simulation,
  },
});
</script>
