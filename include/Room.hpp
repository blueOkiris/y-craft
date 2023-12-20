// Defines a collection of GameObjects. Main style of game state

#pragma once

#include <vector>
#include <SDL2/SDL.h>
#include <GameObject.hpp>
#include <Audio.hpp>

class Room {
    public:
        Room(const bool persists, const std::vector<GameObject *> &roomObjs);
        void handleSdlEvent(std::map<std::string, Audio *> &sounds, const SDL_Event &ev);
        void update(std::map<std::string, Audio *> &sounds, const double delta);
        void render(SDL_Renderer *rndrr, const double elapsed);
        void reset(void);

        const bool persistant;
        std::vector<GameObject *> gameObjs;
};

