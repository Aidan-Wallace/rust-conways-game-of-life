use crate::matrix::Matrix;

pub fn convert(source_matrix: &mut Matrix) {
    let cop = Matrix::new(source_matrix.data.clone());

    for x in 0..cop.data.len() {
        for y in 0..cop.data[x].len() {
            let cells_border = cop.get_bordering_cells(x, y);
            let counts = count_zeros_and_ones(&cells_border);

            let z = cop.get_value(x, y).expect("failed to get value of cell");

            let new_val = determine_cell_value(z, counts.1);

            source_matrix.data[y][x] = new_val;
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

fn count_zeros_and_ones(data: &Vec<u8>) -> (usize, usize) {
    let count_zeros = data.iter().filter(|&&x| x == 0).count();
    let count_ones = data.iter().filter(|&&x| x == 1).count();
    (count_zeros, count_ones)
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

        convert(&mut matrix);

        matrix.print();
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
    fn count_zeros_and_ones_works() {
        let data = vec![0, 0, 1, 0, 1];
        let result = count_zeros_and_ones(&data);

        assert_eq!(result, (3, 2));
    }
}
