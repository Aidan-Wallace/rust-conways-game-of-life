use rand::Rng;

const OFFSETS: [(isize, isize); 8] = [
    (-1, -1), // Top-left
    (0, -1),  // Top
    (1, -1),  // Top-right
    (-1, 0),  // Left
    (1, 0),   // Right
    (-1, 1),  // Bottom-left
    (0, 1),   // Bottom
    (1, 1),   // Bottom-right
];

pub struct Matrix {
    pub data: Vec<Vec<u8>>,
}

impl Matrix {
    pub fn new(data: Vec<Vec<u8>>) -> Self {
        Matrix { data }
    }

    pub fn generate_random_binary(x: u16, y: u16) -> Self {
        let mut rng = rand::thread_rng();
        let mut matrix: Vec<Vec<u8>> = vec![];

        for _ in 0..x {
            let mut row: Vec<u8> = vec![];

            for _ in 0..y {
                row.push(rng.gen_range(0..=1));
            }

            matrix.push(row);
        }

        Matrix { data: matrix }
    }

    pub fn get_value(&self, x: usize, y: usize) -> Option<u8> {
        if x < self.data.len() && y < self.data[0].len() {
            Some(self.data[y][x])
        } else {
            None
        }
    }

    pub fn get_bordering_cells(&self, x: usize, y: usize) -> Vec<u8> {
        let mut neighbors = Vec::new();
        let rows = self.data.len();

        for (dx, dy) in OFFSETS {
            let cols = self.data[0].len();

            let new_x = x as isize + dx;
            let new_y = y as isize + dy;

            if new_x >= 0 && new_y >= 0 && new_x < rows as isize && new_y < cols as isize {
                neighbors.push(self.data[new_y as usize][new_x as usize]);
            }

        }

        neighbors
    }

    pub fn print(&self) {
        let w = self.data.len();

        // print top border
        println!(" {}", "_".repeat(w * 2));

        for i in self.data.clone() {
            let mut s = String::new();

            // print left side border for row
            print!("|");

            for j in i {
                match j {
                    1 => s += "█ ",
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_bordering_cells_works() {
        let matrix = Matrix::new(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ]);

        let result = matrix.get_bordering_cells(1, 2);
        assert_eq!(result, vec![5, 6, 7, 9, 11, 13, 14, 15]);
    }

    #[test]
    fn get_bordering_cells_with_no_borders_works() {
        let matrix = Matrix::new(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ]);

        let result = matrix.get_bordering_cells(1, 0);
        assert_eq!(result, vec![1, 3, 5, 6, 7]);
    }

    #[test]
    fn get_bordering_cells_at_0_0_works() {
        let matrix = Matrix::new(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ]);

        let result = matrix.get_bordering_cells(0, 0);
        assert_eq!(result, vec![2, 5, 6]);
    }
}
