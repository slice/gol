use crate::Board;

pub struct Life {
    pub board: Board,
}

impl Life {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            board: Board::new(width, height),
        }
    }

    pub fn iterate(&mut self) {
        let cloned_board = self.board.clone();

        for (coords, cell) in self.board.iter_cells_mut() {
            let n_alive = cloned_board
                .neighbors(coords)
                .iter()
                .filter(|&&n| n)
                .count();

            match n_alive {
                2 | 3 if *cell => {}
                3 if !*cell => *cell = true,
                _ => *cell = false,
            }
        }
    }

    pub fn render(&self, pixbuf: &mut Vec<u32>) {
        for (i, (_, cell)) in self.board.iter_cells().enumerate() {
            pixbuf[i] = if *cell { u32::max_value() } else { 0 };
        }
    }
}
