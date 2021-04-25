<template>
  <section :class="sectionClass">
    <div class="container">
      <slot></slot>
    </div>
  </section>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";

@Options({
  props: {
    class: String,
  },
})
export default class Section extends Vue {
  class!: string;

  get sectionClass(): string {
    return `simple-section ${this.class}`;
  }
}
</script>

<style lang="scss">
.simple-section {
  padding: 40px 0;

  h2 {
    font-size: 34px;
    font-weight: 800;
    line-height: 1.2;
  }

  p {
    text-align: justify;

    b {
      font-weight: bold;
    }
  }

  .container {
    align-items: center;
  }
}

@media only screen and (max-width: 1024px) {
  .simple-section {
    padding: 40px 0;
  }
}
</style>
