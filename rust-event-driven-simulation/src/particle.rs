use super::geom::{Line, Segment, Vec2};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Particle {
    pub pos: Vec2,
    pub v: Vec2,
    pub m: f64,
    pub r: f64,
    pub collisions_count: u64,
}

impl Particle {
    pub fn mv(&mut self, dt: f64) {
        self.pos += self.v * dt;
    }
}

// Particle vs Particle
pub mod pvp {
    use super::Particle;

    pub fn time_to_hit(left: &Particle, right: &Particle) -> Option<f64> {
        if left == right {
            return None;
        }

        let dr = left.pos - right.pos;
        let sigma = left.r + right.r;

        if dr.len() < sigma {
            return None;
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
pub mod pvc {
    use super::{Line, Particle, Segment};

    pub fn time_to_hit(left: &Particle, right: &Segment) -> Option<f64> {
        if left.v.x == 0. && left.v.y == 0. {
            return None;
        }

        let movement_line = Line::from_vec_n_point(&left.v, &left.pos);
        let segment_line = Line::from_segment(&right);

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

            // We know, that points "bp1" and bp2" lies on the
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

    pub fn collision(left: &Particle, right: &Segment) -> Particle {
        let mut new_left = left.clone();
        new_left.v = new_left.v - right.n * (new_left.v * right.n * 2.);
        new_left.collisions_count += 1;
        new_left
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::compare_floats;

    #[test]
    fn test_particle_v_particle_time_to_hit() {
        let pos_1 = Vec2 { x: 0.0, y: 0.0 };
        let v_1 = Vec2 { x: 1.0, y: 1.0 };

        let pos_2 = Vec2 { x: 3.0, y: 0.0 };
        let v_2 = Vec2 { x: -1.0, y: 1.0 };

        let p_1 = Particle {
            pos: pos_1,
            v: v_1,
            m: 1.0,
            r: 0.2,
            collisions_count: 0,
        };

        let p_2 = Particle {
            pos: pos_2,
            v: v_2,
            m: 1.0,
            r: 0.2,
            collisions_count: 0,
        };

        compare_floats(pvp::time_to_hit(&p_1, &p_2).unwrap(), 1.3);
    }

    #[test]
    fn test_particle_v_segment_time_to_hit() {
        let pos_1 = Vec2 { x: 0.0, y: 0.0 };
        let v_1 = Vec2 { x: 2.0, y: 1.0 };

        let p_1 = Particle {
            pos: pos_1,
            v: v_1,
            m: 1.0,
            r: 0.2,
            collisions_count: 0,
        };

        let seg_1 = Vec2 { x: 3.0, y: 0.0 };
        let seg_2 = Vec2 { x: 3.0, y: 3.0 };

        // Basically vertical line from (3, 0) to (3, 3)
        let seg = Segment::new(seg_1, seg_2);

        compare_floats(pvc::time_to_hit(&p_1, &seg).unwrap(), 1.4);
    }

    #[test]
    fn test_particle_v_segment_time_to_hit_large_particle_horizontal() {
        let pos_1 = Vec2 { x: 3.0, y: -6.0 };
        let v_1 = Vec2 { x: -1.0, y: 1.0 };

        let p_1 = Particle {
            pos: pos_1,
            v: v_1,
            m: 1.0,
            r: 2.5,
            collisions_count: 0,
        };

        let seg_1 = Vec2 { x: 0.0, y: 0.0 };
        let seg_2 = Vec2 { x: 6.0, y: 0.0 };

        let seg = Segment::new(seg_1, seg_2);

        compare_floats(pvc::time_to_hit(&p_1, &seg).unwrap(), 3.5);
    }

    #[test]
    fn test_particle_v_segment_time_to_hit_large_particle_vertical() {
        let pos_1 = Vec2 { x: 3.0, y: -2.0 };
        let v_1 = Vec2 { x: -1.0, y: 1.0 };

        let p_1 = Particle {
            pos: pos_1,
            v: v_1,
            m: 1.0,
            r: 1.4,
            collisions_count: 0,
        };

        let seg_1 = Vec2 { x: 0.0, y: 0.0 };
        let seg_2 = Vec2 { x: 0.0, y: -6.0 };

        let seg = Segment::new(seg_1, seg_2);

        compare_floats(pvc::time_to_hit(&p_1, &seg).unwrap(), 1.6);
    }

    #[test]
    fn test_particle_v_segment_time_to_hit_angle() {
        let pos_1 = Vec2 { x: 0.0, y: 6.0 };
        let v_1 = Vec2 { x: 1.0, y: 0.0 };

        let p_1 = Particle {
            pos: pos_1,
            v: v_1,
            m: 1.0,
            r: 1.0,
            collisions_count: 0,
        };

        let seg_1 = Vec2 { x: 0.0, y: 0.0 };
        let seg_2 = Vec2 { x: 3.0, y: 5.0 };

        let seg = Segment::new(seg_1, seg_2);

        compare_floats(pvc::time_to_hit(&p_1, &seg).unwrap(), 2.43380962103094);
    }

    #[test]
    fn test_particle_v_particle_collision() {
        let pos_1 = Vec2 { x: -1.0, y: 0.0 };
        let v_1 = Vec2 { x: 1.0, y: 0.0 };

        let pos_2 = Vec2 { x: 1.0, y: 0.0 };
        let v_2 = Vec2 { x: -1.0, y: 0.0 };

        let p_1 = Particle {
            pos: pos_1,
            v: v_1,
            m: 1.0,
            r: 1.0,
            collisions_count: 0,
        };

        let p_2 = Particle {
            pos: pos_2,
            v: v_2,
            m: 1.0,
            r: 1.0,
            collisions_count: 0,
        };

        let (pn_1, pn_2) = pvp::collision(&p_1, &p_2);

        compare_floats(pn_1.v.x, -1.0);
        compare_floats(pn_2.v.x, 1.0);
    }

    #[test]
    fn test_particle_v_segment_collision_angle() {
        let pos_1 = Vec2 {
            x: 2.43380962103094,
            y: 6.0,
        };
        let v_1 = Vec2 { x: 1.0, y: 0.0 };

        let p_1 = Particle {
            pos: pos_1,
            v: v_1,
            m: 1.0,
            r: 1.0,
            collisions_count: 0,
        };

        let seg_1 = Vec2 { x: 0.0, y: 0.0 };
        let seg_2 = Vec2 { x: 3.0, y: 5.0 };

        let seg = Segment::new(seg_1, seg_2);

        let p_new = pvc::collision(&p_1, &seg);

        // Summary speed must not change
        compare_floats(p_new.v.len(), p_1.v.len());
    }
}
