use std::f64::consts::PI;
use std::{thread, time};

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use sdl2::keyboard::Keycode;

const K_BUFFER_SIZE: usize = 80;

const WINDOW_WIDTH: u32 = 790;
const WINDOW_HEIGHT: u32 = 800;

const MAX_HEIGHT_X: f64 = 0.5;
const MAX_HEIGHT_Y: f64 = 0.4;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("TwoWaves", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    const WAVELENGTH_X: f64 = 0.8;
    const WAVELENGTH_Y: f64 = 1.2;

    let mut x: f64 = 0.;
    let mut y: f64 = 1.;

    let mut speed_x: f64 = 1.;
    let mut speed_y: f64 = -0.5;

    const FPS: i32 = 200;
    const TIME_INTERVAL: f64 = 1. / FPS as f64;

    let mut height_field: [f64; K_BUFFER_SIZE];

    loop {
        update_wave(&TIME_INTERVAL, &mut x, &mut speed_x);
        update_wave(&TIME_INTERVAL, &mut y, &mut speed_y);

        height_field = [0.; K_BUFFER_SIZE];

        accumulate_wave_to_height_field(&x, &WAVELENGTH_X, &MAX_HEIGHT_X, &mut height_field);
        accumulate_wave_to_height_field(&y, &WAVELENGTH_Y, &MAX_HEIGHT_Y, &mut height_field);

        draw(&mut canvas, &height_field);

        thread::sleep(time::Duration::from_millis((1000f64 / FPS as f64) as u64));

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                    std::process::exit(0)
                },
                _ => {}
            }
        }
    }
}

fn update_wave(time_interval: &f64, x: &mut f64, speed: &mut f64) {
    *x += *time_interval * *speed;

    if *x > 1. {
        *speed *= -1.;

        *x = 1. + *time_interval * *speed;
    }
    else if *x < 0. {
        *speed *= -1.;

        *x= *time_interval * *speed;
    }
}

fn accumulate_wave_to_height_field(
    x: &f64,
    wave_length: &f64,
    max_height: &f64,
    height_field: &mut [f64; K_BUFFER_SIZE]
)
{
    let quarter_wavelength: &f64 = &(0.25 * *wave_length);

    let start: i32 = ((*x - *quarter_wavelength) * K_BUFFER_SIZE as f64) as i32;
    let end: i32 = ((*x + *quarter_wavelength) * K_BUFFER_SIZE as f64) as i32;

    for i in start..end {
        let mut i_new: i32 = i;

        match i {
            ..0 => i_new = -i - 1,
            i if i >= K_BUFFER_SIZE as i32 => i_new = 2 * K_BUFFER_SIZE as i32 - i - 1,
            _=> ()
        }

        let distance: f64 = f64::abs((i as f64 + 0.5) / K_BUFFER_SIZE as f64 - *x);
        let height: f64 = *max_height * 0.5 * (f64::cos(f64::min(distance * PI / *quarter_wavelength, PI)) + 1.);

        height_field[i_new as usize] += height;
    }
}

fn draw(canvas: &mut WindowCanvas, height_field: &[f64; K_BUFFER_SIZE]) {
    canvas.set_draw_color(Color::RGB(0,0,0));

    canvas.clear();

    let gap = WINDOW_WIDTH / (K_BUFFER_SIZE as u32 - 1);

    for (i, (current, next)) in height_field.iter()
        .zip(height_field.iter().skip(1))
        .enumerate()
    {
        const MAX_HEIGHT: f64 = f64::max(MAX_HEIGHT_X, MAX_HEIGHT_Y);

        let p1_x = gap * i as u32;
        let p1_y = f64::floor(WINDOW_HEIGHT as f64 - (WINDOW_HEIGHT as f64 * *current / MAX_HEIGHT)) as u32;

        let p2_x = gap * (i + 1) as u32;
        let p2_y = f64::floor(WINDOW_HEIGHT as f64 - (WINDOW_HEIGHT as f64 * *next / MAX_HEIGHT)) as u32;

        let p1 = Point::new(p1_x as i32, p1_y as i32);
        let p2 = Point::new(p2_x as i32, p2_y as i32);

        canvas.set_draw_color(Color::RGB(38,247,253));

        //canvas.draw_line(p1, p2).unwrap();
        for pixels in 0..gap {
            let x = (p1_x + pixels) as i32;
            let y = equation(&p1, &p2)((p1_x + pixels) as f32) as i32;

            for height in y as u32..=WINDOW_HEIGHT {
                let point = Point::new(x, height as i32);

                canvas.draw_point(point).unwrap();
            }
        }
    }

    canvas.present();
}

fn equation(p1: &Point, p2: &Point) -> impl Fn(f32) -> f32 {
    let m: f32 = (p2.y - p1.y) as f32 / (p2.x - p1.x) as f32;
    let b: f32 = p1.y as f32 - m * p1.x as f32;

    move |x| m * x + b
}