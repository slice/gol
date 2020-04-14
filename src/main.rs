use minifb::{Key, Window, WindowOptions};
use std::ops::{Index, IndexMut};

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

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

fn time<F, R>(label: &str, mut run: F, continuous: bool) -> R
where
    F: FnMut() -> R,
{
    let before = std::time::Instant::now();
    let value = run();
    print!("{}: {}ms    ", label, before.elapsed().as_millis());
    if continuous {
        print!("\r")
    } else {
        println!();
    }
    value
}

fn main() {
    let mut life = Life::new(WIDTH, HEIGHT);
    for ((x, y), cell) in life.board.iter_cells_mut() {
        *cell = ((x * y) as f64).sin() > 0.5;
    }
    let mut pixbuf: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("game of life", WIDTH, HEIGHT, WindowOptions::default())
        .expect("failed to create window");

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        time("iterate", || life.iterate(), true);
        life.render(&mut pixbuf);
        window.update_with_buffer(&pixbuf, WIDTH, HEIGHT).unwrap();
    }
}
