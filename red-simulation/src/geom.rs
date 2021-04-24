use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Add for Vec2 {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vec2 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = f64;

    #[inline]
    fn mul(self, other: Self) -> Self::Output {
        self.x * other.x + self.y * other.y
    }
}

impl Vec2 {
    #[inline]
    pub fn is_zero(&self) -> bool {
        self.x == 0. && self.y == 0.
    }

    pub fn len(&self) -> f64 {
        (self.len_sqr()).sqrt()
    }

    pub fn len_sqr(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    #[allow(dead_code)]
    pub fn is_collinear(&self, other: &Vec2) -> bool {
        self.x * other.y - self.y * other.x == 0.
    }

    pub fn norm(&self) -> Vec2 {
        let x;
        let y;
        if self.y != 0.0 && self.x != 0.0 {
            let k = self.x / self.y;
            let inv_len = 1.0 / (1.0 + k * k).sqrt();

            x = inv_len;
            y = -self.x / self.y * inv_len;
        } else if self.y == 0.0 {
            x = 0.0;
            y = 1.0;
        } else {
            x = 1.0;
            y = 0.0;
        }

        Vec2 { x, y }
    }

    pub fn normalize(&self) -> Vec2 {
        *self * (1. / self.len())
    }

    pub fn angle_cos(&self, other: &Vec2) -> f64 {
        *self * *other / (self.len_sqr() * other.len_sqr()).sqrt()
    }
}

/// Segment is a section between two points.
/// Must always be imutable object, because `n`, `v`, `line`
/// depend on `p1` and `p2` fields.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Segment {
    /// First point of the segment
    pub p1: Vec2,
    /// Second point of the segment
    pub p2: Vec2,
    /// Normal to segment (normalized one)
    pub n: Vec2,
    /// Normalized vector that lies alongside the segment
    pub v: Vec2,
    /// Line correspoding to the segment
    pub line: Line,
}

#[wasm_bindgen]
impl Segment {
    pub fn new(ax: f64, ay: f64, bx: f64, by: f64) -> Segment {
        Segment::from_points(Vec2 { x: ax, y: ay }, Vec2 { x: bx, y: by })
    }
}

impl Segment {
    pub fn from_points(p1: Vec2, p2: Vec2) -> Segment {
        let v = (p2 - p1).normalize();
        // This is normalized normal, lol.
        let n = v.norm();
        let line = Line::from_two_points(&p1, &p2);

        Segment { p1, p2, n, v, line }
    }

    pub fn create_rectangle_domain(origin: Vec2, width: f64, height: f64) -> Vec<Segment> {
        let a = Vec2 {
            x: origin.x,
            y: origin.y,
        };
        let b = Vec2 {
            x: origin.x + width,
            y: origin.y,
        };
        let c = Vec2 {
            x: origin.x + width,
            y: origin.y + height,
        };
        let d = Vec2 {
            x: origin.x,
            y: origin.y + height,
        };

        vec![
            Segment::from_points(a, b),
            Segment::from_points(b, c),
            Segment::from_points(c, d),
            Segment::from_points(d, a),
        ]
    }

