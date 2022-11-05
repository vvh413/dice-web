use actix_web::middleware::Logger;
use actix_web::HttpResponse;
use actix_web::{get, web, App, HttpServer, Result};
use env_logger::Env;
use rand::thread_rng;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Roll {
    x: u64,
    y: u64,
}

#[derive(Serialize)]
struct Dice {
    values: Vec<u64>,
    total: u64,
    min: u64,
    max: u64,
}

impl Dice {
    fn roll(x: u64, y: u64) -> Self {
        let values: Vec<u64> = (0..x).map(|_| thread_rng().gen_range(1..=y)).collect();
        Dice {
            values: values.clone(),
            total: values.iter().sum(),
            min: *values.iter().min().unwrap(),
            max: *values.iter().max().unwrap(),
        }
    }
}

#[get("/{x}d{y}")]
async fn dice(roll: web::Path<Roll>) -> Result<web::Json<Dice>> {
    Ok(web::Json(Dice::roll(roll.x, roll.y)))
}

#[get("/")]
async fn default() -> Result<web::Json<Dice>> {
    Ok(web::Json(Dice::roll(1, 100)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .service(dice)
            .service(default)
            .route("/health", web::get().to(HttpResponse::Ok))
            .default_service(web::to(|| async {
                HttpResponse::NotFound().body("Not Found")
            }))
            .wrap(Logger::default().exclude("/health"))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
