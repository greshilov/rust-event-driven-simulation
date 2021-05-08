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

    // Main function that represents one iteration of the simulation.
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

                // Check whether this collision has already happened
                // during the current tick. This check protects us from
                // infinite loop in case of the multi-particle collision.
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

    // Checks whether the player's particle has collided.
    // Used in "game mode" only.
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

    // Drops event queue and initializes the simulation,
    // in case of any changes in parameters or particles.
    fn init(&mut self) {
        self.events.clear();
        for l in 0..self.particles.len() {
            self.calculate_particle_events(l);
        }
        self.initialized = true;
    }

    // Recalculates events for the specified
    // particle with index `l`.
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

    // Moves all particles in the system using their current velocities.
    #[inline]
    fn mv(&mut self, t: f64) {
        if self.t < t {
            for particle in &mut self.particles {
                particle.mv(t - self.t);
            }
            self.t = t;
        }
    }

    // Updates particle speed after collision.
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

    // Adds player's info and thus activates game mode.
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

    pub fn mv_player_particle(&mut self, px: f64, py: f64) {
        if let Some(g_params) = &self.game_params {
            self.particles[g_params.p_particle].pos = Vec2 { x: px, y: py };
            self.initialized = false;
        } else {
            log!("Warning! Game mode is inactive, add the player's particle first.")
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

    pub fn is_game_mode_enabled(&self) -> bool {
        self.game_params.is_some()
    }

    pub fn get_current_tick(&self) -> f64 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compare_vec2;

    #[test]
    fn test_simulation() {
        let mut sim = Simulation::new(100.0, 100.0, 100, None);

        let p1 = Particle::new(
            //  x    y    vx   vy   m   r   color
            20., 30., 30., 20., 1., 5., None,
        );
        let p2 = Particle::new(
            //  x    y     vx    vy   m   r   color
            80., 70., -30., -20., 2., 5., None,
        );

        sim.add_particle(&p1);
        sim.add_particle(&p2);

        let expected_collision_tick = 85;
        for _ in 0..(expected_collision_tick + 1) {
            sim.tick();
        }

        compare_vec2!(
            sim.particles[0].pos,
            Vec2 { x: 45.8, y: 47.2 },
            "p1 position right before collision",
        );

        compare_vec2!(
            sim.particles[1].pos,
            Vec2 { x: 54.2, y: 52.8 },
            "p2 position right before collision",
        );

        assert_eq!(p1.v, sim.particles[0].v, "p1 collided too early");
        assert_eq!(p2.v, sim.particles[1].v, "p2 collided too early");

        // Moment of truth
        sim.tick();

        // Positions
        compare_vec2!(
            sim.particles[0].pos,
            Vec2 {
                x: 45.3,
                y: 46.866666666666
            },
            "p1 position right after collision",
        );
        compare_vec2!(
            sim.particles[1].pos,
            Vec2 {
                x: 54.3,
                y: 52.866666666666,
            },
            "p2 position right after collision",
        );

        let expected_p1v = Vec2 {
            x: -50.0,
            y: -33.33333333333,
        };

        // Velocity
        compare_vec2!(
            sim.particles[0].v,
            expected_p1v,
            "p1 velocity right after collision",
        );

        compare_vec2!(
            sim.particles[1].v,
            Vec2 {
                x: 10.0,
                y: 6.66666666666,
            },
            "p2 velocity right after collision",
        );

        // And there's must be a wall collision for the p1
        let expected_wall_collision_tick = 79;
        for _ in 0..(expected_wall_collision_tick + 1) {
            sim.tick();
        }

        compare_vec2!(
            sim.particles[0].pos,
            Vec2 { x: 5.3, y: 20.2 },
            "p1 position right before the wall collision",
        );

        compare_vec2!(
            sim.particles[0].v,
            expected_p1v,
            "p1 velocity right before the wall collision",
        );
        // Wall collision tick
        sim.tick();

        compare_vec2!(
            sim.particles[0].pos,
            Vec2 {
                x: 5.8,
                y: 19.8666666666,
            },
            "p1 position right after the wall collision",
        );
        compare_vec2!(
            sim.particles[0].v,
            Vec2 {
                x: 50.,
                y: -33.3333333333,
            },
            "p1 velocity right after the wall collision",
        );
    }
}
