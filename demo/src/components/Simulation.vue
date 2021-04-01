<template>
  <div class="container m-3">
    <div class="row">
      <div class="canvas-container">
        <canvas
          ref="my-canvas"
          class="mb-1"
          :width="width"
          :height="height"
        ></canvas>
      </div>
    </div>
    <div class="row text-center">
      <div class="controls">
        <button
          type="button"
          class="btn btn-primary m-1"
          @click="
            play = true;
            draw();
          "
        >
          Play
        </button>
        <button type="button" class="btn btn-primary m-1" @click="play = false">
          Pause
        </button>
        <div>
          <label for="ticksPerSec" class="form-label">Ticks per second</label>
          <input
            id="ticksPerSec"
            v-model="ticksPerSec"
            type="range"
            class="form-range"
            min="500"
            max="20000"
            @change="updateTicksPerSec"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";
import { Simulation } from "rust-event-driven-simulation";

export interface Particle {
  x: number;
  y: number;
  vx: number;
  vy: number;
  m: number;
  r: number;
}

function getRandomArbitrary(min: number, max: number): number {
  return Math.random() * (max - min) + min;
}

export default defineComponent({
  name: "Simulation",
  data() {
    return {
      ctx: null as any,
      simulation: null as any,
      play: true,
      ticksPerSec: 7000,
    };
  },
  props: {
    width: {
      type: Number,
      default: 100,
    },
    height: {
      type: Number,
      default: 100,
    },
    particles: {
      type: Object as PropType<Array<Particle>>,
      default: [],
    },
  },
  computed: {
    canvasHeight(): number {
      return (this.$refs["canvas-column"] as HTMLElement).clientHeight;
    },
    canvasWidth(): number {
      return (this.$refs["canvas-column"] as HTMLElement).clientWidth;
    },
  },

  mounted() {
    this.ctx = (this.$refs["my-canvas"] as any).getContext("2d");
    this.simulation = Simulation.new(
      this.width,
      this.height,
      Number(this.ticksPerSec)
    );
    const r = 10;
    for (let x = 3 * r; x < this.ctx.canvas.clientWidth - 3 * r; x += 3 * r) {
      for (
        let y = 3 * r;
        y < this.ctx.canvas.clientHeight - 3 * r;
        y += 3 * r
      ) {
        let Ux = getRandomArbitrary(-50, 50);
        let Uy = getRandomArbitrary(-50, 50);
        this.simulation.add_particle(x, y, Ux, Uy, 1, r);
      }
    }
    this.draw();
  },
  methods: {
    updateTicksPerSec: function () {
      this.simulation.set_ticks_per_sec(Number(this.ticksPerSec));
    },

    draw: function () {
      this.simulation.draw(this.ctx);
      this.simulation.tick_for_fps(60);
      if (this.play) {
        requestAnimationFrame(this.draw);
      }
    },
  },
});
</script>

<style scoped>
.canvas-container {
  width: auto;
  margin: 0 auto;
}

.controls {
  margin: 0 auto;
  max-width: 400px;
}
</style>
