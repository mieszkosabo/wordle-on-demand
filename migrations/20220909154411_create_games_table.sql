-- Add migration script here
CREATE TABLE games (
    game_id uuid NOT NULL,
    PRIMARY KEY (game_id),
    created_at timestamptz,
    word_len integer,
    word text
)