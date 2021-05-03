<template>
  <DividedSection>
    <template v-slot:left>
      <SimulationVue ref="sim" :particles="particles"></SimulationVue>
    </template>

    <template v-slot:right>
      <h2>Billiard ball</h2>
      <p>
        One of the obvious applications for this model is a pool game
        simulation. Interestingly enough <b>billiard ball model</b> is the legit
        name!
      </p>
      <p>
        It brings us back to the 19th century when
        <a href="https://en.wikipedia.org/wiki/John_Dalton" target="_blank"
          >John Dalton</a
        >
        introduced his atomic theory of matter. He suggested that matter
        consists of small indestructible elements - atoms. And yeah, he actually
        thought that atoms look like billiard balls :)
      </p>
      <p>
        I personally love this name, because it figuratively represents the
        assumptions lying in the foundation of such a physical system.
      </p>
    </template>
  </DividedSection>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import SimulationVue from "@/components/SimulationVue.vue";
import DividedSection from "@/components/DividedSection.vue";
import { Particle, RGBA } from "red-simulation";

@Options({
  components: {
    SimulationVue,
    DividedSection,
  },
})
export default class Pool extends Vue {
  // Refs
  $refs!: {
    sim: SimulationVue;
  };

  simWidth = 0;
  simHeight = 0;
  playing = false;

  particles: Particle[] = [];

  init(): void {
    const width = this.$refs.sim.canvasWidth;
    const height = this.$refs.sim.canvasHeight;
    const m = 1;
    const r = 10;
    const vertGap = 1.2 * r;
    const horGap = Math.sqrt(3) * (1.2 * r);

    // Cue ball
    this.particles = [
      Particle.new(
        width / 4,
        height / 2,
        width,
        0,
        m,
        r,
        RGBA.new(255, 255, 255)
      ),
    ];

    const poolTriangle: [number, number, string][] = [
      [width / 2, height / 2, "F0FA00"],
      [width / 2 + horGap, height / 2 + vertGap, "F9690E"],
      [width / 2 + horGap, height / 2 - vertGap, "0000FF"],
      [width / 2 + 2 * horGap, height / 2, "FF0000"],
      [width / 2 + 2 * horGap, height / 2 + 2 * vertGap, "1E824C"],
      [width / 2 + 2 * horGap, height / 2 - 2 * vertGap, "000000"],
      [width / 2 + 3 * horGap, height / 2 + vertGap, "9A12B3"],
      [width / 2 + 3 * horGap, height / 2 + 3 * vertGap, "89C4F4"],
      [width / 2 + 3 * horGap, height / 2 - vertGap, "FAD859"],
      [width / 2 + 3 * horGap, height / 2 - 3 * vertGap, "00FF00"],
    ];

    for (const [x, y, color] of poolTriangle) {
      this.particles.push(
        Particle.new(x, y, 0, 0, m, r, RGBA.from_css_hex(color))
      );
    }
  }

  mounted(): void {
    this.init();
  }
}
</script>

<style lang="scss">
.flex-container {
  display: flex;
  align-content: space-between;
  flex-wrap: wrap;
  > * {
    width: 100%;
  }
}
</style>
