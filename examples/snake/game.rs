//! Define custom game objects and rooms here

use sdl2::{
    event::Event, rect::Rect
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
pub enum ImageId {
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
pub enum RoomId {
    Title,
    Game
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum ObjId {
    TitleScreenImage
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum SpriteId {
    Title
}

#[derive(Clone)]
struct TitleScreenImage {
    pub state: GameObjectState<ObjId, SpriteId, ImageId>
}

impl GameObjectBehavior<ObjId, SpriteId, ImageId> for TitleScreenImage {
    fn state(&self) -> GameObjectState<ObjId, SpriteId, ImageId> {
        self.state.clone()
    }

    fn handle_sdl_event(&mut self, _event: &Event) {
        // TODO
    }

    fn update(
        &mut self, _delta: f64,
        _others: &Vec<Box<dyn GameObjectBehavior<ObjId, SpriteId, ImageId>>>) {}
    fn on_collision(
        &mut self, _other: &Box<dyn GameObjectBehavior<ObjId, SpriteId, ImageId>>) {}
    fn on_reset(&mut self) {}
}

pub fn title() -> Room<ObjId, SpriteId, ImageId> {
    Room::new(
        vec![ Box::new(TitleScreenImage {
            state: GameObjectState::new(
                "title", ObjId::TitleScreenImage, (0.0, 0.0),
                CollisionShape::Rect { center: (320, 180), size: (640, 480) },
                SpriteId::Title, &[(
                    SpriteId::Title,
                    Sprite::new(
                        vec![Frame::new(
                            ImageId::Title, Rect::new(0, 0, 640, 360), (640, 360)
                        )], 0.0, (0, 0)
                    )
                )]
            )
        }) ],
        false
    )
}

