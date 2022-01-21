use hmac::{Hmac, Mac};
use sha2::Sha256;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub type HmacSha256 = Hmac<Sha256>;

pub struct GameParams {
    // Player's particle index
    pub p_particle: usize,
    pub player_uuid: String,
    pub player_name: String,
    pub game_end_cb: js_sys::Function,

    pub game_started_tick: f64,
    pub game_ended: bool,
}

impl GameParams {
    pub fn new(
        p_particle: usize,
        player_uuid: String,
        player_name: String,
        game_started_tick: f64,
        game_end_cb: js_sys::Function,
    ) -> GameParams {
        GameParams {
            p_particle,
            player_uuid,
            player_name,
            game_end_cb,
            game_started_tick,
            game_ended: false,
        }
    }

    pub fn get_score(&self, tick: f64) -> u32 {
        ((tick - self.game_started_tick) * 10.).round() as u32
    }

    pub fn game_over(&mut self, tick: f64, ticks_per_sec: u32) {
        // Event must fire only once per attempt
        if !self.game_ended {
            let score = self.get_score(tick);

            let signed_result = SignedGameResult::from_game_result(
                GameResult {
                    player_uuid: self.player_uuid.clone(),
                    player_name: self.player_name.clone(),
                    score,
                    ticks_per_sec,
                },
                &crate::SECRET_KEY,
            );

            let this = JsValue::from(JsValue::null());
            let result = JsValue::from_serde(&signed_result).unwrap();
            self.game_end_cb.call1(&this, &result).unwrap();
            self.game_ended = true;
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameResult {
    pub player_uuid: String,
    pub player_name: String,
    pub score: u32,
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
            HmacSha256::new_from_slice(&secret).expect("Invalid secret, unable to build hmac.");
        mac.update(&self.player_name.as_bytes());
        mac.update(&self.player_uuid.as_bytes());
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
        mac.verify_slice(&self.hex_digest).is_ok()
    }
}
