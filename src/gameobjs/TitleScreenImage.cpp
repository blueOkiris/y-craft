// Implement TitleScreenImage object

#include <utility>
#include <string>
#include <map>
#include <memory>
#include <engine/globals.hpp>
#include <engine/Sprite.hpp>
#include <engine/GameObject.hpp>
#include <gameobjs/TitleScreenImage.hpp>

TitleScreenImage::TitleScreenImage(void):
    GameObject(
        "titleScreen", { 0.0, 0.0 },
        "title", std::map<std::string, Sprite>({
            {
                "title",
                Sprite(std::vector<Frame>({
                    Frame("title.png", { 0, 0, 640, 360 }, { 640, 360 })
                }), 0.0, { 0.0, 0.0 })
            }
        }), {
            .shapeType = CollShapeType::Rect,
            .center = { 320, 180 },
            .width = 640,
            .height = 360
        }
    ) {}

std::string TitleScreenImage::tag(void) const {
    return "TitleScreenImage";
}

void TitleScreenImage::handleSdlEvent(const SDL_Event &ev) {
    switch (ev.type) {
        case SDL_KEYUP:
            switch (ev.key.keysym.scancode) {
                case SDL_SCANCODE_RETURN:
                    globals::win.changeRoom("game");
                    break;
                default:
                    break;
            }
            break;
    }
}

void TitleScreenImage::update(
    const double delta, const std::vector<std::shared_ptr<GameObject>> &others) {}
void TitleScreenImage::onCollision(const std::shared_ptr<GameObject> &other) {}
void TitleScreenImage::reset(void) {}

