//! Entry point for your Y-Craft application

mod app;
mod spr;
mod font;
mod snd;

use std::collections::HashMap;

use font::Font;
use spr::Image;
use snd::Sound;
use app::App;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum ImageId {
    Title,
    Snake,
    Mouse
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum SndId {
    Music,
    Bite
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum FontId {
    Geist
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum RoomId {
    Title,
    Game
}

fn main() -> Result<(), String> {
    let app = App::new("Y-Craft")?;

    let snds = HashMap::from([
        (SndId::Music, Sound::load_music("audio/battleThemeA.mp3")),
        (SndId::Bite, Sound::load_chunk("audio/crack01.mp3.flac"))
    ]);
    let imgs = HashMap::from([
        (ImageId::Title, Image::new("img/title.png", &app.creator)),
        (ImageId::Snake, Image::new("img/snake.png", &app.creator)),
        (ImageId::Mouse, Image::new("img/mouse.png", &app.creator))
    ]);
    let fonts = HashMap::from([
        (FontId::Geist, Font::new("fonts/Geist/GeistVariableVF.ttf", 20, &app.ttf_ctx))
    ]);
    let rooms = HashMap::from([]);

    app.run(RoomId::Title, &snds, &imgs, &fonts, &rooms)
}

