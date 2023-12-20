// Entry point for play-builder engine

#include <SDL2/SDL.h>
#include <string>
#include <utility>
#include <iostream>
#include <Window.hpp>
#include <Sprite.hpp>
#include <GameObject.hpp>
#include <Player.hpp>
#include <Room.hpp>
#include <Brick.hpp>
#include <Audio.hpp>

const int g_screenWidth = 1280;
const int g_screenHeight = 720;
const std::string g_winTitle = "Play-Builder";

int main(void) {
    // Load images
    const auto charImg = Image("img/character.png", g_window.rndrr());
    const auto brickImg = Image("img/brick.png", g_window.rndrr());

    // Load audio
    auto music = Audio("audio/awake10_megaWall.mp3", AudioClipType::Music);
    auto jump = Audio("audio/sfx_movement_jump10.wav", AudioClipType::Chunk);
    std::map<std::string, Audio *> sounds = {
        { "music", &music },
        { "jump", &jump }
    };
    std::cout << "Here!" << std::endl;

    // Define sprites
    auto idle = Sprite(
        std::vector<Frame>({ Frame(charImg, { 0, 0, 32, 32 }, { 128, 128 }), }),
        0.0, { 16, 16 }
    );
    idle.scale = { 0.5, 0.5 };
    auto walk = Sprite(
        std::vector<Frame>({
            Frame(charImg, { 0, 0, 32, 32 }, { 128, 128 }),
            Frame(charImg, { 32, 0, 32, 32 }, { 128, 128 })
        }), 12.0, { 16, 16 }
    );
    walk.scale = { 0.5, 0.5 };
    auto brickSpr = Sprite(
        std::vector<Frame>({ Frame(brickImg, { 0, 0, 32, 32 }, { 64, 64 }) }),
        0.0, { 16, 16 }
    );

    // Create Rooms with Game Objects in them
    const auto playerColl = CollisionShape {
        .shapeType = CollShapeType::Rect,
        .center = { 32, 32 },
        .width = 64,
        .height = 64
    };
    auto player = Player("player", { 256.0, 900.0 }, idle, playerColl, idle, walk);
    auto brick3 = Brick("brick3", { 96.0, 900.0 }, brickSpr, playerColl);
    auto brick4 = Brick("brick4", { 160.0, 1000.0 }, brickSpr, playerColl);
    auto brick5 = Brick("brick5", { 224.0, 1000.0 }, brickSpr, playerColl);
    auto brick6 = Brick("brick6", { 288.0, 1000.0 }, brickSpr, playerColl);
    auto brick1 = Brick("brick1", { 352.0, 1000.0 }, brickSpr, playerColl);
    auto brick2 = Brick("brick2", { 416.0, 1000.0 }, brickSpr, playerColl);
    auto rm1 = Room(
        false,
        std::vector<GameObject *>({
            (GameObject *) (&player),
            (GameObject *) (&brick1),
            (GameObject *) (&brick2),
            (GameObject *) (&brick3),
            (GameObject *) (&brick4),
            (GameObject *) (&brick5),
            (GameObject *) (&brick6)
        })
    );
    std::map<std::string, Room> rooms = {
        { "room1", rm1 }
    };

    // Start the game!
    g_window.run(sounds, rooms, "room1");
    return 0;
}
