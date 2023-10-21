// A simple grid structure that wraps a vector and does the 2D math to access cells
// Can access rows, columns, and cells
#[derive(Debug)]
pub struct RowGrid<T> {
    vec: Vec<T>,
    width: usize,
}

impl<T> RowGrid<T> {
    pub fn from_vec(vec: Vec<T>, width: usize) -> Self {
        Self { vec, width }
    }

    pub fn row_count(&self) -> usize {
        self.vec.len() / self.width
    }

    pub fn col_count(&self) -> usize {
        self.width
    }

    pub fn row(&self, index: usize) -> Option<&[T]> {
        let start_index = index * self.width;
        let end_index = start_index + self.width;

        if end_index > self.vec.len() {
            return None;
        }

        Some(&self.vec[start_index..end_index])
    }

    pub fn column(&self, index: usize) -> Option<Vec<&T>> {
        if index >= self.width {
            return None;
        }

        let height = self.row_count();
        let mut vec: Vec<&T> = Vec::with_capacity(height);

        for y in 0..height {
            let value = &self.vec[y * self.width + index];
            vec.push(value);
        }

        Some(vec)
    }

    // Gets back an immutable reference to a specific cell (if within bounds)
    pub fn cell(&self, x: usize, y: usize) -> Option<&T> {
        let index = y * self.width + x;

        if index >= self.vec.len() {
            return None;
        }

        Some(&self.vec[index])
    }

    // Gets back a mutable reference to a specific cell (if within bounds)
    pub fn cell_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        let index = y * self.width + x;

        if index >= self.vec.len() {
            return None;
        }

        Some(&mut self.vec[index])
    }

    // Allows a function to be called for each grid item, and x/y coordinate
    pub fn enumerate<F>(&self, mut f: F)
    where
        F: FnMut(&T, (usize, usize)),
    {
        let row_count = self.row_count();

        for y in 0..row_count {
            for x in 0..self.width {
                let value = &self.vec[y * self.width + x];
                f(value, (x, y))
            }
        }
    }
}
