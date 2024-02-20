//! Generic "GameObject" system that defines sets of behaviors using enums for resources/obj data

use std::{
    collections::HashMap,
    hash::Hash
};
use sdl2::{
    event::Event,
    rect::Rect,
    render::{
        Canvas, TextureCreator
    }, video::{
        Window, WindowContext
    }
};
use crate::{
    res::{
        Font,
        Sound,
        Image
    }, IndexRestriction
};

/// Every game object should have these parameters to return them via state()
///
/// - name: String,
/// - pos: (f64, f64)
/// - collider: CollisionShape
/// - cur_spr: SprId (a custom enum defined by you to distinguish between sprites)
/// - sprs: HashMap<SprId, Sprite<ImgId>> (a mapping of sprite ids to sprites)
/// - custom: Data (a custom enum containing data for all your objects)
#[derive(Clone)]
pub struct GameObjectState<Img, Spr, Data> where
        Spr: IndexRestriction,
        Img: IndexRestriction,
        Data: Clone {
    pub name: String,
    pub pos: (f64, f64),
    pub collider: CollisionShape,
    pub cur_spr: Spr,
    pub sprs: HashMap<Spr, Sprite<Img>>,
    pub custom: Data
}

/// All game objects should implement these
///
/// Note: the generic types refer to custom enums for indexing items:
///
/// - Img -> Id for going between the various Images loaded in
/// - Snd -> Same but for sounds
/// - Fnt -> Font
/// - Spr -> Sprites
/// - Rm -> Rooms
/// - Data -> Custom data for each object
pub trait GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>:
    GameObjectBehaviorClone<Img, Snd, Fnt, Spr, Rm, Data> where
        Spr: IndexRestriction,
        Img: IndexRestriction,
        Snd: IndexRestriction,
        Fnt: IndexRestriction,
        Rm: IndexRestriction,
        Data: Clone {
    fn state(&self) -> GameObjectState<Img, Spr, Data>;

    fn set_state(&mut self, new_state: &GameObjectState<Img, Spr, Data>);

    /// Let game objects reset their data on room load. If you return true, the object is removed
    /// from the room
    fn on_reset(&mut self) -> bool;

    /// Let game objects modify their state every loop. Return a room to change to and objects to
    /// add to the room.
    fn update(
            &mut self, _delta: f64,
            _ctl_objs: &Vec<Box<dyn ControlObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>,
            _others: &Vec<Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>) -> (
                Option<Rm>, Vec<Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>
            ) {
        (None, vec![])
    }

    fn handle_sdl_event(&mut self, _event: &Event) {}

    fn on_collision(
        &mut self,
        _other: &Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>) {}

    fn render(
            &mut self, cnv: &mut Canvas<Window>,
            imgs: &HashMap<Img, Image>, _snds: &HashMap<Snd, Sound>,
            _fonts: &HashMap<Fnt, Font>, _creator: &TextureCreator<WindowContext>,
            elapsed: f64) -> Result<(), String> {
        let mut state = self.state().clone();
        let GameObjectState { ref mut sprs, ref mut cur_spr, pos, .. } = state;
        if let Some(spr) = sprs.get_mut(cur_spr) {
            spr.update(elapsed);
            spr.render(cnv, imgs, (pos.0 as i32, pos.1 as i32))?;
        }
        self.set_state(&state);
        Ok(())
    }
}

/// A special trait to implement cloning for our dynamic GameObjects
pub trait GameObjectBehaviorClone<Img, Snd, Fnt, Spr, Rm, Data> where
        Spr: IndexRestriction,
        Img: IndexRestriction,
        Snd: IndexRestriction,
        Fnt: IndexRestriction,
        Rm: IndexRestriction,
        Data: Clone {
    fn clone_box(&self) -> Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>;
}

impl<Img, Snd, Fnt, Spr, Rm, Data, T>
    GameObjectBehaviorClone<Img, Snd, Fnt, Spr, Rm, Data> for T where
        T: 'static + GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data> + Clone,
        Spr: IndexRestriction,
        Img: IndexRestriction,
        Snd: IndexRestriction,
        Fnt: IndexRestriction,
        Rm: IndexRestriction,
        Data: Clone {
    fn clone_box(&self) -> Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>> {
        Box::new(self.clone())
    }
}

