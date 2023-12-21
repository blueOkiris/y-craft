// Global values used by the engine, in an easy to tweak place

#pragma once

#include <string>
#include <memory>
#include <engine/Audio.hpp>
#include <engine/Sprite.hpp>
#include <engine/Font.hpp>
#include <engine/Room.hpp>
#include <engine/Window.hpp>

namespace globals {
    // Project organization
    extern std::map<std::string, std::shared_ptr<Audio>> sounds;
    extern std::map<std::string, std::shared_ptr<Image>> images;
    extern std::map<std::string, std::shared_ptr<Font>> fonts;
    extern std::map<std::string, Room> rooms;
    extern Window win;

    // Engine tweaks
    extern const double fps;
    extern const std::string winTitle;
    extern const int winWidth;
    extern const int winHeight;
    extern const bool drawCollisionShapes;
    extern const int minimumUpdateTimeMs;
}

