mod game;
mod title;
mod play;
mod end;

use std::collections::HashMap;
use sdl2::pixels::Color;
use ycraft::{
    app::run,
    obj::ControlObjectBehavior
};
use game::{
    Fnt, Img, Rm, Score, Snd
};

const DEF_WIN_WIDTH: u32 = 640;
const DEF_WIN_HEIGHT: u32 = 360;
const FPS: f64 = 60.0;
const BG_COLOR: Color = Color::RGB(0x60, 0x60, 0x80);

fn main() -> Result<(), String> {
    // Custom identifier, raw byes, is music?
    let snds = [
        (Snd::Music, include_bytes!("audio/battleThemeA.mp3") as &[u8], true),
        (Snd::Bite, include_bytes!("audio/chomp.wav"), false)
    ];

    // Custom identifier, raw bytes
    let imgs = [
        (Img::Title, include_bytes!("img/title.png") as &[u8]),
        (Img::Snake, include_bytes!("img/snake.png")),
        (Img::Mouse, include_bytes!("img/mouse.png")),
        (Img::Board, include_bytes!("img/board.png")),
        (Img::Dead, include_bytes!("img/dead.png")),
        (Img::Win, include_bytes!("img/win.png"))
    ];

    // Custom identifier, render size, raw bytes
    let fonts = [
        (Fnt::Geist, 20, include_bytes!("fonts/Geist/GeistVariableVF.ttf") as &[u8])
    ];

    let rooms = HashMap::from([
        (Rm::Title, title::title()),
        (Rm::Play, play::play()),
        (Rm::Dead, end::dead()),
        (Rm::Win, end::win())
    ]);
    let ctl_objs: Vec<Box<dyn ControlObjectBehavior<_, _, _, _, _, _>>> = vec![
        Box::new(Score::new())
    ];

    run(
        "Y-Craft", DEF_WIN_WIDTH, DEF_WIN_HEIGHT, FPS, &BG_COLOR,
        Rm::Title, &rooms, &ctl_objs, &snds, &imgs, &fonts
    )
}

