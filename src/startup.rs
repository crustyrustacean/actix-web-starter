// src/startup.rs

// dependencies
use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use crate::configuration::Settings;
use crate::routes::health_check;
use std::net::TcpListener;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let address = format!("{}:{}", configuration.application.host, configuration.application.port);
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr()?.port();
        let server = run(listener).await?;
        Ok(Self { port, server })
    }

    #[allow(dead_code)]
    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn run(listener: TcpListener) -> Result<Server, anyhow::Error> {
    let server =
        HttpServer::new(move || App::new().route("/health_check", web::get().to(health_check)))
            .listen(listener)?
            .run();

    Ok(server)
}