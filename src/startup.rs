use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use anyhow::Result;

use crate::{
    configuration::Settings,
    routes::{health_check::health_check, init_game::init_game},
};

pub struct Application {
    port: u16,
    server: Server,
}

async fn run(listener: TcpListener) -> Result<Server> {
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/init_game", web::post().to(init_game))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

impl Application {
    pub async fn build(config: Settings) -> Result<Self> {
        let address = format!("{}:{}", config.application.host, config.application.port);
        println!("{}", &address);
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}
