//! Game objects and room code for title screen

use sdl2::{
    event::Event,
    keyboard::Scancode,
    rect::Rect
};
use ycraft::{
    obj::{
        CollisionShape, GameObjectBehavior, GameObjectState
    }, room::Room,
    spr::{
        Frame, Sprite
    }
};
use crate::game::{
    FontId, ImgId, ObjId, RmId, SndId, SprId
};

#[derive(Clone)]
struct TitleScreenImage {
    state: GameObjectState<ObjId, SprId, ImgId>,
    change_room: bool
}

impl TitleScreenImage {
    pub fn new() -> Self {
        Self {
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
        }
    }
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
            >>) -> (
                Option<RmId>,
                Vec<Box<dyn GameObjectBehavior<ObjId, SprId, ImgId, SndId, FontId, RmId>>>
            ) {
        if self.change_room {
            (Some(RmId::Game), vec![])
        } else {
            (None, vec![])
        }
    }
}

pub fn title() -> Room<ObjId, SprId, ImgId, SndId, FontId, RmId> {
    Room::new(
        vec![ Box::new(TitleScreenImage::new()) ],
        false
    )
}

