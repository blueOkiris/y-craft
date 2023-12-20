// Implement loading images and updating animations for Sprites (and sub-objects)

#include <string>
#include <vector>
#include <iostream>
#include <map>
#include <utility>
#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>
#include <Sprite.hpp>

SDL_Texture *loadTex(
        const std::string &imgSrc, SDL_Renderer *rndrr) {
    SDL_Surface *srfc = IMG_Load(imgSrc.c_str());
    if (!srfc) {
        std::cerr
            << "Unable to load image '" << imgSrc << "'. Error: "
            << IMG_GetError() << std::endl;
        exit(1);
    }
    SDL_Texture *tex = SDL_CreateTextureFromSurface(rndrr, srfc);
    if (!tex) {
        std::cerr
            << "Unable to create texture from '" << imgSrc << "'. Error: "
            << SDL_GetError() << std::endl;
        exit(1);
    }
    SDL_FreeSurface(srfc);
    return tex;
}

Image::Image(const std::string &imgSrc, SDL_Renderer *rndrr): _tex(loadTex(imgSrc, rndrr)) {}

Image::~Image(void) {
    SDL_DestroyTexture(_tex);
    _tex = nullptr;
}

void Image::render(
        SDL_Renderer *rndrr, const SDL_Rect &src, const SDL_Rect &dest,
        const double angle,
        const std::pair<bool, bool> &flip) const {
    SDL_RenderCopyEx(
        rndrr, _tex, &src, &dest, angle, nullptr,
        static_cast<SDL_RendererFlip>(
            (flip.first ? static_cast<int>(SDL_FLIP_HORIZONTAL) : 0)
                | (flip.second ? static_cast<int>(SDL_FLIP_VERTICAL) : 0)
        )
    );
}

Frame::Frame(
        const Image &srcImage, const SDL_Rect &srcClip,
        const std::pair<int, int> &drawSize):
            image(srcImage), clip(srcClip), size(drawSize) {}

void Frame::render(
        SDL_Renderer *rndrr,
        const std::pair<int, int> &pos,
        const std::pair<int, int> &origin,
        const std::pair<double, double> &scale,
        const double angle,
        const std::pair<bool, bool> &flip) const {
    const std::pair<double, double> baseScale = {
        static_cast<double>(size.first) / static_cast<double>(clip.w),
        static_cast<double>(size.second) / static_cast<double>(clip.h)
    };
    SDL_Rect dest = {
        .x = pos.first
            - static_cast<int>(static_cast<double>(origin.first) * baseScale.first * scale.first),
        .y = pos.second
            - static_cast<int>(
                static_cast<double>(origin.second) * baseScale.second * scale.second
            ),
        .w = static_cast<int>(static_cast<double>(size.first) * scale.first),
        .h = static_cast<int>(static_cast<double>(size.second) * scale.second)
    };
    image.render(rndrr, clip, dest, angle, flip);
}

Sprite::Sprite(
        const std::vector<Frame> &subFrames, const double fps,
        const std::pair<int, int> &drawOrigin):
            frames(subFrames), animSpeed(fps), origin(drawOrigin), index(0), scale({ 1.0, 1.0 }),               angle(0.0), flip({ false, false }) {}

void Sprite::update(const double delta) {
    _indexCount += delta * animSpeed;
    if (_indexCount > 1.0) {
        if (index + 1 >= frames.size()) {
            index = 0;
        } else {
            index++;
        }
        _indexCount = 0.0;
    }
}

void Sprite::render(SDL_Renderer *rndrr, const std::pair<int, int> &pos) const {
    const auto frame = frames.at(index);
    frame.render(rndrr, pos, origin, scale, angle, flip);
}

void Sprite::setTo(const Sprite &from) {
    frames = std::vector(from.frames);
    animSpeed = from.animSpeed;
    origin = from.origin;
    index = 0;
    scale = from.scale;
}

