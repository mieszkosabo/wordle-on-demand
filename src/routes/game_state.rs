use actix_web::{web, HttpResponse};
use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{ServerGameState, UserGameState};

#[derive(serde::Deserialize)]
pub struct Input {
    game_id: Uuid,
}

pub async fn game_state(input: web::Json<Input>, db_pool: web::Data<PgPool>) -> HttpResponse {
    match handle_get_state(input.game_id, db_pool.get_ref()).await {
        Ok(ugs) => HttpResponse::Ok().json(ugs),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

async fn handle_get_state(game_id: Uuid, db_pool: &PgPool) -> Result<UserGameState> {
    let game = sqlx::query!(
        r#"
        SELECT word, word_len FROM games
        WHERE game_id = $1
        "#,
        game_id
    )
    .fetch_one(db_pool)
    .await?;

    let all_guesses = sqlx::query!(
        r#"
        SELECT word FROM user_guesses
        WHERE game_id = $1
        ORDER BY created_at
        "#,
        game_id
    )
    .fetch_all(db_pool)
    .await?;

    let sgs = ServerGameState {
        game_id,
        word: game.word,
        word_len: game.word_len as u8,
        user_choices: all_guesses.into_iter().map(|r| r.word).collect(),
    };

    Ok(UserGameState::from(sgs))
}
