mod gol;
mod matrix;

use dotenv::dotenv;
use matrix::Matrix;
use std::{env, fs, path::Path, thread, time::Duration};

const DEFAULT_SIZE: u16 = 10;
const SLEEP_TIME_MS: u64 = 600;

fn main() {
    dotenv().ok();

    let source_grid_path = env::var("SOURCE_GRID_PATH").unwrap_or("".to_string());

    let matrix = {
        if source_grid_path.is_empty() {
            Matrix::generate_random_binary(DEFAULT_SIZE, DEFAULT_SIZE)
        } else {
            load_json(source_grid_path)
        }
    };

    game_loop(matrix)
}

fn game_loop(matrix: Matrix) -> ! {
    let mut gb: Matrix = matrix;

    loop {
        gb.print();

        gol::convert(&mut gb);

        thread::sleep(Duration::from_millis(SLEEP_TIME_MS));
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
