use actix_web::{get, post, web, HttpResponse, Responder};
use game_of_life::matrix;

use crate::models::{AppState, CheckGameBoardOptions, CheckMatrixRequest, GenerateRandom};

pub fn get_router() -> actix_web::Scope {
    web::scope("/api")
        .service(generate_random)
        .service(update_board)
        .service(get_presets)
        .service(health_check)
}

#[post("/check")]
async fn update_board(
    query: web::Query<CheckGameBoardOptions>,
    input: web::Json<CheckMatrixRequest>,
) -> impl Responder {
    let input = input.into_inner();
    let mut matrix = matrix::Matrix::new(input.0);

    game_of_life::convert(&mut matrix, query.use_toroidal.unwrap_or(false));

    web::Json(matrix.data)
}

#[get("/get-presets")]
async fn get_presets(data: web::Data<AppState>) -> impl Responder {
    web::Json(data.preset_matrixes.clone())
}

#[get("/generate-random")]
async fn generate_random(query: web::Query<GenerateRandom>) -> impl Responder {
    let result = matrix::Matrix::generate_random_binary(query.width, query.height);

    web::Json(result.data)
}

#[get("/healthz")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}
