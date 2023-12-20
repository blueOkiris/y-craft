// Brick does basically nothing

#include <utility>
#include <string>
#include <Sprite.hpp>
#include <GameObject.hpp>
#include <Brick.hpp>

Brick::Brick(
    const std::string &name,
    const std::pair<double, double> &defPos,
    const Sprite &defSpr,
    const CollisionShape &collShape):
        GameObject(name, defPos, defSpr, collShape) {}

std::string Brick::tag(void) const {
    return "Brick";
}

void Brick::update(
    std::map<std::string, Audio *> &sounds, const double delta,
    const std::vector<GameObject *> &others
) {}
void Brick::handleSdlEvent(std::map<std::string, Audio *> &sounds, const SDL_Event &ev) {}
void Brick::onCollision(const GameObject *other) {}
void Brick::reset(void) {}

