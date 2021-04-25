<template>
  <section :class="sectionClass">
    <div :class="'container grid ' + gridClass">
      <div class="left">
        <slot name="left"> </slot>
      </div>
      <div class="right">
        <slot name="right"> </slot>
      </div>
    </div>
  </section>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";

@Options({
  props: {
    class: String,
    align: {
      type: String,
      default: "start",
    },
  },
})
export default class DividedSection extends Vue {
  class!: string;
  align!: string;

  get sectionClass(): string {
    return `landing-section ${this.class}`;
  }

  get gridClass(): string {
    return `grid-${this.align}`;
  }
}
</script>

<style lang="scss">
.landing-section {
  padding: 50px 0;

  .container {
    grid-template-columns: 1fr 1fr;
    grid-gap: 40px;

    .left,
    .right {
      p {
        text-align: justify;
      }
    }
  }
}

@media only screen and (max-width: 1024px) {
  .landing-section {
    padding: 40px 0;

    .container {
      grid-template-columns: 1fr;

      .left {
        border-right: 0;
      }
      h2 {
        text-align: center;
      }
    }
  }
}
</style>
