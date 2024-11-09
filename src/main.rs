use anyhow::{Error, Result};
use nes_emul::cpu::CPU;

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

fn read_screen_state(cpu: &CPU, frame: &mut [u8; 32 * 3 * 32]) -> bool {
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


fn main() -> Result<()>{
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
    let mut cpu: CPU = CPU::new();

    // This is specific to the snake game
    cpu.set_program_base(0x0600)?;

    let game_code: Vec<u8> = vec![
        0x20, 0x06, 0x06, 0x20, 0x38, 0x06, 0x20, 0x0d, 0x06, 0x20, 0x2a, 0x06, 0x60, 0xa9, 0x02, 0x85,
        0x02, 0xa9, 0x04, 0x85, 0x03, 0xa9, 0x11, 0x85, 0x10, 0xa9, 0x10, 0x85, 0x12, 0xa9, 0x0f, 0x85,
        0x14, 0xa9, 0x04, 0x85, 0x11, 0x85, 0x13, 0x85, 0x15, 0x60, 0xa5, 0xfe, 0x85, 0x00, 0xa5, 0xfe,
        0x29, 0x03, 0x18, 0x69, 0x02, 0x85, 0x01, 0x60, 0x20, 0x4d, 0x06, 0x20, 0x8d, 0x06, 0x20, 0xc3,
        0x06, 0x20, 0x19, 0x07, 0x20, 0x20, 0x07, 0x20, 0x2d, 0x07, 0x4c, 0x38, 0x06, 0xa5, 0xff, 0xc9,
        0x77, 0xf0, 0x0d, 0xc9, 0x64, 0xf0, 0x14, 0xc9, 0x73, 0xf0, 0x1b, 0xc9, 0x61, 0xf0, 0x22, 0x60,
        0xa9, 0x04, 0x24, 0x02, 0xd0, 0x26, 0xa9, 0x01, 0x85, 0x02, 0x60, 0xa9, 0x08, 0x24, 0x02, 0xd0,
        0x1b, 0xa9, 0x02, 0x85, 0x02, 0x60, 0xa9, 0x01, 0x24, 0x02, 0xd0, 0x10, 0xa9, 0x04, 0x85, 0x02,
        0x60, 0xa9, 0x02, 0x24, 0x02, 0xd0, 0x05, 0xa9, 0x08, 0x85, 0x02, 0x60, 0x60, 0x20, 0x94, 0x06,
        0x20, 0xa8, 0x06, 0x60, 0xa5, 0x00, 0xc5, 0x10, 0xd0, 0x0d, 0xa5, 0x01, 0xc5, 0x11, 0xd0, 0x07,
        0xe6, 0x03, 0xe6, 0x03, 0x20, 0x2a, 0x06, 0x60, 0xa2, 0x02, 0xb5, 0x10, 0xc5, 0x10, 0xd0, 0x06,
        0xb5, 0x11, 0xc5, 0x11, 0xf0, 0x09, 0xe8, 0xe8, 0xe4, 0x03, 0xf0, 0x06, 0x4c, 0xaa, 0x06, 0x4c,
        0x35, 0x07, 0x60, 0xa6, 0x03, 0xca, 0x8a, 0xb5, 0x10, 0x95, 0x12, 0xca, 0x10, 0xf9, 0xa5, 0x02,
        0x4a, 0xb0, 0x09, 0x4a, 0xb0, 0x19, 0x4a, 0xb0, 0x1f, 0x4a, 0xb0, 0x2f, 0xa5, 0x10, 0x38, 0xe9,
        0x20, 0x85, 0x10, 0x90, 0x01, 0x60, 0xc6, 0x11, 0xa9, 0x01, 0xc5, 0x11, 0xf0, 0x28, 0x60, 0xe6,
        0x10, 0xa9, 0x1f, 0x24, 0x10, 0xf0, 0x1f, 0x60, 0xa5, 0x10, 0x18, 0x69, 0x20, 0x85, 0x10, 0xb0,
        0x01, 0x60, 0xe6, 0x11, 0xa9, 0x06, 0xc5, 0x11, 0xf0, 0x0c, 0x60, 0xc6, 0x10, 0xa5, 0x10, 0x29,
        0x1f, 0xc9, 0x1f, 0xf0, 0x01, 0x60, 0x4c, 0x35, 0x07, 0xa0, 0x00, 0xa5, 0xfe, 0x91, 0x00, 0x60,
        0xa6, 0x03, 0xa9, 0x00, 0x81, 0x10, 0xa2, 0x00, 0xa9, 0x01, 0x81, 0x10, 0x60, 0xa2, 0x00, 0xea,
        0xea, 0xca, 0xd0, 0xfb, 0x60
    ];
    cpu.load_program(&game_code)?;
    cpu.reset();

    let mut screen_state: [u8; 32 * 3 * 32] = [0 as u8; 32 * 3 * 32];
    let mut rng = rand::thread_rng();


    // =============================== Game Loop ======================================
    cpu.run_with_callback(move |mut cpu: &mut CPU| {
        handle_user_input(&mut cpu, &mut event_pump);
        cpu.mem_write_u8(0xfe, rng.gen_range(1, 16));
        if read_screen_state(&mut cpu, &mut screen_state) {
            texture.update(None, &screen_state, 32*3).map_err(|e| println!("{e}")).ok();
            canvas.copy(&texture, None, None).map_err(|e| println!("{e}")).ok();
            canvas.present();
        }
        ::std::thread::sleep(std::time::Duration::new(0, 10_000));
    });
    
    // cpu.show_stack();
    // cpu.show_stack();
    Ok(())

}
