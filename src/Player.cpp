// Implementation of example GameObject behavior

#include <utility>
#include <string>
#include <vector>
#include <cmath>
#include <iostream>
#include <SDL2/SDL.h>
#include <math.hpp>
#include <Sprite.hpp>
#include <GameObject.hpp>
#include <Audio.hpp>
#include <Player.hpp>

Player::Player(
    const std::string &name,
    const std::pair<double, double> &defPos,
    const Sprite &defSpr,
    const CollisionShape &collShape,
    const Sprite &idleSpr,
    const Sprite &walkSpr):
        GameObject(name, defPos, defSpr, collShape),
        idle(idleSpr), walk(walkSpr),
        _vel({ 0.0, 0.0 }),
        _right(false), _left(false),
        _sprName("idle"),
        _grounded(false), _extraGrav(false),
        _facingRight(true),
        _startPos(defPos) {}

std::string Player::tag(void) const {
    return "Player";
}

void Player::update(
        std::map<std::string, Audio *> &sounds, const double delta,
        const std::vector<GameObject *> &others) {
    if (!Audio::isMusicPlaying()) {
        sounds.at("music")->play();
    }

    pos = std::make_pair(pos.first + _vel.first * delta, pos.second + _vel.second * delta);

    // Animate
    if (!_grounded && _sprName != "idle") {
        spr.setTo(idle);
        _sprName = "idle";
        // TODO: Have sprites have ids
    } else if (_grounded) {
        if (std::abs(_vel.first) < moveSpd * 0.8 && _sprName != "idle") {
            spr.setTo(idle);
            _sprName = "idle";
        } else if (std::abs(_vel.first) >= moveSpd * 0.8 && _sprName != "walk") {
            spr.setTo(walk);
            _sprName = "walk";
            _facingRight = _vel.first > 0.1;
        }
        spr.flip = std::make_pair(!_facingRight, false);
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
                    pos = std::make_pair(
                        pos.first,
                        otherCollider.center.second - otherCollider.radius
                            - 2 * otherCollider.radius - 1
                    );
                    break;
                case CollShapeType::Rect:
                    pos = std::make_pair(
                        pos.first,
                        otherCollider.center.second - otherCollider.height / 2
                            - otherCollider.height - 1
                    );
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

void Player::handleSdlEvent(std::map<std::string, Audio *> &sounds, const SDL_Event &ev) {
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
                        pos = std::make_pair(pos.first, pos.second - 1);
                        _vel.second = -jumpSpd;
                        sounds.at("jump")->play();
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

void Player::onCollision(const GameObject *other) {
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
    _sprName = "idle";
    spr.setTo(idle);
    _grounded = false;
    _extraGrav = false;
    _facingRight = true;
    pos = _startPos;
}
