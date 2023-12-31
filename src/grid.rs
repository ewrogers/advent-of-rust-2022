// 2D grid that can be used when you have a known column width
// Rows can be added later, but must be of uniform size

#[derive(Debug)]
pub struct RowGrid<T> {
    width: usize,
    cells: Vec<T>,
}

impl<T> RowGrid<T>
where
    T: Clone,
{
    #[must_use]
    pub fn with_width(width: usize) -> Self {
        Self {
            width,
            cells: Vec::new(),
        }
    }

    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    #[must_use]
    pub fn height(&self) -> usize {
        self.cells.len() / self.width
    }

    #[must_use]
    pub fn cell(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width {
            self.cells.get(y * self.width + x)
        } else {
            None
        }
    }

    #[must_use]
    pub fn row(&self, y: usize) -> Option<&[T]> {
        if y < self.height() {
            let start = y * self.width;
            let end = start + self.width;
            Some(&self.cells[start..end])
        } else {
            None
        }
    }

    #[must_use]
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

    pub fn find<F>(&self, predicate: F) -> Option<(usize, usize)>
    where
        F: Fn(&T) -> bool,
    {
        for y in 0..self.height() {
            for x in 0..self.width {
                let value = &self.cells[y * self.width + x];
                if predicate(value) {
                    return Some((x, y));
                }
            }
        }
        None
    }

    pub fn find_all<F>(&self, predicate: F) -> Vec<(usize, usize)>
    where
        F: Fn(&T) -> bool,
    {
        let mut found = vec![];

        for y in 0..self.height() {
            for x in 0..self.width {
                let value = &self.cells[y * self.width + x];
                if predicate(value) {
                    found.push((x, y));
                }
            }
        }

        found
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

#[derive(Debug, Clone)]
pub struct UniformGrid<T> {
    width: usize,
    height: usize,
    cells: Vec<T>,
}

impl<T> UniformGrid<T>
where
    T: Clone + Default,
{
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        UniformGrid {
            width,
            height,
            cells: vec![T::default(); width * height],
        }
    }

    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    #[must_use]
    pub fn height(&self) -> usize {
        self.height
    }

    #[must_use]
    pub fn cell(&self, x: usize, y: usize) -> Option<&T> {
        self.cells.get(y * self.width + x)
    }

    pub fn set_cell(&mut self, x: usize, y: usize, value: T) {
        self.cells[y * self.width + x] = value;
    }

    #[must_use]
    pub fn find_all<F>(&self, predicate: F) -> Vec<(usize, usize)>
    where
        F: Fn(&T) -> bool,
    {
        let mut found = vec![];

        for y in 0..self.height() {
            for x in 0..self.width {
                let value = &self.cells[y * self.width + x];
                if predicate(value) {
                    found.push((x, y));
                }
            }
        }

        found
    }
}
