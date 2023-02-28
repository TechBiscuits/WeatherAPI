use actix_web::{web, App, HttpServer, Responder};
use reqwest::Client;

use serde_json::{json};
use serde::Deserialize;

use dotenv::dotenv;
use std::env;

// get the api key from the .env file
fn get_api_key() -> String {
    dotenv().ok();
    env::var("WEATHERAPICOM_KEY").expect("API_KEY must be set")
}

async fn index() -> impl Responder {
    web::Json(json!({
        "status": true,
    }))
}


#[derive(Deserialize)]
pub struct WeatherLocationQuery { 
    q: String, 
    limit: Option<i32> 
}
#[derive(Deserialize)]
pub struct WeatherDataQuery { 
    id: String, 
    forecast: Option<String>
}

async fn get_weather_locations(query: web::Query<WeatherLocationQuery>) -> impl Responder {
    let api_key: String = get_api_key();

    // make the request to the weatherapi.com api
    let client = Client::new();
    let res = client.get("https://api.weatherapi.com/v1/search.json")
        .query(&[("key", api_key), ("q", query.q.to_string()), ("limit", query.limit.unwrap_or(10).to_string())])
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    
    // parse json bbefore returning
    let parsed: serde_json::Value = serde_json::from_str(&res).unwrap();
    web::Json(json!({
        "status": true,
        "query": query.q,
        "limit": query.limit.unwrap_or(10),
        "data": parsed
    }))
}



async fn get_weather_data_by_id(query: web::Query<WeatherDataQuery>) -> impl Responder {
    let api_key: String = get_api_key();
    if query.forecast.is_some() {
        web::Json(json!({
            "status": false,
            "error": {
                "code": 400,
                "message": "Forecast is not yet supported"
            },
            "query": {
                "id": query.id,
                "forecast": query.forecast
            },
            "data": []
        }))

    } else {
        let client: Client = reqwest::Client::new();
        let res = client.get("https://api.weatherapi.com/v1/current.json")
            .query(&[("key", api_key), ("q", query.id.to_string())])
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        // error catch 
        if res.contains("error") {
            let parsed: serde_json::Value = serde_json::from_str(&res).unwrap();
            return web::Json(json!({
                "status": false,
                "error": {
                    "code": parsed["error"]["code"],
                    "message": parsed["error"]["message"]
                },
                "query": {
                    "id": query.id,
                    "forecast": query.forecast
                },
                "data": []
            }))
        }

        let parsed: serde_json::Value = serde_json::from_str(&res).unwrap();
        web::Json(json!({
            "status": true,
            "error": {
                "code": 200,
                "message": "OK"
            },
            "query": {
                "id": query.id,
                "forecast": query.forecast
            },
            "data": parsed
        }))



    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/api/searchLocations", web::get().to(get_weather_locations))
            .route("/api/getWeatherData", web::get().to(get_weather_data_by_id))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

