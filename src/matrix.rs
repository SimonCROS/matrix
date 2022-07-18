pub struct Matrix<K> {
    rows: u32,
    cols: u32,
    data: Vec<K>
}

impl<K> Matrix<K> {
    pub fn new(rows: u32, cols: u32) -> Matrix<K> {
        Matrix {
            cols: cols,
            rows: rows,
            data: vec![]
        }
    }

    /// Returns the size of the matrix as a tuple
    /// (rows: u32, cols: u32)
    pub fn size(&self) -> (u32, u32) {
        return (self.rows, self.cols)
    }
}