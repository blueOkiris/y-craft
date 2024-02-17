// Way to store and use font data in the engine

#pragma once

#include <cstdint>
#include <string>
#include <SDL2/SDL_ttf.h>
#include <SDL2/SDL.h>

class Font {
    public:
        Font(const std::string &src, const int size);
        ~Font(void);
        void render(
            SDL_Renderer *rndrr,
            const std::string msg, uint8_t r, uint8_t g, uint8_t b, uint8_t a,
            const std::pair<int, int> &pos, const double angle, const std::pair<bool, bool> &flip
        );

    private:
        TTF_Font *_font;
};

