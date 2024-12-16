pub mod palette;
pub mod frame;
pub mod render;

use std::collections::HashMap;

use anyhow::Error;

use frame::Frame;
use sdl2::{Sdl, VideoSubsystem, EventPump};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{WindowContext, Window};
use sdl2::keyboard::Keycode;

use crate::input::{Joypad, JoypadButton};


const SCALE_FACTOR: u16 = 3;

pub struct Screen {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub creator: TextureCreator<WindowContext>,
    pub frame: Frame,

    pub joypad1: Joypad,
    pub joypad2: Joypad,

    pub bindings_joypad1: HashMap<Keycode, JoypadButton>,
    pub bindings_joypad2: HashMap<Keycode, JoypadButton>
}

impl Screen {

    pub fn new(joypad1: Joypad, joypad2: Joypad) -> Self {

        let mut bindings_joypad1: HashMap<Keycode, JoypadButton> = HashMap::new(); 
        bindings_joypad1.insert(Keycode::Down, JoypadButton::DOWN);
        bindings_joypad1.insert(Keycode::Up, JoypadButton::UP);
        bindings_joypad1.insert(Keycode::Right, JoypadButton::RIGHT);
        bindings_joypad1.insert(Keycode::Left, JoypadButton::LEFT);
        bindings_joypad1.insert(Keycode::Space, JoypadButton::SELECT);
        bindings_joypad1.insert(Keycode::Return, JoypadButton::START);
        bindings_joypad1.insert(Keycode::A, JoypadButton::BUTTON_A);
        bindings_joypad1.insert(Keycode::S, JoypadButton::BUTTON_B);

        let mut bindings_joypad2: HashMap<Keycode, JoypadButton> = HashMap::new(); 
        bindings_joypad2.insert(Keycode::O, JoypadButton::UP);
        bindings_joypad2.insert(Keycode::L, JoypadButton::DOWN);
        bindings_joypad2.insert(Keycode::M, JoypadButton::RIGHT);
        bindings_joypad2.insert(Keycode::K, JoypadButton::LEFT);
        bindings_joypad2.insert(Keycode::B, JoypadButton::SELECT);
        bindings_joypad2.insert(Keycode::C, JoypadButton::START);
        bindings_joypad2.insert(Keycode::D, JoypadButton::BUTTON_A);
        bindings_joypad2.insert(Keycode::E, JoypadButton::BUTTON_B);

        let sdl_context: Sdl = sdl2::init().map_err(Error::msg).expect("Cannot init SDL !");
        let video_subsystem: VideoSubsystem = sdl_context.video().map_err(Error::msg).expect("Cannot init VideoSubSystem !");
        let window: Window = video_subsystem
            .window("NES Emulator", (256*SCALE_FACTOR) as u32, (256*SCALE_FACTOR) as u32)
            .position_centered()
            .build()
            .expect("Cannot create window !");

        let mut canvas: Canvas<Window> = window.into_canvas().present_vsync().build().expect("Cannot build canvas !");
        let event_pump: EventPump = sdl_context.event_pump().expect("Cannot create EventPump !");

        canvas.set_scale(SCALE_FACTOR as f32, SCALE_FACTOR as f32).expect("Cannot scale canvas !");

        let creator: TextureCreator<WindowContext> = canvas.texture_creator();
        let frame: Frame = Frame::new();

        Screen {
            canvas: canvas,
            event_pump: event_pump,
            creator: creator,
            frame: frame,
            joypad1: joypad1,
            joypad2: joypad2,
            bindings_joypad1: bindings_joypad1,
            bindings_joypad2: bindings_joypad2
        }
        
    }
}