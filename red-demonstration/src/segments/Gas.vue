<template>
  <DividedSection class="bg-gray" align="center">
    <template v-slot:left>
      <h2>Two dimensional gas</h2>
      <p>
        This model also plays a central role in the kinetic theory of gases. A
        gas is represented as a large number of identical submicroscopic
        particles. All macroscopic properties such as pressure or temperature
        are defined by particle interactions. This particular example represents
        <b>diffusion</b> process. The domain is divided into two sides. The left
        side of the domain contains a lot of particles, and the right side has
        only a few of them. There is a small gate in the center between the
        sides.
      </p>
      <p>
        After one starts the simulation, the presence of the particles in the
        different sides must gradually become equal. Which will indicate that
        the <b>diffusion</b> process is over.
      </p>
      <p class="text-center">Pressure/presence rate:</p>
      <h1 class="text-center">{{ pressure.left }} / {{ pressure.right }}</h1>
    </template>

    <template v-slot:right>
      <SimulationVue
        ref="sim"
        :particles="particles"
        :segments="segments"
        @onPlay="onPlay"
        @onPause="onPause"
        @onStop="onStop"
      ></SimulationVue>
    </template>
  </DividedSection>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import SimulationVue from "@/components/SimulationVue.vue";
import DividedSection from "@/components/DividedSection.vue";
import { Segment, Particle, Simulation } from "red-simulation";
import { generateRandomParticles, len, roundTo } from "@/utils.ts";

@Options({
  components: {
    SimulationVue,
    DividedSection,
  },
})
export default class Gas extends Vue {
  // Refs
  $refs!: {
    sim: SimulationVue;
  };

  simWidth = 0;
  simHeight = 0;
  pressure = {
    left: 0,
    right: 0,
  };
  interval: number | null = null;

  particles: Particle[] = [];
  segments: Segment[] = [];

  onPlay(): void {
    this.calculatePseudoPressure();
    this.interval = setInterval(this.calculatePseudoPressure, 1000);
  }

  onPause(): void {
    if (this.interval) {
      clearInterval(this.interval);
    }
  }

  onStop(): void {
    this.pressure = {
      left: 0,
      right: 0,
    };
  }

  init(): void {
    if (!this.$refs.sim) {
      return;
    }
    const width = this.$refs.sim.canvasWidth;
    const height = this.$refs.sim.canvasHeight;

    const leftSide = generateRandomParticles(width / 2, height, {
      r: 7,
      density: 0.8,
      color: [255, 0, 0],
    });
    const rightSide = generateRandomParticles(width, height, {
      r: 7,
      density: 0.1,
      color: [0, 0, 255],
      startPoint: {
        x: width / 2,
        y: 0,
      },
    });

    this.particles = [...leftSide, ...rightSide];
    this.segments = [
      Segment.new(width / 2, 0, width / 2, height / 2 - 30),
      Segment.new(width / 2, height / 2 + 30, width / 2, height),
    ];
  }

  calculatePseudoPressure(): void {
    if (!this.$refs.sim && this.interval) {
      clearInterval(this.interval);
      return;
    }
    const width = this.$refs.sim.canvasWidth;
    const simulation: Simulation = this.$refs.sim.getSimulation();
    let leftSum = 0;
    let rightSum = 0;
    simulation.get_particles().forEach((p: Particle) => {
      if (p.pos.x < width / 2) {
        leftSum += len(p.v);
      } else {
        rightSum += len(p.v);
      }
    });
    const sum = leftSum + rightSum;
    this.pressure.left = roundTo(leftSum / sum, 2);
    this.pressure.right = roundTo(rightSum / sum, 2);
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
