// Brick does basically nothing

#include <utility>
#include <string>
#include <map>
#include <engine/Sprite.hpp>
#include <engine/GameObject.hpp>
#include <gameobjs/Brick.hpp>

Brick::Brick(
    const std::string &name,
    const std::pair<double, double> &defPos):
        GameObject(
            name, defPos,
            "brick",
            std::map<std::string, Sprite>({
                {
                    "brick",
                    Sprite(std::vector<Frame>({
                        Frame("brick.png", { 0, 0, 32, 32 }, { 64, 64 })
                    }), 0.0, { 16, 16 })
                }
            }), {
                .shapeType = CollShapeType::Rect,
                .center = { 32, 32 },
                .width = 64,
                .height = 64
            }
        ) {}

std::string Brick::tag(void) const {
    return "Brick";
}

void Brick::update(const double delta, const std::vector<std::shared_ptr<GameObject>> &others) {}
void Brick::handleSdlEvent(const SDL_Event &ev) {}
void Brick::onCollision(const std::shared_ptr<GameObject> &other) {}
void Brick::reset(void) {}

