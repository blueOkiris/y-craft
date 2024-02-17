// Implementation of teleporting mouse

#include <utility>
#include <vector>
#include <string>
#include <memory>
#include <random>
#include <cmath>
#include <iostream>
#include <engine/GameObject.hpp>
#include <engine/Sprite.hpp>
#include <engine/globals.hpp>
#include <gameobjs/Mouse.hpp>

std::random_device g_rd;
std::default_random_engine g_re(g_rd());
std::uniform_real_distribution<double> g_randXDist(32.0, 640.0 - 96.0);
std::uniform_real_distribution<double> g_randYDist(32.0, 352.0 - 96.0);

std::pair<double, double> randomMousePos(void) {
    return std::make_pair(
        std::floor(g_randXDist(g_re) / 32.0) * 32.0 + 16.0,
        std::floor(g_randYDist(g_re) / 32.0) * 32.0 + 16.0
    );
}

Mouse::Mouse(void):
    GameObject(
        "mouse", randomMousePos(),
        "mouse", std::map<std::string, Sprite>({
            {
                "mouse",
                Sprite(std::vector<Frame>({
                    Frame("mouse.png", { 0, 0, 32, 32 }, { 32, 32 })
                }), 0.0, { 16.0, 16.0 })
            }
        }), {
            .shapeType = CollShapeType::Circle,
            .center = { 0, 0 },
            .radius = 15
        }
    ) {}

std::string Mouse::tag(void) const {
    return "Mouse";
}

void Mouse::update(
        const double delta, const std::vector<std::shared_ptr<GameObject>> &others) {}
void Mouse::handleSdlEvent(const SDL_Event &ev) {}

void Mouse::onCollision(const std::shared_ptr<GameObject> &other) {
    if (other->id == "snakeHead") {
        reset();
    }
}

void Mouse::reset(void) {
    pos = randomMousePos();
}

