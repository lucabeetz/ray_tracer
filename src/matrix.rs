use std::ops;

use float_cmp::{ApproxEq, F32Margin};

use crate::tuple::Tuple;

#[derive(Debug)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    values: Vec<Vec<f32>>,
    transposed: bool,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            rows,
            cols,
            values: vec![vec![0.; cols]; rows],
            transposed: false,
        }
    }

    pub fn from_values(values: Vec<Vec<f32>>) -> Self {
        Matrix {
            rows: values.len(),
            cols: values[0].len(),
            values,
            transposed: false,
        }
    }

    pub fn identity(size: usize) -> Self {
        let mut values = vec![vec![0.; size]; size];
        for i in 0..size {
            values[i][i] = 1.;
        }
        Matrix::from_values(values)
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        if !self.transposed {
            return self.values[row][col];
        } else {
            return self.values[col][row];
        }
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

    pub fn dot(&self, other: &Matrix) -> Self {
        let res_shape = (self.rows, other.cols);
        let mut res_values = vec![vec![0.; res_shape.1]; res_shape.0];
        for i in 0..res_shape.0 {
            for j in 0..res_shape.1 {
                let mut sum = 0.;
                for a in 0..self.cols {
                    sum += self.get(i, a) * other.get(a, j);
                }
                res_values[i][j] = sum;
            }
        }
        Self::from_values(res_values)
    }

    pub fn transpose(&mut self) {
        self.transposed = !self.transposed;
        let cols = self.cols;
        self.cols = self.rows;
        self.rows = cols;
    }
}

impl<'a> ApproxEq for &'a Matrix {
    type Margin = F32Margin;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        for col in 0..self.cols {
            for row in 0..self.rows {
                if !self.get(row, col).approx_eq(other.get(row, col), margin) {
                    return false;
                }
            }
        }

        return true;
    }
}

impl From<Tuple> for Matrix {
    fn from(tuple: Tuple) -> Self {
        let values = vec![vec![tuple.0], vec![tuple.1], vec![tuple.2], vec![tuple.3]];
        Matrix::from_values(values)
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

    #[test]
    fn multiply_two_matrices() {
        let values1 = vec![
            vec![1., 2., 3., 4.],
            vec![5., 6., 7., 8.],
            vec![9., 8., 7., 6.],
            vec![5., 4., 3., 2.],
        ];
        let values2 = vec![
            vec![-2., 1., 2., 3.],
            vec![3., 2., 1., -1.],
            vec![4., 3., 6., 5.],
            vec![1., 2., 7., 8.],
        ];
        let values_res = vec![
            vec![20., 22., 50., 48.],
            vec![44., 54., 114., 108.],
            vec![40., 58., 110., 102.],
            vec![16., 26., 46., 42.],
        ];
        let m1 = Matrix::from_values(values1);
        let m2 = Matrix::from_values(values2);
        let res = Matrix::from_values(values_res);
        assert!(m1.dot(&m2).equals(&res));
    }

    #[test]
    fn matrix_from_tuple() {
        let vec = Tuple(1., 2., 3., 1.);
        let m_vec: Matrix = vec.into();
        assert_eq!(m_vec.rows, 4);
        assert_eq!(m_vec.cols, 1);

        let res_values = vec![vec![1.], vec![2.], vec![3.], vec![1.]];
        let m_res = Matrix::from_values(res_values);
        assert!(m_vec.equals(&m_res));
    }

    #[test]
    fn multiply_matrix_with_tuple() {
        let values1 = vec![
            vec![1., 2., 3., 4.],
            vec![2., 4., 4., 2.],
            vec![8., 6., 4., 1.],
            vec![0., 0., 0., 1.],
        ];
        let m = Matrix::from_values(values1);
        let vec = Tuple(1., 2., 3., 1.0);
        let res = Tuple(18., 24., 33., 1.);
        assert!(m.dot(&vec.into()).equals(&res.into()));
    }

    #[test]
    fn multply_with_identity() {
        let values = vec![
            vec![1., 2., 3., 4.],
            vec![2., 4., 4., 2.],
            vec![8., 6., 4., 1.],
            vec![0., 0., 0., 1.],
        ];
        let m = Matrix::from_values(values);
        let eye = Matrix::identity(4);
        assert!(m.equals(&m.dot(&eye)));

        let vec = Tuple(1., 2., 3., 1.0);
        let res_vec = eye.dot(&vec.clone().into());
        assert!(vec.equals(&res_vec.into()));
    }

    #[test]
    fn transpose_matrix() {
        let values = vec![
            vec![0., 9., 3., 0.],
            vec![9., 8., 0., 8.],
            vec![1., 8., 5., 3.],
            vec![0., 0., 5., 8.],
        ];
        let t_values = vec![
            vec![0., 9., 1., 0.],
            vec![9., 8., 8., 0.],
            vec![3., 0., 5., 5.],
            vec![0., 8., 3., 8.],
        ];
        let mut m = Matrix::from_values(values);
        m.transpose();
        assert!(m.equals(&Matrix::from_values(t_values)));
    }

    #[test]
    fn transpose_identity_matrix() {
        let mut eye = Matrix::identity(4);
        eye.transpose();
        assert!(eye.equals(&Matrix::identity(4)));
    }
}
