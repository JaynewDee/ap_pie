use crate::parser::{
    games::game_records::GameRecord, wind_power::wind_production::WindRecord,
    world_pop::population::PopRecord,
};
use actix_web::{get, web::Json, web::Query};
use actix_web::{HttpRequest, Responder};
use serde::Serialize;
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
pub async fn game_sales(req: HttpRequest) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let (key, value) = query_params(req);
    let all_records = GameRecord::all_game_data("./input/vgsales.csv")?;

    let filtered = GameRecord::filter_by(&key, value, all_records);

    Ok(Json(filtered))
}

#[get("/wind")]
pub async fn wind_production(_: HttpRequest) -> Result<impl Responder, Box<dyn std::error::Error>> {
    Ok(Json(WindRecord::read_wind_power(
        "./input/wind-power.csv",
    )?))
}

#[get("/population")] 
pub async fn world_pop(_: HttpRequest) -> Result<impl Responder, Box<dyn std::error::Error>> {
    Ok(Json(PopRecord::all_population_data(
        "./input/world-pop.csv",
    )?))
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
