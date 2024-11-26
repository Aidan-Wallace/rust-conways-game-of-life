// a lives cell dies if it has fewer than two live neighbors
// a live cell with two or three live neighbors lives onto the next generation
// a live cell with more than three live neighbors dies
// a dead cell will be brought back to live if it has exactly three neighbors

// struct DetermineLater{living_neighbors:usize,dead_neighbors:usize}impl DetermineLater{fn new(_:&Vec<Vec<u8>>)->Self{Self{living_neighbors:0,dead_neighbors:0}}}

pub fn convert(source_matrix: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut new_matrix: Vec<Vec<u8>> = vec![];

    for x in 0..source_matrix.len() {
        for y in 0..source_matrix[x].len() {
            let cells_border = get_bordering_cells(&source_matrix, x as isize, y as isize);
        }
    }

    return new_matrix;
}

fn get_bordering_cells(source_matrix: &Vec<Vec<u8>>, x: isize, y: isize) -> Vec<u8> {
    let coords: Vec<Vec<isize>> = vec![
        vec![-1, -1], // top left
        vec![0, -1],  // top
        vec![1, -1],  // top right
        vec![-1, 0],  // left
        vec![1, 0],   // right
        vec![-1, 1],  // bottom left
        vec![0, 1],   //bottom
        vec![1, 1],   // bottom right
    ];

    let mut new: Vec<u8> = vec![];

    for i in coords {
        let new_x = x + i[0];
        let new_y = y + i[1];

        if new_x < 0 || new_y < 0 {
            continue;
        }

        new.push(source_matrix[new_x as usize][new_y as usize]);
    }

    new
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_bordering_cells_works() {
        let matrix: Vec<Vec<u8>> = vec![
            vec![0, 1, 0, 0],
            vec![1, 0, 1, 0],
            vec![1, 0, 1, 0],
            vec![0, 0, 1, 0],
        ];

        let result = get_bordering_cells(&matrix, 1, 2);
        assert_eq!(result[0], 1);
        assert_eq!(result[1], 0);
        assert_eq!(result[2], 1);
        assert_eq!(result[3], 1);
        assert_eq!(result[4], 1);
        assert_eq!(result[5], 0);
        assert_eq!(result[6], 0);
        assert_eq!(result[7], 1);
    }
}
