//! The main room for the snake game

use std::collections::HashMap;
use rand::Rng;
use sdl2::{
    event::Event,
    keyboard::Scancode,
    rect::Rect, render::{Canvas, TextureCreator}, video::{Window, WindowContext}
};
use ycraft::{
    obj::{
        CollisionShape, ControlObjectBehavior, Frame, GameObjectBehavior, GameObjectState, Sprite
    }, res::{Font, Image, Sound}, room::Room
};
use crate::game::{
    Img, Snd, Fnt, Spr, Rm, Data, BASE_MOVE_SPD, MOVE_SPD_INC
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right
}

#[derive(Clone)]
struct SnakeHead {
    state: GameObjectState<Img, Spr, Data>,
    move_spd: f64,
    inter_pos: (f64, f64),
    can_change_dir: bool,
    add_body_seg: bool,
    should_die: bool,
    play_eat_snd: bool
}

impl SnakeHead {
    pub fn new() -> Self {
        let pos = (640.0 / 2.0 + 32.0 + 32.0 / 2.0, 352.0 / 2.0);
        Self {
            state: GameObjectState {
                name: "head".to_string(),
                pos,
                collider: CollisionShape::Rect { center: (0, 0), size: (31, 31) },
                cur_spr: Spr::Head,
                sprs: HashMap::from([(
                    Spr::Head,
                    Sprite::new(
                        vec![ Frame::new(Img::Snake, Rect::new(0, 0, 32, 32), (32, 32)) ],
                        0.0, (16, 16)
                    )
                )]), custom: Data::Head {
                    dir: Dir::Right,
                    lurch_propagation: 0
                }
            }, move_spd: BASE_MOVE_SPD,
            inter_pos: pos,
            can_change_dir: true,
            add_body_seg: false,
            should_die: false,
            play_eat_snd: false
        }
    }
}

impl GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data> for SnakeHead {
    fn state(&self) -> GameObjectState<Img, Spr, Data> {
        self.state.clone()
    }

    fn on_reset(&mut self) -> bool {
        let nw = SnakeHead::new();
        self.state = nw.state;
        self.move_spd = nw.move_spd;
        self.inter_pos = nw.inter_pos;
        self.can_change_dir = nw.can_change_dir;
        self.add_body_seg = nw.add_body_seg;
        self.should_die = false;
        false
    }

