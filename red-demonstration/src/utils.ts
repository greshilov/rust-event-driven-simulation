import { Particle, RGBA, Vec2 } from "red-simulation";
import percentile from "percentile";

import { IParticleGenerateOptions } from "@/interfaces.ts";

function getRandomArbitrary(min: number, max: number): number {
  return Math.random() * (max - min) + min;
}

import { IPlayer } from "./interfaces";

export function getRandomColor(): RGBA {
  const red = getRandomArbitrary(0, 255);
  const green = getRandomArbitrary(0, 255);
  const blue = getRandomArbitrary(0, 255);
  return RGBA.new(red, green, blue, 255);
}

// import DATA from "../../red-simulation/data.json";
// export function fillWithRandomParticles(simulation: Simulation): void {
//   (DATA as any).forEach((item: any) => {
//     const { pos, v, m, r, color } = item;

//     simulation.add_particle(Particle.new(pos.x, pos.y, v.x, v.y, m, r, color));
//   });
// }

export function generateRandomParticles(
  width: number,
  height: number,
  options?: IParticleGenerateOptions
): Particle[] {
  const density = options?.density || 0.5;
  const particles = [];

  const speedLimit = options?.speedLimit || (width + height) / 6;
  const gap = options?.r !== undefined ? options.r : 10;
  const step = 12 * gap - 10 * density * gap;
  const x0 = options?.startPoint ? options.startPoint.x + step : step;
  const y0 = options?.startPoint ? options.startPoint.y + step : step;

  for (let x = x0; x <= width - step; x += step) {
    for (let y = y0; y <= height - step; y += step) {
      const Ux = getRandomArbitrary(-speedLimit, speedLimit);
      const Uy = getRandomArbitrary(-speedLimit, speedLimit);
      const _r = options?.r ? options.r : getRandomArbitrary(7, 12);
      const m = _r / 10;
      const _color =
        options?.color !== undefined
          ? RGBA.new(
              options.color![0],
              options.color![1],
              options.color![2],
              255
            )
          : undefined;
      particles.push(Particle.new(x, y, Ux, Uy, m, _r, _color));
    }
  }
  return particles;
}

export class FrameRater {
  lastCall: number;
  counter: number;

  prevFrame: number | null;
  frameStat: number[];

  constructor() {
    this.lastCall = 0;
    this.counter = 0;

    this.prevFrame = null;
    this.frameStat = [];
  }

  calculateFrameRate(): number | undefined {
    const now = performance.now();
    let rate;

    if (this.lastCall > 0) {
      rate = Math.round((this.counter / (now - this.lastCall)) * 1000);
      this.counter = 0;
    }
    this.lastCall = now;
    return rate;
  }

  calculateFrameStats(): number | number[] {
    return percentile([50, 75, 80, 90], this.frameStat);
  }

  startFrame(): void {
    this.prevFrame = performance.now();
  }

  endFrame(): void {
    if (this.prevFrame) {
      this.frameStat.push(performance.now() - this.prevFrame);
    }
    this.counter++;
  }
}

export function len(vec: Vec2): number {
  return Math.sqrt(vec.x * vec.x + vec.y * vec.y);
}

export function roundTo(value: number, places = 2): number {
  const power = Math.pow(10, places);
  return Math.round(value * power) / power;
}

export function getPlayer(): IPlayer | undefined {
  if (localStorage.player !== undefined) {
    return JSON.parse(localStorage.player);
  }
}
