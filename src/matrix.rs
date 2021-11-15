use std::ops;

pub struct Matrix {
    rows: usize,
    cols: usize,
    values: Vec<Vec<f32>>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            values: vec![vec![0.; cols]; rows],
        }
    }

    pub fn with_values(values: Vec<Vec<f32>>) -> Self {
        Matrix {
            rows: values.len(),
            cols: values[0].len(),
            values,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.values[x][y]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_matrix() {
        let values = vec![
            vec![1., 2., 3., 4.],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9., 10., 11., 12.],
            vec![13.5, 14.5, 15.5, 16.5],
        ];

        let m = Matrix::with_values(values);
        assert_eq!(m.get(0, 0), 1.);
        assert_eq!(m.get(0, 3), 4.);
        assert_eq!(m.get(1, 0), 5.5);
        assert_eq!(m.get(1, 2), 7.5);
        assert_eq!(m.get(2, 2), 11.);
        assert_eq!(m.get(3, 0), 13.5);
        assert_eq!(m.get(3, 2), 15.5);
    }
}
