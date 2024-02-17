//! Implement loading images and updating animations for Sprites

use sdl2::{
    surface::Surface,
    render::Texture,
    image::ImageRwOps
};

fn load_tex(src: &str) -> Result<Texture, Box<dyn std::error::Error>> {
    let sfc = img_load
}

