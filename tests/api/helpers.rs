use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use wordle_on_demand::{
    configuration::{get_configuration, DatabaseSettings},
    domain::UserGameState,
    startup::{get_connection_pool, Application},
};

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub db_pool: PgPool,
}

impl TestApp {
    pub async fn init_game(&self, word_len: u8) -> Uuid {
        #[derive(serde::Serialize)]
        struct Payload {
            word_len: u8,
        }
        #[derive(serde::Deserialize)]
        #[allow(dead_code)]
        struct Response {
            game_id: uuid::Uuid,
        }
        let client = reqwest::Client::new();

        let response = client
            .post(&format!("{}/init_game", &self.address))
            .json(&Payload { word_len })
            .send()
            .await
            .expect("Failed to execute request.");

        response
            .json::<Response>()
            .await
            .expect("Failed to parse response")
            .game_id
    }

    pub async fn make_guess(&self, game_id: Uuid, guess: String) -> Result<UserGameState, u16> {
        #[derive(serde::Serialize)]
        struct Payload {
            game_id: Uuid,
            guess: String,
        }
        let client = reqwest::Client::new();

        let res = client
            .post(&format!("{}/make_guess", &self.address))
            .json(&Payload { game_id, guess })
            .send()
            .await
            .expect("Failed to execute request.");

        if res.status().is_success() {
            return Ok(res.json::<UserGameState>().await.unwrap());
        } else {
            return Err(res.status().as_u16());
        }
    }
}

pub async fn spawn_app() -> TestApp {
    let configuration = {
        let mut c = get_configuration().expect("Failed to read configuration.");
        // Use different database for each test
        c.database.database_name = Uuid::new_v4().to_string();
        // use random os port
        c.application.port = 0;
        c
    };

    configure_database(&configuration.database).await;
    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build the app");
    let port = application.port();
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address: format!("http://localhost:{}", port),
        port,
        db_pool: get_connection_pool(&configuration.database),
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(&*format!(r#"CREATE DATABASE "{}";"#, config.database_name))
        .await
        .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
