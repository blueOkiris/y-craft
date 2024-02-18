//! Implement font loading/wrap SDL code

use sdl2::{
    pixels::Color, rect::Rect, render::{
        Canvas, Texture, TextureCreator
    }, ttf::Sdl2TtfContext, video::{
        Window, WindowContext
    }
};

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

