use clap::{App, Arg, ArgMatches};
use minifb::{Key, Window, WindowOptions};

use gol::*;

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

fn arg<T>(matches: &ArgMatches, name: &str) -> T
where
    T: std::str::FromStr,
{
    matches
        .value_of(name)
        .unwrap()
        .parse()
        .unwrap_or_else(|_| panic!("invalid {}", name))
}

fn main() {
    let matches = App::new("gol")
        .author("slice")
        .about("game of life")
        .arg(
            Arg::with_name("max_fps")
                .short("f")
                .long("max-fps")
                .help("the maximum fps to run at")
                .default_value("60"),
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .help("the width of the window")
                .default_value("640"),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .help("the height of the window")
                .default_value("480"),
        )
        .get_matches();

    let max_fps: u64 = arg(&matches, "max_fps");
    let width: usize = arg(&matches, "width");
    let height: usize = arg(&matches, "height");

    let mut life = Life::new(width, height);
    for ((x, y), cell) in life.board.iter_cells_mut() {
        *cell = ((x * y) as f64).sin() > 0.5;
    }
    let mut pixbuf: Vec<u32> = vec![0; width * height];

    let mut window = Window::new("game of life", width, height, WindowOptions::default())
        .expect("failed to create window");

    window.limit_update_rate(Some(std::time::Duration::from_millis(1000 / max_fps)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        time("iterate", || life.iterate(), true);
        life.render(&mut pixbuf);
        window.update_with_buffer(&pixbuf, width, height).unwrap();
    }
}
