mod game;
mod title;
mod play;
mod dead;

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
        (Snd::Music, "examples/snake/audio/battleThemeA.mp3"),
        (Snd::Bite, "examples/snake/audio/crack01.mp3.flac")
    ];
    let imgs = [
        (Img::Title, "examples/snake/img/title.png"),
        (Img::Snake, "examples/snake/img/snake.png"),
        (Img::Mouse, "examples/snake/img/mouse.png"),
        (Img::Board, "examples/snake/img/board.png"),
        (Img::Dead, "examples/snake/img/dead.png")
    ];
    let fonts = [
        (Fnt::Geist, 20, "examples/snake/fonts/Geist/GeistVariableVF.ttf")
    ];

    let rooms = HashMap::from([
        (Rm::Title, title::title()),
        (Rm::Play, play::play()),
        (Rm::Dead, dead::dead())
    ]);

    run(
        "Y-Craft", DEF_WIN_WIDTH, DEF_WIN_HEIGHT, FPS, &BG_COLOR,
        Rm::Title, &rooms, &snds, &imgs, &fonts
    )
}

