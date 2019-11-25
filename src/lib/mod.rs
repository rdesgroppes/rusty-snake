use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::EventPump;

pub fn init(width: u32, height: u32) -> (Canvas<Window>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Game", width + 1, height + 1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let event_pump = sdl_context.event_pump().unwrap();
    (canvas, event_pump)
}

pub fn display_rectangle(canvas: &mut Canvas<Window>, canvas_width: u32, canvas_height: u32) {
    let red: u8 = rand::random();
    let green: u8 = rand::random();
    let blue: u8 = rand::random();

    canvas.clear();

    canvas.set_draw_color(Color::RGB(red, green, blue));

    match canvas.fill_rect(Rect::new(0, 0, canvas_width, canvas_height)) {
        Ok(()) => {}
        Err(error) => println!("{}", error),
    }

    canvas.present();
}
