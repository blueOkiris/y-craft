//! Game objects and resource enums

use std::collections::HashMap;
use sdl2::{
    event::Event,
    keyboard::Scancode,
    rect::Rect,
    render::{
        Canvas, TextureCreator
    }, video::{
        Window, WindowContext
    }
};
use ycraft::{
    obj::{
        CollisionShape, ControlObjectBehavior, Frame, GameObjectBehavior, GameObjectState, Sprite
    }, res::{
        Font, Image, Sound
    }, room::Room
};

const MOVE_SPD: f64 = 512.0;
const JUMP_SPD: f64 = 1500.0;
const ACC: f64 = 50.0;
const GRAVITY: f64 = 4096.0;
const EXTRA_GRAV: f64 = 13000.0;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Img {
    Brick,
    Character
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Snd {
    Music,
    Jump
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Fnt {
    Geist
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Spr {
    Brick,
    Idle,
    Walk
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Rm {
    Room0
}

#[derive(Clone, Copy)]
pub enum Data {
    Brick,
    Player
}

#[derive(Clone)]
struct Brick {
    state: GameObjectState<Img, Spr, Data>,
    def_pos: (f64, f64)
}

impl Brick {
    pub fn new(def_pos: (f64, f64)) -> Self {
        Self {
            state: GameObjectState {
                name: "brick".to_string(),
                pos: def_pos,
                collider: CollisionShape::Rect { center: (0, 0), size: (64, 64) },
                cur_spr: Spr::Brick,
                sprs: HashMap::from([(
                    Spr::Brick,
                    Sprite::new(
                        vec![ Frame::new(Img::Brick, Rect::new(0, 0, 32, 32), (64, 64)) ],
                        0.0, (16, 16)
                    )
                )]), custom: Data::Brick
            }, def_pos
        }
    }
}

impl GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data> for Brick {
    fn state(&self) -> GameObjectState<Img, Spr, Data> {
        self.state.clone()
    }

    fn set_state(&mut self, new_state: &GameObjectState<Img, Spr, Data>) {
        self.state = new_state.clone();
    }

    fn on_reset(&mut self) -> bool {
        self.state.pos = self.def_pos;
        false
    }
}

#[derive(Clone)]
struct Player {
    state: GameObjectState<Img, Spr, Data>,
    def_pos: (f64, f64),
    vel: (f64, f64),
    right: bool,
    left: bool,
    grounded: bool,
    extra_grav: bool,
    facing_right: bool,
    play_jump_sound: bool
}

impl Player {
    pub fn new(def_pos: (f64, f64)) -> Self {
        Self {
            state: GameObjectState {
                name: "player".to_string(),
                pos: def_pos,
                collider: CollisionShape::Rect { center: (0, 0), size: (64, 64) },
                cur_spr: Spr::Idle,
                sprs: HashMap::from([
                    (
                        Spr::Idle,
                        Sprite::new(
                            vec![Frame::new(Img::Character, Rect::new(0, 0, 32, 32), (128, 128))],
                            0.0, (16, 16)
                        )
                    ), (
                        Spr::Walk,
                        Sprite::new(
                            vec![
                                Frame::new(Img::Character, Rect::new(0, 0, 32, 32), (128, 128)),
                                Frame::new(Img::Character, Rect::new(32, 0, 32, 32), (128, 128))
                            ], 12.0, (16, 16)
                        )
                    )
                ]), custom: Data::Player
            }, def_pos,
            vel: (0.0, 0.0),
            right: false,
            left: false,
            grounded: false,
            extra_grav: false,
            facing_right: true,
            play_jump_sound: false
        }
    }
}

impl GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data> for Player {
    fn state(&self) -> GameObjectState<Img, Spr, Data> {
        self.state.clone()
    }

    fn set_state(&mut self, new_state: &GameObjectState<Img, Spr, Data>) {
        self.state = new_state.clone();
    }

    fn on_reset(&mut self) -> bool {
        self.state.pos = self.def_pos;
        self.right = false;
        self.left = false;
        self.grounded = false;
        self.state.cur_spr = Spr::Idle;
        self.extra_grav = false;
        self.facing_right = true;
        self.play_jump_sound = false;
        false
    }

    fn handle_sdl_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown { scancode, .. } if *scancode == Some(Scancode::Up) => {
                if self.grounded {
                    self.state.pos.1 -= 1.0;
                    self.vel.1 = -JUMP_SPD;
                    self.play_jump_sound = true;
                }
            }, Event::KeyDown { scancode, .. } if *scancode == Some(Scancode::Left) => {
                self.left = true;
            }, Event::KeyDown { scancode, .. } if *scancode == Some(Scancode::Right) => {
                self.right = true;
            }, Event::KeyUp { scancode, .. } if *scancode == Some(Scancode::Up) => {
                if self.vel.1 < -0.1 {
                    self.extra_grav = true;
                }
            }, Event::KeyUp { scancode, .. } if *scancode == Some(Scancode::Left) => {
                self.left = false;
            }, Event::KeyUp { scancode, .. } if *scancode == Some(Scancode::Right) => {
                self.right = false;
            }, _ => {}
        }
    }

    fn update(
            &mut self, delta: f64,
            _ctl_objs: &Vec<Box<dyn ControlObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>,
            others: &Vec<Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>) -> (
                Option<Rm>, Vec<Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>
            ) {
        self.state.pos.0 += self.vel.0 * delta;
        self.state.pos.1 += self.vel.1 * delta;

        // Animate
        if let Some(spr) = self.state.sprs.get_mut(&self.state.cur_spr) {
            spr.scale = (0.5, 0.5);
            spr.flip = (!self.facing_right, false);
        }
        if !self.grounded && self.state.cur_spr != Spr::Idle {
            self.state.cur_spr = Spr::Idle;
        } else if self.grounded {
            if self.vel.0.abs() < MOVE_SPD * 0.8 && self.state.cur_spr != Spr::Idle {
                self.state.cur_spr = Spr::Idle;
            } else if self.vel.0.abs() >= MOVE_SPD * 0.8 && self.state.cur_spr != Spr::Walk {
                self.state.cur_spr = Spr::Walk;
                self.facing_right = self.vel.0 > 0.1;
            }
        }

        // Update vel at the end, so collisions can affect it!
        let hor = (if self.right { 1.0 } else { 0.0 }) + (if self.left { -1.0 } else { 0.0 });
        self.vel.0 = ycraft::util::lerp(self.vel.0, hor * MOVE_SPD, ACC * delta);

        if let CollisionShape::Rect { center, size } = self.state.collider {
            let gnd_check = CollisionShape::Rect {
                center: (
                    center.0 + self.state.pos.0 as i32,
                    center.1 + (self.state.pos.1 + self.vel.1 * delta) as i32 + 1
                ), size
            };
            self.grounded = false;
            for other in others.iter() {
                if self.state.pos.1 > other.state().pos.1 - size.1 as f64 * 0.8 {
                    continue;
                }
                if let Data::Brick = other.state().custom {
                    let mut other_col = other.state().collider.clone();
                    if let CollisionShape::Rect { center: ref mut other_center, .. } = other_col {
                        other_center.0 += other.state().pos.0 as i32;
                        other_center.1 += other.state().pos.1 as i32;
                    };
                    if gnd_check.collides_with(&other_col) {
                        if let CollisionShape::Rect { center, size } = other_col {
                            self.grounded = true;
                            self.vel.1 = 0.0;
                            self.state.pos.1 = (center.1 - size.1 as i32 - 1) as f64;
                        }
                    }
                }
            }
        }
        if self.grounded || self.vel.1 > 0.1 {
            self.extra_grav = false;
        }
        if self.extra_grav {
            self.vel.1 += EXTRA_GRAV * delta;
        } else {
            self.vel.1 += GRAVITY * delta;
        }

        (None, vec![])
    }

    fn on_collision(
            &mut self,
            other: &Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>) {
        if let Data::Brick = other.state().custom {
            if (other.state().pos.0 <= self.state.pos.0 && self.vel.0 < -0.1)
                    || (other.state().pos.0 >= self.state.pos.0 && self.vel.0 > 0.1) {
                self.state.pos.0 -= if self.vel.0 > 0.1 { 1.0 } else { -1.0 };
                self.vel.0 = 0.0;
            }
        }
    }

    fn render(
            &mut self, cnv: &mut Canvas<Window>,
            imgs: &HashMap<Img, Image>, snds: &HashMap<Snd, Sound>,
            _fonts: &HashMap<Fnt, Font>, _creator: &TextureCreator<WindowContext>,
            elapsed: f64) -> Result<(), String> {
        if self.play_jump_sound {
            snds[&Snd::Jump].play()?;
            self.play_jump_sound = false;
        }

        if !Sound::is_music_playing() {
            snds[&Snd::Music].play()?;
        }

        // Default render
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

pub fn room0() -> Room<Img, Snd, Fnt, Spr, Rm, Data> {
    Room::new(
        vec![
            Box::new(Player::new((256.0, 900.0))),
            Box::new(Brick::new((96.0, 900.0))),
            Box::new(Brick::new((160.0, 1000.0))),
            Box::new(Brick::new((224.0, 1000.0))),
            Box::new(Brick::new((288.0, 1000.0))),
            Box::new(Brick::new((352.0, 1000.0))),
            Box::new(Brick::new((416.0, 1000.0)))
        ], false
    )
}

