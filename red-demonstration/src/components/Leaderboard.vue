<template>
  <table class="leaderboard-table">
    <thead>
      <tr>
        <th>#</th>
        <th>Name</th>
        <th>Score</th>
        <th>Played at</th>
      </tr>
    </thead>
    <tbody>
      <tr
        v-for="res in formattedResults"
        :key="res.id"
        :class="{ 'bg-green': res.is_ours }"
      >
        <td>{{ res.i }}</td>
        <td>{{ res.player_name }}</td>
        <td>{{ res.score }}</td>
        <td>{{ res.created_at }}</td>
      </tr>
    </tbody>
  </table>
</template>

<script lang="ts">
import { PropType } from "@vue/runtime-core";
import { format } from "timeago.js";

import { Options, Vue } from "vue-class-component";
import { IResult } from "@/interfaces.ts";
import { getPlayer } from "@/utils.ts";

export interface IFormattedResult extends IResult {
  i: number;
  is_ours: boolean;
}

@Options({
  props: {
    results: Object as PropType<IResult[]>,
  },
})
export default class Leaderboard extends Vue {
  results!: IResult[];

  get formattedResults(): IFormattedResult[] {
    let index = 0;
    const player = getPlayer();
    return this.results.map((r) => {
      index++;
      const copied = {
        ...r,
        i: index,
        is_ours: player?.uuid === r.player_uuid,
      };
      copied.created_at = format(r.created_at);
      return copied;
    });
  }
}
</script>

<style lang="scss" scoped>
table.leaderboard-table {
  table-layout: fixed;
  width: 100%;
  border-collapse: collapse;

  td {
    padding: 10px;
  }

  tbody > tr:nth-of-type(2n + 1) {
    background: $gray;
  }
}
</style>
