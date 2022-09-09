use crate::helpers::spawn_app;

#[tokio::test]
async fn returns_200_and_game_id_when_correct_data() {
    #[derive(serde::Serialize)]
    struct Payload {
        word_len: u8,
    }

    #[derive(serde::Deserialize)]
    #[allow(dead_code)]
    struct Response {
        game_id: String,
    }

    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/init_game", &app.address))
        .json(&Payload { word_len: 4 })
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert!(response.json::<Response>().await.is_ok());
}

#[tokio::test]
async fn returns_400_with_incorrect_payload() {
    #[derive(serde::Serialize)]
    struct Payload {
        // a purposely made typo
        word_lenn: u8,
    }

    #[derive(serde::Deserialize)]
    #[allow(dead_code)]
    struct Response {
        game_id: String,
    }

    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .post(&format!("{}/init_game", &app.address))
        .json(&Payload { word_lenn: 4 })
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(response.status().as_u16(), 400);
}
