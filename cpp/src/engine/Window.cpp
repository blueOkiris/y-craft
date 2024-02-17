// Implement Window class functionality

#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>
#include <SDL2/SDL_mixer.h>
#include <SDL2/SDL_ttf.h>
#include <string>
#include <iostream>
#include <utility>
#include <chrono>
#include <thread>
#include <vector>
#include <map>
#include <engine/Sprite.hpp>
#include <engine/GameObject.hpp>
#include <engine/Room.hpp>
#include <engine/Audio.hpp>
#include <engine/Window.hpp>
#include <engine/globals.hpp>

Window::Window() {
    if (SDL_Init(SDL_INIT_VIDEO | SDL_INIT_AUDIO) < 0) {
        std::cerr << "SDL could not initialize! Error: " << SDL_GetError() << std::endl;
        exit(1);
    }
    if (!(IMG_Init(IMG_INIT_PNG) & IMG_INIT_PNG)) {
        std::cerr << "SDL_image could not initialize! Error: " << IMG_GetError() << std::endl;
        exit(1);
    }
    if (Mix_OpenAudio(44100, MIX_DEFAULT_FORMAT, 2, 2048) < 0) {
        std::cerr << "SDL_mixer could not initialize! Error: " << Mix_GetError() << std::endl;
        exit(1);
    }
    if (TTF_Init() == -1) {
        std::cerr << "SDL_ttf could not initialize! Error: " << TTF_GetError() << std::endl;
        exit(1);
    }
    _win = SDL_CreateWindow(
        globals::winTitle.c_str(),
        SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED,
        globals::winWidth, globals::winHeight,
        SDL_WINDOW_SHOWN
    );
    if (!_win) {
        std::cerr << "Window could not be created! Error: " << SDL_GetError() << std::endl;
        exit(1);
    }
    _rndrr = SDL_CreateRenderer(_win, -1, SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC);
    if (!_rndrr) {
        std::cerr << "Failed to create renderer! Error: " << SDL_GetError() << std::endl;
        exit(1);
    }
}

Window::~Window() {
    SDL_DestroyRenderer(_rndrr);
    SDL_DestroyWindow(_win);
    _rndrr = nullptr;
    _win = nullptr;
    IMG_Quit();
    Mix_Quit();
    SDL_Quit();
}

SDL_Renderer *Window::rndrr(void) const {
    return _rndrr;
}

void Window::run(const std::string &startRoom) {
    auto quit = false;
    SDL_Event event;
    auto startTime = std::chrono::high_resolution_clock::now();
    double elapsed = 0.0;
    _room = startRoom;
    while (!quit) {
        // Maintain fps
        std::this_thread::sleep_for(std::chrono::duration<int, std::milli>(
            globals::minimumUpdateTimeMs
        ));
        auto endTime = std::chrono::high_resolution_clock::now();
        const auto delta = endTime - startTime;
        const auto deltaUs = std::chrono::duration_cast<std::chrono::milliseconds>(delta);
        const auto deltaTime = static_cast<double>(deltaUs.count()) / 1000.0;
        startTime = std::chrono::high_resolution_clock::now();
        elapsed += deltaTime;

        if (_roomReset) {
            globals::rooms.at(_room).reset();
            _roomReset = false;
        }

        const auto loopRoom = _room; // Only change room at end of loop

        while (SDL_PollEvent(&event) != 0) {
            switch (event.type) {
                case SDL_KEYUP:
                    if (event.key.keysym.scancode == SDL_SCANCODE_F4) {
                        SDL_SetWindowFullscreen(_win, SDL_WINDOW_FULLSCREEN);
                    }
                    break;
                case SDL_QUIT:
                    quit = true;
                    break;
            }
            globals::rooms.at(loopRoom).handleSdlEvent(event);
        }
        globals::rooms.at(loopRoom).update(deltaTime);
        if (elapsed > 1.0 / globals::fps) {
            SDL_SetRenderDrawColor(_rndrr, bgColor.r, bgColor.g, bgColor.b, bgColor.a);
            globals::rooms.at(loopRoom).render(_rndrr, elapsed);
            SDL_RenderPresent(_rndrr);
            elapsed = 0.0;
        }
    }
}

void Window::changeRoom(const std::string &room) {
    _room = room;
    _roomReset = true;
}

void Window::tryRmObjRoom(const std::string &id) {
    for (
            std::vector<std::shared_ptr<GameObject>>::iterator it =
                globals::rooms.at(_room).gameObjs.begin();
            it != globals::rooms.at(_room).gameObjs.end();
            ++it) {
        if ((*it)->id == id) {
            globals::rooms.at(_room).gameObjs.erase(it);
            return;
        }
    }
}

void Window::addObjRoom(const std::shared_ptr<GameObject> &newObj) {
    globals::rooms.at(_room).gameObjs.push_back(newObj);
}

