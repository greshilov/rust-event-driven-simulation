import { Particle, RGBA } from "red-simulation";

export function getRandomArbitrary(min, max) {
  return Math.random() * (max - min) + min;
}

export function getRandomColor() {
    const colors = [0, 0, 0].map(_ => getRandomArbitrary(0, 255));
    return RGBA.new(...colors, 1);
}

export function generateRandomParticles(
  width,
  height,
  r = 10,
  density = 0.5,
  colorized = false
) {
  const speedLimit = (width + height) / 6;
  const gap = r;
  const step = 12 * gap - 10 * density * gap;
  const x0 = step;
  const y0 = step;

  const particles = [];

  for (let x = x0; x <= width - step; x += step) {
    for (let y = y0; y <= height - step; y += step) {
      const Ux = getRandomArbitrary(-speedLimit, speedLimit);
      const Uy = getRandomArbitrary(-speedLimit, speedLimit);
      const color = colorized ? getRandomColor() : undefined;
      particles.push(Particle.new(x, y, Ux, Uy, 1, r, color));
    }
  }
  return particles;
}
