<template>
  <div ref="canvasHolder" class="canvas-holder">
    <canvas ref="canvas"></canvas>
  </div>
  <div class="canvas-buttons">
    <p>
      <template v-if="!running">
        <SButton class="bg-green" @click="play"
          ><font-awesome-icon icon="play"
        /></SButton>
      </template>
      <template v-else>
        <SButton class="bg-red" @click="pause"
          ><font-awesome-icon icon="pause"
        /></SButton>
      </template>
      <SButton class="bg-blue" @click="stop"
        ><font-awesome-icon icon="sync"
      /></SButton>
    </p>
  </div>
</template>

<script lang="ts">
import { PropType } from "vue";
import { Options, Vue } from "vue-class-component";
import { Simulation, Particle, DrawParams, Segment } from "red-simulation";
import SButton from "@/components/SButton.vue";
import { FrameRater } from "@/utils";

@Options({
  components: { SButton },
  props: {
    width: Number,
    height: Number,
    autoplay: Boolean,
    particles: Object as PropType<Particle[]>,
    segments: Object as PropType<Segment[]>,
    drawParams: Object as PropType<DrawParams>,
  },
  watch: {
    width: "init",
    height: "init",
    particles: "init",
    segments: "init",
  },
  emits: ["onPlay", "onPause", "onStop"],
})
export default class SimulationVue extends Vue {
  // Refs
  $refs!: {
    canvasHolder: HTMLElement;
    canvas: HTMLCanvasElement;
  };
  // Props
  width?: number;
  height?: number;
  autoplay?: boolean;
  particles!: Particle[];
  segments!: Segment[];
  drawParams?: DrawParams;

  // Attrs
  ctx!: CanvasRenderingContext2D;
  sim!: Simulation;
  frameRater: FrameRater = new FrameRater();
  running = false;

  get canvasWidth(): number {
    return this.width ? this.width : this.$refs.canvasHolder.clientWidth;
  }

  get canvasHeight(): number {
    return this.height ? this.height : this.$refs.canvasHolder.clientHeight;
  }

  getSimulation(): Simulation {
    return this.sim;
  }

  play() {
    if (!this.running) {
      this.running = true;
      this.$emit("onPlay");
      this.draw();
    }
  }

  pause() {
    this.$emit("onPause");
    this.running = false;
  }

  stop() {
    this.$emit("onStop");
    this.init();
  }

  getFrameRate(): number {
    return this.frameRater.calculateFrameRate() || 0;
  }

  draw() {
    this.sim.draw(this.ctx);

    this.frameRater.startFrame();
    this.sim.tick();
    this.frameRater.endFrame();

    if (this.running) {
      requestAnimationFrame(this.draw);
    }
  }

  init() {
    this.$refs.canvas.width = this.canvasWidth;
    this.$refs.canvas.height = this.canvasHeight;
    this.sim = Simulation.new(
      this.canvasWidth,
      this.canvasHeight,
      60,
      this.drawParams
    );
    for (const particle of this.particles) {
      this.sim.add_particle(particle);
    }
    if (this.segments) {
      for (const segment of this.segments) {
        this.sim.add_segment(segment);
      }
    }
    this.sim.draw(this.ctx);
  }

  mounted() {
    this.ctx = this.$refs.canvas.getContext("2d")!;
    this.init();
    if (this.autoplay) {
      this.play();
    }
  }
}
</script>

<style lang="scss" scoped>
canvas {
  background-color: white;
  box-shadow: inset 0 0 1em #d2d0d069;
}
.canvas-holder {
  min-height: 400px;
  width: 100%;
  height: 100%;
}

.canvas-buttons {
  display: flex;
  justify-content: center;
}
</style>
