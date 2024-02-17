//! Entry point for your Y-Craft application

mod spr;
mod app;

use std::collections::HashMap;

use spr::Image;
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

    let audio = HashMap::from([]);
    let images = HashMap::from([
        (ImageId::Title, Image::new("img/title.png", &app.creator)),
        (ImageId::Snake, Image::new("img/snake.png", &app.creator)),
        (ImageId::Mouse, Image::new("img/mouse.png", &app.creator))
    ]);
    let fonts = HashMap::from([]);
    let rooms = HashMap::from([]);

    app.run(RoomId::Title, &snds, &imgs, &fonts, &rooms);
}

