// 2D grid that can be used when you have a known column width
// Rows can be added later, but must be of uniform size

#[derive(Debug)]
pub struct RowGrid<T> {
    pub width: usize,
    cells: Vec<T>,
}

impl<T> RowGrid<T>
where
    T: Clone,
{
    pub fn with_width(width: usize) -> Self {
        Self {
            width,
            cells: Vec::new(),
        }
    }

    pub fn height(&self) -> usize {
        self.cells.len() / self.width
    }

    pub fn cell(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width {
            self.cells.get(y * self.width + x)
        } else {
            None
        }
    }

    pub fn row(&self, y: usize) -> Option<&[T]> {
        if y < self.height() {
            let start = y * self.width;
            let end = start + self.width;
            Some(&self.cells[start..end])
        } else {
            None
        }
    }

    pub fn column(&self, x: usize) -> Option<Vec<&T>> {
        if x < self.width {
            let col = (0..self.height())
                .map(|y| &self.cells[y * self.width + x])
                .collect();
            Some(col)
        } else {
            None
        }
    }

    pub fn enumerate<F>(&self, mut func: F)
    where
        F: FnMut(usize, usize),
    {
        for y in 0..self.height() {
            for x in 0..self.width {
                func(x, y);
            }
        }
    }

    pub fn push_row(&mut self, row: Vec<T>) {
        if row.len() == self.width {
            self.cells.extend(row);
        } else {
            panic!("Row length does not match grid width of {}!", self.width);
        }
    }
}
