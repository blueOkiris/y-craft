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
    let snds = [
        (Snd::Music, "examples/snake/audio/battleThemeA.mp3"),
        (Snd::Bite, "examples/snake/audio/crack01.mp3.flac")
    ];
    let imgs = [
        (Img::Title, "examples/snake/img/title.png"),
        (Img::Snake, "examples/snake/img/snake.png"),
        (Img::Mouse, "examples/snake/img/mouse.png"),
        (Img::Board, "examples/snake/img/board.png"),
        (Img::Dead, "examples/snake/img/dead.png"),
        (Img::Win, "examples/snake/img/win.png")
    ];
    let fonts = [
        (Fnt::Geist, 20, "examples/snake/fonts/Geist/GeistVariableVF.ttf")
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

