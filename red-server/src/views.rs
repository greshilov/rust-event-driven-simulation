use diesel::prelude::*;

use crate::models::*;
use crate::schema::*;

use rocket_contrib::json::{Json, JsonValue};

use red_simulation::simulation::SignedGameResult;

#[get("/api/top")]
pub fn top_scores() -> Json<Vec<Score>> {
    let scores: Vec<Score> = scores::table
        .select(scores::all_columns)
        .load::<Score>(&crate::establish_connection())
        .expect("Whoops, like this went bananas!");

    Json(scores)
}

#[post("/api/submit", format = "application/json", data = "<sgr_json>")]
pub fn submit_scores(sgr_json: Json<SignedGameResult>) -> JsonValue {
    let sgr = sgr_json.into_inner();

    if !sgr.verify(&dotenv::var("SECRET_KEY").unwrap().as_bytes()) {
        return json!({ "status": "error", "msg": "Invalid hex_digest, you hacker!"});
    }

    let insert = diesel::insert_into(scores::table)
        .values(NewScore {
            name: &sgr.game_result.player_name,
            score: sgr.game_result.score as i64,
        })
        .execute(&crate::establish_connection());

    match insert {
        Ok(_) => json!({ "status": "ok" }),
        Err(err_msg) => json!({ "status": "error", "msg": format!(
            "Database error: {}",
            err_msg
        )}),
    }
}
