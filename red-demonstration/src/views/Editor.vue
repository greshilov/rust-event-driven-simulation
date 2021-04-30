<template>
  <GithubCorner />
  <Section>
    <div class="text-center">
      <h2>Editor</h2>
      <p>
        Try to create your own simulation! Edit the configuration below to add
        particles/segments or change width or height. The background color of
        the editor will signal the problems in the configuration. If the color
        is <span class="b-yellow">yellow</span>, then some particles are invalid
        and will be ignored. If the color is <span class="b-red">red</span> then
        your configuration is not a valid JSON and thus can not be loaded.
      </p>
    </div>
  </Section>
  <DividedSection class="section-editor" align="center">
    <template v-slot:left>
      <JsonEditor
        :value="initialValue"
        :divClass="getEditorClass()"
        @onChange="onChange"
      />
    </template>
    <template v-slot:right>
      <SimulationVue
        :width="width"
        :height="height"
        :particles="particles"
        :segments="segments"
      />
    </template>
  </DividedSection>
</template>

<script lang="ts">
import { Options, Vue } from "vue-class-component";
import GithubCorner from "@/components/GithubCorner.vue";
import Section from "@/components/Section.vue";
import DividedSection from "@/components/DividedSection.vue";

import SimulationVue from "@/components/SimulationVue.vue";
import JsonEditor from "@/components/JsonEditor.vue";

import { Particle, RGBA, Segment } from "red-simulation";

@Options({
  components: {
    GithubCorner,
    JsonEditor,
    Section,
    DividedSection,
    SimulationVue,
  },
})
export default class Editor extends Vue {
  width = 400;
  height = 400;
  particles: Particle[] = [];
  segments: Segment[] = [];
  state = State.Ok;

  initialValue = `{
  "width": 400,
  "height": 400,
  "particles": [
    {
      "pos": [300, 75],
      "v": [-100, 100],
      "m": 1,
      "r": 10,
      "color": "ff0000"
    },
    {
      "pos": [100, 335],
      "v": [100, -100],
      "m": 2,
      "r": 20
    }
  ],
  "segments": [
    {
      "p1": [200, 125],
      "p2": [400, 125]
    },
    {
      "p1": [0, 275],
      "p2": [100, 275]
    }
  ]
}`;

  getEditorClass(): string {
    switch (this.state) {
      case State.Ok:
        return "";
      case State.Warning:
        return "box-warning";
      case State.Error:
        return "box-error";
    }
  }

  load(config: string): void {
    const data = JSON.parse(config) as ISceneConfig;

    let problemParticles = [];
    let problemSegments = [];

    if (data.width <= 0 || data.height <= 0) {
      this.state = State.Error;
      return;
    }

    this.width = data.width;
    this.height = data.height;
    this.particles = data.particles
      .filter((p) => {
        if (validateParticle(p, this.width, this.height)) {
          return true;
        } else {
          problemParticles.push(p);
        }
      })
      .map((p) => {
        return Particle.new(
          p.pos[0],
          p.pos[1],
          p.v[0],
          p.v[1],
          p.m,
          p.r,
          RGBA.from_css_hex(p.color || "")
        );
      });

    this.segments = data.segments
      .filter((s) => {
        if (validateSegment(s)) {
          return true;
        } else {
          problemSegments.push(s);
        }
      })
      .map((s) => Segment.new(s.p1[0], s.p1[1], s.p2[0], s.p2[1]));

    if (problemParticles.length > 0 || problemSegments.length > 0) {
      this.state = State.Warning;
    } else {
      this.state = State.Ok;
    }
  }

  onChange(value: string): void {
    try {
      JSON.parse(value);
      this.load(value);
    } catch (e) {
      this.state = State.Error;
    }
  }

  mounted(): void {
    this.load(this.initialValue);
  }
}

function validateParticle(
  p: IParticle,
  width: number,
  height: number
): boolean {
  return (
    p.pos?.length == 2 &&
    p.v?.length == 2 &&
    p.pos[0] > 0 &&
    p.pos[0] < width &&
    p.pos[1] < height &&
    p.pos[1] > 0 &&
    p.m >= 0 &&
    p.r >= 0
  );
}

function validateSegment(s: ISegment): boolean {
  return s.p1?.length == 2 && s.p2?.length == 2;
}

interface IParticle {
  pos: number[];
  v: number[];
  m: number;
  r: number;
  color?: string;
}

interface ISegment {
  p1: number[];
  p2: number[];
}

interface ISceneConfig {
  width: number;
  height: number;
  particles: IParticle[];
  segments: ISegment[];
}

enum State {
  Ok,
  Warning,
  Error,
}
</script>
<style lang="scss">
.section-editor {
  padding-top: 0;
}

span.b-yellow {
  background-color: $lyellow;
}

span.b-red {
  background-color: $lred;
}

.editor {
  box-shadow: rgba(0, 0, 0, 0.16) 0px 1px 4px;

  &.box-warning .cm-content {
    background-color: $lyellow;
    .cm-activeLine {
      background-color: #d1d199;
    }
  }

  &.box-error .cm-content {
    background-color: $lred;

    .cm-activeLine {
      background-color: #cc8e8e;
    }
  }
}
</style>
