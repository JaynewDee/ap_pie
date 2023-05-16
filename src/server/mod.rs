pub mod routes {
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
}

pub mod connection {
    use super::routes::index;
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
        println!(
            "Server listening @ http://{}:{} ",
            server.address(),
            server.port()
        );

        HttpServer::new(|| App::new().service(index))
            .bind((server.address(), server.port()))?
            .run()
            .await
    }
}
