mod game;
mod util;

use std::io;
use std::io::Write;
use std::thread::sleep;
use std::time::{Duration, Instant};
use rand::prelude::ThreadRng;
use rand::{Rng, RngCore};
use sdl2::event::Event;
use sdl2::{EventPump};
use sdl2::controller::Button;
use sdl2::gfx::framerate::FPSManager;
use sdl2::keyboard::Keycode;
use sdl2::mixer::{AUDIO_S16LSB, DEFAULT_CHANNELS, InitFlag, Music};
use sdl2::mouse::SystemCursor::No;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use crate::game::Game;


fn initialise_display(resolution: (u32, u32)) -> (Canvas<Window>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    // sdl_context.mouse().show_cursor(false);

    let window = video_subsystem
        .window("Tetris", resolution.0, resolution.1)
        .position_centered()
        // .fullscreen()
        .build()
        .unwrap();

    // sdl_context.mouse().set_relative_mouse_mode(true);

    let canvas = window.into_canvas().build().unwrap();

    let event_pump = sdl_context.event_pump().unwrap();

    (canvas, event_pump)
}

fn reset_timer(frame_count: u64, log_rate: u64, instant: &mut Instant) {
    if frame_count % log_rate != 0 { return; }
    *instant = Instant::now();
}

fn log_elapsed_time(frame_count: u64, log_rate: u64, name: &str, instant: Instant) -> Option<Duration> {
    if frame_count % log_rate != 0 { return None; }
    let elapsed = instant.elapsed();
    col_println!((blue, bold), "{}: {:?}", name, elapsed);
    Some(elapsed)
}

pub fn main() -> Result<(), String> {
    const RESOLUTION: (u32, u32) = (1080, 1080);
    const FRAMERATE: u32 = 60;

    let (mut canvas, mut event_pump) = initialise_display(RESOLUTION);
    let mut rng = rand::thread_rng();
    let mut fps = FPSManager::new();
    fps.set_framerate(FRAMERATE)?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    const PIXEL_SIZE: usize = 1;
    const SQUARE_PIXEL_WIDTH: usize = 40;
    const SQUARE_WIDTH: usize = 15;
    const SQUARE_HEIGHT: usize = 23;
    const WIDTH: usize = SQUARE_PIXEL_WIDTH * SQUARE_WIDTH;
    const HEIGHT: usize = SQUARE_PIXEL_WIDTH * SQUARE_HEIGHT;

    // println!("{:?}", (((RESOLUTION.0 / 2) as usize - (WIDTH / 2)) as i32, ((RESOLUTION.1 / 2) as usize - (HEIGHT / 2)) as i32));

    let mut game =
        Game::<WIDTH, HEIGHT>::new(
            PIXEL_SIZE as u32,
            RESOLUTION,
            (((RESOLUTION.0 / 2) as usize - ((WIDTH * PIXEL_SIZE) / 2)) as i32, ((RESOLUTION.1 / 2) as usize - ((HEIGHT * PIXEL_SIZE) / 2)) as i32),
            SQUARE_PIXEL_WIDTH as u32,
            Color::BLACK
        );

    // sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG)?;
    // let music = Music::from_file("static/Tetris.mp3")?;
    // music.play(-1)?;

    #[cfg(log)]
    const FPS_LOG_RATE: u64 = 100;
    #[cfg(log)]
    let mut frame_start = Instant::now();
    #[cfg(log)]
    const PROFILING_LOG_RATE: u64 = 100;
    #[cfg(log)]
        let mut profile_timer = Instant::now();

    const KEY_SCAN_RATE: u64 = 30;

    let mut frame_count: u64 = 0;
    'main_loop: loop {
        #[cfg(log)]
        if frame_count % FPS_LOG_RATE == 1 {
            frame_start = Instant::now();
        }

        #[cfg(log)]
        if frame_count % PROFILING_LOG_RATE == 0 {
            col_println!((green, bold), "<=====[PROFILE]=====>");
        }


        #[cfg(log)]
        reset_timer(frame_count, PROFILING_LOG_RATE, &mut profile_timer);
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::P), .. } => break 'main_loop,
                Event::KeyDown { keycode: Some(Keycode::Left), .. } | Event::ControllerButtonDown { button: Button::DPadLeft, .. } => game.move_left(),
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => game.move_right(),
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => game.move_down(),
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => game.move_down_amount(10_000),
                Event::KeyDown { keycode: Some(Keycode::R), .. } => game.rotate(),
                _ => {}
            }
        }

        // if frame_count % KEY_SCAN_RATE == 0 {
        //     let keyboard_state = event_pump.keyboard_state();
        //
        //     let keys = keyboard_state.pressed_scancodes().filter_map(Keycode::from_scancode);
        //
        //     for k in keys {
        //         match k {
        //             Keycode::Left => game.move_left(),
        //             Keycode::Right => game.move_right(),
        //             Keycode::Down => game.move_down(),
        //             Keycode::Up => game.move_down_amount(10_000),
        //             Keycode::R => game.rotate(),
        //             _ => {}
        //         }
        //     }
        // }

        #[cfg(log)]
        log_elapsed_time(frame_count, PROFILING_LOG_RATE, "Event Handling", profile_timer);

        #[cfg(log)]
        reset_timer(frame_count, PROFILING_LOG_RATE, &mut profile_timer);
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        #[cfg(log)]
        log_elapsed_time(frame_count, PROFILING_LOG_RATE, "Canvas Events", profile_timer);


        #[cfg(log)]
        reset_timer(frame_count, PROFILING_LOG_RATE, &mut profile_timer);
        game.game_update(&mut rng, frame_count);
        #[cfg(log)]
        log_elapsed_time(frame_count, PROFILING_LOG_RATE, "Game Logic Update", profile_timer);

        #[cfg(log)]
        reset_timer(frame_count, PROFILING_LOG_RATE, &mut profile_timer);
        game.physics_update();
        game.physics_update();
        #[cfg(log)]
        log_elapsed_time(frame_count, PROFILING_LOG_RATE, "Physics Update", profile_timer);

        #[cfg(log)]
        reset_timer(frame_count, PROFILING_LOG_RATE, &mut profile_timer);
        game.draw(&mut canvas);
        #[cfg(log)]
        log_elapsed_time(frame_count, PROFILING_LOG_RATE, "Game Draw", profile_timer);

        #[cfg(log)]
        reset_timer(frame_count, PROFILING_LOG_RATE, &mut profile_timer);
        canvas.present();
        #[cfg(log)]
        log_elapsed_time(frame_count, PROFILING_LOG_RATE, "Canvas Present", profile_timer);

        #[cfg(log)]
        reset_timer(frame_count, PROFILING_LOG_RATE, &mut profile_timer);
        fps.delay();
        #[cfg(log)]
        {
            let t = log_elapsed_time(frame_count, PROFILING_LOG_RATE, "FPS Lock Delay", profile_timer);
            if let Some(t) = t {
                if t < Duration::from_millis(1) {
                    col_println!((red, bold), "Can't keep up!");
                }
            }
        }



        #[cfg(log)]
        if frame_count % FPS_LOG_RATE == 1 {
            let duration = frame_start.elapsed();
            let fps = (1.0 / duration.as_secs_f64()) as u32;
            col_println!((green, bold), "<====[FRAMETIME]====>");
            if fps < FRAMERATE {
                col_println!((red, bold), "{:?} fps - ({:?})", fps, duration);
            }
            else {
                col_println!((blue, bold), "{:?} fps - ({:?})", fps, duration);
            }

        }

        frame_count = frame_count.wrapping_add(1);
    }

    Ok(())
}
