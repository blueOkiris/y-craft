//! Load Images, Fonts, and Sounds (i.e. resources)

use std::io::{
    Read, Seek, SeekFrom
};
use image::{
    ImageBuffer, Rgba
};
use sdl2::{
    mixer::{
        Channel, Chunk, Music
    }, pixels::{
        Color, PixelFormatEnum
    }, rect::Rect,
    render::{
        Canvas, Texture, TextureCreator
    }, rwops::RWops,
    surface::Surface,
    ttf::Sdl2TtfContext,
    video::{
        Window, WindowContext
    }
};

/// Container for textures with functionality for drawing to screen. This is a "resource" and does
/// not go with GameObjects
pub struct Image<'a> {
    tex: Texture<'a>
}

impl<'a> Image<'a> {
    pub fn new(
            img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
            creator: &'a TextureCreator<WindowContext>) -> Result<Self, String> {
        let mut img_data = img.pixels().flat_map(|px| px.0).collect::<Vec<u8>>();
        let pitch = img.width() * 4;
        let sfc = Surface::from_data(
            &mut img_data, img.width(), img.height(), pitch, PixelFormatEnum::RGBA32
        )?;
        let tex = Texture::from_surface(&sfc, creator).map_err(|e| e.to_string())?;
        Ok(Self {
            tex
        })
    }

    pub fn render(
            &self, cnv: &mut Canvas<Window>, src: &Rect, dest: &Rect,
            angle: f64, flip: (bool, bool)) -> Result<(), String> {
        cnv.copy_ex(&self.tex, Some(*src), Some(*dest), angle, None, flip.0, flip.1)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

pub struct Font<'a, 'b> {
    font: sdl2::ttf::Font<'a, 'b>
}

impl<'a, 'b> Font<'a, 'b> {
    pub fn new(font_data: &'b [u8], size: u16, ttf_ctx: &'a Sdl2TtfContext) -> Result<Self, String> {
        Ok(Self {
            font: Self::load_font_from_bytes(ttf_ctx, font_data, size)?
        })
    }

    fn load_font_from_bytes(
            ttf_context: &'a Sdl2TtfContext,
            font_data: &'b [u8],
            size: u16) -> Result<sdl2::ttf::Font<'a, 'b>, String> {
        let font_reader = RWops::from_bytes(font_data)?;
        let font = ttf_context.load_font_from_rwops(font_reader, size);
        font.map_err(|e| e.to_string())
    }

    pub fn render(
            &self, cnv: &mut Canvas<Window>, creator: &TextureCreator<WindowContext>,
            msg: &str, color: &Color, pos: (i32, i32), angle: f64,
            flip: (bool, bool)) -> Result<(), String> {
        let sfc = self.font.render(msg)
            .solid(*color)
            .map_err(|e| e.to_string())?;
        let tex = Texture::from_surface(&sfc, creator).map_err(|e| e.to_string())?;
        let width = sfc.width();
        let height = sfc.height();
        let dest = Rect::new(pos.0, pos.1, width, height);
        cnv.copy_ex(&tex, None, Some(dest), angle, None, flip.0, flip.1)
    }
}

pub enum Sound<'a> {
    Music(Music<'a>),
    Chunk(Chunk)
}

impl<'a> Sound<'a> {
    pub fn load_music(src: &'static [u8]) -> Result<Self, String> {
        Ok(Self::Music(Music::from_static_bytes(src)?))
    }

    pub fn load_chunk(src: &[u8]) -> Result<Self, String> {
        Ok(Self::Chunk(Self::load_chunk_from_bytes(src)?))
    }

    fn load_chunk_from_bytes(src: &[u8]) -> Result<sdl2::mixer::Chunk, String> {
        let rw_ops = RWops::from_bytes(src)?;
        let buff = Self::rwops_to_boxed_slice(rw_ops)?;
        Chunk::from_raw_buffer(buff)
    }

    fn rwops_to_boxed_slice(mut rwops: RWops) -> Result<Box<[u8]>, String> {
        let mut buffer = Vec::<u8>::new();
        rwops.seek(SeekFrom::Start(0))
            .map_err(|e| e.to_string())?;
        rwops.read_to_end(&mut buffer).map_err(|e| e.to_string())?;
        let boxed_slice = buffer.into_boxed_slice();
        Ok(boxed_slice)
    }

    pub fn is_music_playing() -> bool {
        Music::is_playing()
    }

    pub fn pause_music() {
        Music::pause()
    }

    pub fn resume_music() {
        Music::resume()
    }

    pub fn halt_music() {
        Music::halt()
    }

    pub fn play(&self) -> Result<(), String> {
        match self {
            Sound::Music(music) => {
                music.play(0).map_err(|e| e.to_string())?;
            }, Sound::Chunk(chunk) => {
                Channel::all().play(chunk, 0).map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }
}


