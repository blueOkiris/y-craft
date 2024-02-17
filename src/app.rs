//! Keep track of SDL context and window state as well as run main game loop

use sdl2::{
    render::{
        Canvas, TextureCreator
    }, video::{
        Window, WindowContext
    }, Sdl, VideoSubsystem
};

const DEF_WIN_WIDTH: u32 = 1280;
const DEF_WIN_HEIGHT: u32 = 720;

pub struct App {
    ctx: Sdl,
    subsys: VideoSubsystem,
    win: Window,
    cnv: Canvas<Window>,
    pub creator: TextureCreator<WindowContext>
}

impl App {
    pub fn new(title: &str) -> Result<Self, String> {
        let ctx = sdl2::init()?;
        let subsys = ctx.video()?;
        let win = subsys
            .window(title, DEF_WIN_WIDTH, DEF_WIN_HEIGHT)
            .position_centered()
            .vulkan()
            .build()
            .map_err(|e| e.to_string())?;
        let cnv = win.into_canvas().build().map_err(|e| e.to_string())?;
        let creator = cnv.texture_creator();
        Ok(Self {
            ctx,
            subsys,
            win,
            cnv,
            creator
        })
    }

    pub fn run<SndEnum, ImgEnum, FontEnum, RoomEnum>(
            &mut self, start_room: RoomEnum,
            snds: &Vec<SndEnum>, imgs: &Vec<ImgEnum>, fonts: &Vec<FontEnum>,
            rooms: &Vec<RoomEnum>) -> Result<(), String> {
        Ok(())
    }
}

