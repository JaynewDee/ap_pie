pub mod routes {
    use crate::parser::{all_game_data, read_game_sales};
    use actix_web::http::StatusCode;
    use actix_web::{get, web::Json};
    use actix_web::{App, HttpResponse, HttpServer, Responder};
    use serde::Serialize;

    #[derive(Serialize, Debug)]
    struct Response {
        text: String,
    }

    #[get("/")]
    pub async fn index() -> Result<impl Responder, Box<dyn std::error::Error>> {
        let json_res = Response {
            text: String::from("Hello, index!"),
        };

        Ok(Json(json_res))
    }

    #[get("/games")]
    pub async fn game_sales() -> Result<impl Responder, Box<dyn std::error::Error>> {
        let all_records = all_game_data("./input/vgsales.csv")?;

        Ok(Json(all_records))
    }
}

pub mod connection {
    use super::routes::{game_sales, index};
    use actix_web::{web, App, HttpResponse, HttpServer, Responder};

    pub struct Server<'a> {
        addr: &'a str,
        port: u16,
    }

    impl<'a> Server<'a> {
        pub fn new() -> Self {
            Self {
                addr: "127.0.0.1",
                port: 8080,
            }
        }

        fn address(&self) -> &str {
            self.addr
        }

        fn port(&self) -> u16 {
            self.port
        }
    }

    #[tokio::main]
    pub async fn launch(server: Server) -> std::io::Result<()> {
        let (addr, port) = (server.address(), server.port());

        println!("Server listening @ http://{}:{} ", &addr, &port);

        HttpServer::new(|| App::new().service(index).service(game_sales))
            .bind((addr, port))?
            .run()
            .await
    }
}
