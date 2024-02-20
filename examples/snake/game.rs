//! Global game stuff used by all objs/rooms

use std::collections::HashMap;
use sdl2::{
    pixels::Color,
    render::{
        Canvas, TextureCreator
    }, video::{
        Window, WindowContext
    }
};
use ycraft::{
    obj::{
        ControlObjectBehavior, GameObjectBehavior
    }, res::{
        Font, Image, Sound
    }
};
use crate::play::Dir;

pub const BASE_MOVE_SPD: f64 = 24.0;
pub const MOVE_SPD_INC: f64 = 4.0;
const END_STATE: usize = ((640 - 64) / 32) * ((360 - 64) / 32);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Img {
    Title,
    Snake,
    Mouse,
    Board,
    Dead,
    Win
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
    Tail,
    Mouse,
    Board,
    Dead,
    Win
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Rm {
    Title,
    Play,
    Dead,
    Win
}

/// Used for sharing data between objects via GameObjectBehavior trait
#[derive(Clone, Copy)]
pub enum Data {
    Title,
    Head {
        dir: Dir,
        lurch_propagation: usize
    }, Body {
        index: isize,
        dir: Dir
    }, Tail, Mouse,
    Board,
    Dead,
    Score(usize),
    Win
}

#[derive(Clone)]
pub struct Score {
    data: Data
}

impl Score {
    pub fn new() -> Self {
        Self {
            data: Data::Score(4)
        }
    }
}

impl ControlObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data> for Score {
    fn data(&self) -> Data {
        self.data.clone()
    }

    fn update(
            &mut self, _delta: f64, cur_room: &Rm,
            _others: &Vec<Box<dyn ControlObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>,
            room_objs: &Vec<Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>) -> (
                Option<Rm>, Vec<Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>
            ) {
        match cur_room {
            Rm::Title => self.data = Data::Score(4),
            Rm::Play => if let Data::Score(ref mut score) = self.data {
                *score = 2; // Start with tail and head
                for obj in room_objs.iter() {
                    if let Data::Body { .. } = obj.state().custom {
                        *score += 1;
                    }
                }
                if *score >= END_STATE {
                    return (Some(Rm::Win), vec![]);
                }
            }, Rm::Dead => {},
            Rm::Win => {}
        }
        (None, vec![])
    }

    fn render(
            &mut self, cnv: &mut Canvas<Window>, cur_room: &Rm,
            _imgs: &HashMap<Img, Image>, snds: &HashMap<Snd, Sound>,
            fonts: &HashMap<Fnt, Font>, creator: &TextureCreator<WindowContext>,
            _elapsed: f64) -> Result<(), String> {
        if !Sound::is_music_playing() {
            snds[&Snd::Music].play()?;
        }
        match cur_room {
            Rm::Play | Rm::Dead => if let Data::Score(score) = self.data {
                fonts[&Fnt::Geist].render(
                    cnv, creator, format!("Score: {}", score).as_str(), &Color::WHITE,
                    (16, 16), 0.0, (false, false)
                )?;
            }, _ => {}
        }
        Ok(())
    }
}

