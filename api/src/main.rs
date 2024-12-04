mod data;

use actix_cors::Cors;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use data::{AppState, CheckGameBoardOptions, GenerateRandom, InputData};
use game_of_life::matrix;
use std::{env, fs};

const HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let host = env::var("HOST").unwrap_or(HOST.to_string());
    let port = match env::var("PORT").unwrap_or(DEFAULT_PORT.to_string()).parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!(
                "Failed to parse PORT. Falling back to default: {}",
                DEFAULT_PORT
            );
            DEFAULT_PORT
        }
    };

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(AppState {
                html: match fs::read_to_string("index.html") {
                    Ok(content) => content,
                    Err(_) => {
                        panic!("Could not read html from index.html")
                    }
                },
            }))
            .route("/", web::get().to(index))
            .route("/generate-random", web::get().to(generate_random))
            .service(update_board)
    })
    .bind((host, port))?
    .run()
    .await
}

#[post("/check")]
async fn update_board(
    query: web::Query<CheckGameBoardOptions>,
    input: web::Json<InputData>,
) -> impl Responder {
    let input = input.into_inner();
    let mut matrix = matrix::Matrix::new(input.0);

    game_of_life::convert(&mut matrix, query.use_toroidal.unwrap_or(false));

    web::Json(matrix.data)
}

async fn generate_random(query: web::Query<GenerateRandom>) -> impl Responder {
    let result = matrix::Matrix::generate_random_binary(query.width, query.height);

    web::Json(result.data)
}

async fn index(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(data.html.clone())
}
