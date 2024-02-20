//! Game over screen

use std::collections::HashMap;

use sdl2::{
    event::Event,
    keyboard::Scancode,
    rect::Rect
};
use ycraft::{
    obj::{
        CollisionShape, Frame, GameObjectBehavior, GameObjectState, Sprite
    }, room::Room
};
use crate::game::{
    Data, Fnt, Img, Rm, Snd, Spr
};

#[derive(Clone)]
struct DeadScreen {
    state: GameObjectState<Img, Spr, Data>,
    change_room: bool
}

impl DeadScreen {
    pub fn new() -> Self {
        Self {
            state: GameObjectState {
                name: "dead".to_string(),
                pos: (0.0, 0.0),
                collider: CollisionShape::Rect { center: (320, 180), size: (640, 480) },
                cur_spr: Spr::Dead,
                sprs: HashMap::from([(
                    Spr::Dead,
                    Sprite::new(
                        vec![Frame::new(
                            Img::Dead, Rect::new(0, 0, 640, 360), (640, 360)
                        )], 0.0, (0, 0)
                    )
                )]), custom: Data::Dead
            }, change_room: false
        }
    }
}

impl GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data> for DeadScreen {
    fn state(&self) -> GameObjectState<Img, Spr, Data> {
        self.state.clone()
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
            _others: &Vec<Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>) -> (
                Option<Rm>,
                Vec<Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>
            ) {
        if self.change_room {
            (Some(Rm::Title), vec![])
        } else {
            (None, vec![])
        }
    }
}

pub fn dead() -> Room<Img, Snd, Fnt, Spr, Rm, Data> {
    Room::new(vec![ Box::new(DeadScreen::new()) ], false)
}

