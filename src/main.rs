mod gol;

use rand::Rng;
use std::{thread, time::Duration};

const SIZE: u16 = 10;
const SLEEP_TIME_MS: u64 = 600;

fn main() {
    let matrix = generate_default(SIZE, SIZE);

    game_loop(&matrix)
}

fn game_loop(matrix: &Vec<Vec<u8>>) -> ! {
    loop {
        print(&matrix);

        gol::convert(matrix);

        thread::sleep(Duration::from_millis(SLEEP_TIME_MS));
    }
}

fn generate_default(x: u16, y: u16) -> Vec<Vec<u8>> {
    let mut rng = rand::thread_rng();
    let mut matrix: Vec<Vec<u8>> = vec![];

    for _ in 0..x {
        let mut row: Vec<u8> = vec![];

        for _ in 0..y {
            row.push(rng.gen_range(0..=1));
        }

        matrix.push(row);
    }

    matrix
}

fn print(matrix: &Vec<Vec<u8>>) {
    let w = matrix.len();

    // print top border
    println!(" {}", "_".repeat(w * 2));

    for i in matrix {
        let mut s = String::new();

        // print left side border for row
        print!("|");

        for j in i {
            match j {
                1 => s += "0 ",
                0 => s += "  ",
                _ => panic!(""),
            }
        }

        // print row cells and right border for row
        println!("{}|", s);
    }

    // print bottom border
    println!("|{}|", "-".repeat(w * 2));
}
