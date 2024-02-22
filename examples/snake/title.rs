//! Game objects and room code for title screen

use std::collections::HashMap;
use sdl2::{
    event::Event,
    keyboard::Scancode,
    rect::Rect
};
use ycraft::{
    collision::CollisionShape,
    obj::{
        ControlObjectBehavior, Frame, GameObjectBehavior, GameObjectState, Sprite
    }, room::Room
};
use crate::game::{
    Img, Snd, Fnt, Spr, Rm, Data
};

#[derive(Clone)]
struct TitleScreenImage {
    state: GameObjectState<Img, Spr, Data>,
    change_room: bool
}

impl TitleScreenImage {
    pub fn new() -> Self {
        Self {
            state: GameObjectState {
                name: "title".to_string(),
                pos: (0.0, 0.0),
                collider: CollisionShape::Rect { center: (320, 180), size: (640, 480) },
                cur_spr: Spr::Title,
                sprs: HashMap::from([(
                    Spr::Title,
                    Sprite::new(
                        vec![Frame::new(
                            Img::Title, Rect::new(0, 0, 640, 360), (640, 360)
                        )], 0.0, (0, 0)
                    )
                )]), custom: Data::Title
            }, change_room: false
        }
    }
}

impl GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data> for TitleScreenImage {
    fn state(&self) -> GameObjectState<Img, Spr, Data> {
        self.state.clone()
    }

    fn set_state(&mut self, new_state: &GameObjectState<Img, Spr, Data>) {
        self.state = new_state.clone();
    }

    fn on_reset(&mut self) -> bool {
        self.change_room = false;
        false
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
            _ctl_objs: &Vec<Box<dyn ControlObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>,
            _others: &Vec<Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>) -> (
                Option<Rm>,
                Vec<Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>
            ) {
        if self.change_room {
            (Some(Rm::Play), vec![])
        } else {
            (None, vec![])
        }
    }
}

pub fn title() -> Room<Img, Snd, Fnt, Spr, Rm, Data> {
    Room::new(
        vec![ Box::new(TitleScreenImage::new()) ],
        false
    )
}

