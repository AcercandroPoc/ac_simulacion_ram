use std::mem;

#[derive(Clone)]
pub struct Grid2D<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

impl<T: Copy + Clone> Grid2D<T> {
    pub fn new(width: usize, height: usize, default_val: T) -> Self {
        Self {
            width,
            height,
            data: vec![default_val; width * height],
        }
    }

    #[inline(always)]
    pub fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    #[inline(always)]
    pub fn get(&self, x: usize, y: usize) -> T {
        self.data[self.idx(x, y)]
    }

    #[inline(always)]
    pub fn set(&mut self, x: usize, y: usize, val: T) {
        let i = self.idx(x, y);
        self.data[i] = val;
    }

    #[inline(always)]
    pub fn fill(&mut self, val: T) {
        self.data.fill(val);
    }
}

pub struct DoubleBufferGrid<T> {
    pub current: Grid2D<T>,
    pub next: Grid2D<T>,
}

impl<T: Copy + Clone> DoubleBufferGrid<T> {
    pub fn new(width: usize, height: usize, default_val: T) -> Self {
        Self {
            current: Grid2D::new(width, height, default_val),
            next: Grid2D::new(width, height, default_val),
        }
    }

    #[inline(always)]
    pub fn swap(&mut self) {
        mem::swap(&mut self.current, &mut self.next);
    }
}
