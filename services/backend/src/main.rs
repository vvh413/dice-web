use actix_cors::Cors;
use actix_files::{Files, NamedFile};
use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, ResponseError};
use derive_more::{Display, Error};
use env_logger::Env;
use hex::FromHexError;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use serde::{Deserialize, Serialize};

const MAX_DICE: u32 = 2 << 25;
const API_SCOPE: &str = "/api";
const DEFAULT_FRONTEND_PATH: &str = "./frontend";

#[derive(Deserialize, Display)]
#[display("{}d{} ({:?})", x, y, seed)]
struct Roll {
    x: u32,
    y: u64,
    seed: Option<String>,
}

#[derive(Debug, Display, Error)]
#[display("roll error: {}", message)]
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
    seed: String,
}

impl Dice {
    fn roll(x: u32, y: u64, seed: [u8; 32]) -> Result<Self, RollError> {
        if x > MAX_DICE {
            return Err(RollError {
                message: "too many dice",
            });
        };
        let mut rng = ChaCha20Rng::from_seed(seed);
        let values: Vec<u64> = (0..x).map(|_| rng.random_range(1..=y)).collect();
        Ok(Dice {
            values: values.clone(),
            total: values.iter().sum(),
            min: *values.iter().min().unwrap(),
            max: *values.iter().max().unwrap(),
            seed: hex::encode(seed),
        })
    }
}

#[get("/{x}d{y}/{seed}")]
async fn seeded_dice(roll: web::Path<Roll>) -> Result<web::Json<Dice>, RollError> {
    let mut seed = [0u8; 32];
    hex::decode_to_slice(roll.seed.clone().unwrap(), &mut seed)?;
    Ok(web::Json(Dice::roll(roll.x, roll.y, seed)?))
}

#[get("/{x}d{y}")]
async fn dice(roll: web::Path<Roll>) -> Result<web::Json<Dice>, RollError> {
    let seed = ChaCha20Rng::from_os_rng().get_seed();
    Ok(web::Json(Dice::roll(roll.x, roll.y, seed)?))
}

#[get("")]
async fn default() -> Result<web::Json<Dice>, RollError> {
    let seed = ChaCha20Rng::from_os_rng().get_seed();
    Ok(web::Json(Dice::roll(1, 100, seed)?))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        let frontend_path =
            std::env::var("FRONTEND_PATH").unwrap_or(DEFAULT_FRONTEND_PATH.to_string());
        App::new()
            .wrap(Cors::permissive())
            .route("/health", web::get().to(HttpResponse::Ok))
            .service(
                web::scope(API_SCOPE)
                    .service(seeded_dice)
                    .service(dice)
                    .service(default)
                    .default_service(web::to(|| async {
                        HttpResponse::NotFound().body("Not Found")
                    })),
            )
            .service(
                Files::new("/", frontend_path.clone())
                    .index_file("index.html")
                    .default_handler(
                        NamedFile::open(format!("{}/index.html", frontend_path)).unwrap(),
                    ),
            )
            .wrap(Logger::default().exclude("/health"))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
