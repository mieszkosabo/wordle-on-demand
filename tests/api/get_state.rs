use uuid::Uuid;

use crate::helpers::spawn_app;

#[tokio::test]
async fn gets_correct_state_after_few_guesses() {
    let app = spawn_app().await;
    let game_id = app.init_game(5).await;

    let _ = app.make_guess(game_id, "awans".into()).await.unwrap();
    let gs = app.make_guess(game_id, "awizo".into()).await.unwrap();

    let gs2 = app.get_state(game_id).await.expect("Could not get state!");

    assert_eq!(gs, gs2);
}

#[tokio::test]
async fn gets_correct_state_before_making_guesses() {
    let app = spawn_app().await;
    let game_id = app.init_game(5).await;

    let gs = app.get_state(game_id).await.expect("Could not get state!");

    assert_eq!(gs.word_len, 5);
    assert_eq!(gs.inputs.len(), 0);
}

#[tokio::test]
async fn gets_400_when_game_id_is_incorrect() {
    let app = spawn_app().await;
    let _ = app.init_game(5).await;

    let err_code = app.get_state(Uuid::new_v4()).await.err().unwrap();

    assert_eq!(err_code, 400);
}
