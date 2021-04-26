<template>
  <div class="game-cell">
    <div class="overlay before-game" v-bind:class="{ enabled: playerIsNew }">
      <div class="game-nickname">
        <p>Enter your nickname*</p>
        <div class="inline">
          <input
            placeholder="Input your nickname"
            v-model="player.name"
            @keyup="submitName"
          />
          <SButton class="bg-blue" @click="submitName"
            ><font-awesome-icon icon="sign-in-alt"
          /></SButton>
        </div>
        <p class="small">*optional</p>
      </div>
    </div>

    <div
      class="overlay after-game"
      v-bind:class="{ enabled: gameScore !== null }"
    >
      <div class="game-result">
        <p>Your score is: {{ gameScore }}</p>
        <div class="text-center">
          <SButton class="bg-blue" @click="init"
            >Try again <font-awesome-icon icon="sync"
          /></SButton>
        </div>
      </div>
    </div>

    <canvas
      ref="canvas"
      :width="width"
      :height="height"
      @click="click"
      @mousemove="mouseMove"
    ></canvas>
    <p class="text-center">
      Score: <span class="current-score">{{ currentScore }}</span>
    </p>
  </div>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import {
  Simulation,
  Particle,
  DrawParams,
  RGBA,
  Segment,
} from "red-simulation";
import SButton from "@/components/SButton.vue";
import { IPlayer } from "@/interfaces.ts";
import {
  FrameRater,
  generateRandomParticles,
  getPlayer,
  roundTo,
} from "@/utils.ts";

import { v4 } from "uuid";

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

@Options({
  props: {
    status: Boolean,
  },
  components: { SButton },
  emits: ["gameOver"],
})
export default class SimulationGame extends Vue {
  // Refs
  $refs!: {
    canvas: HTMLCanvasElement;
  };
  // Props
  status!: boolean;

  // Attrs
  width = 350;
  height = 450;
  playerPoint = {
    x: 165,
    y: 165,
    r: 10,
  };

  playerIsNew = false;
  player: IPlayer = {
    uuid: "",
    name: "",
  };
  currentScore = 0;
  gameScore: number | null = null;

  particles!: Particle[];
  segments!: Segment[];
  drawParams?: DrawParams;

  // Attrs
  ctx!: CanvasRenderingContext2D;
  sim!: Simulation;
  frameRater: FrameRater = new FrameRater();
  running = false;

  start() {
    if (!this.running) {
      this.running = true;
      this.draw();
    }
  }

  stop() {
    this.running = false;
  }

  draw() {
    this.sim.draw(this.ctx);
    this.sim.tick();
    this.currentScore = this.sim.get_current_score() || 0;

    if (this.running) {
      requestAnimationFrame(this.draw);
    }
  }

  click(event: any) {
    if (!this.status || this.running) {
      return;
    }

    const x = event.layerX - this.$refs.canvas.offsetLeft;
    const y = event.layerY - this.$refs.canvas.offsetTop;
    const px = this.playerPoint.x;
    const py = this.playerPoint.y;
    const r = this.playerPoint.r;

    if ((x - px) * (x - px) + (y - py) * (y - py) < r * r) {
      this.start();
    }
  }

  mouseMove(event: any) {
    if (!this.status) {
      return;
    }

    const x = event.layerX - this.$refs.canvas.offsetLeft;
    const y = event.layerY - this.$refs.canvas.offsetTop;

    this.sim.mv_player_particle(x, y);
  }

  submitName(event: any) {
    if (
      (event.type == "keyup" && event.key === "Enter") ||
      event.type == "click"
    ) {
      localStorage.player = JSON.stringify(this.player);
      this.playerIsNew = false;
      this.init();
    }
  }

  async gameOver(result: SignedGameResult) {
    this.stop();
    this.gameScore = result.game_result.score;
    this.currentScore = result.game_result.score;
    this.$emit("gameOver", result);
  }

  init() {
    this.ctx = this.$refs.canvas.getContext("2d")!;
    this.gameScore = null;
    this.currentScore = 0;
    this.sim = Simulation.new(this.width, this.height, 60, this.drawParams);

    const pPoint = Particle.new(
      this.playerPoint.x,
      this.playerPoint.y,
      0,
      0,
      1,
      this.playerPoint.r,
      RGBA.new(255, 0, 0, 1)
    );

    this.sim.add_player_particle(
      pPoint,
      this.player.uuid,
      this.player.name,
      this.gameOver
    );

    const particles = generateRandomParticles(this.width, this.height, {
      density: 0.65,
      speedLimit: 100,
    });
    for (const particle of particles) {
      this.sim.add_particle(particle);
    }

    this.sim.draw(this.ctx);
  }

  getOrCreatePlayer() {
    const player = getPlayer();
    if (player !== undefined) {
      this.player = player;
    } else {
      this.playerIsNew = true;
      this.player = {
        uuid: v4(),
        name: "anonymous",
      } as IPlayer;
    }
  }

  mounted() {
    this.getOrCreatePlayer();
    this.init();
  }
}
</script>

<style lang="scss" scoped>
canvas {
  background-color: white;
  box-shadow: inset 0 0 1em #d2d0d069;
}

.canvas-buttons {
  display: flex;
  justify-content: center;
}

.game-cell {
  position: relative;

  .overlay {
    border-radius: 10px;
    position: absolute;
    width: 100%;
    height: 100%;
    opacity: 0;
    transition: 0.7s ease;
    background-color: $gray;
    z-index: -1;
  }

  .before-game {
    position: absolute;
    width: 100%;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);

    .game-nickname {
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);

      p {
        margin: 0 0 10px 0;
        color: black;
        text-align: center;
        font-weight: 600;
      }

      p.small {
        font-size: 12px;
        margin-top: 10px;
        color: black;
        text-align: right;
        font-weight: 300;
      }

      input {
        border-radius: 5px 5px 0px 0px;
        padding: 5px;
      }
    }

    &.enabled {
      z-index: 4;
      opacity: 0.9;
    }
  }

  .after-game {
    .game-result {
      z-index: 3;
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);

      p {
        margin: 0 0 10px 0;
        color: black;
        text-align: center;
        font-weight: 600;
      }

      input {
        border-radius: 5px 5px 0px 0px;
        padding: 5px;
      }
    }

    &.enabled {
      z-index: 2;
      opacity: 0.8;
    }
  }

  span.current-score {
    font-weight: bold;
    display: inline-block;
    min-width: 40px;
  }
}

.inline {
  display: flex;
}
</style>
