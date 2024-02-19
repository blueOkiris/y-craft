//! Load Images, Fonts, and Sounds (i.e. resources)

use sdl2::{
    image::LoadSurface,
    mixer::{
        Channel, Chunk, Music
    }, pixels::Color,
    rect::Rect,
    render::{
        Canvas, Texture, TextureCreator
    }, surface::Surface,
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
    pub fn new(src: &str, creator: &'a TextureCreator<WindowContext>) -> Result<Self, String> {
        let sfc = Surface::from_file(src)?;
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
    pub fn new(src: &str, size: u16, ttf_ctx: &'a Sdl2TtfContext) -> Result<Self, String> {
        let font = ttf_ctx.load_font(src, size)?;
        Ok(Self {
            font
        })
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
    pub fn load_music(src: &str) -> Result<Self, String> {
        Ok(Self::Music(Music::from_file(src)?))
    }

    pub fn load_chunk(src: &str) -> Result<Self, String> {
        Ok(Self::Chunk(Chunk::from_file(src)?))
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
                music.play(-1).map_err(|e| e.to_string())?;
            }, Sound::Chunk(chunk) => {
                Channel::all().play(chunk, -1).map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }
}


