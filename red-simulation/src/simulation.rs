use std::cmp::Ordering;
use std::collections::BinaryHeap;

use hmac::{Mac, NewMac};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use web_sys::CanvasRenderingContext2d;

use super::geom::{Segment, Vec2};
use super::particle::{pvc, pvp, Particle, RGBA};
use super::utils::HmacSha256;

#[wasm_bindgen]
pub struct Simulation {
    w: f64,
    h: f64,
    initialized: bool,
    segments: Vec<Segment>,
    particles: Vec<Particle>,
    events: BinaryHeap<CollisionEvent>,
    t: u64,
    ticks_per_sec: u32,
    tick_time: f64,

    game_params: Option<GameParams>,
}

#[wasm_bindgen]
impl Simulation {
    pub fn new(width: f64, height: f64, ticks_per_sec: f64) -> Self {
        Simulation {
            w: width,
            h: height,
            initialized: false,
            segments: Segment::create_rectangle_domain(Vec2 { x: 0., y: 0. }, width, height),
            particles: Vec::new(),
            events: BinaryHeap::new(),
            t: 0,
            ticks_per_sec: ticks_per_sec as u32,
            tick_time: 1. / ticks_per_sec,
            game_params: None,
        }
    }

    pub fn game_mode_enabled(&self) -> bool {
        self.game_params.is_some()
    }

    fn time_to_ticks(&self, t: f64) -> u64 {
        self.t + (t * self.ticks_per_sec as f64).round() as u64
    }

    /// Drops event queue and initializes the simulation,
    /// in case of any changes in parameters or particles.
    fn init(&mut self) {
        self.events.clear();
        for l in 0..self.particles.len() {
            self.calculate_particle_events(l);
        }
        self.initialized = true;
    }

    // Adds particle with `(px, py)` coordinates, `(vx, vy)` speed vector, `m` mass and `r` radius.
    // Returns index of the added particle.
    pub fn add_particle(
        &mut self,
        px: f64,
        py: f64,
        vx: f64,
        vy: f64,
        m: f64,
        r: f64,
        color: Option<RGBA>,
    ) -> Option<usize> {
        let particle = Particle {
            pos: Vec2 { x: px, y: py },
            v: Vec2 { x: vx, y: vy },
            m,
            r,
            collisions_count: 0,
            color,
        };

        if !self.is_collission(&particle) {
            self.particles.push(particle);
            self.initialized = false;
            Some(self.particles.len() - 1)
        } else {
            None
        }
    }

    // Choose one of the particles as player's
    // and thus activate game mode.
    pub fn set_player_particle(
        &mut self,
        index: usize,
        player_name: &str,
        game_end_cb: js_sys::Function,
    ) {
        if index < self.particles.len() {
            self.particles[index].v = Vec2 { x: 0., y: 0. };

            self.game_params = Some(GameParams::new(
                index,
                player_name.to_owned(),
                self.t,
                game_end_cb,
            ));
        }
    }

    // Check wether any collision with `particle` is happening now
    fn is_collission(&self, particle: &Particle) -> bool {
        for p in &self.particles {
            if pvp::is_collision(&p, &particle) {
                return true;
            }
        }
        for s in &self.segments {
            if pvc::is_collision(&particle, &s) {
                return true;
            }
        }
        false
    }

    // Move previously chosen players's particle to some point
    pub fn mv_player_particle(&mut self, px: f64, py: f64) {
        if let Some(g_params) = &self.game_params {
            self.particles[g_params.p_particle].pos = Vec2 { x: px, y: py };
            self.initialized = false;
        }
    }

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

    pub fn add_segment(&mut self, ax: f64, ay: f64, bx: f64, by: f64) {
        self.segments
            .push(Segment::new(Vec2 { x: ax, y: ay }, Vec2 { x: bx, y: by }));
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

        for segment in &self.segments {
            ctx.begin_path();
            ctx.move_to(segment.p1.x, segment.p1.y);
            ctx.line_to(segment.p2.x, segment.p2.y);
            ctx.close_path();
            ctx.stroke();
        }
    }

    // This function called after a collision
    fn update_particle(&mut self, i: usize, new_particle: Particle) {
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

    pub fn tick_for_fps(&mut self, fps: u32) {
        let target = self.t + (self.ticks_per_sec / fps) as u64;
        while self.t <= target {
            self.tick();
        }
    }

    pub fn tick(&mut self) {
        if !self.initialized {
            self.init();
        }

        self.explicitly_check_player_particle();

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

                            self.update_particle(p1, n_left);
                            self.update_particle(p2, n_right);
                        }
                    }
                    Collision::ParticleVsSegment { p, s, p_cc } => {
                        let particle = self.particles[p];

                        if particle.collisions_count == p_cc {
                            let segment = self.segments[s];
                            let n_particle = pvc::collision(&particle, &segment);

                            self.update_particle(p, n_particle);
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

    pub fn set_ticks_per_sec(&mut self, ticks_per_sec: u32) {
        self.ticks_per_sec = ticks_per_sec;
        self.tick_time = 1. / ticks_per_sec as f64;
        self.initialized = false;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Collision {
    ParticleVsParticle {
        // Indexes of particles
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
        // Reversed order for a min queue.
        other.t.cmp(&self.t)
    }
}

impl PartialOrd for CollisionEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct GameParams {
    // Player's particle index
    p_particle: usize,
    player_name: String,
    game_end_cb: js_sys::Function,

    game_started_tick: u64,
    game_ended: bool,
}

impl GameParams {
    pub fn new(
        p_particle: usize,
        player_name: String,
        game_started_tick: u64,
        game_end_cb: js_sys::Function,
    ) -> GameParams {
        GameParams {
            p_particle,
            player_name,
            game_end_cb,
            game_started_tick,
            game_ended: false,
        }
    }

    pub fn game_over(&mut self, tick: u64, ticks_per_sec: u32) {
        // Event must fire only once per attempt
        if !self.game_ended {
            self.game_ended = true;

            let score = tick - self.game_started_tick;

            let signed_result = SignedGameResult::from_game_result(
                GameResult {
                    player_name: self.player_name.clone(),
                    score,
                    ticks_per_sec,
                },
                &crate::SECRET_KEY,
            );

            let this = JsValue::from(JsValue::null());
            let result = JsValue::from_serde(&signed_result).unwrap();
            self.game_end_cb.call1(&this, &result).unwrap();
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameResult {
    pub player_name: String,
    pub score: u64,
    pub ticks_per_sec: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignedGameResult {
    pub game_result: GameResult,
    pub hex_digest: Vec<u8>,
}

impl GameResult {
    pub fn hmac(&self, secret: &[u8]) -> HmacSha256 {
        let mut mac =
            HmacSha256::new_varkey(&secret).expect("Invalid secret, unable to build hmac.");
        mac.update(&self.player_name.as_bytes());
        mac.update(&self.score.to_be_bytes());
        mac.update(&self.ticks_per_sec.to_be_bytes());
        mac
    }

    pub fn hex_digest(&self, secret: &[u8]) -> Vec<u8> {
        self.hmac(secret).finalize().into_bytes().as_slice().into()
    }
}

impl SignedGameResult {
    pub fn from_game_result(gr: GameResult, secret: &[u8]) -> SignedGameResult {
        let hex_digest = gr.hex_digest(secret);
        SignedGameResult {
            game_result: gr,
            hex_digest,
        }
    }

    pub fn verify(&self, secret: &[u8]) -> bool {
        let mac = self.game_result.hmac(secret);
        mac.verify(&self.hex_digest).is_ok()
    }
}
