//! A simple API that proxies all requests to the OSRS hiscore API, and
//! translates all responses to structured JSON.

#![deny(clippy::all)]
#![feature(backtrace)]

mod cors;
mod error;
mod hiscore;

use crate::{cors::Cors, error::ApiResult, hiscore::HiscorePlayer};
use rocket::{routes, serde::json::Json};

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Cors)
        .mount("/", routes![get_hiscore])
}

/// Load data for a player from the hiscores
#[rocket::get("/hiscore/<player_name>")]
async fn get_hiscore(player_name: &str) -> ApiResult<Json<HiscorePlayer>> {
    let data = HiscorePlayer::load(player_name).await?;
    Ok(Json(data))
}
