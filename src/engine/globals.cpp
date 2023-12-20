// Actual starting value of globals

#include <string>
#include <memory>
#include <engine/Audio.hpp>
#include <engine/Sprite.hpp>
#include <engine/Room.hpp>
#include <engine/Window.hpp>
#include <engine/globals.hpp>

// Project organization
std::map<std::string, std::shared_ptr<Audio>> globals::sounds;
std::map<std::string, std::shared_ptr<Image>> globals::images;
std::map<std::string, Room> globals::rooms;
Window globals::win;

// Engine tweaks
const double globals::fps = 60.0;
const std::string globals::winTitle = "x-craft";
const int globals::winWidth = 1920;
const int globals::winHeight = 1080;
const bool globals::drawCollisionShapes = false;
const int globals::minimumUpdateTimeMs = 1;

