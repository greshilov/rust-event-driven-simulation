<template>
  <Section class="text-center">
    <h2>Rust Event Driven Simulation*</h2>
    <p>
      Event-driven programming is a widely spread technique used in many areas,
      from web servers to physical simulations. This project aims to implement a
      relatively <b>simple 2D particle collision system</b> using it. That's it,
      we have several obstacles, and a bunch of moving particles confined in
      some closed domain. Particles can have different radiuses and masses.
      Obstacles and domain consist of straight-line segments. All collisions
      considered being perfectly inelastic.
    </p>
    <p>
      As you may have guessed the primary language for this project is rust. But
      wait, it's a web page, isn't it? Yep, the code was compiled to a
      <a href="https://rustwasm.github.io/" target="_blank">WebAssembly</a>! To
      implement the simulation I used
      <a
        href="https://rustwasm.github.io/docs/book/game-of-life/implementing.html"
        target="_blank"
        >convey's game of life</a
      >
      from rust-wasm book as an example. Worth reading, I think.
    </p>
    <p class="small text-right">
      *Rust event driven simulation of a 2D particle system
    </p>
  </Section>
  <Init />
  <Pool />
  <Gas />
  <Game />
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import Section from "@/components/Section.vue";
import Init from "@/segments/Init.vue";
import Pool from "@/segments/Pool.vue";
import Gas from "@/segments/Gas.vue";
import DividedSection from "@/components/DividedSection.vue";
import Game from "@/segments/Game.vue";

@Options({
  components: {
    DividedSection,
    Game,
    Init,
    Pool,
    Gas,
    Section,
  },
})
export default class Main extends Vue {}
</script>
<style lang="scss" scoped>
p.small {
  text-align: right !important;
}
</style>
