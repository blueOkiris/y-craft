//! Collect game objects to maintain a type of game object

use std::{
    collections::HashMap, hash::Hash, ops::IndexMut
};
use sdl2::{
    event::Event,
    render::Canvas,
    video::Window
};
use crate::{
    obj::GameObjectBehavior,
    spr::Image
};

pub struct Room<ObjEnum, SpriteEnum, ImageEnum> {
    pub objs: Vec<Box<dyn GameObjectBehavior<ObjEnum, SpriteEnum, ImageEnum>>>,
    pub persistant: bool
}

impl<
    O: Eq + Hash + Clone + Copy,
    S: Eq + Hash + Clone + Copy,
    I: Eq + Hash + Clone + Copy>
        Room<O, S, I> {
    pub fn new(
            objs: Vec<Box<dyn GameObjectBehavior<O, S, I>>>,
            persistant: bool) -> Self {
        Self {
            objs,
            persistant
        }
    }

    pub fn handle_sdl_event(&mut self, event: &Event) {
        for obj in self.objs.iter_mut() {
            obj.handle_sdl_event(event);
        }
    }

    pub fn update(&mut self, delta: f64) {
        let others = self.objs.clone();
        for obj in self.objs.iter_mut() {
            obj.update(delta, &others);
        }
        for obj in self.objs.iter_mut() {
            let collider = obj.state().collider;
            for other in others.iter() {
                if obj.state().name == other.state().name {
                    continue;
                }
                let other_collider = other.state().collider;
                if collider.collides_with(&other_collider) {
                    obj.on_collision(&other);
                }
            }
        }
    }

    pub fn render(
            &mut self, cnv: &mut Canvas<Window>, imgs: &HashMap<I, Image>,
            elapsed: f64) -> Result<(), String> {
        cnv.clear();
        for obj in self.objs.iter_mut() {
            obj.state().render(cnv, imgs, elapsed)?;
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        if self.persistant {
            return;
        }
        for obj in self.objs.iter_mut() {
            obj.on_reset();
        }
    }
}

