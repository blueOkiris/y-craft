//! The main room for the snake game

use std::any::Any;
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
    FontId, ImgId, ObjId, RmId, SndId, SprId, BASE_MOVE_SPD, MOVE_SPD_INC
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    Up,
    Down,
    Left,
    Right
}

#[derive(Clone)]
struct SnakeHead {
    state: GameObjectState<ObjId, SprId, ImgId>,
    dir: Dir,
    move_spd: f64,
    inter_pos: (f64, f64),
    can_change_dir: bool,
    add_body_seg: bool
}

impl SnakeHead {
    pub fn new() -> Self {
        let pos = (640.0 / 2.0 + 32.0 + 32.0 / 2.0, 352.0 / 2.0);
        Self {
            state: GameObjectState::new(
                "head", ObjId::SnakeHead, pos,
                CollisionShape::Rect { center: (0, 0), size: (32, 32) },
                SprId::Head, &[(
                    SprId::Head,
                    Sprite::new(
                        vec![ Frame::new(ImgId::Snake, Rect::new(0, 0, 32, 32), (32, 32)) ],
                        0.0, (16, 16)
                    )
                )]
            ), dir: Dir::Right,
            move_spd: BASE_MOVE_SPD,
            inter_pos: pos,
            can_change_dir: true,
            add_body_seg: false
        }
    }
}

impl GameObjectBehavior<ObjId, SprId, ImgId, SndId, FontId, RmId> for SnakeHead {
    fn state(&self) -> GameObjectState<ObjId, SprId, ImgId> {
        self.state.clone()
    }

    fn handle_sdl_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown { scancode, .. } => if scancode.is_some() {
                if self.can_change_dir {
                    // Keep how much you've moved in a dir when switching.
                    // This keeps each increment the same time length.
                    // It's like momentum
                    let pos_dif = match self.dir {
                        Dir::Up | Dir::Down => (self.inter_pos.1 - self.state.pos.1).abs(),
                        Dir::Left | Dir::Right => (self.inter_pos.0 - self.state.pos.0).abs()
                    };
                    self.inter_pos = self.state.pos;
                    match scancode.unwrap() {
                        Scancode::Up => {
                            self.can_change_dir = false;
                            self.dir = Dir::Up;
                            self.inter_pos.1 -= pos_dif;
                        }, Scancode::Down => {
                            self.can_change_dir = false;
                            self.dir = Dir::Down;
                            self.inter_pos.1 += pos_dif;
                        }, Scancode::Left => {
                            self.can_change_dir = false;
                            self.dir = Dir::Left;
                            self.inter_pos.0 -= pos_dif;
                        }, Scancode::Right => {
                            self.can_change_dir = false;
                            self.dir = Dir::Right;
                            self.inter_pos.0 += pos_dif;
                        }, _ => {}
                    }
                }
            }, _ => {}
        }
    }

    fn update(
            &mut self, delta: f64,
            others: &Vec<Box<
                dyn GameObjectBehavior<ObjId, SprId, ImgId, SndId, FontId, RmId>
            >>) -> (
                Option<RmId>,
                Vec<Box<dyn GameObjectBehavior<ObjId, SprId, ImgId, SndId, FontId, RmId>>>
            ) {
        let mut added_objs: Vec<Box<dyn GameObjectBehavior<_, _, _, _, _, _>>> = Vec::new();
        if self.add_body_seg {
            let mut max_body = -1;
            let mut max_body_pos = (0.0, 0.0);
            for other in others.iter() {
                if other.state().class == self.state.class {
                    let body = SnakeBody::downcast(other);
                    if body.index > max_body {
                        max_body = body.index;
                        max_body_pos = body.state.pos;
                    }
                }
            }
            added_objs.push(Box::new(SnakeBody::new(max_body + 1, max_body_pos)));
            self.add_body_seg = false;
            self.move_spd += MOVE_SPD_INC;
        }
        match self.dir {
            Dir::Up => {
                let mut spr = self.state.sprs[&self.state.cur_spr].clone();
                spr.angle = 0.0;
                self.state.sprs.insert(self.state.cur_spr, spr);
                self.inter_pos.1 -= delta * self.move_spd;
                if self.inter_pos.1.floor() < self.state.pos.1 - 32.0 {
                    self.can_change_dir = true;
                    self.state.pos.1 -= 32.0;
                }
            }, Dir::Down => {
                let mut spr = self.state.sprs[&self.state.cur_spr].clone();
                spr.angle = 180.0;
                self.state.sprs.insert(self.state.cur_spr, spr);
                self.inter_pos.1 += delta * self.move_spd;
                if self.inter_pos.1.floor() > self.state.pos.1 + 32.0 {
                    self.can_change_dir = true;
                    self.state.pos.1 += 32.0;
                }
            }, Dir::Left => {
                let mut spr = self.state.sprs[&self.state.cur_spr].clone();
                spr.angle = 270.0;
                self.state.sprs.insert(self.state.cur_spr, spr);
                self.inter_pos.0 -= delta * self.move_spd;
                if self.inter_pos.0.floor() < self.state.pos.0 - 32.0 {
                    self.can_change_dir = true;
                    self.state.pos.0 -= 32.0;
                }
            }, Dir::Right => {
                let mut spr = self.state.sprs[&self.state.cur_spr].clone();
                spr.angle = 90.0;
                self.state.sprs.insert(self.state.cur_spr, spr);
                self.inter_pos.0 += delta * self.move_spd;
                if self.inter_pos.0.floor() > self.state.pos.0 + 32.0 {
                    /*for other in others.iter_mut() {
                        if other.state().name == "snake_body_0" {
                            
                        }
                    }*/
                    self.can_change_dir = true;
                    self.state.pos.0 += 32.0;
                }
            }
        }
        (None, added_objs)
    }

    fn on_collision(
            &mut self,
            other: &Box<dyn GameObjectBehavior<ObjId, SprId, ImgId, SndId, FontId, RmId>>) {
        if other.state().class == ObjId::Mouse {
            self.add_body_seg = true;
        }
    }

    fn on_reset(&mut self) -> bool {
        self.state.pos = self.state.def_pos;
        self.dir = Dir::Right;
        self.move_spd = BASE_MOVE_SPD;
        self.inter_pos = self.state.def_pos;
        self.can_change_dir = true;
        false
    }
}

