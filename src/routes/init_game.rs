use actix_web::{web, HttpResponse};

use crate::domain::{ServerGameState, UserGameState};

#[derive(serde::Deserialize)]
pub struct Input {
    word_len: u8,
}

#[derive(serde::Serialize)]
pub struct Response {
    game_id: String,
}

pub async fn init_game(input: web::Json<Input>) -> HttpResponse {
    let server_game_state = match ServerGameState::new(input.word_len) {
        Ok(sgs) => sgs,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let user_state: UserGameState = server_game_state.into();

    HttpResponse::Ok().json(Response {
        game_id: user_state.game_id.as_ref().to_string(),
    })
}
