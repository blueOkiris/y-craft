// Auto-animating collections of images that get attached to game objects

#pragma once

#include <string>
#include <vector>
#include <map>
#include <utility>
#include <SDL2/SDL.h>

class Image {
    public:
        Image(const std::string &imgSrc, SDL_Renderer *rndrr);
        ~Image(void);
        void render(
            SDL_Renderer *rndrr, const SDL_Rect &src, const SDL_Rect &dest,
            const double angle,
            const std::pair<bool, bool> &flip
        ) const;
    private:
        SDL_Texture *_tex;
};

class Frame {
    public:
        Frame(
            const std::string &srcImage,
            const SDL_Rect &srcClip,
            const std::pair<int, int> &drawSize
        );
        void render(
            SDL_Renderer *rndrr,
            const std::pair<int, int> &pos,
            const std::pair<int, int> &origin,
            const std::pair<double, double> &scale,
            const double angle,
            const std::pair<bool, bool> &flip
        ) const;
        const std::string image;
        const SDL_Rect clip;
        const std::pair<int, int> size;
};

class Sprite {
    public:
        Sprite(
            const std::vector<Frame> &subFrames,
            const double fps,
            const std::pair<int, int> &drawOrigin
        );
        void update(const double delta);
        void render(SDL_Renderer *rndrr, const std::pair<int, int> &pos) const;

        std::vector<Frame> frames;
        double animSpeed;
        std::pair<int, int> origin;
        size_t index;
        std::pair<double, double> scale;
        double angle;
        std::pair<bool, bool> flip;

    private:
        double _indexCount;
};

