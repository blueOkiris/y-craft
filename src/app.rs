//! Keep track of SDL context and window state as well as run main game loop

use std::{
    collections::HashMap,
    time::{
        Duration, Instant
    }
};
use sdl2::{
    event::Event,
    keyboard::Scancode,
    mixer::{
        AUDIO_S16LSB, DEFAULT_CHANNELS
    }, pixels::Color
};
use crate::{
    res::{
        Font,
        Image,
        Sound
    }, room::Room,
    IndexRestriction
};

pub fn run<'a, 'b, Img, Snd, Fnt, Spr, Rm, Data>(
    title: &str, width: u32, height: u32, fps: f64, bg_color: &Color,
    start_room: Rm, rooms: &HashMap<Rm, Room<Img, Snd, Fnt, Spr, Rm, Data>>,
    snd_srcs: &[(Snd, &str)], img_srcs: &[(Img, &str)],
    font_srcs: &[(Fnt, u16, &str)]) -> Result<(), String> where
        Spr: IndexRestriction,
        Img: IndexRestriction,
        Snd: IndexRestriction,
        Fnt: IndexRestriction,
        Rm: IndexRestriction,
        Data: Clone {
    let ctx = sdl2::init()?;
    let _ = ctx.audio()?;
    let subsys = ctx.video()?;
    let win = subsys
        .window(title, width, height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut event_pump = ctx.event_pump()?;
    let mut cnv = win.into_canvas().build().map_err(|e| e.to_string())?;
    let creator = cnv.texture_creator();
    
    let ttf_ctx = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let freq = 44100;
    let format = AUDIO_S16LSB;
    let channels = DEFAULT_CHANNELS;
    let chunk_size = 1024;
    sdl2::mixer::open_audio(freq, format, channels, chunk_size)?;
    
    // Load resources from file paths
    let mut snds = HashMap::new();
    for (key, src) in snd_srcs.iter() {
        snds.insert(*key, Sound::load_music(src)?);
    }
    let mut imgs = HashMap::new();
    for (key, src) in img_srcs.iter() {
        imgs.insert(*key, Image::new(src, &creator)?);
    }
    let mut fonts = HashMap::new();
    for (key, size, src) in font_srcs.iter() {
        fonts.insert(*key, Font::new(src, *size, &ttf_ctx)?);
    }

    let mut rooms = rooms.clone();

    // Create a timed 60fps game loop
    let mut room_reset = false;
    let mut start = Instant::now();
    let mut elapsed = 0.0;
    let mut room = start_room;
    'game: loop {
        // Maintain fps
        std::thread::sleep(Duration::from_millis(1)); // Force a sleep bc CPU is really fast lol
        let delta = start.elapsed().as_secs_f64();
        start = Instant::now();
        elapsed += delta;

        if let Some(rm) = rooms.get_mut(&room) {
            // Only reset at start of loop, triggered by something in the loop
            if room_reset {
                if !rm.persistant {
                    rm.reset();
                }
                room_reset = false;
            }

            // Update
            for event in event_pump.poll_iter() {
                match event {
                    Event::KeyUp { scancode, .. } if scancode == Some(Scancode::F4) => {
                        break 'game;
                    }, Event::Quit { .. } => {
                        break 'game;
                    }, _ => {}
                }
                rm.handle_sdl_event(&event);
            }
            let new_room = rm.update(delta);

            if elapsed > 1.0 / fps {
                cnv.set_draw_color(*bg_color);
                rm.render(&mut cnv, &imgs, &snds, &fonts, &creator, elapsed)?;
                cnv.present();
                elapsed = 0.0;
            }

            if new_room.is_some() {
                room = new_room.unwrap();
                room_reset = true;
            }
        }
    }

    Ok(())
}

