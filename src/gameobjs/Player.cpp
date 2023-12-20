// Implementation of example GameObject behavior

#include <utility>
#include <string>
#include <vector>
#include <cmath>
#include <iostream>
#include <SDL2/SDL.h>
#include <engine/math.hpp>
#include <engine/Sprite.hpp>
#include <engine/GameObject.hpp>
#include <engine/Audio.hpp>
#include <engine/globals.hpp>
#include <gameobjs/Player.hpp>

Player::Player(
    const std::string &name,
    const std::pair<double, double> &defPos):
        GameObject(
            name, defPos,
            "idle",
            std::map<std::string, Sprite>({
                {
                    "idle", Sprite(
                        std::vector<Frame>({
                            Frame("character.png", { 0, 0, 32, 32 }, { 128, 128 })
                        }), 0.0, { 16, 16 }
                    )
                }, {
                    "walk", Sprite(
                        std::vector<Frame>({
                            Frame("character.png", { 0, 0, 32, 32 }, { 128, 128 }),
                            Frame("character.png", { 32, 0, 32, 32 }, { 128, 128 })
                        }), 12.0, { 16, 16 }
                    )
                }
            }), {
                .shapeType = CollShapeType::Rect,
                .center = { 32, 32 },
                .width = 64,
                .height = 64
            }
        ), _vel({ 0.0, 0.0 }),
        _right(false), _left(false),
        _grounded(false), _extraGrav(false),
        _facingRight(true),
        _startPos(defPos) {}

std::string Player::tag(void) const {
    return "Player";
}

void Player::update(
        const double delta,
        const std::vector<std::shared_ptr<GameObject>> &others) {
    if (!Audio::isMusicPlaying()) {
        globals::sounds.at("music")->play();
    }

    pos = { pos.first + _vel.first * delta, pos.second + _vel.second * delta };

    // Animate
    _sprs.at(_curSpr).scale = { 0.5, 0.5 };
    _sprs.at(_curSpr).flip = std::make_pair(!_facingRight, false);
    if (!_grounded && _sprName != "idle") {
        _curSpr = "idle";
    } else if (_grounded) {
        if (std::abs(_vel.first) < moveSpd * 0.8 && _sprName != "idle") {
            _curSpr = "idle";
        } else if (std::abs(_vel.first) >= moveSpd * 0.8 && _sprName != "walk") {
            _curSpr = "walk";
            _facingRight = _vel.first > 0.1;
        }
    }

    // Update vel at the end, so collisions can affect it!
    const auto hor = ((_right ? 1.0 : 0.0) + (_left ? -1.0 : 0.0));
    _vel.first = math::lerp(_vel.first, hor * moveSpd, acc * delta);

    const auto groundCheck = CollisionShape {
        .shapeType = collider.shapeType,
        .center = std::make_pair(
            static_cast<int>(pos.first) + collider.center.first,
            static_cast<int>(pos.second) + collider.center.second
                + static_cast<int>(_vel.second * delta) + 1
        ), .width = collider.width,
        .height = collider.height
    };
    _grounded = false;
    for (size_t i = 0; i < others.size(); i++) {
        if (others[i]->tag() != "Brick") {
            continue;
        }
        if (pos.second > others[i]->pos.second - collider.height * 0.8) {
            continue;
        }
        const auto otherCollider = CollisionShape {
            .shapeType = others[i]->collider.shapeType,
            .center = std::make_pair(
                others[i]->collider.center.first
                    + static_cast<int>(others[i]->pos.first),
                others[i]->collider.center.second
                    + static_cast<int>(others[i]->pos.second)
            ), .width = others[i]->collider.width,
            .height = others[i]->collider.height
        };
        if (groundCheck.collidesWith(otherCollider)) {
            _grounded = true;
            _vel.second = 0.0;
            switch (otherCollider.shapeType) {
                case CollShapeType::Circle:
                    pos = {
                        pos.first,
                        otherCollider.center.second - otherCollider.radius
                            - 2 * otherCollider.radius - 1
                    };
                    break;
                case CollShapeType::Rect:
                    pos = {
                        pos.first,
                        otherCollider.center.second - otherCollider.height / 2
                            - otherCollider.height - 1
                    };
                    break;
            }
        }
    }
    if (_grounded || _vel.second > 0.1) {
        _extraGrav = false;
    }
    if (_extraGrav) {
        _vel.second += extraGrav * delta;
    } else {
        _vel.second += gravity * delta;
    }
}

void Player::handleSdlEvent(const SDL_Event &ev) {
    switch (ev.type) {
        case SDL_KEYDOWN:
            switch (ev.key.keysym.scancode) {
                case SDL_SCANCODE_RIGHT:
                    _right = true;
                    break;
                case SDL_SCANCODE_LEFT:
                    _left = true;
                    break;
                case SDL_SCANCODE_UP:
                    if (_grounded) {
                        pos = { pos.first, pos.second - 1 };
                        _vel.second = -jumpSpd;
                        globals::sounds.at("jump")->play();
                    }
                default:
                    break;
            }
            break;
        case SDL_KEYUP:
            switch (ev.key.keysym.scancode) {
                case SDL_SCANCODE_RIGHT:
                    _right = false;
                    break;
                case SDL_SCANCODE_LEFT:
                    _left = false;
                    break;
                case SDL_SCANCODE_UP:
                    if (_vel.second < -0.1) {
                        _extraGrav = true;
                    }
                default:
                    break;
            }
            break;
    }
}

void Player::onCollision(const std::shared_ptr<GameObject> &other) {
    if (other->tag() == "Brick") {
        if ((other->pos.first <= pos.first && _vel.first < -0.1)
                || (other->pos.first >= pos.first && _vel.first > 0.1)) {
            pos.first -= _vel.first > 0.1 ? 1 : -1;
            _vel.first = 0;
        }
    }
}

void Player::reset(void) {
    _vel = std::make_pair(0.0, 0.0);
    _right = false;
    _left = false;
    _curSpr = "idle";
    _grounded = false;
    _extraGrav = false;
    _facingRight = true;
    pos = _startPos;
}
