//#![allow(unused_variables)]
use std::{thread, time};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod lib;

fn main() {
    let canvas_width = 720_u32;
    let canvas_height = 720_u32;
    let (mut canvas, mut events) = lib::init(canvas_width, canvas_height);

    'game: loop {
        lib::display_rectangle(&mut canvas, canvas_width, canvas_height);
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game,
                _ => continue 'game,
            }
        }
        thread::sleep(time::Duration::from_millis(800));
    }
}
