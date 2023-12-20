// Implement Window class functionality

#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>
#include <SDL2/SDL_mixer.h>
#include <string>
#include <iostream>
#include <utility>
#include <chrono>
#include <thread>
#include <vector>
#include <map>
#include <Sprite.hpp>
#include <GameObject.hpp>
#include <Room.hpp>
#include <Audio.hpp>
#include <Window.hpp>

Window g_window;

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
    _win = SDL_CreateWindow(
        title.c_str(),
        SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, width, height,
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

void Window::run(
        std::map<std::string, Audio *> &sounds, std::map<std::string, Room> &rooms,
        const std::string &startRoom) {
    auto quit = false;
    SDL_Event event;
    auto startTime = std::chrono::high_resolution_clock::now();
    double elapsed = 0.0;
    _room = startRoom;
    while (!quit) {
        // Maintain fps
        std::this_thread::sleep_for(std::chrono::duration<int, std::milli>(intentionalDelayMs));
        auto endTime = std::chrono::high_resolution_clock::now();
        const auto delta = endTime - startTime;
        const auto deltaUs = std::chrono::duration_cast<std::chrono::milliseconds>(delta);
        const auto deltaTime = static_cast<double>(deltaUs.count()) / 1000.0;
        startTime = std::chrono::high_resolution_clock::now();
        elapsed += deltaTime;

        if (_roomReset) {
            rooms.at(_room).reset();
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
            rooms.at(loopRoom).handleSdlEvent(sounds, event);
        }
        rooms.at(loopRoom).update(sounds, deltaTime);

        if (elapsed > 1.0 / 60.0) {
            std::cout << "Fps: " << (1.0 / elapsed) << std::endl;
            SDL_SetRenderDrawColor(_rndrr, bgColor.r, bgColor.g, bgColor.b, bgColor.a);
            rooms.at(loopRoom).render(_rndrr, elapsed);
            SDL_RenderPresent(_rndrr);
            elapsed = 0.0;
        }
    }
}

void Window::changeRoom(const std::string &room) {
    _room = room;
    _roomReset = true;
}

