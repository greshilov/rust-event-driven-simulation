<template>
  <Section class="main text-center">
    <h2>Rust Event Driven Simulation*</h2>
    <p>
      Event-driven programming is a widely spread technique used in many areas,
      from web servers to physical simulations. This project aims to implement a
      relatively <b>simple 2D particle collision system</b> using it. That's it,
      we have several obstacles, and a bunch of moving particles confined in
      some closed domain. Particles can have different radiuses and masses.
      Obstacles and domain consist of straight-line segments. To calculate
      physics I used an idealized
      <a
        href="https://introcs.cs.princeton.edu/java/assignments/collisions.html"
        >hard disc model</a
      >.
    </p>
    <p>
      As you may have guessed the primary language for this project is rust. But
      wait, it's a web page, isn't it? Yep, the code was compiled to a
      <a href="https://rustwasm.github.io/">WebAssembly</a>!
    </p>
    <div>
      The project is mainly based on the following sources:
      <ul>
        <li>
          <a href="https://algs4.cs.princeton.edu/61event/">The chapter</a>
          from Princeton algorithm course. Title is originated from it!
        </li>
        <li>
          <a
            href="https://rustwasm.github.io/docs/book/game-of-life/implementing.html"
            target="_blank"
            >Convey's game of life</a
          >
          from the rust-wasm book. Beautiful and detailed example.
        </li>
      </ul>
    </div>
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
.main div {
  text-align: justify;
}

p.small {
  text-align: right !important;
}
</style>
