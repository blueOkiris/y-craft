// Implement SnakeHead motion

#include <utility>
#include <string>
#include <map>
#include <memory>
#include <sstream>
#include <engine/globals.hpp>
#include <engine/Sprite.hpp>
#include <engine/GameObject.hpp>
#include <gameobjs/SnakeHead.hpp>

SnakeHead::SnakeHead(void):
    GameObject(
        "snakeHead", { 640.0 / 2.0 + 32.0 + 32.0 / 2.0, 360.0 / 2.0 - 32.0 / 2.0 },
        "head", std::map<std::string, Sprite>({
            {
                "head",
                Sprite(std::vector<Frame>({
                    Frame("snake.png", { 0, 0, 32, 32 }, { 32, 32 })
                }), 0.0, { 16.0, 16.0 })
            }
        }), {
            .shapeType = CollShapeType::Rect,
            .center = { 16, 16 },
            .width = 32,
            .height = 32
        }
    ) {}

std::string SnakeHead::tag(void) const {
    return "SnakeHead";
}

void SnakeHead::update(
        const double delta, const std::vector<std::shared_ptr<GameObject>> &others) {
    _sprs.at(_curSpr).angle = 90.0;
}

void SnakeHead::handleSdlEvent(const SDL_Event &ev) {}
void SnakeHead::onCollision(const std::shared_ptr<GameObject> &other) {}
void SnakeHead::reset(void) {}

std::string snakeBodyName(const int ind) {
    std::stringstream name;
    name << "snakeBody" << ind;
    return name.str();
}

SnakeBody::SnakeBody(const int ind, const std::pair<double, double> &defPos):
    GameObject(
        snakeBodyName(ind),
        defPos,
        "head", std::map<std::string, Sprite>({
            {
                "head",
                Sprite(std::vector<Frame>({
                    Frame("snake.png", { 32, 0, 32, 32 }, { 32, 32 })
                }), 0.0, { 16.0, 16.0 })
            }
        }), {
            .shapeType = CollShapeType::Rect,
            .center = { 16, 16 },
            .width = 32,
            .height = 32
        }
    ) {}

std::string SnakeBody::tag(void) const {
    return "SnakeBody";
}

void SnakeBody::update(
        const double delta, const std::vector<std::shared_ptr<GameObject>> &others) {
    _sprs.at(_curSpr).angle = 90.0;
}

void SnakeBody::handleSdlEvent(const SDL_Event &ev) {}
void SnakeBody::onCollision(const std::shared_ptr<GameObject> &other) {}
void SnakeBody::reset(void) {}

SnakeTail::SnakeTail(void):
    GameObject(
        "snakeTail", { 640.0 / 2.0 - 32.0 - 32.0 / 2.0, 360.0 / 2.0 - 32.0 / 2.0 },
        "tail", std::map<std::string, Sprite>({
            {
                "tail",
                Sprite(std::vector<Frame>({
                    Frame("snake.png", { 0, 32, 32, 32 }, { 32, 32 })
                }), 0.0, { 16.0, 16.0 })
            }
        }), {
            .shapeType = CollShapeType::Rect,
            .center = { 16, 16 },
            .width = 32,
            .height = 32
        }
    ) {}

std::string SnakeTail::tag(void) const {
    return "SnakeTail";
}

void SnakeTail::update(
        const double delta, const std::vector<std::shared_ptr<GameObject>> &others) {
    _sprs.at(_curSpr).angle = 90.0;
}

void SnakeTail::handleSdlEvent(const SDL_Event &ev) {}
void SnakeTail::onCollision(const std::shared_ptr<GameObject> &other) {}
void SnakeTail::reset(void) {}
