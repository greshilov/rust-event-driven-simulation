use super::geom::{Circle, Vec2};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub struct RGBA {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[wasm_bindgen]
impl RGBA {
    pub fn new(red: u8, green: u8, blue: u8, _alpha: Option<f64>) -> RGBA {
        let alpha = _alpha.unwrap_or(1.);
        let alpha = if alpha < 0. || alpha >= 1. {
            255
        } else {
            (255. * alpha).round() as u8
        };

        RGBA {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl RGBA {
    pub fn as_css_hex(&self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}{:02X}",
            self.red, self.green, self.blue, self.alpha
        )
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Particle {
    pub pos: Vec2,
    pub v: Vec2,
    pub m: f64,
    pub r: f64,
    pub collisions_count: u64,
    pub color: Option<RGBA>,
}

#[wasm_bindgen]
impl Particle {
    pub fn new(
        px: f64,
        py: f64,
        vx: f64,
        vy: f64,
        m: f64,
        r: f64,
        color: Option<RGBA>,
    ) -> Particle {
        Particle {
            pos: Vec2 { x: px, y: py },
            v: Vec2 { x: vx, y: vy },
            m,
            r,
            collisions_count: 0,
            color,
        }
    }
}

impl Particle {
    #[inline]
    pub fn mv(&mut self, dt: f64) {
        self.pos += self.v * dt;
    }

    pub fn circle(&self) -> Circle {
        Circle {
            p: self.pos,
            r: self.r,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_rgba() {
        let rgba = RGBA {
            red: 255,
            green: 0,
            blue: 0,
            alpha: 0,
        };
        assert_eq!(rgba.as_css_hex(), "#FF000000");

        let rgba = RGBA {
            red: 128,
            green: 64,
            blue: 192,
            alpha: 192,
        };
        assert_eq!(rgba.as_css_hex(), "#8040C0C0");
    }
}
