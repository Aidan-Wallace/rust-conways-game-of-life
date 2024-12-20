use dotenv::dotenv;
use game_of_life::matrix::Matrix;
use std::{env, fs, path::Path, thread, time::Duration};

const DEFAULT_SIZE: u16 = 10;
const SLEEP_TIME_MS: u64 = 600;

fn main() {
    dotenv().ok();

    let source_grid_path = env::var("SOURCE_GRID_PATH").unwrap_or("".to_string());
    let use_toroidal = env::var("USE_TOROIDAL")
        .map(|v| v.to_lowercase() == "true")
        .unwrap_or(false);
    let sleep_time = match env::var("SLEEP_TIME_MS")
        .unwrap_or(SLEEP_TIME_MS.to_string())
        .parse()
    {
        Ok(value) => value,
        Err(_) => {
            eprintln!(
                "Failed to parse SLEEP_TIME_MS. Falling back to default: {}",
                SLEEP_TIME_MS
            );
            SLEEP_TIME_MS
        }
    };

    let matrix = {
        if source_grid_path.is_empty() {
            Matrix::generate_random_binary(DEFAULT_SIZE, DEFAULT_SIZE)
        } else {
            load_json(source_grid_path)
        }
    };

    game_loop(matrix, sleep_time, use_toroidal)
}

fn game_loop(matrix: Matrix, sleep_time: u64, use_toroidal: bool) -> ! {
    let mut gb: Matrix = matrix;

    let mut iter = 0;
    loop {
        gb.print(iter);

        game_of_life::convert(&mut gb, use_toroidal);

        iter += 1;
        thread::sleep(Duration::from_millis(sleep_time));
    }
}

fn load_json(source_grid_path: String) -> Matrix {
    let path = Path::new(&source_grid_path);
    if path.exists() && path.is_file() {
        let file_content = fs::read_to_string(path).expect("Unable to read the file");

        let data: Vec<Vec<u8>> = serde_json::from_str(&file_content)
            .expect("Unable to deserialize JSON into Vec<Vec<u8>>");

        Matrix::new(data)
    } else {
        panic!(
            "The provided path is invalid or does not exist: {}",
            source_grid_path
        );
    }
}
