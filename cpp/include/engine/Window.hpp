// Define an interface into the SDL code and global state

#pragma once

#include <SDL2/SDL.h>
#include <string>
#include <vector>
#include <map>
#include <engine/GameObject.hpp>
#include <engine/Room.hpp>
#include <engine/Audio.hpp>

class Window {
    public:
        Window();
        ~Window();
        SDL_Renderer *rndrr(void) const;
        void run(const std::string &startRoom);
        void changeRoom(const std::string &room);
        void tryRmObjRoom(const std::string &id);
        void addObjRoom(const std::shared_ptr<GameObject> &newObj);

        const SDL_Color bgColor = { 0x60, 0x60, 0x80, 0xFF };

    private:
        SDL_Window *_win;
        SDL_Renderer *_rndrr;
        std::string _room;
        bool _roomReset;
};

