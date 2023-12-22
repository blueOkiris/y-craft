// Implement SnakeHead motion

#include <utility>
#include <string>
#include <map>
#include <memory>
#include <sstream>
#include <cmath>
#include <engine/globals.hpp>
#include <engine/Sprite.hpp>
#include <engine/GameObject.hpp>
#include <gameobjs/SnakeHead.hpp>

SnakeHead::SnakeHead(void):
    GameObject(
        "snakeHead", { 640.0 / 2.0 + 32.0 + 32.0 / 2.0, 352.0 / 2.0 },
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
    ), _dir(0), _moveSpd(baseMoveSpd), _interPos(pos), _canChangeDir(false) {}

std::string SnakeHead::tag(void) const {
    return "SnakeHead";
}

void SnakeHead::update(
        const double delta, const std::vector<std::shared_ptr<GameObject>> &others) {
    switch (_dir) {
        default:
        case 0:
            _sprs.at(_curSpr).angle = 90.0;
            _interPos.first += delta * _moveSpd;
            if (std::floor(_interPos.first) > pos.first + 32.0) {
                for (const auto &other : others) {
                    if (other->id == "snakeBody0") {
                        other->pos = pos;
                        std::dynamic_pointer_cast<SnakeBody>(other)->dir = 0;
                    }
                }
                _canChangeDir = true;
                pos.first += 32.0;
            }
            break;
        case 1:
            _sprs.at(_curSpr).angle = 180.0;
            _interPos.second += delta * _moveSpd;
            if (std::floor(_interPos.second) > pos.second + 32.0) {
                for (const auto &other : others) {
                    if (other->id == "snakeBody0") {
                        other->pos = pos;
                        std::dynamic_pointer_cast<SnakeBody>(other)->dir = 1;
                    }
                }
                pos.second += 32.0;
                _canChangeDir = true;
            }
            break;
        case 2:
            _sprs.at(_curSpr).angle = 270.0;
            _interPos.first -= delta * _moveSpd;
            if (std::ceil(_interPos.first) < pos.first - 32.0) {
                for (const auto &other : others) {
                    if (other->id == "snakeBody0") {
                        other->pos = pos;
                        std::dynamic_pointer_cast<SnakeBody>(other)->dir = 2;
                    }
                }
                pos.first -= 32.0;
                _canChangeDir = true;
            }
            break;
        case 3:
            _sprs.at(_curSpr).angle = 0.0;
            _interPos.second -= delta * _moveSpd;
            if (std::ceil(_interPos.second) < pos.second - 32.0) {
                for (const auto &other : others) {
                    if (other->id == "snakeBody0") {
                        other->pos = pos;
                        std::dynamic_pointer_cast<SnakeBody>(other)->dir = 3;
                    }
                }
                pos.second -= 32.0;
                _canChangeDir = true;
            }
            break;
    }
}

void SnakeHead::handleSdlEvent(const SDL_Event &ev) {
    switch (ev.type) {
        case SDL_KEYDOWN:
            if (_canChangeDir) {
                double posDif = 0.0;
                switch (_dir) {
                    default:
                    case 0:
                    case 2:
                        posDif = std::abs(_interPos.first - pos.first);
                        break;
                    case 1:
                    case 3:
                        posDif = std::abs(_interPos.second - pos.second);
                        break;
                }
                _interPos = pos;
                switch (ev.key.keysym.scancode) {
                    case SDL_SCANCODE_UP:
                        _canChangeDir = false;
                        _dir = 3;
                        _interPos.second -= posDif;
                        break;
                    case SDL_SCANCODE_DOWN:
                        _canChangeDir = false;
                        _dir = 1;
                        _interPos.second += posDif;
                        break;
                    case SDL_SCANCODE_LEFT:
                        _canChangeDir = false;
                        _dir = 2;
                        _interPos.first -= posDif;
                        break;
                    case SDL_SCANCODE_RIGHT:
                        _canChangeDir = false;
                        _dir = 0;
                        _interPos.first += posDif;
                        break;
                    default:
                        break;
                }
            }
            break;
        default:
            break;
    }
}

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
    ), dir(0), lastDir(0), index(ind), lastPos(pos) {}

std::string SnakeBody::tag(void) const {
    return "SnakeBody";
}

void SnakeBody::update(
        const double delta, const std::vector<std::shared_ptr<GameObject>> &others) {
    if (pos.first != lastPos.first || pos.second != lastPos.second) {
        std::stringstream child;
        child << "snakeBody" << (index + 1);

        bool foundChild = false;
        for (const auto &other : others) {
            if (other->id == child.str()) {
                foundChild = true;
                other->pos = lastPos;
                std::dynamic_pointer_cast<SnakeBody>(other)->dir = lastDir;
            }
        }
        if (!foundChild) {
            // Do tail for last body
            for (const auto &other : others) {
                if (other->id == "snakeTail") {
                    other->pos = lastPos;
                    std::dynamic_pointer_cast<SnakeTail>(other)->dir = lastDir;
                }
            }
        }

        lastPos = pos;
        lastDir = dir;
    }
    switch(dir) {
        default:
        case 0:
            _sprs.at(_curSpr).angle = 90.0;
            break;
        case 1:
            _sprs.at(_curSpr).angle = 180.0;
            break;
        case 2:
            _sprs.at(_curSpr).angle = 270.0;
            break;
        case 3:
            _sprs.at(_curSpr).angle = 0.0;
            break;
    }
}

void SnakeBody::handleSdlEvent(const SDL_Event &ev) {}
void SnakeBody::onCollision(const std::shared_ptr<GameObject> &other) {}
void SnakeBody::reset(void) {}

SnakeTail::SnakeTail(void):
    GameObject(
        "snakeTail", { 640.0 / 2.0 - 32.0 - 32.0 / 2.0, 352.0 / 2.0 },
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
    switch(dir) {
        default:
        case 0:
            _sprs.at(_curSpr).angle = 90.0;
            break;
        case 1:
            _sprs.at(_curSpr).angle = 180.0;
            break;
        case 2:
            _sprs.at(_curSpr).angle = 270.0;
            break;
        case 3:
            _sprs.at(_curSpr).angle = 0.0;
            break;
    }
}

void SnakeTail::handleSdlEvent(const SDL_Event &ev) {}
void SnakeTail::onCollision(const std::shared_ptr<GameObject> &other) {}
void SnakeTail::reset(void) {}
