// src/main.rs

// dependencies
use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use std::net::TcpListener;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

struct Application {
    port: u16,
    server: Server,
}

impl Application {
    async fn build() -> Result<Self, anyhow::Error> {
        let address = format!("{}:{}", "127.0.0.1", 8080);
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr()?.port();
        let server = run(listener).await?;
        Ok(Self { port, server })
    }

    #[allow(dead_code)]
    fn port(&self) -> u16 {
        self.port
    }

    async fn run_until_stopped(self) -> Result<(), std::io::Error> {
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

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let application = Application::build().await?;
    application.run_until_stopped().await?;

    Ok(())
}
