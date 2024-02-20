//! Collect game objects to maintain a type of game object

use std::collections::HashMap;
use sdl2::{
    event::Event,
    render::{
        Canvas, TextureCreator
    }, video::{
        Window, WindowContext
    }
};
use crate::{
    obj::{
        CollisionShape, ControlObjectBehavior, GameObjectBehavior
    }, res::{
        Font, Image, Sound
    }, IndexRestriction
};

#[derive(Clone)]
pub struct Room<Img, Snd, Fnt, Spr, Rm, Data> where
        Spr: IndexRestriction,
        Img: IndexRestriction,
        Snd: IndexRestriction,
        Fnt: IndexRestriction,
        Rm: IndexRestriction,
        Data: Clone {
    pub objs: Vec<Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>,
    pub persistant: bool
}

impl<Img, Snd, Fnt, Spr, Rm, Data> Room<Img, Snd, Fnt, Spr, Rm, Data> where
        Spr: IndexRestriction,
        Img: IndexRestriction,
        Snd: IndexRestriction,
        Fnt: IndexRestriction,
        Rm: IndexRestriction,
        Data: Clone {
    pub fn new(
            objs: Vec<Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>,
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

    pub fn update(
            &mut self, delta: f64,
            ctl_objs: &Vec<Box<dyn ControlObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>
            ) -> Option<Rm> {
        let others = self.objs.clone();
        let mut ret = None;
        let mut objs = Vec::new();
        for obj in self.objs.iter_mut() {
            let check_ret = obj.update(delta, ctl_objs, &others);
            if check_ret.0.is_some() && ret.is_none() && objs.len() < 1 {
                (ret, objs) = check_ret.clone();
            }
            if check_ret.1.len() > 0 && objs.len() < 1 && ret.is_none() {
                (ret, objs) = check_ret;
            }
        }
        let others = self.objs.clone();
        for obj in self.objs.iter_mut() {
            let mut collider = obj.state().collider;

            // Adjust collider for position
            match collider {
                CollisionShape::Rect { ref mut center, .. } => {
                    center.0 += obj.state().pos.0 as i32;
                    center.1 += obj.state().pos.1 as i32;
                }, CollisionShape::Circle { ref mut center, .. } => {
                    center.0 += obj.state().pos.0 as i32;
                    center.1 += obj.state().pos.1 as i32;
                }
            }

            for other in others.iter() {
                if obj.state().name == other.state().name {
                    continue;
                }

                let mut other_collider = other.state().collider;

                // Also adjust other collider
                match other_collider {
                    CollisionShape::Rect { ref mut center, .. } => {
                        center.0 += other.state().pos.0 as i32;
                        center.1 += other.state().pos.1 as i32;
                    }, CollisionShape::Circle { ref mut center, .. } => {
                        center.0 += other.state().pos.0 as i32;
                        center.1 += other.state().pos.1 as i32;
                    }
                }

                if collider.collides_with(&other_collider) {
                    obj.on_collision(&other);
                }
            }
        }
        self.objs.append(&mut objs);
        ret
    }

    pub fn render(
            &mut self, cnv: &mut Canvas<Window>,
            imgs: &HashMap<Img, Image>, snds: &HashMap<Snd, Sound>,
            fonts: &HashMap<Fnt, Font>, creator: &TextureCreator<WindowContext>,
            elapsed: f64) -> Result<(), String> {
        cnv.clear();
        for obj in self.objs.iter_mut() {
            obj.render(cnv, imgs, snds, fonts, creator, elapsed)?;
        }
        Ok(())
    }

    pub fn reset(&mut self) {
        if self.persistant {
            return;
        }
        let mut new_obs = Vec::new();
        for obj in self.objs.iter() {
            let mut obj_clone = obj.clone();
            let res = obj_clone.on_reset();
            if !res {
                new_obs.push(obj_clone);
            }
        }
        self.objs = new_obs.clone();
    }
}