    fn handle_sdl_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown { scancode, .. } => if scancode.is_some() {
                if let Data::Head { ref mut dir, .. } = self.state.custom {
                    if self.can_change_dir {
                        // Keep how much you've moved in a dir when switching.
                        // This keeps each increment the same time length.
                        // It's like momentum
                        let pos_dif = match *dir {
                            Dir::Up | Dir::Down => (self.inter_pos.1 - self.state.pos.1).abs(),
                            Dir::Left | Dir::Right => (self.inter_pos.0 - self.state.pos.0).abs()
                        };
                        self.inter_pos = self.state.pos;
                        match scancode.unwrap() {
                            Scancode::Up => {
                                self.can_change_dir = false;
                                *dir = Dir::Up;
                                self.inter_pos.1 -= pos_dif;
                            }, Scancode::Down => {
                                self.can_change_dir = false;
                                *dir = Dir::Down;
                                self.inter_pos.1 += pos_dif;
                            }, Scancode::Left => {
                                self.can_change_dir = false;
                                *dir = Dir::Left;
                                self.inter_pos.0 -= pos_dif;
                            }, Scancode::Right => {
                                self.can_change_dir = false;
                                *dir = Dir::Right;
                                self.inter_pos.0 += pos_dif;
                            }, _ => {}
                        }
                    }
                }
            }, _ => {}
        }
    }

    fn update(
            &mut self, delta: f64,
            ctl_objs: &Vec<Box<dyn ControlObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>,
            others: &Vec<Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>) -> (
                Option<Rm>,
                Vec<Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>
            ) {
        let mut score = 0;
        for obj in ctl_objs.iter() {
            if let Data::Score(sc) = obj.data() {
                score = sc;
                break;
            }
        }
        let mut added_objs: Vec<Box<dyn GameObjectBehavior<_, _, _, _, _, _>>> = Vec::new();
        if let Data::Head { ref mut dir, ref mut lurch_propagation } =
                self.state.custom {
            if *lurch_propagation == 0 {
                match dir {
                    Dir::Up => {
                        if let Some(spr) = self.state.sprs.get_mut(&self.state.cur_spr) {
                            spr.angle = 0.0;
                        }
                        self.inter_pos.1 -= delta * self.move_spd;
                        if self.inter_pos.1.floor() < self.state.pos.1 - 32.0 {
                            self.can_change_dir = true;
                            self.state.pos.1 -= 32.0;
                            *lurch_propagation = score;
                        }
                    }, Dir::Down => {
                        if let Some(spr) = self.state.sprs.get_mut(&self.state.cur_spr) {
                            spr.angle = 180.0;
                        }
                        self.inter_pos.1 += delta * self.move_spd;
                        if self.inter_pos.1.floor() > self.state.pos.1 + 32.0 {
                            self.can_change_dir = true;
                            self.state.pos.1 += 32.0;
                            *lurch_propagation = score;
                        }
                    }, Dir::Left => {
                        if let Some(spr) = self.state.sprs.get_mut(&self.state.cur_spr) {
                            spr.angle = 270.0;
                        }
                        self.inter_pos.0 -= delta * self.move_spd;
                        if self.inter_pos.0.floor() < self.state.pos.0 - 32.0 {
                            self.can_change_dir = true;
                            self.state.pos.0 -= 32.0;
                            *lurch_propagation = score;
                        }
                    }, Dir::Right => {
                        if let Some(spr) = self.state.sprs.get_mut(&self.state.cur_spr) {
                            spr.angle = 90.0;
                        }
                        self.inter_pos.0 += delta * self.move_spd;
                        if self.inter_pos.0.floor() > self.state.pos.0 + 32.0 {
                            self.can_change_dir = true;
                            self.state.pos.0 += 32.0;
                            *lurch_propagation = score;
                        }
                    }
                }
            } else {
                *lurch_propagation -= 1;
            }

            if self.add_body_seg {
                let mut max_body = -1;
                let mut max_body_pos = (0.0, 0.0);
                for other in others.iter() {
                    if let Data::Body { index, .. } = other.state().custom {
                        if index > max_body {
                            max_body = index;
                            max_body_pos = other.state().pos;
                        }
                    }
                }
                added_objs.push(Box::new(SnakeBody::new(max_body + 1, max_body_pos)));
                self.add_body_seg = false;
                self.move_spd += MOVE_SPD_INC;
            }

            if self.state.pos.0 < 32.0 || self.state.pos.1 < 32.0
                    || self.state.pos.0 > 640.0 - 32.0 || self.state.pos.1 > 360.0 - 32.0 {
                return (Some(Rm::Dead), vec![]);
            }

            if self.should_die {
                return (Some(Rm::Dead), vec![]);
            }
        }
        (None, added_objs)
    }

    fn on_collision(
            &mut self,
            other: &Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>) {
        match other.state().custom {
            Data::Mouse => {
                self.add_body_seg = true;
                self.play_eat_snd = true;
            }, Data::Tail | Data::Body { .. } => {
                if let Data::Head { lurch_propagation, .. } = self.state.custom {
                    if lurch_propagation == 0 {
                        self.should_die = true;
                    }
                }
            }
            _ => {}
        }
    }

    fn render(
                &mut self, cnv: &mut Canvas<Window>,
                imgs: &HashMap<Img, Image>, snds: &HashMap<Snd, Sound>,
                _fonts: &HashMap<Fnt, Font>, _creator: &TextureCreator<WindowContext>,
                elapsed: f64) -> Result<(), String> {
        if self.play_eat_snd {
            snds[&Snd::Bite].play()?;
            self.play_eat_snd = false;
        }

        // Default render
        let GameObjectState { ref mut sprs, ref mut cur_spr, pos, .. } = self.state();
        if let Some(spr) = sprs.get_mut(cur_spr) {
            spr.update(elapsed);
            spr.render(cnv, imgs, (pos.0 as i32, pos.1 as i32))?;
        }
        Ok(())
    }
}

