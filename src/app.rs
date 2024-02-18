//! Keep track of SDL context and window state as well as run main game loop

use std::{collections::HashMap, hash::Hash};
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
    snd::Sound,
    room::Room
};

pub struct App {
    ctx: Sdl,
    pub ttf_ctx: Sdl2TtfContext,
    subsys: VideoSubsystem,
    cnv: Canvas<Window>,
    pub creator: TextureCreator<WindowContext>
}

impl App {
    pub fn new(title: &str, width: u32, height: u32) -> Result<Self, String> {
        let ctx = sdl2::init()?;
        let _ = ctx.audio()?;
        let subsys = ctx.video()?;
        let win = subsys
            .window(title, width, height)
            .position_centered()
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
            cnv,
            creator
        })
    }

    /// After opening a window, load game resources and code and enter a game room
    pub fn run<
        'a, 'b, SndEnum: Hash + Eq, ImgEnum: Hash + Eq, FontEnum: Hash + Eq,
        RoomEnum, ObjEnum, SpriteEnum>(
            &mut self, start_room: RoomEnum,
            rooms: &mut HashMap<RoomEnum, Room<ObjEnum, SpriteEnum, ImgEnum>>,
            snd_srcs: &[(SndEnum, &str)], img_srcs: &[(ImgEnum, &str)],
            font_srcs: &[(FontEnum, u16, &str)]) -> Result<(), String> {
        // Load resources from file paths
        let mut snds = HashMap::new();
        for (key, src) in snd_srcs.iter() {
            snds.insert(key, Sound::load_music(src)?);
        }
        let mut imgs = HashMap::new();
        for (key, src) in img_srcs.iter() {
            imgs.insert(key, Image::new(src, &self.creator)?);
        }
        let mut fonts = HashMap::new();
        for (key, size, src) in font_srcs.iter() {
            fonts.insert(key, Font::new(src, *size, &self.ttf_ctx)?);
        }
        Ok(())
    }
}

