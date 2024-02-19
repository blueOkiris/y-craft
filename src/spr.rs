//! Implement loading images and updating animations for Sprites

use std::{
    collections::HashMap,
    hash::Hash
};
use sdl2::{
    image::LoadSurface, rect::Rect, render::{
        Canvas, Texture, TextureCreator
    }, surface::Surface, video::{
        Window, WindowContext
    }
};

/// Container for textures with functionality for drawing to screen. This is a "resource" and does
/// not go with GameObjects
pub struct Image<'a> {
    tex: Texture<'a>
}

impl<'a> Image<'a> {
    pub fn new(src: &str, creator: &'a TextureCreator<WindowContext>) -> Result<Self, String> {
        let sfc = Surface::from_file(src)?;
        let tex = Texture::from_surface(&sfc, creator).map_err(|e| e.to_string())?;
        Ok(Self {
            tex
        })
    }

    pub fn render(
            &self, cnv: &mut Canvas<Window>, src: &Rect, dest: &Rect,
            angle: f64, flip: (bool, bool)) -> Result<(), String> {
        cnv.copy_ex(&self.tex, Some(*src), Some(*dest), angle, None, flip.0, flip.1)
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

/// A single frame of animation - where to clip and how to draw
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
pub struct Sprite<ImgId> where ImgId: Hash + Eq + Clone + Copy {
    frames: Vec<Frame<ImgId>>,
    pub anim_spd: f64,
    pub origin: (i32, i32),
    pub anim_idx: usize,
    anim_idx_smooth: f64,
    pub scale: (f64, f64),
    pub angle: f64,
    pub flip: (bool, bool)
}

impl<ImgId> Sprite<ImgId> where ImgId: Hash + Eq + Clone + Copy {
    pub fn new(frames: Vec<Frame<ImgId>>, anim_spd: f64, origin: (i32, i32)) -> Self {
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

    pub fn update(&mut self, delta: f64) {
        self.anim_idx_smooth += delta * self.anim_spd;
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
            &self, cnv: &mut Canvas<Window>, imgs: &HashMap<ImgId, Image>,
            pos: (i32, i32)) -> Result<(), String> {
        self.frames[self.anim_idx].render(
            cnv, imgs, pos, self.origin, self.scale, self.angle, self.flip
        )
    }
}

