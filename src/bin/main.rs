use wordle_on_demand::{configuration::get_configuration, startup::Application};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration)
        .await
        .expect("Failed to build the application.");

    application.run_until_stopped().await
}
