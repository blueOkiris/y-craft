//! Collect game objects to maintain a type of game object

use std::{
    collections::HashMap, hash::Hash
};
use sdl2::{
    event::Event,
    render::Canvas,
    video::Window
};
use crate::{
    font::Font, obj::GameObjectBehavior, snd::Sound, spr::Image
};

#[derive(Clone)]
pub struct Room<ObjId, SprId, ImgId, SndId, FontId, RmId> where
        ObjId: Hash + Clone + Copy + Eq,
        SprId: Hash + Clone + Copy + Eq,
        ImgId: Hash + Clone + Copy + Eq,
        SndId: Hash + Clone + Copy + Eq,
        FontId: Hash + Clone + Copy + Eq,
        RmId: Hash + Clone + Copy + Eq {
    pub objs: Vec<Box<
        dyn GameObjectBehavior<ObjId, SprId, ImgId, SndId, FontId, RmId>
    >>, pub persistant: bool
}

impl<ObjId, SprId, ImgId, SndId, FontId, RmId> Room<ObjId, SprId, ImgId, SndId, FontId, RmId> where
        ObjId: Hash + Clone + Copy + Eq,
        SprId: Hash + Clone + Copy + Eq,
        ImgId: Hash + Clone + Copy + Eq,
        SndId: Hash + Clone + Copy + Eq,
        FontId: Hash + Clone + Copy + Eq,
        RmId: Hash + Clone + Copy + Eq {
    pub fn new(
            objs: Vec<Box<dyn GameObjectBehavior<ObjId, SprId, ImgId, SndId, FontId, RmId>>>,
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

    pub fn update(&mut self, delta: f64) -> Option<RmId> {
        let others = self.objs.clone();
        let mut ret = None;
        let mut objs = Vec::new();
        for obj in self.objs.iter_mut() {
            let check_ret = obj.update(delta, &others);
            if check_ret.0.is_some() && ret.is_none() && objs.len() < 1 {
                (ret, objs) = check_ret.clone();
            }
            if check_ret.1.len() > 0 && objs.len() < 1 && ret.is_none() {
                (ret, objs) = check_ret;
            }
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
        self.objs.append(&mut objs);
        ret
    }

    pub fn render(
            &mut self, cnv: &mut Canvas<Window>,
            imgs: &HashMap<ImgId, Image>, snds: &HashMap<SndId, Sound>,
            fonts: &HashMap<FontId, Font>,
            elapsed: f64) -> Result<(), String> {
        cnv.clear();
        for obj in self.objs.iter_mut() {
            obj.render(cnv, imgs, snds, fonts, elapsed)?;
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

