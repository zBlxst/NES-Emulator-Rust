pub mod palette;
pub mod frame;
pub mod render;

use anyhow::{Error, Result};

use frame::Frame;
use sdl2::{Sdl, VideoSubsystem, EventPump};
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{WindowContext, Window};
use sdl2::keyboard::Keycode;
use sdl2::event::Event;


const SCALE_FACTOR: u16 = 3;

pub struct Screen {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub creator: TextureCreator<WindowContext>,
    pub frame: Frame
}

impl Screen {

    pub fn new() -> Self {
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
        
        Screen{
            canvas: canvas,
            event_pump: event_pump,
            creator: creator,
            frame: frame
        }
        
    }
}