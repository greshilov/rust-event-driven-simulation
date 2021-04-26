use std::collections::{BinaryHeap, HashSet};

use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};
use web_sys::CanvasRenderingContext2d;

use super::collisions::{pvp, pvs, Collision, CollisionEvent, CollisionPair};
use super::game::GameParams;
use super::geom::{Segment, Vec2};
use super::particle::Particle;

use crate::log;

#[wasm_bindgen]
pub struct Simulation {
    w: f64,
    h: f64,
    initialized: bool,
    segments: Vec<Segment>,
    particles: Vec<Particle>,
    events: BinaryHeap<CollisionEvent>,
    t: f64,
    ticks_per_sec: u32,
    tick_time: f64,

    game_params: Option<GameParams>,
    draw_params: DrawParams,
}

#[wasm_bindgen]
impl Simulation {
    pub fn new(
        width: f64,
        height: f64,
        ticks_per_sec: u32,
        draw_params: Option<DrawParams>,
    ) -> Self {
        let draw_params = if let Some(config) = draw_params {
            config
        } else {
            Default::default()
        };

        Simulation {
            w: width,
            h: height,
            initialized: false,
            segments: Segment::create_rectangle_domain(Vec2 { x: 0., y: 0. }, width, height),
            particles: Vec::new(),
            events: BinaryHeap::new(),
            t: 0.,
            ticks_per_sec: ticks_per_sec,
            tick_time: 1. / (ticks_per_sec as f64),
            game_params: None,
            draw_params,
        }
    }

    pub fn game_mode_enabled(&self) -> bool {
        self.game_params.is_some()
    }

    // Drops event queue and initializes the simulation,
    // in case of any changes in parameters or particles.
    fn init(&mut self) {
        self.events.clear();
        for l in 0..self.particles.len() {
            self.calculate_particle_events(l);
        }
        self.initialized = true;
    }

    // Adds particle to simulation
    // Returns index of the added particle.
    pub fn add_particle(&mut self, particle: &Particle) -> Option<usize> {
        if self.is_collission(&particle) {
            log!(
                "Warning: can't add particle {:?}, that collides with other particles.",
                particle
            );
            None
        } else {
            self.particles.push(*particle);
            self.initialized = false;
            Some(self.particles.len() - 1)
        }
    }

    // Player's to simulation
    // and thus activates game mode.
    pub fn add_player_particle(
        &mut self,
        particle: &Particle,
        player_uuid: &str,
        player_name: &str,
        game_end_cb: js_sys::Function,
    ) -> Option<usize> {
        let mut particle = particle.clone();
        particle.v = Vec2 { x: 0., y: 0. };
        let add_result = self.add_particle(&particle);

        if let Some(index) = add_result {
            self.game_params = Some(GameParams::new(
                index,
                player_uuid.to_owned(),
                player_name.to_owned(),
                self.t,
                game_end_cb,
            ));
        }
        add_result
    }

    // Checks wether any collision with `particle` is happening now.
    fn is_collission(&self, particle: &Particle) -> bool {
        for p in &self.particles {
            if pvp::is_collision(&p, &particle) {
                return true;
            }
        }
        for s in &self.segments {
            if pvs::is_collision(&particle, &s) {
                return true;
            }
        }
        false
    }

    // Moves previously added players's particle to some point.
    pub fn mv_player_particle(&mut self, px: f64, py: f64) {
        if let Some(g_params) = &self.game_params {
            self.particles[g_params.p_particle].pos = Vec2 { x: px, y: py };
            self.initialized = false;
        } else {
            log!("Warning! Game mode is inactive, add the player's particle first.")
        }
    }

    // Checks whether the player's particle has collided.
    #[inline]
    fn explicitly_check_player_particle(&mut self) {
        if let Some(gp) = &self.game_params {
            if self.is_collission(&self.particles[gp.p_particle]) {
                self.game_params
                    .as_mut()
                    .unwrap()
                    .game_over(self.t, self.ticks_per_sec);
            }
        }
    }

