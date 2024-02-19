//! Global game stuff used by all objs/rooms

use crate::play::Dir;

pub const BASE_MOVE_SPD: f64 = 24.0;
pub const MOVE_SPD_INC: f64 = 8.0;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Img {
    Title,
    Snake,
    Mouse
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Snd {
    Music,
    Bite
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Fnt {
    Geist
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Spr {
    Title,
    Head,
    Body,
    Tail
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Rm {
    Title,
    Play
}

/// Used for sharing data between objects via GameObjectBehavior trait
#[derive(Clone, Copy)]
pub enum Data {
    Title,
    Head {
        dir: Dir,
    }, Body {
        index: isize,
        dir: Dir,
    }, Tail,
    Mouse
}

