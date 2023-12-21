// Set up the main game room

#include <vector>
#include <memory>
#include <engine/GameObject.hpp>
#include <engine/Room.hpp>
#include <gameobjs/TitleScreenImage.hpp>
#include <rooms/game.hpp>

Room rooms::game(void) {
    return Room(
        false, std::vector<std::shared_ptr<GameObject>>({
        })
    );
}

