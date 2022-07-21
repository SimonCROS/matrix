pub struct Vector<K> {
    size: u16,
    data: Vec<K>
}

impl<K> Vector<K> {
    pub fn new(size: u16) -> Vector<K> {
        Vector {
            size,
            data: Vec::with_capacity(size as usize)
        }
    }

    /// Returns the size of the vector
    /// (rows: u16, cols: u16)
    pub fn size(&self) -> u16 {
        self.size
    }
}
