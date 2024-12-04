pub trait Coordinates2D {
    /// Gets all the coordinates of a 2D data structure, like a vector of vectors.
    /// That allows to iterate directly with a map against (i, j)
    /// instead to need to combine a flatmap i with a map j.
    ///
    /// For example, to sum the product of every number in a matrix with the sum of its coordinates:
    ///
    /// ```
    /// use enontekio::collections::Coordinates2D;
    ///
    /// let numbers = vec![vec![1 as i32, 2, 3, 4, 5, 5], vec![8, 9, 1, 2, 4, 5]];
    ///
    /// let product_coordinates: i32 = numbers.coordinates_2d()
    ///     .iter()
    ///     .map(|&(i, j)| numbers[i][j] * (i + j) as i32)
    ///     .sum();
    /// ```
    ///
    /// This function is mainly useful to navigate in rows and columns of a numbers grid.
    fn coordinates_2d(&self) -> Vec<(usize, usize)>;
}

pub trait Coordinates3D {
    fn coordinates_3d(&self) -> Vec<(usize, usize, usize)>;
}

impl<T> Coordinates2D for Vec<Vec<T>> {
    fn coordinates_2d(&self) -> Vec<(usize, usize)> {
        (0..self.len())
            .flat_map(|i| (0..self[0].len()).map(move |j| (i, j)))
            .collect()
    }
}

impl<T> Coordinates3D for Vec<Vec<Vec<T>>> {
    fn coordinates_3d(&self) -> Vec<(usize, usize, usize)> {
        (0..self.len())
            .flat_map(|i| (0..self[0].len())
                .flat_map(move |j| (0..self[0][0].len()).map(move |k| (i, j, k))))
            .collect()
    }
}

/// Returns all the possible vectors to move in a grid.
/// The moves can be horizontal, vertical or in diagonal.
pub fn get_all_direction_vectors_2d() -> Vec<(i32, i32)> {
    vec![
        // Vertically
        (1, 0),
        (-1, 0),

        // Horizontally
        (0, 1),
        (0, -1),

        // Diagonally
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ]
}

/// Returns all the possible vectors to move in a grid in taxicab geometry.
/// The moves can be horizontal or vertical.
pub fn get_taxicab_direction_vectors_2d() -> Vec<(i32, i32)> {
    vec![
        // Vertically
        (1, 0),
        (-1, 0),

        // Horizontally
        (0, 1),
        (0, -1),
    ]
}

/// Returns all the possible vectors to move in a grid in diagonal.
pub fn get_diagonal_direction_vectors_2d() -> Vec<(i32, i32)> {
    vec![
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ]
}

/// Checks if the specified coordinates are valid for a specified 2D grid.
/// A coordinate is valid if it doesn't overflow and is positive.
///
/// This method assumes the grid argument to be a grid,
/// ie each line is expected to have the same length.
///
/// This method has been designed to compute coordinates and filter the result.
pub fn are_valid_coordinates_for_2d_grid<T>(grid: &Vec<Vec<T>>, coords: (i32, i32)) -> bool {
    if grid.is_empty() {
        return false;
    }

    let i = coords.0;
    let j = coords.1;

    if i < 0 || j < 0 {
        return false;
    }

    let max_i = grid.len() - 1;
    let max_j = grid[0].len() - 1;

    (i as usize) <= max_i && (j as usize) <= max_j
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinates_for_vec2d() {
        let digits: Vec<Vec<u32>> = vec![
            vec![1, 2],
            vec![3, 4],
        ];

        let expected = vec![(0 as usize, 0 as usize), (0, 1), (1, 0), (1, 1)];
        assert_eq!(expected, digits.coordinates_2d());
    }

    #[test]
    fn test_coordinates_for_empty_vec2d() {
        let empty_vector: Vec<Vec<u32>> = Vec::new();

        assert_eq!(0, empty_vector.coordinates_2d().len());
    }

    #[test]
    fn test_coordinates_for_vec3d() {
        let digits: Vec<Vec<Vec<u32>>> = vec![
            vec![vec![1, 2], vec![3, 4]],
            vec![vec![5, 6], vec![7, 8]],
        ];

        let expected = vec![
            (0 as usize, 0 as usize, 0 as usize), (0, 0, 1), (0, 1, 0), (0, 1, 1),
            (1, 0, 0), (1, 0, 1), (1, 1, 0), (1, 1, 1),
        ];
        assert_eq!(expected, digits.coordinates_3d());
    }

    #[test]
    fn test_coordinates_2d_for_vec3d() {
        let digits: Vec<Vec<Vec<u32>>> = vec![
            vec![vec![1, 2], vec![3, 4]],
            vec![vec![5, 6], vec![7, 8]],
        ];

        let expected = vec![(0 as usize, 0 as usize), (0, 1), (1, 0), (1, 1)];
        assert_eq!(expected, digits.coordinates_2d());
    }

    #[test]
    fn test_are_valid_2d_grid_coordinates() {
        let grid: Vec<Vec<u32>> = vec![
            vec![1, 2],
            vec![3, 4],
        ];

        assert_eq!(true, are_valid_coordinates_for_2d_grid(&grid, (0, 0)));
        assert_eq!(true, are_valid_coordinates_for_2d_grid(&grid, (0, 1)));
        assert_eq!(true, are_valid_coordinates_for_2d_grid(&grid, (1, 0)));
        assert_eq!(true, are_valid_coordinates_for_2d_grid(&grid, (1, 1)));

        assert_eq!(false, are_valid_coordinates_for_2d_grid(&grid, (-1, 1)));
        assert_eq!(false, are_valid_coordinates_for_2d_grid(&grid, (1, 3)));
    }
}
