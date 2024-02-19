//! Keep track of SDL context and window state as well as run main game loop

use std::{
    collections::HashMap,
    hash::Hash,
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
    font::Font,
    spr::Image,
    snd::Sound,
    room::Room
};

const FPS: f64 = 60.0;
const BG_COLOR: Color = Color::WHITE;

pub fn run<'a, 'b, ObjId, SprId, ImgId, SndId, FontId, RmId>(
    title: &str, width: u32, height: u32,
    start_room: RmId, rooms: &mut HashMap<RmId, Room<ObjId, SprId, ImgId, SndId, FontId, RmId>>,
    snd_srcs: &[(SndId, &str)], img_srcs: &[(ImgId, &str)],
    font_srcs: &[(FontId, u16, &str)]) -> Result<(), String> where
        ObjId: Hash + Eq + Clone + Copy,
        SprId: Hash + Eq + Clone + Copy,
        SndId: Hash + Eq + Clone + Copy,
        ImgId: Hash + Eq + Clone + Copy,
        FontId: Hash + Eq + Clone + Copy,
        RmId: Hash + Eq + Clone + Copy {
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

        let mut rm = rooms[&room].clone(); // Grab a mut room bc HashMaps are weird about mut

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

        if elapsed > 1.0 / FPS {
            cnv.set_draw_color(BG_COLOR);
            rm.render(&mut cnv, &imgs, &snds, &fonts, elapsed)?;
            cnv.present();
            elapsed = 0.0;
        }

        rooms.insert(room, rm);
        if new_room.is_some() {
            room = new_room.unwrap();
            room_reset = true;
        }
    }

    Ok(())
}

