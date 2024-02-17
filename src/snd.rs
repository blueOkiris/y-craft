//! Play music and sound effects

use sdl2::mixer::{
    Channel, Chunk, Music
};

pub enum Sound<'a> {
    Music(Music<'a>),
    Chunk(Chunk)
}

impl<'a> Sound<'a> {
    pub fn load_music(src: &str) -> Result<Self, String> {
        Ok(Self::Music(Music::from_file(src)?))
    }

    pub fn load_chunk(src: &str) -> Result<Self, String> {
        Ok(Self::Chunk(Chunk::from_file(src)?))
    }

    pub fn is_music_playing() -> bool {
        Music::is_playing()
    }

    pub fn pause_music() {
        Music::pause()
    }

    pub fn resume_music() {
        Music::resume()
    }

    pub fn halt_music() {
        Music::halt()
    }

    pub fn play(&self) -> Result<(), String> {
        match self {
            Sound::Music(music) => {
                music.play(-1).map_err(|e| e.to_string())?;
            }, Sound::Chunk(chunk) => {
                Channel::all().play(chunk, -1).map_err(|e| e.to_string())?;
            }
        }
        Ok(())
    }
}