#[derive(Clone)]
struct SnakeBody {
    state: GameObjectState<Img, Spr, Data>,
    last_dir: Dir,
    last_pos: (f64, f64),
    def_pos: (f64, f64)
}

impl SnakeBody {
    pub fn new(index: isize, def_pos: (f64, f64)) -> Self {
        Self {
            state: GameObjectState {
                name: format!("snake_body_{}", index),
                pos: def_pos,
                collider: CollisionShape::Rect { center: (0, 0), size: (31, 31) },
                cur_spr: Spr::Body,
                sprs: HashMap::from([(
                    Spr::Body,
                    Sprite::new(
                        vec![ Frame::new(Img::Snake, Rect::new(32, 0, 32, 32), (32, 32)) ],
                        0.0, (16, 16)
                    )
                )]), custom: Data::Body {
                    index,
                    dir: Dir::Right
                }
            }, last_dir: Dir::Right,
            last_pos: def_pos,
            def_pos
        }
    }
}

impl GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data> for SnakeBody {
    fn state(&self) -> GameObjectState<Img, Spr, Data> {
        self.state.clone()
    }

    fn on_reset(&mut self) -> bool {
        if let Data::Body { index, .. } = self.state.custom {
            if index > 1 {
                true
            } else {
                let nw = SnakeBody::new(index, self.def_pos);
                self.state = nw.state;
                self.last_dir = nw.last_dir;
                self.last_pos = nw.last_pos;
                false
            }
        } else {
            true
        }
    }

    fn update(
            &mut self, _delta: f64,
            _ctl_objs: &Vec<Box<dyn ControlObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>,
            others: &Vec<Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>) -> (
                Option<Rm>,
                Vec<Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>
            ) {
        if let Data::Body { ref mut index, ref mut dir } = self.state.custom {
            let parent_id = if *index == 0 {
                "head".to_string()
            } else {
                format!("snake_body_{}", *index - 1)
            };
            for other in others.iter() {
                if *other.state().name == parent_id && other.state().pos != self.last_pos {
                    self.state.pos = self.last_pos;
                    *dir = self.last_dir;
                    self.last_pos = other.state().pos;
                    if *index == 0 {
                        if let Data::Head { dir: other_dir, .. } = other.state().custom {
                            self.last_dir = other_dir;
                        }
                    } else {
                        if let Data::Body { dir: other_dir, .. } = other.state().custom {
                            self.last_dir = other_dir;
                        }
                    }
                }
            }
            if let Some(spr) = self.state.sprs.get_mut(&self.state.cur_spr) {
                spr.angle = match dir {
                    Dir::Up => 0.0,
                    Dir::Down => 180.0,
                    Dir::Left => 270.0,
                    Dir::Right => 90.0
                };
            }
        }
        (None, vec![])
    }
}

#[derive(Clone)]
struct SnakeTail {
    state: GameObjectState<Img, Spr, Data>,
    dir: Dir,
    last_dir: Dir,
    last_pos: (f64, f64),
}

impl SnakeTail {
    pub fn new() -> Self {
        Self {
            state: GameObjectState {
                name: "snake_tail".to_string(),
                pos: (640.0 / 2.0 - 32.0 - 32.0 / 2.0, 352.0 / 2.0),
                cur_spr: Spr::Tail,
                sprs: HashMap::from([(
                    Spr::Tail,
                    Sprite::new(
                        vec![ Frame::new(Img::Snake, Rect::new(0, 32, 32, 32), (32, 32)) ],
                        0.0, (16, 16)
                    )
                )]), collider: CollisionShape::Rect { center: (0, 0), size: (31, 31) },
                custom: Data::Tail
            }, dir: Dir::Right,
            last_dir: Dir::Right,
            last_pos: (640.0 / 2.0 - 32.0 - 32.0 / 2.0, 352.0 / 2.0),
        }
    }
}

impl GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data> for SnakeTail {
    fn state(&self) -> GameObjectState<Img, Spr, Data> {
        self.state.clone()
    }