    pub fn contains_point(&self, p: &Vec2) -> bool {
        (*p - self.p1) * (*p - self.p2) <= 0.
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub p: Vec2,
    pub r: f64,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LCIntersection {
    None,
    OnePoint(Vec2),
    TwoPoint((Vec2, Vec2)),
}

impl Line {
    #[allow(dead_code)]
    pub fn distance_to_point(&self, point: &Vec2) -> f64 {
        (self.a * point.x + self.b * point.y + self.c).abs()
            / (self.a * self.a + self.b * self.b).sqrt()
    }

    pub fn from_vec_n_point(vec: &Vec2, point: &Vec2) -> Line {
        Line {
            a: vec.y,
            b: -vec.x,
            c: vec.x * point.y - vec.y * point.x,
        }
    }

    pub fn from_two_points(p1: &Vec2, p2: &Vec2) -> Line {
        Line {
            a: p1.y - p2.y,
            b: p2.x - p1.x,
            c: p1.x * p2.y - p2.x * p1.y,
        }
    }

    pub fn from_segment(seg: &Segment) -> Line {
        Line::from_two_points(&seg.p1, &seg.p2)
    }

    pub fn intersect_line(&self, other: &Line) -> Option<Vec2> {
        let det = self.a * other.b - self.b * other.a;
        if det != 0. {
            Some(Vec2 {
                x: (-1.) * (self.c * other.b - other.c * self.b) / det,
                y: (-1.) * (self.a * other.c - self.c * other.a) / det,
            })
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn intersect_segment(&self, other: &Segment) -> Option<Vec2> {
        let segment_line = Line::from_segment(&other);
        let line_intersection = self.intersect_line(&segment_line);
        match line_intersection {
            Some(p) => {
                //Check that this point located inside segment.
                if other.contains_point(&p) {
                    Some(p)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    #[allow(dead_code)]
    #[allow(non_snake_case)]
    pub fn intersect_circle(&self, other: &Circle) -> LCIntersection {
        let (A, B) = (self.a, self.b);
        let C = self.c + A * other.p.x + B * other.p.y;
        let r = other.r;

        let p0 = Vec2 {
            x: -A * C / (A * A + B * B),
            y: -B * C / (A * A + B * B),
        };

        if C * C > r * r * (A * A + B * B) {
            LCIntersection::None
        } else if (C * C - r * r * (A * A + B * B)).abs() < 1e-9 {
            LCIntersection::OnePoint(p0 + other.p)
        } else {
            let d = r * r - C * C / (A * A + B * B);
            let mult = (d / (A * A + B * B)).sqrt();
            let p1 =
                p0 + Vec2 {
                    x: B * mult,
                    y: -A * mult,
                } + other.p;
            let p2 =
                p0 + Vec2 {
                    x: -B * mult,
                    y: A * mult,
                } + other.p;

            LCIntersection::TwoPoint((p1, p2))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::compare_floats;

    #[test]
    fn test_add() {
        let a = Vec2 { x: 1.0, y: 2.0 };
        let b = Vec2 { x: -3.0, y: 5.0 };

        assert_eq!(a + b, Vec2 { x: -2.0, y: 7.0 });
    }

    #[test]
    fn test_add_assign() {
        let mut a = Vec2 { x: 1.0, y: 2.0 };
        let b = Vec2 { x: -3.0, y: 5.0 };
        a += b;

        assert_eq!(a, Vec2 { x: -2.0, y: 7.0 });
    }

    #[test]
    fn test_sub() {
        let a = Vec2 { x: 1.0, y: 2.0 };
        let b = Vec2 { x: -3.0, y: 5.0 };

        assert_eq!(a - b, Vec2 { x: 4.0, y: -3.0 });
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Vec2 { x: 1.0, y: 2.0 };
        let b = Vec2 { x: -3.0, y: 5.0 };
        a -= b;

        assert_eq!(a, Vec2 { x: 4.0, y: -3.0 });
    }

    #[test]
    fn test_mul_float() {
        let a = Vec2 { x: 1.0, y: -2.0 };
        let b = -3.0;

        assert_eq!(a * b, Vec2 { x: -3.0, y: 6.0 });
    }

    #[test]
    fn test_mul_vec2() {
        let a = Vec2 { x: 1.0, y: 2.0 };
        let b = Vec2 { x: -3.0, y: 5.0 };

        assert_eq!(a * b, 7.0);
    }

    #[test]
    fn test_len() {
        let a = Vec2 { x: 3.0, y: 4.0 };

        assert_eq!(a.len(), 5.0);
    }

    #[test]
    fn test_is_collinear() {
        let a = Vec2 { x: 1.0, y: 2.0 };
        let b = Vec2 { x: 4.0, y: 8.0 };
        let c = Vec2 { x: -1.0, y: 3.0 };

        assert_eq!(a.is_collinear(&b), true);
        assert_eq!(a.is_collinear(&c), false);
    }

    #[test]
    fn test_norm() {
        let a = Vec2 { x: 1.0, y: 2.0 };

        assert_eq!(a * a.norm(), 0.);
        compare_floats(a.norm().len(), 1.);
    }

    #[test]
    fn test_norm_with_zeroes() {
        let a = Vec2 { x: -2.0, y: 0.0 };
        assert_eq!(a.norm(), Vec2 { x: 0.0, y: 1.0 });

        let b = Vec2 { x: 0.0, y: 2.0 };
        assert_eq!(b.norm(), Vec2 { x: 1.0, y: 0.0 });
    }

    #[test]
    fn test_line_distance_to_point() {
        let line = Line {
            a: -4.0,
            b: 3.0,
            c: -35.0,
        };
        let point = Vec2 { x: -1.0, y: 2.0 };
        assert_eq!(line.distance_to_point(&point), 5.0);
    }

    #[test]
    fn test_intersect_circle() {
        let circle = Circle {
            p: Vec2 { x: 4.0, y: 1.0 },
            r: 2.0,
        };
        let line = Line {
            a: 1.0,
            b: -1.0,
            c: -1.0,
        };
        let expected = LCIntersection::TwoPoint((Vec2 { x: 2.0, y: 1.0 }, Vec2 { x: 4.0, y: 3.0 }));

        assert_eq!(line.intersect_circle(&circle), expected);

        let circle = Circle {
            p: Vec2 { x: -2.0, y: 1.0 },
            r: 2.0,
        };
        let line = Line {
            a: 3.0,
            b: -1.0,
            c: 0.0,
        };
        let expected = LCIntersection::None;

        assert_eq!(line.intersect_circle(&circle), expected);

        let circle = Circle {
            p: Vec2 { x: 1.0, y: -1.0 },
            r: f64::sqrt(5.0),
        };
        let line = Line {
            a: 1.0,
            b: 2.0,
            c: -4.0,
        };
        let expected = LCIntersection::OnePoint(Vec2 { x: 2.0, y: 1.0 });

        assert_eq!(line.intersect_circle(&circle), expected);
    }
}