impl<Img, Snd, Fnt, Spr, Rm, Data> Clone
    for Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>> where
        Spr: IndexRestriction,
        Img: IndexRestriction,
        Snd: IndexRestriction,
        Fnt: IndexRestriction,
        Rm: IndexRestriction,
        Data: Clone {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// Control objects are basically game objects, but they are aware of the current room and do not
/// possess colliders. They are the way for doing dynamic memory
pub trait ControlObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>:
    ControlObjectBehaviorClone<Img, Snd, Fnt, Spr, Rm, Data> where
        Spr: IndexRestriction,
        Img: IndexRestriction,
        Snd: IndexRestriction,
        Fnt: IndexRestriction,
        Rm: IndexRestriction,
        Data: Clone {
    fn data(&self) -> Data;
    
    fn handle_sdl_event(&mut self, _event: &Event) {}

    fn update(
            &mut self, _delta: f64, _cur_room: &Rm,
            _others: &Vec<Box<dyn ControlObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>,
            _room_objs: &Vec<Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>) -> (
                Option<Rm>, Vec<Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>
            ) {
        (None, vec![])
    }

    fn render(
            &mut self, _cnv: &mut Canvas<Window>, _cur_room: &Rm,
            _imgs: &HashMap<Img, Image>, _snds: &HashMap<Snd, Sound>,
            _fonts: &HashMap<Fnt, Font>, _creator: &TextureCreator<WindowContext>,
            _elapsed: f64) -> Result<(), String> {
        Ok(())
    }
}

pub trait ControlObjectBehaviorClone<Img, Snd, Fnt, Spr, Rm, Data> where
        Spr: IndexRestriction,
        Img: IndexRestriction,
        Snd: IndexRestriction,
        Fnt: IndexRestriction,
        Rm: IndexRestriction,
        Data: Clone {
    fn clone_box(&self) -> Box<dyn ControlObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>;
}

impl<Img, Snd, Fnt, Spr, Rm, Data, T>
    ControlObjectBehaviorClone<Img, Snd, Fnt, Spr, Rm, Data> for T where
        T: 'static + ControlObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data> + Clone,
        Spr: IndexRestriction,
        Img: IndexRestriction,
        Snd: IndexRestriction,
        Fnt: IndexRestriction,
        Rm: IndexRestriction,
        Data: Clone {
    fn clone_box(&self) -> Box<dyn ControlObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>> {
        Box::new(self.clone())
    }
}

impl<Img, Snd, Fnt, Spr, Rm, Data> Clone
    for Box<dyn ControlObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>> where
        Spr: IndexRestriction,
        Img: IndexRestriction,
        Snd: IndexRestriction,
        Fnt: IndexRestriction,
        Rm: IndexRestriction,
        Data: Clone {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// Colliders that attach to GameObjects. Support Circle and Rect colliders
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CollisionShape {
    Circle {
        center: (i32, i32),
        radius: u32
    }, Rect {
        center: (i32, i32),
        size: (u32, u32)
    }
}

impl CollisionShape {
    pub fn collides_with(&self, other: &CollisionShape) -> bool {
        match self {
            CollisionShape::Circle { center, radius } => {
                match other {
                    CollisionShape::Circle { center: other_center, radius: other_radius } => {
                        let a = other_center.0 - center.0;
                        let b = other_center.1 - center.1;
                        let c = other_radius + radius;
                        ((a * a) + (b * b)) as u32 <= (c * c)
                    }, CollisionShape::Rect { center: other_center, size: other_size } => {
                        let mut test = center.clone();
                        let rect = Rect::new(
                            other_center.0 - other_size.0 as i32 / 2,
                            other_center.1 - other_size.1 as i32 / 2,
                            other_size.0,
                            other_size.1
                        );
                        if center.0 < rect.x {
                            test.0 = rect.x;
                        } else if center.0 > rect.x + rect.w {
                            test.0 = rect.x + rect.w;
                        }
                        if center.1 < rect.y {
                            test.1 = rect.y;
                        } else if center.1 > rect.y + rect.h {
                            test.1 = rect.y + rect.h;
                        }
                        let dist_lat = (center.0 - test.0, center.1 - test.1);
                        let dist_sqrd = (dist_lat.0 * dist_lat.0) + (dist_lat.1 * dist_lat.1);
                        dist_sqrd as u32 <= radius * radius
                    }
                }
            }, CollisionShape::Rect { center, size } => {
                match other {
                    CollisionShape::Circle { center: other_center, radius: other_radius } => {
                        let mut test = other_center.clone();
                        let rect = Rect::new(
                            center.0 - size.0 as i32 / 2,
                            center.1 - size.1 as i32 / 2,
                            size.0,
                            size.1
                        );
                        if other_center.0 < rect.x {
                            test.0 = rect.x;
                        } else if other_center.0 > rect.x + rect.w {
                            test.0 = rect.x + rect.w;
                        }
                        if other_center.1 < rect.y {
                            test.1 = rect.y;
                        } else if other_center.1 > rect.y + rect.h {
                            test.1 = rect.y + rect.h;
                        }
                        let dist_lat = (other_center.0 - test.0, other_center.1 - test.1);
                        let dist_sqrd = (dist_lat.0 * dist_lat.0) + (dist_lat.1 * dist_lat.1);
                        dist_sqrd as u32 <= other_radius * other_radius
                    }, CollisionShape::Rect { center: other_center, size: other_size } => {
                        let r1 = Rect::new(
                            center.0 - size.0 as i32 / 2,
                            center.1 - size.1 as i32 / 2,
                            size.0,
                            size.1
                        );
                        let r2 = Rect::new(
                            other_center.0 - other_size.1 as i32 / 2,
                            other_center.1 - other_size.1 as i32 / 2,
                            other_size.0,
                            other_size.1
                        );
                        r1.x + r1.w >= r2.x
                            && r1.x <= r2.x + r2.w
                            && r1.y + r1.h >= r2.y
                            && r1.y <= r2.y + r2.h
                    }
                }
            }
        }
    }
}

/// A single frame of animation - where to clip an image and how to draw
///
/// ImgId refers to an enum that distinguishes between all the image resources in your game
#[derive(Clone, Copy)]
pub struct Frame<ImgId> where ImgId: Hash + Eq + Clone + Copy {
    src: ImgId,
    clip: Rect,
    size: (i32, i32)
}

impl<ImgId> Frame<ImgId> where ImgId: Hash + Eq + Clone + Copy {
    pub fn new(src: ImgId, clip: Rect, size: (i32, i32)) -> Self {
        Self {
            src,
            clip,
            size
        }
    }

    pub fn render(
            &self, cnv: &mut Canvas<Window>, imgs: &HashMap<ImgId, Image>,
            pos: (i32, i32), origin: (i32, i32), scale: (f64, f64),
            angle: f64, flip: (bool, bool)) -> Result<(), String> {
        let base_scale = (
            self.size.0 as f64 / self.clip.w as f64,
            self.size.1 as f64 / self.clip.h as f64
        );
        let dest = Rect::new(
            (pos.0 as f64 - origin.0 as f64 * base_scale.0 * scale.0) as i32,
            (pos.1 as f64 - origin.1 as f64 * base_scale.1 * scale.1) as i32,
            (self.size.0 as f64 * scale.0) as u32,
            (self.size.1 as f64 * scale.1) as u32
        );
        imgs[&self.src].render(cnv, &self.clip, &dest, angle, flip)
    }
}

/// A collection of different animation frames that can be moved around a screen
#[derive(Clone)]
pub struct Sprite<Img> where Img: IndexRestriction {
    pub frames: Vec<Frame<Img>>,
    pub anim_spd: f64,
    pub origin: (i32, i32),
    pub anim_idx: usize,
    pub anim_idx_smooth: f64,
    pub scale: (f64, f64),
    pub angle: f64,
    pub flip: (bool, bool)
}

impl<Img> Sprite<Img> where Img: IndexRestriction {
    pub fn new(frames: Vec<Frame<Img>>, anim_spd: f64, origin: (i32, i32)) -> Self {
        Self {
            frames: frames.clone(),
            anim_spd,
            origin,
            anim_idx: 0,
            anim_idx_smooth: 0.0,
            scale: (1.0, 1.0),
            angle: 0.0,
            flip: (false, false)
        }
    }

    pub fn update(&mut self, elapsed: f64) {
        self.anim_idx_smooth += elapsed * self.anim_spd;
        if self.anim_idx_smooth > 1.0 {
            if self.anim_idx + 1 >= self.frames.len() {
                self.anim_idx = 0;
            } else {
                self.anim_idx += 1;
            }
            self.anim_idx_smooth = 0.0;
        }
    }

    pub fn render(
            &self, cnv: &mut Canvas<Window>, imgs: &HashMap<Img, Image>,
            pos: (i32, i32)) -> Result<(), String> {
        self.frames[self.anim_idx].render(
            cnv, imgs, pos, self.origin, self.scale, self.angle, self.flip
        )
    }
}

