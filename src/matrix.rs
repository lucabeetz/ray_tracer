use std::ops;

use float_cmp::{ApproxEq, F32Margin};

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

    pub fn from_values(values: Vec<Vec<f32>>) -> Self {
        Matrix {
            rows: values.len(),
            cols: values[0].len(),
            values,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.values[x][y]
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn equals(&self, other: &Matrix) -> bool {
        self.approx_eq(
            other,
            F32Margin {
                epsilon: 0.0,
                ulps: 2,
            },
        )
    }
}

impl<'a> ApproxEq for &'a Matrix {
    type Margin = F32Margin;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        for x in 0..self.cols {
            for y in 0..self.rows {
                if !self.get(x, y).approx_eq(other.get(x, y), margin) {
                    return false;
                }
            }
        }

        return true;
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

        let m = Matrix::from_values(values);
        assert_eq!(m.shape(), (4, 4));
        assert_eq!(m.get(0, 0), 1.);
        assert_eq!(m.get(0, 3), 4.);
        assert_eq!(m.get(1, 0), 5.5);
        assert_eq!(m.get(1, 2), 7.5);
        assert_eq!(m.get(2, 2), 11.);
        assert_eq!(m.get(3, 0), 13.5);
        assert_eq!(m.get(3, 2), 15.5);
    }

    #[test]
    fn create_different_sized_matrices() {
        let values = vec![vec![-3., 5.], vec![1., -2.]];
        let m1 = Matrix::from_values(values);
        assert_eq!(m1.shape(), (2, 2));
        assert_eq!(m1.get(0, 0), -3.);
        assert_eq!(m1.get(0, 1), 5.);
        assert_eq!(m1.get(1, 0), 1.);
        assert_eq!(m1.get(1, 1), -2.);

        let values = vec![vec![-3., 5., 0.], vec![1., -2., -7.], vec![0., 1., 1.]];
        let m2 = Matrix::from_values(values);
        assert_eq!(m2.shape(), (3, 3));
        assert_eq!(m2.get(0, 0), -3.);
        assert_eq!(m2.get(1, 1), -2.);
        assert_eq!(m2.get(2, 2), 1.);
    }

    #[test]
    fn compare_matrices() {
        let values = vec![
            vec![1., 2., 3., 4.],
            vec![5., 6., 7., 8.],
            vec![9., 8., 7., 6.],
            vec![5., 4., 3., 2.],
        ];
        let m1 = Matrix::from_values(values.clone());
        let m2 = Matrix::from_values(values);
        assert!(m1.equals(&m2));

        let values2 = vec![
            vec![5., 6., 7., 8.],
            vec![9., 8., 7., 6.],
            vec![5., 4., 3., 2.],
            vec![1., 2., 3., 4.],
        ];
        let m3 = Matrix::from_values(values2);
        assert!(!m1.equals(&m3));
    }
}
