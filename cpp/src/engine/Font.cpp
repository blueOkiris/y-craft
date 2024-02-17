// Implement font functionality

#include <cstdint>
#include <string>
#include <iostream>
#include <utility>
#include <SDL2/SDL_ttf.h>
#include <engine/Font.hpp>

Font::Font(const std::string &src, const int size) {
    _font = TTF_OpenFont(src.c_str(), size);
    if (!_font) {
        std::cerr
            << "Failed to load font '" << src
            << "'. TTF Error: " << TTF_GetError() << std::endl;
        exit(1);
    }
}

Font::~Font(void) {
    TTF_CloseFont(_font);
    _font = nullptr;
}

void Font::render(
        SDL_Renderer *rndrr,
        const std::string msg, uint8_t r, uint8_t g, uint8_t b, uint8_t a,
        const std::pair<int, int> &pos, const double angle, const std::pair<bool, bool> &flip) {
    auto txtSfc = TTF_RenderText_Solid(_font, msg.c_str(), { r, g, b, a });
    if (!txtSfc) {
        std::cerr
            << "Warning! Failed to render text surface! TTF Error: " << TTF_GetError()
            << std::endl;
        return;
    }
    auto tex = SDL_CreateTextureFromSurface(rndrr, txtSfc);
    if (!tex) {
        std::cerr
            << "Warning! Failed to create optimized texture from text surface!" << TTF_GetError()
            << std::endl;
        return;
    }
    const auto width = txtSfc->w;
    const auto height = txtSfc->h;
    SDL_FreeSurface(txtSfc);

    auto dest = SDL_Rect {
        .x = pos.first,
        .y = pos.second,
        .w = width,
        .h = height
    };
    SDL_RenderCopyEx(
        rndrr, tex, nullptr, &dest, angle, nullptr,
        static_cast<SDL_RendererFlip>(
            (flip.first ? static_cast<int>(SDL_FLIP_HORIZONTAL) : 0)
                | (flip.second ? static_cast<int>(SDL_FLIP_VERTICAL) : 0)
        )
    );
    SDL_DestroyTexture(tex);
}

