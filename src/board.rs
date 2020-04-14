use std::ops::{Index, IndexMut};

type Coords = (usize, usize);

#[derive(Clone)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<bool>,
}

impl Index<Coords> for Board {
    type Output = bool;

    fn index(&self, (x, y): Coords) -> &Self::Output {
        &self.cells[x + y * self.width]
    }
}

impl IndexMut<Coords> for Board {
    fn index_mut(&mut self, (x, y): Coords) -> &mut Self::Output {
        &mut self.cells[x + y * self.width]
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

    pub fn neighbors(&self, (x, y): Coords) -> [bool; 8] {
        if x == 0 || y == 0 || x == self.width - 1 || y == self.height - 1 {
            return [false, false, false, false, false, false, false, false];
        }

        [
            // above neighbors
            self[(x - 1, y + 1)],
            self[(x, y + 1)],
            self[(x + 1, y + 1)],
            // side neighbors
            self[(x - 1, y)],
            self[(x + 1, y)],
            // below neighbors
            self[(x - 1, y - 1)],
            self[(x, y - 1)],
            self[(x + 1, y - 1)],
        ]
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = (Coords, &bool)> {
        self.cells.iter().enumerate().map(move |(index, cell)| {
            let x = index % self.width;
            let y = index / self.width;

            ((x, y), cell)
        })
    }

    pub fn iter_cells_mut(&mut self) -> impl Iterator<Item = (Coords, &mut bool)> + '_ {
        let width = self.width;

        self.cells.iter_mut().enumerate().map(move |(index, cell)| {
            let x = index % width;
            let y = index / width;

            ((x, y), cell)
        })
    }
}