#[derive(Clone)]
struct SnakeBody {
    state: GameObjectState<ObjId, SprId, ImgId>,
    index: isize
}

impl SnakeBody {
    pub fn new(index: isize, def_pos: (f64, f64)) -> Self {
        Self {
            state: GameObjectState::new(
                format!("snake_body_{}", index).as_str(), ObjId::SnakeBody, def_pos,
                CollisionShape::Rect { center: (0, 0), size: (32, 32) },
                SprId::Body, &[(
                    SprId::Body,
                    Sprite::new(
                        vec![ Frame::new(ImgId::Snake, Rect::new(32, 0, 32, 32), (32, 32)) ],
                        0.0, (16, 16)
                    )
                )]
            ), index
        }
    }

    /// Always check before downcasting via class
    pub fn downcast(obj: &dyn Any) -> &Self {
        *obj.downcast_ref().unwrap()
    }
}

impl GameObjectBehavior<ObjId, SprId, ImgId, SndId, FontId, RmId> for SnakeBody {
    fn state(&self) -> GameObjectState<ObjId, SprId, ImgId> {
        self.state.clone()
    }

    fn on_reset(&mut self) -> bool {
        self.state.pos = self.state.def_pos;
        self.index > 1
    }
}

#[derive(Clone)]
struct SnakeTail {
    state: GameObjectState<ObjId, SprId, ImgId>
}

impl SnakeTail {
}

impl GameObjectBehavior<ObjId, SprId, ImgId, SndId, FontId, RmId> for SnakeTail {
    fn state(&self) -> GameObjectState<ObjId, SprId, ImgId> {
        self.state.clone()
    }

    fn on_reset(&mut self) -> bool {
        false
    }
}

pub fn game() -> Room<ObjId, SprId, ImgId, SndId, FontId, RmId> {
    Room::new(
        vec![
            Box::new(SnakeHead::new()),
            Box::new(SnakeBody::new(0, (640.0 / 2.0 + 32.0 / 2.0, 352.0 / 2.0))),
            Box::new(SnakeBody::new(1, (640.0 / 2.0 - 32.0 / 2.0, 352.0 / 2.0))),
            //SnakeTail
            //Mouse
        ], false
    )
}

