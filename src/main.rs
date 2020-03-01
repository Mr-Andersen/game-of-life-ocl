use std::iter::once;

use minifb::{Key, KeyRepeat, Window, WindowOptions};

mod consts;
mod game;
mod table;

use consts::*;
use game::*;
use table::*;

fn main() -> ocl::Result<()> {
    print!("Creating game... ");
    // In each row, place single alive cell in the middle
    let single = once(DEAD).cycle().take(TABLE_WIDTH / 2).chain(once(ALIVE));
    let mut game = Game::new(once(single).cycle().take(300))?;
    println!("done.");

    let mut buffer = Table::default();
    print!("Reading initial value... ");
    game.buffer().read(&mut *buffer as &mut [u32]).enq()?;
    println!("done.");

    print!("Creating window... ");
    let mut window = Window::new(
        "Game of Life (Esc to exit, Space to toggle pause)",
        TABLE_WIDTH,
        TABLE_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();
    window
        .update_with_buffer(&*buffer, TABLE_WIDTH, TABLE_HEIGHT)
        .unwrap();
    println!("done.");

    // Limit to max ~15 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(67000)));

    let mut pause = true;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Press Space to toggle pause
        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            pause = !pause;
        }
        // Hold 'S' to step forward
        if !pause || window.is_key_down(Key::S) {
            game.next()?.read(&mut *buffer as &mut [u32]).enq()?;
            window
                .update_with_buffer(&*buffer, TABLE_WIDTH, TABLE_HEIGHT)
                .unwrap();
        } else {
            window.update();
        }
    }

    Ok(())
}
