-- Add migration script here
CREATE TABLE user_guesses (
    id SERIAL PRIMARY KEY,
    game_id uuid NOT NULL REFERENCES games(game_id),
    created_at timestamptz NOT NULL,
    word text NOT NULL
)