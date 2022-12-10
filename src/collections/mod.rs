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
}
