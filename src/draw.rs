use std::iter::FromIterator;

use minifb::{Key, Window, WindowOptions};

const BM_WIDTH: u32 = 20;
const BM_HEIGHT: u32 = 15;

const WIDTH: usize = (BM_WIDTH * 32) as usize;
const HEIGHT: usize = (BM_HEIGHT * 32) as usize;

fn main() {
    let bitmap: &Vec<u32> = &Vec::from_iter(
        (0..BM_HEIGHT)
            .map(|row| {
                (0..BM_WIDTH).map(move |col| ((col ^ row) & 1) * 0xffffff)
            })
            .flatten(),
    );
    let buffer: Vec<u32> = Vec::from_iter(
        (0..HEIGHT)
            .map(|height| {
                (0..WIDTH).map(move |width| {
                    let row = height >> 5;
                    let col = width >> 5;
                    bitmap[row * BM_WIDTH as usize + col]
                })
            })
            .flatten(),
    );

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        /*for i in buffer.iter_mut() {
            *i = 0; // write something more funny here!
        }*/

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
