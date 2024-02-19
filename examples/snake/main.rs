//! Entry point for your Y-Craft application

mod game;
mod title;
mod game_room;

use std::collections::HashMap;
use game::{
    ImgId, SndId, FontId, RmId
};
use sdl2::pixels::Color;
use ycraft::app::run;

const DEF_WIN_WIDTH: u32 = 640;
const DEF_WIN_HEIGHT: u32 = 360;
const FPS: f64 = 60.0;
const BG_COLOR: Color = Color::RGB(0x60, 0x60, 0x80);

fn main() -> Result<(), String> {
    let snds = [
        (SndId::Music, "audio/battleThemeA.mp3"),
        (SndId::Bite, "audio/crack01.mp3.flac")
    ];
    let imgs = [
        (ImgId::Title, "img/title.png"),
        (ImgId::Snake, "img/snake.png"),
        (ImgId::Mouse, "img/mouse.png")
    ];
    let fonts = [
        (FontId::Geist, 20, "fonts/Geist/GeistVariableVF.ttf")
    ];
    let mut rooms = HashMap::from([
        (RmId::Title, title::title()),
        (RmId::Game, game_room::game())
    ]);

    run(
        "Y-Craft", DEF_WIN_WIDTH, DEF_WIN_HEIGHT, FPS, &BG_COLOR,
        RmId::Title, &mut rooms, &snds, &imgs, &fonts
    )
}

