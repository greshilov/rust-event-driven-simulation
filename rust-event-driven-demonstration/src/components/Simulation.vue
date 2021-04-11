<template>
  <div class="container m-3">
    <div class="row">
      <div class="canvas-container">
        <canvas
          ref="my-canvas"
          class="mb-1"
          :width="width"
          :height="height"
          @click="click"
          @mousemove="mouseMove"
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
    <div class="row text-center">
      <div>Frame rate is: {{ frame.rate }}</div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, ref, PropType, Ref } from "vue";
import { Simulation, RGBA } from "rust-event-driven-simulation";
import axios from 'axios';

export interface Particle {
  x: number;
  y: number;
  vx: number;
  vy: number;
  m: number;
  r: number;
}

export interface GameResult {
  player_name: string;
  score: number;
  ticks_per_sec: number;
}

export interface SignedGameResult {
  game_result: GameResult;
  // Int8Array
  hex_digest: Array<number>;
}

function getRandomArbitrary(min: number, max: number): number {
  return Math.random() * (max - min) + min;
}

function fillWithRandomParticles(simulation: Simulation, width: number, height: number, r: number) {
  for (let x = 3 * r; x < width - 3 * r; x += 6 * r) {
    for (
      let y = 3 * r;
      y < height - 3 * r;
      y += 6 * r
    ) {
      let Ux = getRandomArbitrary(-50, 50);
      let Uy = getRandomArbitrary(-50, 50);
      simulation.add_particle(x, y, Ux, Uy, 1, r);
    }
  }
}

export default defineComponent({
  name: "Simulation",
  data() {
    return {
      // Dirty hack
      canvas: (null as any) as HTMLCanvasElement,
      ctx: (null as any) as CanvasRenderingContext2D,
      simulation: (null as any) as Simulation,
      play: false,
      playerPoint: {
        x: 150,
        y: 150,
        r: 10,
      },
      frame: {
        counter: 0,
        rate: 0,
        lastCall: 0,
      },
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

  },

  async mounted() {
    this.canvas = this.$refs["my-canvas"] as HTMLCanvasElement;
    this.ctx = this.canvas.getContext("2d")!;
    this.simulation = Simulation.new(
      this.width,
      this.height,
      Number(this.ticksPerSec),
    );
    
    this.simulation.add_segment(300, 0, 300, 300);
    const red = RGBA.new(255, 0, 0, 255);
    const userParticle = this.simulation.add_particle(this.playerPoint.x, this.playerPoint.y, 0, 0, 1, this.playerPoint.r, red) as number;

    fillWithRandomParticles(this.simulation, this.ctx.canvas.clientWidth, this.ctx.canvas.clientHeight, 13);
    this.simulation.set_player_particle(userParticle, 'Slavik', this.gameOver);
    this.draw();
  },
  methods: {
    click: function(event: any) {
      if (!this.play) {
        const x = event.pageX - this.canvas.offsetLeft;
        const y = event.pageY - this.canvas.offsetTop;
        const px = this.playerPoint.x;
        const py = this.playerPoint.y;
        const r = this.playerPoint.r;
        if (
          (x - px)*(x - px) + (y - py) * (y - py) < r*r
        ) {
          this.start();
        }
      }
    },

    mouseMove: function(event: any) {
      const x = event.pageX - this.canvas.offsetLeft;
      const y = event.pageY - this.canvas.offsetTop;
      
      this.simulation.mv_player_particle(x, y);
    },

    updateTicksPerSec: function () {
      this.simulation.set_ticks_per_sec(Number(this.ticksPerSec));
    },

    calculateFrameRate: function() {
      if (this.play) {
        let now = performance.now();
        if (this.frame.lastCall > 0) {
          this.frame.rate = Math.round(this.frame.counter / (now - this.frame.lastCall) * 1000);
          this.frame.counter = 0;
        }
        this.frame.lastCall = now;
        setTimeout(this.calculateFrameRate, 1000);
      }
    },

    start: function() {
      this.play = true;
      this.calculateFrameRate();
      this.draw();
    },

    stop: function() {
      this.play = false;
    },

    gameOver: async function(result: SignedGameResult) {
      console.log(result);
      this.stop();
      const resp = await axios.post('/api/submit', result);
      console.log(resp);
    },

    draw: function () {
      this.simulation.draw(this.ctx);
      this.simulation.tick_for_fps(60);
      this.frame.counter++;
      //this.simulation.tick();
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