    fn on_reset(&mut self) -> bool {
        let nw = SnakeTail::new();
        self.state = nw.state;
        self.last_pos = nw.last_pos;
        self.dir = nw.dir;
        self.last_dir = nw.dir;
        false
    }

    fn update(
            &mut self, _delta: f64,
            _ctl_objs: &Vec<Box<dyn ControlObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>,
            others: &Vec<Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>) -> (
                Option<Rm>, Vec<Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>
            ) {
        let mut max_body = -1;
        for other in others.iter() {
            if let Data::Body { index, .. } = other.state().custom {
                if index > max_body {
                    max_body = index;
                }
            }
        }
        for other in others.iter() {
            if let Data::Body { index, dir: other_dir} = other.state().custom {
                if index == max_body && self.last_pos != other.state().pos {
                    self.dir = self.last_dir;
                    self.state.pos = self.last_pos;
                    self.last_dir = other_dir;
                    self.last_pos = other.state().pos;
                }
            }
        }
        if let Some(spr) = self.state.sprs.get_mut(&self.state.cur_spr) {
            spr.angle = match self.dir {
                Dir::Up => 0.0,
                Dir::Down => 180.0,
                Dir::Left => 270.0,
                Dir::Right => 90.0
            };
        }
        (None, vec![])
    }
}

#[derive(Clone)]
struct Mouse {
    state: GameObjectState<Img, Spr, Data>
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            state: GameObjectState {
                name: "mouse".to_string(),
                pos: Self::random_mouse_pos(),
                cur_spr: Spr::Mouse,
                sprs: HashMap::from([(
                    Spr::Mouse,
                    Sprite::new(
                        vec![ Frame::new(Img::Mouse, Rect::new(0, 0, 32, 32), (32, 32)) ],
                        0.0, (16, 16)
                    )
                )]), collider: CollisionShape::Circle { center: (0, 0), radius: 15 },
                custom: Data::Mouse
            }
        }
    }

    fn random_mouse_pos() -> (f64, f64) {
        let mut rng = rand::thread_rng();
        (
            (rng.gen_range(32.0..640.0 - 96.0) / 32.0 as f64).floor() * 32.0 + 16.0,
            (rng.gen_range(32.0..360.0 - 96.0) / 32.0 as f64).floor() * 32.0 + 16.0
        )
    }
}

impl GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data> for Mouse {
    fn state(&self) -> GameObjectState<Img, Spr, Data> {
        self.state.clone()
    }

    fn on_reset(&mut self) -> bool {
        let nw = Mouse::new();
        self.state = nw.state;
        false
    }

    fn on_collision(
            &mut self,
            other: &Box<dyn GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>) {
        if let Data::Head { .. } = other.state().custom {
            self.on_reset();
        }
    }
}

#[derive(Clone)]
struct Board {
    state: GameObjectState<Img, Spr, Data>
}

impl Board {
    pub fn new() -> Self {
        Self {
            state: GameObjectState {
                name: "board".to_string(),
                pos: (0.0, 0.0),
                collider: CollisionShape::Rect { center: (320, 180), size: (640, 480) },
                cur_spr: Spr::Board,
                sprs: HashMap::from([(
                    Spr::Board,
                    Sprite::new(
                        vec![Frame::new(
                            Img::Board, Rect::new(0, 0, 640, 360), (640, 360)
                        )], 0.0, (0, 0)
                    )
                )]), custom: Data::Board
            }
        }
    }
}

impl GameObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data> for Board {
    fn state(&self) -> GameObjectState<Img, Spr, Data> {
        self.state.clone()
    }

    fn on_reset(&mut self) -> bool {
        false
    }
}

pub fn play() -> Room<Img, Snd, Fnt, Spr, Rm, Data> {
    Room::new(
        vec![
            Box::new(Board::new()),
            Box::new(SnakeHead::new()),
            Box::new(SnakeBody::new(0, (640.0 / 2.0 + 32.0 / 2.0, 352.0 / 2.0))),
            Box::new(SnakeBody::new(1, (640.0 / 2.0 - 32.0 / 2.0, 352.0 / 2.0))),
            Box::new(SnakeTail::new()),
            Box::new(Mouse::new())
        ], false
    )
}

