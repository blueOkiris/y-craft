// Implement audio functionality

#include <string>
#include <iostream>
#include <SDL2/SDL_mixer.h>
#include <engine/Audio.hpp>

bool Audio::isMusicPlaying() {
    return Mix_PlayingMusic() != 0;
}

void Audio::pauseMusic() {
    Mix_PauseMusic();
}

void Audio::resumeMusic() {
    Mix_ResumeMusic();
}

void Audio::haltMusic() {
    Mix_HaltMusic();
}

Audio::Audio(const std::string &src, const AudioClipType clipType) {
    _clip.clipType = clipType;
    switch (clipType) {
        case AudioClipType::Music:
            _clip.music = Mix_LoadMUS(src.c_str());
            if (!_clip.music) {
                std::cerr << "Failed to load music from '" << src << "'" << std::endl;
                exit(1);
            }
            break;
        case AudioClipType::Chunk:
            _clip.chunk = Mix_LoadWAV(src.c_str());
            if (!_clip.chunk) {
                std::cerr << "Failed to load sound effect from '" << src << "'" << std::endl;
                exit(1);
            }
            break;
    }
}

Audio::~Audio(void) {
    switch (_clip.clipType) {
        case AudioClipType::Music:
            Mix_FreeMusic(_clip.music);
            _clip.music = nullptr;
            break;
        case AudioClipType::Chunk:
            Mix_FreeChunk(_clip.chunk);
            _clip.chunk = nullptr;
            break;
    }
}

void Audio::play(void) const {
    switch (_clip.clipType) {
        case AudioClipType::Music:
            Mix_PlayMusic(_clip.music, -1);
            break;
        case AudioClipType::Chunk:
            Mix_PlayChannel(-1, _clip.chunk, 0);
            break;
    }
}

