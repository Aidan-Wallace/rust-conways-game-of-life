pub mod matrix;

use matrix::Matrix;

pub fn convert(source_matrix: &mut Matrix, use_toroidal: bool) {
    let cop = Matrix::new(source_matrix.data.clone());
    let x_len = cop.data.len();

    for x in 0..x_len {
        let y_len = cop.data[x].len();

        for y in 0..y_len {
            let cells_border = cop.get_bordering_cells(x, y, use_toroidal);
            let num_living_cells = count_living_cells(&cells_border);

            let current_cell_value = cop.get_value(x, y).expect("failed to get value of cell");
            let new_cell_val = determine_cell_value(current_cell_value, num_living_cells);

            if use_toroidal && new_cell_val == 1 {
                let new_y = (y + y_len) % y_len;
                let new_x = (x + x_len) % x_len;

                source_matrix.data[new_y][new_x] = new_cell_val;
            } else {
                source_matrix.data[y][x] = new_cell_val; // why is this [y][x]?
            }
        }
    }
}

fn determine_cell_value(value: u8, alive_cell_count: usize) -> u8 {
    // a live cell dies if it has fewer than two live neighbors
    // a live cell lives if two or three live neighbors
    // a live cell dies if more than three live neighbors
    // a dead cell will be brought back to live if it has exactly three neighbors

    if value == 1 {
        if alive_cell_count < 2 || alive_cell_count > 3 {
            return 0;
        } else if 2 <= alive_cell_count && alive_cell_count <= 3 {
            return 1;
        }
    } else if value == 0 {
        if alive_cell_count == 3 {
            return 1;
        } else {
            return value;
        }
    }

    value
}

fn count_living_cells(data: &Vec<u8>) -> usize {
    let count_ones = data.iter().filter(|&&x| x == 1).count();
    count_ones
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_works() {
        let mut matrix = Matrix::new(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 1, 0],
            vec![0, 0, 1, 1, 1, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
        ]);

        let expected = vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 1, 0],
            vec![0, 0, 0, 1, 1, 0],
            vec![0, 0, 0, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
        ];

        convert(&mut matrix, false);

        matrix.print(1);
        assert_eq!(matrix.data, expected);
    }

    #[test]
    fn determine_cell_value_alive_with_less_than_2_live_neighbors_dies() {
        let result = determine_cell_value(1, 1);
        assert_eq!(result, 0);
    }

    #[test]
    fn determine_cell_value_alive_with_more_than_3_live_neighbors_dies() {
        let result = determine_cell_value(1, 4);
        assert_eq!(result, 0);
    }

    #[test]
    fn determine_cell_value_alive_with_2_live_neighbors_lives() {
        let result = determine_cell_value(1, 2);
        assert_eq!(result, 1);
    }

    #[test]
    fn determine_cell_value_alive_with_3_live_neighbors_lives() {
        let result = determine_cell_value(1, 3);
        assert_eq!(result, 1);
    }

    #[test]
    fn determine_cell_value_dead_with_3_live_neighbors_lives() {
        let result = determine_cell_value(0, 3);
        assert_eq!(result, 1);
    }

    #[test]
    fn determine_cell_value_dead_with_4_live_neighbors_dies() {
        let result = determine_cell_value(0, 4);
        assert_eq!(result, 0);
    }

    #[test]
    fn count_living_cells_works() {
        let data = vec![0, 0, 1, 0, 1];
        let result = count_living_cells(&data);

        assert_eq!(result, 2);
    }
}
