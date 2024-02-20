mod game;

use std::collections::HashMap;
use sdl2::pixels::Color;
use ycraft::{
    app::run,
    obj::ControlObjectBehavior
};
use game::{
    Fnt, Img, Rm, Snd
};

const DEF_WIN_WIDTH: u32 = 1920;
const DEF_WIN_HEIGHT: u32 = 1080;
const FPS: f64 = 60.0;
const BG_COLOR: Color = Color::RGB(0x60, 0x60, 0x80);

fn main() -> Result<(), String> {
    // Custom identifier, raw byes, is music?
    let snds = [
        (Snd::Music, include_bytes!("audio/awake10_megaWall.mp3") as &[u8], true),
        (Snd::Jump, include_bytes!("audio/sfx_movement_jump10.wav"), false)
    ];

    // Custom identifier, raw bytes
    let imgs = [
        (Img::Brick, include_bytes!("img/brick.png") as &[u8]),
        (Img::Character, include_bytes!("img/character.png"))
    ];

    // Custom identifier, render size, raw bytes
    let fonts = [
        (Fnt::Geist, 20, include_bytes!("fonts/Geist/GeistVariableVF.ttf") as &[u8])
    ];

    let rooms = HashMap::from([
        (Rm::Room0, game::room0())
    ]);
    let ctl_objs: Vec<Box<dyn ControlObjectBehavior<_, _, _, _, _, _>>> = vec![
    ];

    run(
        "Y-Craft", DEF_WIN_WIDTH, DEF_WIN_HEIGHT, FPS, &BG_COLOR,
        Rm::Room0, &rooms, &ctl_objs, &snds, &imgs, &fonts
    )
}

