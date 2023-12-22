// Set up the main game room

#include <vector>
#include <memory>
#include <utility>
#include <engine/GameObject.hpp>
#include <engine/Room.hpp>
#include <gameobjs/SnakeHead.hpp>
#include <gameobjs/Mouse.hpp>
#include <rooms/game.hpp>

Room rooms::game(void) {
    return Room(
        false, std::vector<std::shared_ptr<GameObject>>({
            std::dynamic_pointer_cast<GameObject>(std::make_shared<SnakeHead>()),
            std::dynamic_pointer_cast<GameObject>(std::make_shared<SnakeBody>(
                0, std::make_pair(640.0 / 2 + 32.0 / 2.0, 352.0 / 2.0)
            )),
            std::dynamic_pointer_cast<GameObject>(std::make_shared<SnakeBody>(
                1, std::make_pair(640.0 / 2 - 32.0 / 2.0, 352.0 / 2.0)
            )), std::dynamic_pointer_cast<GameObject>(std::make_shared<SnakeTail>()),
            std::dynamic_pointer_cast<GameObject>(std::make_shared<Mouse>())
        })
    );
}

