use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render;
use sdl2::video;
use std::process;

fn render_board_cells(
    canvas: &mut render::Canvas<video::Window>,
    background_texture: &mut render::Texture,
) {
    canvas.with_texture_canvas(background_texture, |texture_canvas| {
        for y in 0..8 {
            for x in 0..8 {
                if x % 2 == y % 2 {
                    texture_canvas.set_draw_color(Color::RGB(255, 255, 255));
                } else {
                    texture_canvas.set_draw_color(Color::RGB(0, 0, 0));
                }
                texture_canvas.fill_rect(Rect::new(x * 50, y * 50, 50, 50));
            }
        }
    });
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("chess-engine", 800, 600)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let mut background_texture = texture_creator
        .create_texture_target(texture_creator.default_pixel_format(), 400, 400)
        .unwrap();

    render_board_cells(&mut canvas, &mut background_texture);

    let mut event_pump = sdl_context.event_pump().unwrap();

    loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.copy(&background_texture, None, Rect::new(0, 0, 400, 400));
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => process::exit(0),
                _ => {}
            }
        }

        canvas.present();
    }
}
