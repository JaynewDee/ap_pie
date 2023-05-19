mod routing;

pub mod connection {
    use crate::server::routing::{game_sales, index, wind_production, world_pop};
    use actix_web::{App, HttpServer};

    pub struct Config<'a> {
        addr: &'a str,
        port: u16,
    }

    impl<'a> Config<'a> {
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
    pub async fn launch(config: Config) -> std::io::Result<()> {
        let (addr, port) = (config.address(), config.port());

        println!("Server listening @ http://{}:{} ", &addr, &port);

        HttpServer::new(|| {
            App::new()
                .service(index)
                .service(world_pop)
                .service(game_sales)
                .service(wind_production)
        })
        .bind((addr, port))?
        .run()
        .await
    }
}
