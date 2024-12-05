mod api_routes;
mod models;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use models::{AppState, GolPreset};
use std::{env, fs, path::Path};

const HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();

    let host = env::var("HOST").unwrap_or(HOST.to_string());
    let port = match env::var("DEFAULT_PORT")
        .unwrap_or(DEFAULT_PORT.to_string())
        .parse()
    {
        Ok(value) => value,
        Err(_) => {
            eprintln!(
                "Failed to parse PORT. Falling back to default: {}",
                DEFAULT_PORT
            );
            DEFAULT_PORT
        }
    };
    let html_file_fp = env::var("HTML_FILE_FP").unwrap_or("./api/templates/index.html".to_string());
    let static_dir_fp = env::var("STATIC_DIR_FP").unwrap_or("./api/static".to_string());
    let presets_file_fp = env::var("PRESETS_FILE_FP").unwrap_or("./presets.json".to_string());

    let state = AppState {
        html: match fs::read_to_string(&html_file_fp) {
            Ok(content) => content,
            Err(_) => {
                panic!("Could not read html from {}", html_file_fp)
            }
        },
        preset_matrixes: load_json(presets_file_fp.to_string()),
    };

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(Cors::permissive())
            .wrap(logger)
            .app_data(web::Data::new(state.clone()))
            .service(Files::new("/static", &static_dir_fp).show_files_listing())
            .service(index)
            .service(api_routes::get_router())
    })
    .bind((host, port))?
    .run()
    .await
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(data.html.clone())
}

fn load_json(fp: String) -> Vec<GolPreset> {
    let path = Path::new(&fp);
    if path.exists() && path.is_file() {
        let file_content = fs::read_to_string(path).expect("Unable to read the file");

        let data: Vec<GolPreset> = serde_json::from_str(&file_content)
            .expect("Unable to deserialize JSON into Vec<GolPreset>");

        data
    } else {
        panic!("The provided path is invalid or does not exist: {}", fp);
    }
}
