use std::fmt;

pub struct Matrix<K: Clone> {
    rows: u16,
    cols: u16,
    data: Vec<K>
}

impl<K: Clone> Matrix<K> {
    pub fn new(rows: u16, cols: u16) -> Matrix<K> {
        Matrix {
            cols,
            rows,
            data: Vec::with_capacity((rows * cols) as usize)
        }
    }

    /// Returns the size of the matrix in a tuple
    /// (rows: u16, cols: u16)
    pub fn size(&self) -> (u16, u16) {
        (self.rows, self.cols)
    }

    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for i in 0..self.rows {
            result += "|";
            for j in 0..self.cols {
                result += &format!("{}", 0);//self.data[i * self.cols + j]
            }
            result += "|\n";
        }
        result
    }
}

impl fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
