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
        InitFlag, AUDIO_S16LSB, DEFAULT_CHANNELS
    }, pixels::Color
};
use crate::{
    obj::ControlObjectBehavior,
    res::{
        Font,
        Image,
        Sound
    }, room::Room, IndexRestriction
};

/// Create a window and run the game
///
/// - Title, width, height, fps, and bg_color all refer to window params
/// - start_room and rooms are the "scenes" of your game
/// - ctl_objs are objects that are updated and exist outside of the room
/// - snd_srcs, img_srcs, and font_srcs are file paths to resources
pub fn run<'a, 'b, Img, Snd, Fnt, Spr, Rm, Data>(
    title: &str, width: u32, height: u32, fps: f64, bg_color: &Color,
    start_room: Rm, rooms: &HashMap<Rm, Room<Img, Snd, Fnt, Spr, Rm, Data>>,
    ctl_objs: &Vec<Box<dyn ControlObjectBehavior<Img, Snd, Fnt, Spr, Rm, Data>>>,
    snd_srcs: &[(Snd, &'static [u8], bool)], img_srcs: &[(Img, &[u8])],
    font_srcs: &[(Fnt, u16, &[u8])]) -> Result<(), String> where
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
    let _ = sdl2::mixer::init(InitFlag::MP3 | InitFlag::FLAC | InitFlag::MOD | InitFlag::OGG)?;
    sdl2::mixer::allocate_channels(16);
    
    // Load resources from file paths
    let mut snds = HashMap::new();
    for (key, src, is_music) in snd_srcs.iter() {
        if *is_music {
            snds.insert(*key, Sound::load_music(src)?);
        } else {
            snds.insert(*key, Sound::load_chunk(src)?);
        }
    }
    let mut imgs = HashMap::new();
    for (key, src) in img_srcs.iter() {
        let mut img = image::load_from_memory(src)
            .map_err(|e| e.to_string())?
            .to_rgba8();
        imgs.insert(*key, Image::new(&mut img, &creator)?);
    }
    let mut fonts = HashMap::new();
    for (key, size, src) in font_srcs.iter() {
        fonts.insert(*key, Font::new(src, *size, &ttf_ctx)?);
    }

    let mut rooms = rooms.clone();
    let mut ctl_objs = ctl_objs.clone();

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
            
            // TODO: ADD PRE-EMPTIVE COLLISIONS

            for event in event_pump.poll_iter() {
                match event {
                    Event::KeyUp { scancode, .. } if scancode == Some(Scancode::F4) => {
                        break 'game;
                    }, Event::Quit { .. } => {
                        break 'game;
                    }, _ => {}
                }
                rm.handle_sdl_event(&event);
                for obj in ctl_objs.iter_mut() {
                    obj.handle_sdl_event(&event);
                }
            }
            let other_ctls = ctl_objs.clone();
            let mut new_room = rm.update(delta, &other_ctls);
            let rm_objs = rm.objs.clone();
            let mut to_add = vec![];
            for obj in ctl_objs.iter_mut() {
                let ret = obj.update(delta, &room, &other_ctls, &rm_objs);
                if ret.0.is_some() && new_room.is_none() {
                    new_room = ret.0;
                }
                if ret.1.len() > 0 && to_add.len() < 1 {
                    to_add = ret.1.clone();
                }
            }
            if to_add.len() > 0 {
                rm.objs.append(&mut to_add);
            }

            if elapsed > 1.0 / fps {
                cnv.set_draw_color(*bg_color);
                rm.render(&mut cnv, &imgs, &snds, &fonts, &creator, elapsed)?;
                for obj in ctl_objs.iter_mut() {
                    obj.render(&mut cnv, &room, &imgs, &snds, &fonts, &creator, elapsed)?;
                }
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

