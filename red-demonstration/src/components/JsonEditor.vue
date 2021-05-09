<template>
  <div :class="`editor ${divClass}`" ref="editorEl"></div>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";

import { EditorState, EditorView, basicSetup } from "@codemirror/basic-setup";
import { ViewUpdate } from "@codemirror/view";
import { json } from "@codemirror/lang-json";

const debounce = (fn: (...args: any[]) => any, ms = 300) => {
  let timeoutId: ReturnType<typeof setTimeout>;
  return function (this: any, ...args: any[]) {
    clearTimeout(timeoutId);
    timeoutId = setTimeout(() => fn.apply(this, args), ms);
  };
};

@Options({
  props: {
    debounceTime: {
      type: Number,
      default: 1000,
    },
    initialValue: String,
    divClass: {
      type: String,
      default: "",
    },
  },
  emits: ["onChange"],
})
export default class JsonTextarea extends Vue {
  // Refs
  $refs!: {
    editorEl: HTMLTextAreaElement;
  };

  debounceTime!: number;
  initialValue!: string;
  editor!: EditorView;

  mounted(): void {
    const debouncedEmit = debounce((u: ViewUpdate) => {
      this.$emit("onChange", u.state.doc.toString());
    }, this.debounceTime);

    this.editor = new EditorView({
      state: EditorState.create({
        doc: this.initialValue,
        extensions: [
          basicSetup,
          json(),
          EditorView.updateListener.of((v: ViewUpdate) => {
            if (v.docChanged) {
              debouncedEmit(v);
            }
          }),
        ],
      }),
      parent: this.$refs.editorEl,
    });
  }

  beforeUnmount(): void {
    this.editor.destroy();
  }
}
</script>
<style lang="scss" scoped>
.editor {
  text-align: auto;
  font-size: 15px;
}
</style>
