use std::cmp::Ordering;
use std::convert::Into;
use std::hash::{Hash, Hasher};

// Particle vs Particle
pub mod pvp {
    use crate::particle::Particle;

    pub fn time_to_hit(left: &Particle, right: &Particle) -> Option<f64> {
        if left == right {
            return None;
        }

        if left.v.is_zero() && right.v.is_zero() {
            return None;
        }

        let dr = left.pos - right.pos;
        let sigma = left.r + right.r;

        if dr.len() < sigma {
            return Some(0.);
        }

        let dv = left.v - right.v;
        let dv_dr = dv * dr;
        if dv_dr > 0. {
            return None;
        }
        let dv_dv = dv * dv;

        let d = dv_dr * dv_dr - dv_dv * (dr * dr - sigma * sigma);

        if d < 0. {
            return None;
        }
        if dv_dv != 0. {
            return Some(-(dv_dr + d.sqrt()) / dv_dv);
        } else {
            return None;
        }
    }

    pub fn is_collision(left: &Particle, right: &Particle) -> bool {
        if left == right {
            return false;
        }
        (right.pos - left.pos).len() < (right.r + left.r)
    }

    pub fn collision(left: &Particle, right: &Particle) -> (Particle, Particle) {
        let dr = right.pos - left.pos;
        let dv = right.v - left.v;
        let dv_dr = dv * dr;
        let dist_sqr = dr.len_sqr();

        let j_norm = 2. * left.m * right.m * dv_dr / (left.m + right.m);
        let j = dr * (j_norm / dist_sqr);

        let mut new_left = left.clone();
        let mut new_right = right.clone();

        new_left.v += j * (1. / left.m);
        new_right.v -= j * (1. / right.m);

        new_left.collisions_count += 1;
        new_right.collisions_count += 1;

        (new_left, new_right)
    }
}

// Particle vs Segment
pub mod pvs {
    use crate::geom::{LCIntersection, Line, Segment};
    use crate::particle::Particle;

    pub fn time_to_hit(left: &Particle, right: &Segment) -> Option<f64> {
        if left.v.is_zero() {
            return None;
        }

        let movement_line = Line::from_vec_n_point(&left.v, &left.pos);
        let segment_line = right.line;

        if let Some(intrsct_p) = movement_line.intersect_line(&segment_line) {
            let ray = intrsct_p - left.pos;

            // Particle is moving in opposite direction
            if ray * left.v < 0. {
                return None;
            }

            // Speed projection on the segment
            let speed_norm = ray.norm() * left.r;
            let circle_proj = right.v * (left.r * 1. / right.v.angle_cos(&speed_norm));

            let bp1 = intrsct_p - circle_proj;
            let bp2 = intrsct_p + circle_proj;

            // We know, that points "bp1" and bp2" lie on the
            // "segment_line", so we can just compare coordinates
            // to check whether "bp1" or "bp2" placed inside the segment "right"
            if !right.contains_point(&bp1) && !right.contains_point(&bp2) {
                return None;
            }

            // Yes, point is inside the segment
            let proj = (ray * right.n).abs();
            let speed_proj = (left.v * right.n).abs();

            if proj < left.r {
                return Some(0.);
            } else {
                return Some((proj - left.r) / speed_proj);
            }
        }
        None
    }

    pub fn is_collision(left: &Particle, right: &Segment) -> bool {
        match right.line.intersect_circle(&left.circle()) {
            LCIntersection::OnePoint(p) => right.contains_point(&p),
            LCIntersection::TwoPoint((p1, p2)) => {
                right.contains_point(&p1) || right.contains_point(&p2)
            }
            LCIntersection::None => false,
        }
    }

