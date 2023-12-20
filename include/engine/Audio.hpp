// Audio objects in the world

#pragma once

#include <string>
#include <SDL2/SDL_mixer.h>

enum class AudioClipType {
    Music,
    Chunk
};

struct AudioClip {
    AudioClipType clipType;
    union {
        Mix_Music *music;
        struct Mix_Chunk *chunk;
    };
};

class Audio {
    public:
        static bool isMusicPlaying(void);
        static void pauseMusic(void);
        static void resumeMusic(void);
        static void haltMusic(void);

        Audio(const std::string &src, const AudioClipType clipType);
        ~Audio(void);
        void play(void) const;

    private:
        AudioClip _clip;
};

