// The moving part and start of snake chain

#pragma once

#include <utility>
#include <vector>
#include <string>
#include <memory>
#include <engine/GameObject.hpp>

class SnakeHead: public GameObject {
    public:
        SnakeHead(void);
        std::string tag(void) const override;
        void update(
            const double delta, const std::vector<std::shared_ptr<GameObject>> &others
        ) override;
        void handleSdlEvent(const SDL_Event &ev) override;
        void onCollision(const std::shared_ptr<GameObject> &other) override;
        void reset(void) override;
    
        const double baseMoveSpd = 64.0;
        const double moveSpdInc = 8.0;

    private:
        int _dir;
        double _moveSpd;
        std::pair<double, double> _interPos;
        bool _canChangeDir;
        std::pair<double, double> _startPos;
        bool _addBodySeg;
};

class SnakeBody: public GameObject {
    public:
        SnakeBody(const int ind, const std::pair<double, double> &defPos);
        std::string tag(void) const override;
        void update(
            const double delta, const std::vector<std::shared_ptr<GameObject>> &others
        ) override;
        void handleSdlEvent(const SDL_Event &ev) override;
        void onCollision(const std::shared_ptr<GameObject> &other) override;
        void reset(void) override;

        int dir;
        int lastDir;
        int index;
    
    private:
        std::pair<double, double> _lastPos;
        std::pair<double, double> _startPos;
};

class SnakeTail: public GameObject {
    public:
        SnakeTail(void);
        std::string tag(void) const override;
        void update(
            const double delta, const std::vector<std::shared_ptr<GameObject>> &others
        ) override;
        void handleSdlEvent(const SDL_Event &ev) override;
        void onCollision(const std::shared_ptr<GameObject> &other) override;
        void reset(void) override;

        int dir;

    private:
        std::pair<double, double> _startPos;
};

