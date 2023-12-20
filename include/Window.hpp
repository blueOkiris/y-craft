// Define an interface into the SDL code and global state

#pragma once

#include <SDL2/SDL.h>
#include <string>
#include <vector>
#include <map>
#include <GameObject.hpp>
#include <Room.hpp>
#include <Audio.hpp>

class Window {
    public:
        Window();
        ~Window();
        SDL_Renderer *rndrr(void) const;
        void run(
            std::map<std::string, Audio *> &sounds, std::map<std::string, Room> &rooms,
            const std::string &startRoom
        );
        void changeRoom(const std::string &room);

        const int width = 1920;
        const int height = 1080;
        const std::string title = "Play-Builder";
        const bool drawCollisionShapes = false;
        const int intentionalDelayMs = 1;
        const SDL_Color bgColor = { 0x60, 0x60, 0x80, 0xFF };

    private:
        SDL_Window *_win;
        SDL_Renderer *_rndrr;
        std::string _room;
        bool _roomReset;
};

extern Window g_window;

