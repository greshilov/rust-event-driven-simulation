import { Simulation } from "red-simulation";
import { generateRandomParticles } from "./utils";

const canvas = document.getElementById("sim");
const ctx = canvas.getContext("2d");
const simulation = Simulation.new(canvas.width, canvas.height, 60);
const particles = generateRandomParticles(canvas.width, canvas.height, 10, 0.7, true);

for (const particle of particles) {
  simulation.add_particle(particle);
}

const renderLoop = () => {
  simulation.draw(ctx);
  simulation.tick();
  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
