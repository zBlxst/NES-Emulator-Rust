use anyhow::Result;
use nes_emul::bus::Bus;
use nes_emul::cpu::CPU;
use nes_emul::input::Joypad;
use nes_emul::ppu::PPU;
use nes_emul::rom::Rom;
use nes_emul::screen::{render, Screen};

use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Texture;
use sdl2::keyboard::Keycode;
use sdl2::event::Event;

use std::io::{self, Read, Write};
use std::fs::File;

fn main() -> Result<()>{
    let mut game_path: String = String::from("rom_examples/");
    let mut buffer: String = String::new();
    print!("Enter the rom path: rom_examples/");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    game_path.push_str(&buffer[..&buffer.len()-1]);
        
    let mut file: File = File::open(game_path.clone())?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;


    // ================================== CPU initialization ========================================

    let rom: Rom = Rom::new(&data)?; 
    let joypad1: Joypad = Joypad::new();
    let joypad2: Joypad = Joypad::new();
    let bus: Bus = Bus::new(rom, |ppu: &PPU, screen: &mut Screen| {
        render::Renderer::render(ppu, &mut screen.frame);
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
              Event::KeyDown { keycode, .. } => {
                if let Some(key) = screen.bindings_joypad1.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                    screen.joypad1.set_button_pressed_status(*key, true);
                } 

                if let Some(key) = screen.bindings_joypad2.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                    screen.joypad2.set_button_pressed_status(*key, true);
                } 
            },
            Event::KeyUp { keycode, .. } => {
                if let Some(key) = screen.bindings_joypad1.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                    screen.joypad1.set_button_pressed_status(*key, false);
                }
                if let Some(key) = screen.bindings_joypad2.get(&keycode.unwrap_or(Keycode::Ampersand)) {
                    screen.joypad2.set_button_pressed_status(*key, false);
                } 
            },
              _ => ()
            }
         }
        
    }, joypad1, joypad2);

    let mut cpu: CPU = CPU::new(bus);
    cpu.reset();
    cpu.run();

    Ok(())

}
