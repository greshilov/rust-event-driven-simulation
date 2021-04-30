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
      Particle.new(width / 2, height / 2, 0, 0, m, r, RGBA.new(240, 255, 0)),
      Particle.new(
        width / 2 + horGap,
        height / 2 + vertGap,
        0,
        0,
        m,
        r,
        RGBA.new(249, 105, 14)
      ),
      Particle.new(
        width / 2 + horGap,
        height / 2 - vertGap,
        0,
        0,
        m,
        r,
        RGBA.new(0, 0, 255)
      ),

      Particle.new(
        width / 2 + 2 * horGap,
        height / 2,
        0,
        0,
        m,
        r,
        RGBA.new(255, 0, 0)
      ),
      Particle.new(
        width / 2 + 2 * horGap,
        height / 2 + 2 * vertGap,
        0,
        0,
        m,
        r,
        RGBA.new(30, 130, 76)
      ),
      Particle.new(
        width / 2 + 2 * horGap,
        height / 2 - 2 * vertGap,
        0,
        0,
        m,
        r,
        RGBA.new(0, 0, 0)
      ),

      Particle.new(
        width / 2 + 3 * horGap,
        height / 2 + vertGap,
        0,
        0,
        m,
        r,
        RGBA.new(154, 18, 179)
      ),
      Particle.new(
        width / 2 + 3 * horGap,
        height / 2 + 3 * vertGap,
        0,
        0,
        m,
        r,
        RGBA.new(137, 196, 244)
      ),
      Particle.new(
        width / 2 + 3 * horGap,
        height / 2 - vertGap,
        0,
        0,
        m,
        r,
        RGBA.new(250, 216, 89)
      ),
      Particle.new(
        width / 2 + 3 * horGap,
        height / 2 - 3 * vertGap,
        0,
        0,
        m,
        r,
        RGBA.new(0, 255, 0)
      ),
    ];
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
