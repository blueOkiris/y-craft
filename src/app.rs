//! Keep track of SDL context and window state as well as run main game loop

use std::collections::HashMap;
use sdl2::{
    mixer::{
        AUDIO_S16LSB, DEFAULT_CHANNELS
    }, render::{
        Canvas, TextureCreator
    }, ttf::Sdl2TtfContext,
    video::{
        Window, WindowContext
    }, Sdl, VideoSubsystem
};

use crate::{
    font::Font,
    spr::Image,
    snd::Sound
};

const DEF_WIN_WIDTH: u32 = 1280;
const DEF_WIN_HEIGHT: u32 = 720;

pub struct App {
    ctx: Sdl,
    pub ttf_ctx: Sdl2TtfContext,
    subsys: VideoSubsystem,
    win: Window,
    cnv: Canvas<Window>,
    pub creator: TextureCreator<WindowContext>
}

impl App {
    pub fn new(title: &str) -> Result<Self, String> {
        let ctx = sdl2::init()?;
        let _ = ctx.audio()?;
        let subsys = ctx.video()?;
        let win = subsys
            .window(title, DEF_WIN_WIDTH, DEF_WIN_HEIGHT)
            .position_centered()
            .vulkan()
            .build()
            .map_err(|e| e.to_string())?;
        let cnv = win.into_canvas().build().map_err(|e| e.to_string())?;
        let creator = cnv.texture_creator();
        
        let ttf_ctx = sdl2::ttf::init().map_err(|e| e.to_string())?;

        let freq = 44100;
        let format = AUDIO_S16LSB;
        let channels = DEFAULT_CHANNELS;
        let chunk_size = 1024;
        sdl2::mixer::open_audio(freq, format, channels, chunk_size)?;

        Ok(Self {
            ctx,
            ttf_ctx,
            subsys,
            win,
            cnv,
            creator
        })
    }

    pub fn run<SndEnum, ImgEnum, FontEnum, RoomEnum>(
            &mut self, start_room: RoomEnum,
            snds: &HashMap<SndEnum, Sound>, imgs: &HashMap<ImgEnum, Image>,
            fonts: &HashMap<FontEnum, Font>, rooms: &HashMap<RoomEnum, Room>) -> Result<(), String> {
        Ok(())
    }
}

