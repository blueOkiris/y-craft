//! Generic "GameObject" system that defines sets of behaviors

use std::{
    collections::HashMap,
    hash::Hash
};
use sdl2::{
    event::Event,
    rect::Rect,
    render::Canvas,
    video::Window
};
use crate::spr::{
    Image, Sprite
};

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
                        let mut test = center;
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
                        let mut test = other_center;
                        let rect = Rect::new(
                            center.0 - size.0 as i32 / 2,
                            center.1 - size.1 as i32 / 2,
                            size.0,
                            size.1
                        );
                        if other_center.0 < rect.x {
                            test.0 = rect.x;
                        } else if center.0 > rect.x + rect.w {
                            test.0 = rect.x + rect.w;
                        }
                        if other_center.1 < rect.y {
                            test.1 = rect.y;
                        } else if center.1 > rect.y + rect.h {
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

/// All game objects should have one of these as a member
pub struct GameObjectState<TypeEnum, SpriteEnum, ImgEnum> {
    pub name: String,
    pub class: TypeEnum,
    pub pos: (f64, f64),
    pub collider: CollisionShape,
    cur_spr: SpriteEnum,
    sprs: HashMap<SpriteEnum, Sprite<ImgEnum>>,
    def_pos: (f64, f64),
    def_spr: SpriteEnum
}

/// All game objects should implement these
pub trait GameObjectBehavior<TypeEnum, SpriteEnum, ImgEnum> {
    fn state(&self) -> GameObjectState<TypeEnum, SpriteEnum, ImgEnum>;
    fn update(
        &mut self, delta: f64,
        others: &Vec<Box<dyn GameObjectBehavior<TypeEnum, SpriteEnum, ImgEnum>>>
    );
    fn handle_sdl_event(&mut self, event: &Event);
    fn on_collision(&mut self, other: &Box<dyn GameObjectBehavior<TypeEnum, SpriteEnum, ImgEnum>>);
    fn on_reset(&mut self);
}

impl<TypeEnum, SpriteEnum: Eq + Hash, ImgEnum: Eq + Hash>
        GameObjectState<TypeEnum, SpriteEnum, ImgEnum> {
    pub fn render(
            &mut self, cnv: &mut Canvas<Window>, imgs: &HashMap<ImgEnum, Image>, elapsed: f64) {
        self.sprs[&self.cur_spr].update(elapsed);
        self.sprs[&self.cur_spr].render(cnv, imgs, (self.pos.0 as i32, self.pos.1 as i32));
    }
}

