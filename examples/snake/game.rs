//! Define custom game objects and rooms here

use sdl2::{
    event::Event, keyboard::Scancode, rect::Rect
};
use ycraft::{
    obj::{
        CollisionShape, GameObjectBehavior, GameObjectState
    }, room::Room,
    spr::{
        Frame, Sprite
    }
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ImgId {
    Title,
    Snake,
    Mouse
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum SndId {
    Music,
    Bite
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum FontId {
    Geist
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum RmId {
    Title,
    Game
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ObjId {
    TitleScreenImage
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum SprId {
    Title
}

#[derive(Clone)]
struct TitleScreenImage {
    pub state: GameObjectState<ObjId, SprId, ImgId>,
    change_room: bool
}

impl GameObjectBehavior<ObjId, SprId, ImgId, SndId, FontId, RmId> for TitleScreenImage {
    fn state(&self) -> GameObjectState<ObjId, SprId, ImgId> {
        self.state.clone()
    }

    fn handle_sdl_event(&mut self, event: &Event) {
        match event {
            Event::KeyUp { scancode, .. } if *scancode == Some(Scancode::Return) => {
                self.change_room = true;
            }, _ => {}
        }
    }

    fn update(
            &mut self, _delta: f64,
            _others: &Vec<Box<
                dyn GameObjectBehavior<ObjId, SprId, ImgId, SndId, FontId, RmId>
            >>) -> Option<RmId> {
        if self.change_room {
            Some(RmId::Game)
        } else {
            None
        }
    }

    fn on_collision(
        &mut self,
        _other: &Box<dyn GameObjectBehavior<ObjId, SprId, ImgId, SndId, FontId, RmId>>) {}
    fn on_reset(&mut self) {}
}

pub fn title() -> Room<ObjId, SprId, ImgId, SndId, FontId, RmId> {
    Room::new(
        vec![ Box::new(TitleScreenImage {
            state: GameObjectState::new(
                "title", ObjId::TitleScreenImage, (0.0, 0.0),
                CollisionShape::Rect { center: (320, 180), size: (640, 480) },
                SprId::Title, &[(
                    SprId::Title,
                    Sprite::new(
                        vec![Frame::new(
                            ImgId::Title, Rect::new(0, 0, 640, 360), (640, 360)
                        )], 0.0, (0, 0)
                    )
                )]
            ), change_room: false
        }) ],
        false
    )
}

pub fn game() -> Room<ObjId, SprId, ImgId, SndId, FontId, RmId> {
    Room::new(
        vec![

        ], false
    )
}

