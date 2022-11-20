use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpServer};
use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error};
use env_logger::Env;
use hex::FromHexError;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};

const MAX_DICE: u32 = 2 << 25;

#[derive(Deserialize, Display)]
#[display(fmt = "{}d{}", x, y)]
struct Roll {
    x: u32,
    y: u64,
}

#[derive(Deserialize)]
struct SeededRoll {
    x: u32,
    y: u64,
    seed: String,
}

#[derive(Debug, Display, Error)]
#[display(fmt = "roll error: {}", message)]
struct RollError {
    message: &'static str,
}
impl ResponseError for RollError {}
impl From<FromHexError> for RollError {
    fn from(_: FromHexError) -> Self {
        RollError {
            message: "invalid hex value for seed",
        }
    }
}

#[derive(Serialize)]
struct Dice {
    values: Vec<u64>,
    total: u64,
    min: u64,
    max: u64,
}

impl Dice {
    fn roll(x: u32, y: u64, seed: [u8; 32]) -> Result<Self, RollError> {
        if x > MAX_DICE {
            return Err(RollError {
                message: "too many dice",
            });
        };
        let mut rng = ChaCha20Rng::from_seed(seed);
        let values: Vec<u64> = (0..x).map(|_| rng.gen_range(1..=y)).collect();
        Ok(Dice {
            values: values.clone(),
            total: values.iter().sum(),
            min: *values.iter().min().unwrap(),
            max: *values.iter().max().unwrap(),
        })
    }
}

#[get("/{x}d{y}:{seed}")]
async fn seeded_dice(roll: web::Path<SeededRoll>) -> Result<web::Json<Dice>, RollError> {
    let mut seed = [0u8; 32];
    hex::decode_to_slice(roll.seed.clone(), &mut seed)?;
    Ok(web::Json(Dice::roll(roll.x, roll.y, seed)?))
}

#[get("/{x}d{y}")]
async fn dice(roll: web::Path<Roll>) -> HttpResponse {
    let seed = hex::encode(ChaCha20Rng::from_entropy().get_seed());
    HttpResponse::Found()
        .append_header(("location", format!("/{}:{}", roll, seed)))
        .finish()
}

#[get("/")]
async fn default() -> HttpResponse {
    let seed = hex::encode(ChaCha20Rng::from_entropy().get_seed());
    HttpResponse::Found()
        .append_header(("location", format!("/1d100:{}", seed)))
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(seeded_dice)
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
