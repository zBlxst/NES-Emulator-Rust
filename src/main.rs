use anyhow::{Error, Result};
use nes_emul::cpu::CPU;
use nes_emul::mem::Mem;
use nes_emul::rom::Rom;

use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::video::Window;
use sdl2::{Sdl, VideoSubsystem};

use std::io::Read;
use std::fs::File;

use rand::Rng;

const SCALE_FACTOR: u16 = 30;

fn handle_user_input(cpu: &mut CPU, event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. }=> std::process::exit(0),
            Event::KeyDown { keycode: Some(Keycode::Z), .. } => cpu.mem_write_u8(0xff, 0x77),
            Event::KeyDown { keycode: Some(Keycode::S), .. } => cpu.mem_write_u8(0xff, 0x73),
            Event::KeyDown { keycode: Some(Keycode::Q), .. } => cpu.mem_write_u8(0xff, 0x61),
            Event::KeyDown { keycode: Some(Keycode::D), .. } => cpu.mem_write_u8(0xff, 0x64),
            _ => ()
        }
    }
}

fn color(byte: u8) -> Color {
    match byte {
        0 => sdl2::pixels::Color::BLACK,
        1 => sdl2::pixels::Color::WHITE,
        2 | 9 => sdl2::pixels::Color::GREY,
        3 | 10 => sdl2::pixels::Color::RED,
        4 | 11 => sdl2::pixels::Color::GREEN,
        5 | 12 => sdl2::pixels::Color::BLUE,
        6 | 13 => sdl2::pixels::Color::MAGENTA,
        7 | 14 => sdl2::pixels::Color::YELLOW,
        _ => sdl2::pixels::Color::CYAN,
    }
}

fn read_screen_state(cpu: &mut CPU, frame: &mut [u8; 32 * 3 * 32]) -> bool {
    let mut frame_idx: usize = 0;
    let mut update: bool = false;
    for i in 0x0200..0x0600 {
        let color_idx = cpu.mem_read_u8(i as u16);
        let (b1, b2, b3): (u8, u8, u8) = color(color_idx).rgb();
        if frame[frame_idx] != b1 || frame[frame_idx+1] != b2 || frame[frame_idx+2] != b3 {
            frame[frame_idx] = b1;
            frame[frame_idx+1] = b2;
            frame[frame_idx+2] = b3;
            update = true;
        }
        frame_idx += 3;
    }
    update
}   


fn main() -> Result<()> {
    println!("Hello, world!");

    // ================================ Graphics Initialization ==================================
    let sdl_context: Sdl = sdl2::init().map_err(Error::msg)?;
    let video_subsystem: VideoSubsystem = sdl_context.video().map_err(Error::msg)?;
    let window: Window = video_subsystem
        .window("NES Emulator", (32*SCALE_FACTOR) as u32, (32*SCALE_FACTOR) as u32)
        .position_centered()
        .build()?;

    let mut canvas: Canvas<Window> = window.into_canvas().present_vsync().build()?;
    let mut event_pump: EventPump = sdl_context.event_pump().map_err(Error::msg)?;

    canvas.set_scale(SCALE_FACTOR as f32, SCALE_FACTOR as f32).map_err(Error::msg)?;

    let creator: TextureCreator<WindowContext> = canvas.texture_creator();
    let mut texture: Texture<'_> = creator.create_texture_target(PixelFormatEnum::RGB24, 32, 32)?;




    // ================================== CPU initialization ========================================

    let game_path: String = String::from("rom_examples/snake.nes");
    let mut file: File = File::open(game_path.clone())?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;
    
    let mut cpu: CPU = CPU::new(Rom::new(&data)?);
    // cpu.load_program(&game_code)?;
    cpu.reset();
    
    let mut screen_state: [u8; 32 * 3 * 32] = [0 as u8; 32 * 3 * 32];
    let mut rng = rand::thread_rng();


    // =============================== Game Loop ======================================
    cpu.run_with_logs(game_path.as_str())?;


    /*
    cpu.run_with_callback_debug(move |mut cpu: &mut CPU| {
        handle_user_input(&mut cpu, &mut event_pump);
        cpu.mem_write_u8(0xfe, rng.gen_range(1, 16));
        if read_screen_state(&mut cpu, &mut screen_state) {
            texture.update(None, &screen_state, 32*3).map_err(|e| println!("{e}")).ok();
            canvas.copy(&texture, None, None).map_err(|e| println!("{e}")).ok();
            canvas.present();
        }
        ::std::thread::sleep(std::time::Duration::new(0, 10_000));
    });
     */

    
    // cpu.show_stack();
    // cpu.show_stack();
    Ok(())

}