    pub fn add_segment(&mut self, segment: &Segment) {
        self.segments.push(*segment);
        self.initialized = false;
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.clear_rect(0.0, 0.0, self.w, self.h);

        for particle in &self.particles {
            ctx.begin_path();
            ctx.arc(
                particle.pos.x,
                particle.pos.y,
                particle.r,
                0.0,
                2.0 * std::f64::consts::PI,
            )
            .unwrap();
            ctx.close_path();
            if let Some(color) = particle.color {
                ctx.set_fill_style(&JsValue::from_str(&color.as_css_hex()));
                ctx.fill();
            };
            ctx.stroke();
        }

        let segments_to_draw = if self.draw_params.borders {
            &self.segments[..]
        } else {
            &self.segments[4..]
        };

        for segment in segments_to_draw {
            ctx.begin_path();
            ctx.move_to(segment.p1.x, segment.p1.y);
            ctx.line_to(segment.p2.x, segment.p2.y);
            ctx.close_path();
            ctx.stroke();
        }
    }

    // This function called after a collision
    fn update_particle(&mut self, i: usize, new_particle: Particle, _cp: &CollisionPair) {
        // Whoops, player's particle collided - game over.
        if self
            .game_params
            .as_ref()
            .map_or(false, |gp| gp.p_particle == i)
        {
            self.game_params
                .as_mut()
                .unwrap()
                .game_over(self.t, self.ticks_per_sec);
            return;
        }

        self.particles[i] = new_particle;
        self.calculate_particle_events(i);
    }

    fn calculate_particle_events(&mut self, l: usize) {
        let left = self.particles[l];

        for (r, right) in self.particles.iter().enumerate() {
            if let Some(hit_time) = pvp::time_to_hit(&left, &right) {
                self.events.push(CollisionEvent {
                    t: self.t + hit_time,
                    collision: Collision::ParticleVsParticle {
                        p1: l,
                        p2: r,
                        p1_cc: left.collisions_count,
                        p2_cc: right.collisions_count,
                    },
                })
            }
        }

        for (s, segment) in self.segments.iter().enumerate() {
            if let Some(t) = pvs::time_to_hit(&left, &segment) {
                self.events.push(CollisionEvent {
                    t: self.t + t,
                    collision: Collision::ParticleVsSegment {
                        p: l,
                        s: s,
                        p_cc: left.collisions_count,
                    },
                })
            }
        }
    }

    #[inline]
    fn mv(&mut self, t: f64) {
        if self.t < t {
            for particle in &mut self.particles {
                particle.mv(t - self.t);
            }
            self.t = t;
        }
    }

    pub fn tick(&mut self) {
        if !self.initialized {
            self.init();
        }

        self.explicitly_check_player_particle();

        let mut collisions_happend: HashSet<CollisionPair> = HashSet::new();
        let target_time = self.t + self.tick_time;

        while let Some(event) = self.events.peek() {
            if event.t <= target_time {
                let event = self.events.pop().unwrap();
                let collision_pair: CollisionPair = event.collision.into();

                if collisions_happend.contains(&collision_pair) {
                    continue;
                }

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

                            self.update_particle(p1, n_left, &collision_pair);
                            self.update_particle(p2, n_right, &collision_pair);
                            collisions_happend.insert(collision_pair);
                        }
                    }
                    Collision::ParticleVsSegment { p, s, p_cc } => {
                        let particle = self.particles[p];

                        if particle.collisions_count == p_cc {
                            let segment = self.segments[s];
                            let n_particle = pvs::collision(&particle, &segment);

                            self.update_particle(p, n_particle, &collision_pair);
                            collisions_happend.insert(collision_pair);
                        }
                    }
                }

                if self.t < event.t {
                    collisions_happend.clear();
                    self.mv(event.t);
                }
            } else {
                break;
            }
        }

        self.mv(target_time);
    }

    pub fn current_tick(&self) -> f64 {
        self.t as f64
    }

    pub fn get_current_score(&self) -> Option<u32> {
        self.game_params
            .as_ref()
            .map_or(None, |gp| Some(gp.get_score(self.t)))
    }

    // This function has serious performance penalties.
    // Use it with caution.
    pub fn get_particles(&self) -> JsValue {
        JsValue::from_serde(&self.particles.clone()).unwrap()
    }
}

#[wasm_bindgen]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct DrawParams {
    pub borders: bool,
}

#[wasm_bindgen]
impl DrawParams {
    pub fn new(borders: bool) -> DrawParams {
        DrawParams { borders }
    }
}
