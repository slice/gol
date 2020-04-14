use minifb::{Key, Window, WindowOptions};

use gol::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 480;

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
