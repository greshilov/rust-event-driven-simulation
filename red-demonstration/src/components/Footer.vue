<template>
  <footer>
    <div class="container">
      <div class="text-center">
        Footer is absolutely useless here. It just exists.
      </div>
    </div>
  </footer>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";

@Options({})
export default class Footer extends Vue {}
</script>

<style lang="scss" scoped>
footer {
  padding: 60px 0;
  background-color: $dark;
  color: $gray;
  font-size: 22px;
}
</style>
