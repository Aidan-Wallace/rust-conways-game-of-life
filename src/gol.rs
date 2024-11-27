// a lives cell dies if it has fewer than two live neighbors
// a live cell with two or three live neighbors lives onto the next generation
// a live cell with more than three live neighbors dies
// a dead cell will be brought back to live if it has exactly three neighbors

// struct DetermineLater{living_neighbors:usize,dead_neighbors:usize}impl DetermineLater{fn new(_:&Vec<Vec<u8>>)->Self{Self{living_neighbors:0,dead_neighbors:0}}}

use crate::matrix::Matrix;

pub fn convert(source_matrix: &Matrix) -> Vec<Vec<u8>> {
    let mut new_matrix: Vec<Vec<u8>> = vec![];

    for x in 0..source_matrix.data.len() {
        for y in 0..source_matrix.data[x].len() {
            let cells_border = source_matrix.get_bordering_cells(x, y);
        }
    }

    return new_matrix;
}
