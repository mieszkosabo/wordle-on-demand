use wordle_on_demand::{configuration::get_configuration, startup::Application};

pub struct TestApp {
    pub address: String,
    pub port: u16,
}

pub async fn spawn_app() -> TestApp {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration.clone())
        .await
        .expect("Failed to build the app");
    let port = application.port();
    let _ = tokio::spawn(application.run_until_stopped());

    TestApp {
        address: format!("http://localhost:{}", port),
        port,
    }
}
