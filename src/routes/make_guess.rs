use actix_web::{web, HttpResponse};
use anyhow::{anyhow, Result};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::{ServerGameState, UserGameState},
    words::check_if_valid_word,
};

#[derive(serde::Deserialize)]
pub struct Input {
    game_id: Uuid,
    guess: String,
}

pub async fn make_guess(input: web::Json<Input>, db_pool: web::Data<PgPool>) -> HttpResponse {
    match handle_guess(input.game_id, &input.guess, db_pool.get_ref()).await {
        Ok(ugs) => HttpResponse::Ok().json(ugs),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

async fn handle_guess(game_id: Uuid, guess: &String, db_pool: &PgPool) -> Result<UserGameState> {
    let game = sqlx::query!(
        r#"
        SELECT word, word_len FROM games
        WHERE game_id = $1
        "#,
        game_id
    )
    .fetch_one(db_pool)
    .await?;

    if game.word_len != (guess.len() as i32) {
        // return generic error for now
        return Err(anyhow!("Guess has invalid length."));
    }

    check_if_valid_word(guess)?;

    sqlx::query!(
        r#"
        INSERT INTO user_guesses (game_id, created_at, word)
        VALUES ($1, $2, $3)
        "#,
        game_id,
        Utc::now(),
        guess
    )
    .execute(db_pool)
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
