use crate::helpers::spawn_app;

#[tokio::test]
async fn returns_correct_game_state_after_guess() {
    let app = spawn_app().await;
    let game_id = app.init_game(5).await;
    let game_state = app.make_guess(game_id, "awizo".into()).await.unwrap();

    assert_eq!(game_state.game_id, game_id);
    assert_eq!(game_state.word_len, 5);
    assert_eq!(game_state.inputs.len(), 1);
}

#[tokio::test]
async fn guesses_are_persisted() {
    let app = spawn_app().await;
    let game_id = app.init_game(5).await;
    let _ = app.make_guess(game_id, "awizo".into()).await.unwrap();
    let game_state = app.make_guess(game_id, "awans".into()).await.unwrap();

    assert_eq!(game_state.game_id, game_id);
    assert_eq!(game_state.word_len, 5);
    assert_eq!(game_state.inputs.len(), 2); // here is the important part
}

#[tokio::test]
async fn returns_400_when_guess_len_is_incorrect() {
    let app = spawn_app().await;
    let game_id = app.init_game(5).await;
    let err_code = app.make_guess(game_id, "ate".into()).await.err().unwrap();

    assert_eq!(err_code, 400);
}

#[tokio::test]
async fn returns_400_when_game_id_does_not_exist() {
    let app = spawn_app().await;
    let _ = app.init_game(5).await;
    let err_code = app
        .make_guess(uuid::Uuid::new_v4(), "atena".into())
        .await
        .err()
        .unwrap();

    assert_eq!(err_code, 400);
}

#[tokio::test]
async fn returns_400_when_word_is_not_allowed_in_games() {
    let app = spawn_app().await;
    let _ = app.init_game(5).await;
    let err_code = app
        .make_guess(uuid::Uuid::new_v4(), "abbbb".into())
        .await
        .err()
        .unwrap();

    assert_eq!(err_code, 400);
}
