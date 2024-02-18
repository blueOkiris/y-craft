//! Entry point for your Y-Craft application

mod app;
mod spr;
mod font;
mod snd;
mod obj;
mod room;
mod game;

use std::collections::HashMap;
use game::{
    ImageId, SndId, FontId, RoomId
};
use app::App;

const DEF_WIN_WIDTH: u32 = 1280;
const DEF_WIN_HEIGHT: u32 = 720;

fn main() -> Result<(), String> {
    let snds = [
        (SndId::Music, "audio/battleThemeA.mp3"),
        (SndId::Bite, "audio/crack01.mp3.flac")
    ];
    let imgs = [
        (ImageId::Title, "img/title.png"),
        (ImageId::Snake, "img/snake.png"),
        (ImageId::Mouse, "img/mouse.png")
    ];
    let fonts = [
        (FontId::Geist, 20, "fonts/Geist/GeistVariableVF.ttf")
    ];
    let mut rooms = HashMap::from([
        (RoomId::Title, game::title())
    ]);

    App::new("Y-Craft", DEF_WIN_WIDTH, DEF_WIN_HEIGHT)?
        .run(RoomId::Title, &mut rooms, &snds, &imgs, &fonts)
}

