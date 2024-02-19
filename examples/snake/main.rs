mod game;
mod title;
mod play;

use std::collections::HashMap;
use game::{
    Img,
    Snd,
    Fnt,
    Rm
};
use sdl2::pixels::Color;
use ycraft::app::run;

const DEF_WIN_WIDTH: u32 = 640;
const DEF_WIN_HEIGHT: u32 = 360;
const FPS: f64 = 60.0;
const BG_COLOR: Color = Color::RGB(0x60, 0x60, 0x80);

fn main() -> Result<(), String> {
    let snds = [
        (Snd::Music, "audio/battleThemeA.mp3"),
        (Snd::Bite, "audio/crack01.mp3.flac")
    ];
    let imgs = [
        (Img::Title, "img/title.png"),
        (Img::Snake, "img/snake.png"),
        (Img::Mouse, "img/mouse.png")
    ];
    let fonts = [
        (Fnt::Geist, 20, "fonts/Geist/GeistVariableVF.ttf")
    ];

    let mut rooms = HashMap::from([
        (Rm::Title, title::title()),
        (Rm::Play, play::play())
    ]);

    run(
        "Y-Craft", DEF_WIN_WIDTH, DEF_WIN_HEIGHT, FPS, &BG_COLOR,
        Rm::Title, &rooms, &snds, &imgs, &fonts
    )
}

