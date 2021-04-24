<template>
  <a :class="buttonClass"><slot /></a>
</template>

<style lang="scss" scoped>
.button {
  display: inline-block;
  border: none;
  color: #fff;
  text-decoration: none;
  padding: 10px 25px;
  font-size: 14px;
  text-transform: uppercase;
  font-weight: 600;
  letter-spacing: 3px;
  border-radius: 2px;
  text-align: center;
  user-select: none;
  position: relative;
  outline: none;
  transition: background-color 0.1s ease;
  cursor: pointer;

  &::after {
    transition: background-color 0.2s ease;
    position: absolute;
    content: "";
    height: 4px;
    bottom: 0;
    width: 100%;
    background-color: darken($accent, 50%);
    opacity: 0.18;
    border-bottom-left-radius: 2px;
    border-bottom-right-radius: 2px;
    left: 0;
  }

  &:focus,
  &:hover {
    text-decoration: none;
    color: #fff;
  }

  &:hover {
    filter: brightness(85%);
    &::after {
      opacity: 0.22;
    }
  }

  &:active {
    filter: brightness(85%);
    &::after {
      opacity: 0.32;
    }
  }
}
</style>

<script lang="ts">
import { Options, Vue } from "vue-class-component";

@Options({
  props: {
    class: String,
  },
})
export default class SButton extends Vue {
  class!: string;

  get buttonClass(): string {
    return `button ${this.class}`;
  }
}
</script>
