-- Add migration script here
CREATE TABLE games (
    game_id uuid NOT NULL,
    PRIMARY KEY (game_id),
    created_at timestamptz NOT NULL,
    word_len integer NOT NULL,
    word text NOT NULL
)