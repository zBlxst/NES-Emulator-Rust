use anyhow::{Error, Result};
use nes_emul::bus::Bus;
use nes_emul::cpu::CPU;
use nes_emul::mem::Mem;
use nes_emul::ppu::PPU;
use nes_emul::rom::Rom;
use nes_emul::screen::{render, Screen};

use nes_emul::screen::frame::Frame;
use sdl2::{Sdl, VideoSubsystem, EventPump};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{WindowContext, Window};
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

use std::io::Read;
use std::fs::File;

use rand::Rng;

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


fn main() -> Result<()>{
    println!("Hello, world!");

    // ================================== CPU initialization ========================================

    let game_path: String = String::from("rom_examples/Pac-Man.nes");
    // let game_path: String = String::from("rom_examples/nestest.nes");
    let mut file: File = File::open(game_path.clone())?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;

    let rom: Rom = Rom::new(&data)?; 
    let bus = Bus::new(rom, |ppu: &PPU, screen: &mut Screen| {
        render::render(ppu, &mut screen.frame);
        let mut texture: Texture<'_> = screen.creator.create_texture_target(PixelFormatEnum::RGB24, 256, 240).expect("Cannot create texture !");
        texture.update(None, &screen.frame.data, 256 * 3).unwrap();
 
        screen.canvas.copy(&texture, None, None).unwrap();
 
        screen.canvas.present();

        for event in screen.event_pump.poll_iter() {
            match event {
              Event::Quit { .. }
              | Event::KeyDown {
                  keycode: Some(Keycode::Escape),
                  ..
              } => std::process::exit(0),
              _ => { /* do nothing */ }
            }
         }
        
    });
    let mut cpu: CPU = CPU::new(bus);
    // cpu.load_program(&game_code)?;
    cpu.reset();

    println!("Start at : {:04x}", cpu.reg_pc);
    
    // let mut screen_state: [u8; 32 * 3 * 32] = [0 as u8; 32 * 3 * 32];
    // let mut rng = rand::thread_rng();

    
    // =============================== Game Loop ======================================
    // cpu.run_with_logs(game_path.as_str())?;
    cpu.run();


    
    // cpu.run_with_callback(move |mut cpu: &mut CPU| {
    //     handle_user_input(&mut cpu, &mut event_pump);
    //     cpu.mem_write_u8(0xfe, rng.gen_range(1, 16));
    //     if read_screen_state(&mut cpu, &mut screen_state) {
    //         texture.update(None, &screen_state, 32*3).map_err(|e| println!("{e}")).ok();
    //         canvas.copy(&texture, None, None).map_err(|e| println!("{e}")).ok();
    //         canvas.present();
    //     }
    //     ::std::thread::sleep(std::time::Duration::new(0, 10_000));
    // }, true);
    
    // =============================== Frame Rendering ======================================
    



    Ok(())

}
