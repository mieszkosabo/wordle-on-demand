use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;

use crate::domain::ServerGameState;

#[derive(serde::Deserialize)]
pub struct Input {
    word_len: u8,
}

#[derive(serde::Serialize)]
pub struct Response {
    game_id: String,
}

pub async fn init_game(input: web::Json<Input>, db_ool: web::Data<PgPool>) -> HttpResponse {
    let server_game_state = match ServerGameState::new(input.word_len) {
        Ok(sgs) => sgs,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let game_id = server_game_state.game_id;

    match sqlx::query!(
        r#"
        INSERT INTO games (game_id, created_at, word_len, word)
        VALUES ($1, $2, $3, $4)
        "#,
        game_id.as_ref(),
        Utc::now(),
        server_game_state.word_len as i32,
        server_game_state.word
    )
    .execute(db_ool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().json(Response {
            game_id: game_id.as_ref().to_string(),
        }),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
