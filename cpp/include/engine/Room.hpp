// Defines a collection of GameObjects. Main style of game state

#pragma once

#include <vector>
#include <memory>
#include <SDL2/SDL.h>
#include <engine/GameObject.hpp>

class Room {
    public:
        Room(const bool persists, const std::vector<std::shared_ptr<GameObject>> &roomObjs);
        void handleSdlEvent(const SDL_Event &ev);
        void update(const double delta);
        void render(SDL_Renderer *rndrr, const double elapsed);
        void reset(void);

        const bool persistant;
        std::vector<std::shared_ptr<GameObject>> gameObjs;
};

