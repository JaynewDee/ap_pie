pub mod routes {
    use crate::parser::{all_game_data, read_game_sales, GameRecord};
    use actix_web::http::StatusCode;
    use actix_web::{get, web::Json, web::Query};
    use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Serialize, Debug)]
    struct IndexResponse {
        text: String,
    }

    #[get("/")]
    pub async fn index() -> Result<impl Responder, Box<dyn std::error::Error>> {
        let json_res = IndexResponse {
            text: String::from(
                "Ping! Read the documentation for available endpoints and parameter options.",
            ),
        };

        Ok(Json(json_res))
    }

    #[get("/games")]
    pub async fn game_sales(
        req: HttpRequest,
    ) -> Result<impl Responder, Box<dyn std::error::Error>> {
        let (key, value) = query_params(req);

        let all_records = all_game_data("./input/vgsales.csv")?;

        let filtered = filter_by(&key, value, all_records);

        Ok(Json(filtered))
    }

    fn query_params(req: HttpRequest) -> (String, String) {
        let params: Query<HashMap<String, String>> = Query::from_query(req.query_string()).unwrap();

        println!("Query Params from request to /games ::: {:#?}", &params);

        if params.len() == 0 {
            (String::new(), String::new())
        } else {
            let param_key: &String = params.keys().collect::<Vec<&String>>()[0];
            let param_value: &String = params.values().collect::<Vec<&String>>()[0];
            (param_key.to_string(), param_value.to_string())
        }
    }

    fn filter_by(param: &str, value: String, records: Vec<GameRecord>) -> Vec<GameRecord> {
        match param {
            "year" => records
                .into_iter()
                .filter(|rec| {
                    rec.year != None && rec.year.unwrap() == value.parse::<u16>().unwrap()
                })
                .collect(),
            "publisher" => records
                .into_iter()
                .filter(|rec| rec.publisher != None && rec.publisher.clone().unwrap() == value)
                .collect(),
            "platform" => records
                .into_iter()
                .filter(|rec| rec.platform != None && rec.platform.clone().unwrap() == value)
                .collect(),
            "genre" => records
                .into_iter()
                .filter(|rec| rec.genre != None && rec.genre.clone().unwrap() == value)
                .collect(),
            _ => records,
        }
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