    pub fn collision(left: &Particle, right: &Segment) -> Particle {
        let mut new_left = left.clone();
        new_left.v = new_left.v - right.n * (new_left.v * right.n * 2.);
        new_left.collisions_count += 1;
        new_left
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

impl Into<CollisionPair> for Collision {
    fn into(self) -> CollisionPair {
        match self {
            Self::ParticleVsParticle { p1, p2, .. } => CollisionPair::PvP(p1, p2),
            Self::ParticleVsSegment { p, s, .. } => CollisionPair::PvE(p, s),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CollisionEvent {
    pub t: f64,
    pub collision: Collision,
}

impl PartialEq for CollisionEvent {
    #[inline]
    fn eq(&self, other: &CollisionEvent) -> bool {
        self.t == other.t
    }
}

impl Eq for CollisionEvent {}

impl Ord for CollisionEvent {
    #[inline]
    fn cmp(&self, other: &CollisionEvent) -> Ordering {
        // Reversed order for a min queue.
        // Kind a dangerous, but no NaN is expected.
        other.t.partial_cmp(&self.t).unwrap()
    }
}

impl PartialOrd for CollisionEvent {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Debug)]
pub enum CollisionPair {
    PvP(usize, usize),
    PvE(usize, usize),
}

impl Hash for CollisionPair {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::PvP(p1, p2) => {
                state.write_u8(0);
                if p1 < p2 {
                    p1.hash(state);
                    p2.hash(state);
                } else {
                    p2.hash(state);
                    p1.hash(state);
                }
            }
            Self::PvE(p, s) => {
                state.write_u8(1);
                p.hash(state);
                s.hash(state);
            }
        };
    }
}

impl PartialEq for CollisionPair {
    fn eq(&self, other: &CollisionPair) -> bool {
        match (self, other) {
            (Self::PvP(p1, p2), Self::PvP(p3, p4)) => {
                (p1 == p3 && p2 == p4) || (p1 == p4 && p2 == p3)
            }
            (Self::PvE(p1, p2), Self::PvE(p3, p4)) => p1 == p3 && p2 == p4,
            _ => false,
        }
    }
}

impl Eq for CollisionPair {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compare_floats;
    use crate::geom::{Segment, Vec2};
    use crate::particle::Particle;

    fn particle(pos: Vec2, v: Vec2, m: f64, r: f64) -> Particle {
        Particle {
            pos,
            v,
            m,
            r,
            collisions_count: 0,
            color: None,
        }
    }

    #[test]
    fn test_particle_v_particle_time_to_hit() {
        let pos_1 = Vec2 { x: 0.0, y: 0.0 };
        let v_1 = Vec2 { x: 1.0, y: 1.0 };

        let pos_2 = Vec2 { x: 3.0, y: 0.0 };
        let v_2 = Vec2 { x: -1.0, y: 1.0 };

        let p_1 = particle(pos_1, v_1, 1.0, 0.2);

        let p_2 = particle(pos_2, v_2, 1.0, 0.2);

        compare_floats!(pvp::time_to_hit(&p_1, &p_2).unwrap(), 1.3);
    }

    #[test]
    fn test_particle_v_segment_time_to_hit() {
        let pos_1 = Vec2 { x: 0.0, y: 0.0 };
        let v_1 = Vec2 { x: 2.0, y: 1.0 };

        let p_1 = particle(pos_1, v_1, 1.0, 0.2);

        let seg_1 = Vec2 { x: 3.0, y: 0.0 };
        let seg_2 = Vec2 { x: 3.0, y: 3.0 };

        // Basically vertical line from (3, 0) to (3, 3)
        let seg = Segment::from_points(seg_1, seg_2);

        compare_floats!(pvs::time_to_hit(&p_1, &seg).unwrap(), 1.4);
    }

    #[test]
    fn test_particle_v_segment_time_to_hit_large_particle_horizontal() {
        let pos_1 = Vec2 { x: 3.0, y: -6.0 };
        let v_1 = Vec2 { x: -1.0, y: 1.0 };

        let p_1 = particle(pos_1, v_1, 1.0, 2.5);

        let seg_1 = Vec2 { x: 0.0, y: 0.0 };
        let seg_2 = Vec2 { x: 6.0, y: 0.0 };

        let seg = Segment::from_points(seg_1, seg_2);

        compare_floats!(pvs::time_to_hit(&p_1, &seg).unwrap(), 3.5);
    }

    #[test]
    fn test_particle_v_segment_time_to_hit_large_particle_vertical() {
        let pos_1 = Vec2 { x: 3.0, y: -2.0 };
        let v_1 = Vec2 { x: -1.0, y: 1.0 };

        let p_1 = particle(pos_1, v_1, 1.0, 1.4);

        let seg_1 = Vec2 { x: 0.0, y: 0.0 };
        let seg_2 = Vec2 { x: 0.0, y: -6.0 };

        let seg = Segment::from_points(seg_1, seg_2);

        compare_floats!(pvs::time_to_hit(&p_1, &seg).unwrap(), 1.6);
    }

    #[test]
    fn test_particle_v_segment_time_to_hit_angle() {
        let pos_1 = Vec2 { x: 0.0, y: 6.0 };
        let v_1 = Vec2 { x: 1.0, y: 0.0 };

        let p_1 = particle(pos_1, v_1, 1.0, 1.0);

        let seg_1 = Vec2 { x: 0.0, y: 0.0 };
        let seg_2 = Vec2 { x: 3.0, y: 5.0 };

        let seg = Segment::from_points(seg_1, seg_2);

        compare_floats!(pvs::time_to_hit(&p_1, &seg).unwrap(), 2.43380962103094);
    }

    #[test]
    fn test_particle_v_particle_collision() {
        let pos_1 = Vec2 { x: -1.0, y: 0.0 };
        let v_1 = Vec2 { x: 1.0, y: 0.0 };

        let pos_2 = Vec2 { x: 1.0, y: 0.0 };
        let v_2 = Vec2 { x: -1.0, y: 0.0 };

        let p_1 = particle(pos_1, v_1, 1.0, 1.0);

        let p_2 = particle(pos_2, v_2, 1.0, 1.0);

        let (pn_1, pn_2) = pvp::collision(&p_1, &p_2);

        compare_floats!(pn_1.v.x, -1.0);
        compare_floats!(pn_2.v.x, 1.0);
    }

    #[test]
    fn test_particle_v_segment_collision_angle() {
        let pos_1 = Vec2 {
            x: 2.43380962103094,
            y: 6.0,
        };
        let v_1 = Vec2 { x: 1.0, y: 0.0 };

        let p_1 = particle(pos_1, v_1, 1.0, 1.0);

        let seg_1 = Vec2 { x: 0.0, y: 0.0 };
        let seg_2 = Vec2 { x: 3.0, y: 5.0 };

        let seg = Segment::from_points(seg_1, seg_2);

        let p_new = pvs::collision(&p_1, &seg);

        // Summary speed must not change
        compare_floats!(p_new.v.len(), p_1.v.len());
    }
}
