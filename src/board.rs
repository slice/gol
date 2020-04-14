use std::ops::{Index, IndexMut};

pub struct Coords(usize, usize);

impl Coords {
    pub fn from_index(index: usize, scanline_width: usize) -> Self {
        Self(index % scanline_width, index / scanline_width)
    }

    pub fn as_index(&self, scanline_width: usize) -> usize {
        self.0 + self.1 * scanline_width
    }
}

#[derive(Clone)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<bool>,
}

impl Index<Coords> for Board {
    type Output = bool;

    fn index(&self, coords: Coords) -> &Self::Output {
        &self.cells[coords.as_index(self.width)]
    }
}

impl IndexMut<Coords> for Board {
    fn index_mut(&mut self, coords: Coords) -> &mut Self::Output {
        &mut self.cells[coords.as_index(self.width)]
    }
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![false; width * height],
        }
    }

    pub fn from_vec(width: usize, height: usize, cells: Vec<bool>) -> Self {
        assert_eq!(
            cells.len(),
            width * height,
            "width * height isn't cells.len()"
        );

        Self {
            width,
            height,
            cells,
        }
    }

    pub fn neighbors(&self, Coords(x, y): Coords) -> [bool; 8] {
        // TODO: actually handle this
        if x == 0 || y == 0 || x == self.width - 1 || y == self.height - 1 {
            return [false, false, false, false, false, false, false, false];
        }

        [
            // top left diagonal neighbor
            self[Coords(x - 1, y + 1)],
            // upper neighbor
            self[Coords(x, y + 1)],
            // top right diagonal neighbor
            self[Coords(x + 1, y + 1)],
            // left neighbor
            self[Coords(x - 1, y)],
            // right neighbor
            self[Coords(x + 1, y)],
            // bottom left diagonal neighbor
            self[Coords(x - 1, y - 1)],
            // bottom neighbor
            self[Coords(x, y - 1)],
            // bottom right diagonal neighbor
            self[Coords(x + 1, y - 1)],
        ]
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = (Coords, &bool)> {
        self.cells
            .iter()
            .enumerate()
            .map(move |(index, cell)| (Coords::from_index(index, self.width), cell))
    }

    pub fn iter_cells_mut(&mut self) -> impl Iterator<Item = (Coords, &mut bool)> {
        let width = self.width;

        self.cells
            .iter_mut()
            .enumerate()
            .map(move |(index, cell)| (Coords::from_index(index, width), cell))
    }
}
