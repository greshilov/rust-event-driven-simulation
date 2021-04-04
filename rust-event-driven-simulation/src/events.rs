use std::cmp::Ordering;
use std::collections::BinaryHeap;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

use super::geom::{Segment, Vec2};
use super::particle::{pvc, pvp, Particle};

#[derive(Debug, Clone, Copy)]
pub enum Collision {
    ParticleVsParticle {
        p1: usize,
        p2: usize,
        // cc - Collisions count
        p1_cc: u64,
        p2_cc: u64,
    },
    ParticleVsSegment {
        p: usize,
        s: usize,
        p_cc: u64,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct CollisionEvent {
    pub t: u64,
    pub collision: Collision,
}

impl PartialEq for CollisionEvent {
    fn eq(&self, other: &CollisionEvent) -> bool {
        self.t == other.t
    }
}

impl Eq for CollisionEvent {}

impl Ord for CollisionEvent {
    fn cmp(&self, other: &CollisionEvent) -> Ordering {
        // Reversed order for min queue.
        other.t.cmp(&self.t)
    }
}

impl PartialOrd for CollisionEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[wasm_bindgen]
pub struct Simulation {
    w: f64,
    h: f64,
    initialized: bool,
    segments: Vec<Segment>,
    particles: Vec<Particle>,
    events: BinaryHeap<CollisionEvent>,
    t: u64,
    tick_per_sec: f64,
    tick_time: f64,
}

#[wasm_bindgen]
impl Simulation {
    pub fn new(width: f64, height: f64, tick_per_sec: f64) -> Self {
        Simulation {
            w: width,
            h: height,
            initialized: false,
            segments: Segment::create_rectangle_domain(Vec2 { x: 0., y: 0. }, width, height),
            particles: Vec::new(),
            events: BinaryHeap::new(),
            t: 0,
            tick_per_sec,
            tick_time: 1. / tick_per_sec,
        }
    }

    fn time_to_ticks(&self, t: f64) -> u64 {
        self.t + (t * self.tick_per_sec).round() as u64
    }

    pub fn add_particle(&mut self, px: f64, py: f64, vx: f64, vy: f64, m: f64, r: f64) {
        self.particles.push(Particle {
            pos: Vec2 { x: px, y: py },
            v: Vec2 { x: vx, y: vy },
            m,
            r,
            collisions_count: 0,
        });
    }

    pub fn add_segment(&mut self, ax: f64, ay: f64, bx: f64, by: f64) {
        self.segments
            .push(Segment::new(Vec2 { x: ax, y: ay }, Vec2 { x: bx, y: by }));
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.clear_rect(0.0, 0.0, self.w, self.h);

        for particle in &self.particles {
            ctx.begin_path();
            ctx.arc(particle.pos.x, particle.pos.y, particle.r, 0.0, 2.0 * 3.14)
                .unwrap();
            ctx.close_path();
            ctx.stroke();
        }

        for segment in &self.segments {
            ctx.begin_path();
            ctx.move_to(segment.p1.x, segment.p1.y);
            ctx.line_to(segment.p2.x, segment.p2.y);
            ctx.close_path();
            ctx.stroke();
        }
    }

    fn calculate_particle_events(&mut self, l: usize) {
        let left = self.particles[l];

        for r in 0..self.particles.len() {
            let right = self.particles[r];

            if let Some(hit_time) = pvp::time_to_hit(&left, &right) {
                self.events.push(CollisionEvent {
                    t: self.time_to_ticks(hit_time),
                    collision: Collision::ParticleVsParticle {
                        p1: l,
                        p2: r,
                        p1_cc: left.collisions_count,
                        p2_cc: right.collisions_count,
                    },
                })
            }
        }

        for s in 0..self.segments.len() {
            let segment = self.segments[s];

            if let Some(t) = pvc::time_to_hit(&left, &segment) {
                self.events.push(CollisionEvent {
                    t: self.time_to_ticks(t),
                    collision: Collision::ParticleVsSegment {
                        p: l,
                        s: s,
                        p_cc: left.collisions_count,
                    },
                })
            }
        }
    }

    fn init(&mut self) {
        for l in 0..self.particles.len() {
            self.calculate_particle_events(l);
        }
        self.initialized = true;
    }

    pub fn tick_for_fps(&mut self, fps: f64) {
        let target = self.t + (self.tick_per_sec / fps).round() as u64;
        while self.t <= target {
            self.tick();
        }
    }

    pub fn tick(&mut self) {
        if !self.initialized {
            self.init();
        }

        while let Some(event) = self.events.peek() {
            if event.t == self.t {
                let event = self.events.pop().unwrap();
                match event.collision {
                    Collision::ParticleVsParticle {
                        p1,
                        p2,
                        p1_cc,
                        p2_cc,
                    } => {
                        let left = self.particles[p1];
                        let right = self.particles[p2];

                        if left.collisions_count == p1_cc && right.collisions_count == p2_cc {
                            let (n_left, n_right) = pvp::collision(&left, &right);
                            self.particles[p1] = n_left;
                            self.particles[p2] = n_right;

                            self.calculate_particle_events(p1);
                            self.calculate_particle_events(p2);
                        }
                    }
                    Collision::ParticleVsSegment { p, s, p_cc } => {
                        let particle = self.particles[p];

                        if particle.collisions_count == p_cc {
                            let segment = self.segments[s];
                            let n_particle = pvc::collision(&particle, &segment);
                            self.particles[p] = n_particle;

                            self.calculate_particle_events(p);
                        }
                    }
                }
            } else {
                break;
            }
        }

        for particle in &mut self.particles {
            particle.mv(self.tick_time);
        }

        self.t += 1;
    }

    pub fn current_tick(&self) -> f64 {
        self.t as f64
    }

    pub fn set_ticks_per_sec(&mut self, ticks_per_sec: f64) {
        self.tick_per_sec = ticks_per_sec;
        self.tick_time = 1. / ticks_per_sec;
        self.events.clear();
        self.init();
    }
}
