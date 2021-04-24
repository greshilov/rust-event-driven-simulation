#![feature(test)]

use rand::{rngs::StdRng, Rng, SeedableRng};

use std::fs::File;
use std::io::prelude::*;

extern crate red_simulation;
extern crate test;

use red_simulation::geom::Vec2;
use red_simulation::particle::Particle;
use red_simulation::simulation::Simulation;

#[bench]
fn simulation_ticks(b: &mut test::Bencher) {
    let width = 600.;
    let height = 400.;
    let r = 10.;
    let density = 0.2;
    let seed = 42;

    let mut simulation = generate_simultaion(width, height, r, density, seed);

    b.iter(|| {
        simulation.tick();
    });
}

#[bench]
fn simulation_ticks_crowded(b: &mut test::Bencher) {
    let width = 600.;
    let height = 400.;
    let r = 7.;
    let density = 0.7;
    let seed = 42;

    let mut simulation = generate_simultaion(width, height, r, density, seed);

    b.iter(|| {
        simulation.tick();
    });
}

fn generate_simultaion(width: f64, height: f64, r: f64, density: f64, seed: u64) -> Simulation {
    let fps = 60;
    let mut simulation = Simulation::new(width, height, fps, None);

    let particles = generate_particles(width, height, r, density, seed);

    for p in &particles {
        simulation.add_particle(p);
    }

    simulation
}

fn generate_particles(width: f64, height: f64, r: f64, density: f64, seed: u64) -> Vec<Particle> {
    let mut seeded_rng = StdRng::seed_from_u64(seed);

    let mut particles = Vec::new();

    if density < 0. || density > 0.9 {
        panic!("Density must be in 0..0.9 range")
    }

    let step = 6. * r - 4. * density * r;
    let mut x = 2. * r;

    while x <= width - 2. * r {
        let mut y = 2. * r;

        while y <= height - 2. * r {
            let pos = Vec2 { x, y };

            let speed_limit = (width + height) / 2. * 1. / 5.;

            let v = Vec2 {
                x: seeded_rng.gen_range(-speed_limit..speed_limit),
                y: seeded_rng.gen_range(-speed_limit..speed_limit),
            };

            particles.push(Particle {
                pos,
                v,
                m: 1.0,
                r,
                collisions_count: 0,
                color: None,
            });
            y += step;
        }
        x += step;
    }
    particles
}

fn save_particles(particles: &Vec<Particle>, file_name: &str) {
    let mut file = File::create(file_name).unwrap();
    let data = serde_json::to_string(particles).unwrap();
    file.write_all(&data.as_bytes());
}
