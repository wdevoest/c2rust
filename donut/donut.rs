use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::{Canvas, Texture, TextureCreator};
use std::f64::consts::PI;
use std::ops::Range;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const FONT: &[u8] = b".,-~:;=!*#$@";

fn main() -> Result<(), String> {
    let mut sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut canvas = video_subsystem
        .window("Rust OpenGL", SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let texture_creator: TextureCreator<_> = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_target(sdl2::pixels::TextureAccess::Streaming, SCREEN_WIDTH, SCREEN_HEIGHT)
        .expect("could not create texture");

    let mut frame_buffer = texture.as_mut().lock().unwrap();
    frame_buffer.clear(Color::RGB(0, 0, 0));

    let (mut sin_table, mut cos_table) = build_trig_tables();
    let mut frame_number = 0;
    loop {
        frame_buffer.clear(Color::RGB(0, 0, 0));

        let (mut sin_a, mut cos_a, mut sin_b, mut cos_b);
        unsafe {
            sin_a = sin_table[(frame_number % (sin_table.len() - 1))];
            cos_a = cos_table[(frame_number % (cos_table.len() - 1))];

            frame_number += 1;

            sin_b = sin_table[(frame_number % (sin_table.len() - 1))];
            cos_b = cos_table[(frame_number % (cos_table.len() - 1))];
        }

        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let mut mess = 0.0;
                let mut o = 0;
                for j in 0.0..2.0 * PI {
                    let cos_j = cos_table[(j * 50.0).floor() as usize];
                    for i in 0.0..2.0 * PI {
                        let sin_i = sin_table[(i * 50.0).floor() as usize];
                        mess += 1.0
                            / (sin_i * cos_j * cos_a + cos_table[(j * 50.0).floor() as usize] * sin_a + 5.0);
                        let cosi = cos_table[(i * 50.0).floor() as usize];
                        let t = sin_i * cos_j * cos_a - cos_table[(j * 50.0).floor() as usize] * sin_a;
                        o = (x as usize) + (y as usize) * (80) as usize;
                        if (y as i32 - 22).abs() < 22
                            && x as i32 > 0
                            && x as i32 < 80
                            && mess > frame_buffer[o]
                        {
                            frame_buffer[o] = mess;
                            let mut n = 8.0
                                * ((cos_table[(j * 50.0).floor() as usize] * sin_a
                                    - sin_i * cos_table[(j * 50.0).floor() as usize] * cos_a)
                                    * cos_b
                                    - sin_i * cos_table[(j * 50.0).floor() as usize] * sin_a
                                    - cos_table[(j * 50.0).floor() as usize] * cos_a
                                    - cosi * cos_table[(j * 50.0).floor() as usize] * sin_b);
                            n = if n.is_sign_negative() { 0.0 } else { n };
                            frame_buffer[o] = FONT[n as usize];
                        }
                    }
                }
            }
        }

        texture.update(None, &frame_buffer, SCREEN_WIDTH as usize);
        canvas.clear();
        canvas.copy(&texture, None, None).expect("could not copy texture");
        canvas.present();

        let mut event_pump = sdl_context.event_pump()?;
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => return Ok(()),
                _ => (),
            }
        }
    }
}

fn build_trig_tables() -> (Vec<f64>, Vec<f64>) {
    let sin_arr: Vec<f64> = Range {
        start: 0.0,
        end: 2.0 * PI,
    }
    .step_by(0.02)
    .map(f64::sin)
    .collect();

    let cos_arr: Vec<f64> = Range {
        start: 0.0,
        end: 2.0 * PI,
    }
    .step_by(0.02)
    .map(f64::cos)
    .collect();

    (sin_arr, cos_arr)
}
