use std::iter::once;

use minifb::{Key, Window, WindowOptions};

mod consts;
mod game;
mod table;

use consts::*;
use game::*;
use table::*;

fn main() -> ocl::Result<()> {
    print!("Creating game... ");
    // let single: Vec<u32> = once(0x00ffffff).cycle().take(100).collect();
    let single = once(0u32).cycle().take(360).chain(once(0x00ffffff));
    let mut game = Game::new(
        once(single).cycle().take(300)
    )?;
    /*let mut game = Game::new(&[
        &[0, 0, 0, 0, 0],
        &[0, 0, 0xffffffff, 0, 0],
        &[0, 0, 0, 0xffffffff, 0],
        &[0, 0xffffffff, 0xffffffff, 0xffffffff, 0],
        &[0, 0, 0, 0, 0],
    ])?;*/
    println!("done.");

    let mut buffer = Table::default();
    print!("Reading initial value... ");
    game.buffer().read(&mut *buffer as &mut [u32]).enq()?;
    println!("done.");

    print!("Creating window... ");
    let mut window = Window::new(
        "Test - ESC to exit",
        WIN_WIDTH,
        WIN_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();
    window
        .update_with_buffer(&*buffer, WIN_WIDTH, WIN_HEIGHT)
        .unwrap();
    println!("done.");

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut pause = true;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::Space) {
            pause = !pause;
        }
        if !pause || window.is_key_down(Key::S) {
            game.next()?.read(&mut *buffer as &mut [u32]).enq()?;
            window
                .update_with_buffer(&*buffer, WIN_WIDTH, WIN_HEIGHT)
                .unwrap();
        } else {
            window.update();
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    Ok(())
}
