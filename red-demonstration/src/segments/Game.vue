<template>
  <Section class="text-center">
    <h2>The Game</h2>
    <p>
      Even though game engines usually don't use this approach directly to
      simulate physics, event-driven is still widely adopted in game
      development. For example, it can be used to implement communication
      mechanisms between different parts of the game engine. Nevertheless, I've
      built my own game with <s>blackjack and hookers</s> particles and collisions.
    </p>
    <p>
      The rules of the game are simple. Use your mouse to control the red
      particle <font-awesome-icon icon="circle" style="color: red" />. Avoid
      collisions with other particles and walls of the domain as long as you
      can. The score of the game is just a number of deciseconds passed since
      the start.
      <b
        >Click on the red particle
        <font-awesome-icon icon="circle" style="color: red"
      /></b>
      to start the game.
    </p>
    <div class="grid game-grid">
      <SimulationGame @gameOver="gameOver" :status="true" />
      <div class="game-leaderboard">
        <Leaderboard :results="results" />
      </div>
    </div>
  </Section>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import Leaderboard from "@/components/Leaderboard.vue";
import Section from "@/components/Section.vue";
import SButton from "@/components/SButton.vue";
import SimulationGame, {
  SignedGameResult,
} from "@/components/SimulationGame.vue";

import { IResult } from "@/interfaces.ts";

import axios from "axios";

const HOST =
  process.env.NODE_ENV === "development"
    ? "http://localhost:8080"
    : "https://b.greshilov.me";

@Options({
  components: {
    Leaderboard,
    SButton,
    Section,
    SimulationGame,
  },
})
export default class Game extends Vue {
  results: IResult[] = [];

  async gameOver(result: SignedGameResult) {
    await axios.post(`${HOST}/reds/api/submit`, result);
    await this.refresh();
  }

  async refresh(): Promise<void> {
    const resp = await axios.get(`${HOST}/reds/api/top`);
    this.results = resp.data;
  }

  async mounted(): Promise<void> {
    await this.refresh();
  }
}
</script>

<style lang="scss" scoped>
.game-grid {
  grid-template-columns: 1fr 2fr;
  grid-gap: 1em;
  align-items: start;
}

@media only screen and (max-width: 1024px) {
  .game-grid {
    grid-template-columns: 1fr;
  }

  .game-leaderboard {
    margin-top: 10px;
  }
}

.game-cell {
  position: relative;
}

.inline {
  display: flex;
}

.game-overlay {
  z-index: -1;
  border-radius: 10px;
  position: absolute;
  width: 100%;
  height: 100%;
  opacity: 0;
  transition: 0.5s ease;
  background-color: $gray;

  div.nickname {
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

    &:hover {
      opacity: 1;
    }
  }
}
</style>
