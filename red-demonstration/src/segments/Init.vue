<template>
  <DividedSection class="bg-gray">
    <template v-slot:left>
      <h2>Simulate the motion of N colliding particles</h2>
      <p>
        Enough talking, let's check out the simulation itself. Use
        <font-awesome-icon icon="play" /> to play,
        <font-awesome-icon icon="pause" /> to pause, and
        <font-awesome-icon icon="sync" /> to reload the simulation.
      </p>
      <p>
        Here we have {{ particles.length }} colliding particles simulated
        according to the laws of elastic collision using event-driven
        simulation. The domain has size {{ simWidth }} x {{ simHeight }} px.
        Total number of collisions happend so far <b>{{ collisionCount }}</b
        >.
      </p>
      <p>
        Your frame rate is <b>{{ frameRate }}</b> fps.
      </p>
    </template>

    <template v-slot:right>
      <SimulationVue
        ref="smv"
        :particles="particles"
        @onPlay="onPlay"
        @onPause="onPause"
        @onStop="onStop"
      ></SimulationVue>
    </template>
  </DividedSection>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import SButton from "@/components/SButton.vue";
import DividedSection from "@/components/DividedSection.vue";
import SimulationVue from "@/components/SimulationVue.vue";

import { Particle } from "red-simulation";

import { generateRandomParticles } from "@/utils.ts";

@Options({
  components: {
    DividedSection,
    SimulationVue,
    SButton,
  },
})
export default class Init extends Vue {
  // Refs
  $refs!: {
    smv: SimulationVue;
  };

  simWidth = 0;
  simHeight = 0;
  collisionCount = 0;
  frameRate = 0;
  interval: number | null = null;
  playing = true;

  particles: Particle[] = [];

  onPlay(): void {
    this.interval = setInterval(this.updateCollisionsCount, 1000);
  }

  onPause(): void {
    if (this.interval) {
      clearInterval(this.interval);
    }
  }

  onStop(): void {
    this.collisionCount = 0;
    this.frameRate = 0;
  }

  updateCollisionsCount(): void {
    if (!this.$refs.smv) {
      return;
    }

    let cc = 0;

    this.$refs.smv
      .getSimulation()
      ?.get_particles()
      .forEach((p: Particle) => (cc += p.collisions_count as any));

    this.collisionCount = cc;
    this.frameRate = this.$refs.smv?.getFrameRate();
  }

  mounted(): void {
    this.simWidth = this.$refs.smv.canvasWidth;
    this.simHeight = this.$refs.smv.canvasHeight;

    this.particles = generateRandomParticles(this.simWidth, this.simHeight, {
      density: 0.87,
      speedLimit: 50,
    });
  }
}
</script>
<style lang="scss">
@media only screen and (max-width: 1024px) {
  .canvas-holder {
    width: 80%;
    margin: 0 auto;
  }
}
</style>
