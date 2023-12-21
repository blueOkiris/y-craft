# Build file for play-builder engine

# Settings

## Compiler

CPPC :=				g++
ifndef DEBUG
CPPFLAGS :=			-Wall -Werror -std=c++17 -O2 \
					-Iinclude $(addprefix -I,$(SDL2_PATH))
else
CPPFLAGS :=			-Wall -Werror -std=c++17 -g \
					-Iinclude $(addprefix -I,$(SDL2_PATH))
endif
LD :=				g++
LDFLAGS :=			-lSDL2 -lSDL2_image -lSDL2_mixer -lSDL2_ttf

## Project

BIN :=				x-craft
SRC :=				$(wildcard src/*.cpp) \
					$(wildcard src/engine/*.cpp) \
					$(wildcard src/rooms/*.cpp) \
					$(wildcard src/gameobjs/*.cpp)
HFILES :=			$(wildcard include/*.hpp) \
					$(wildcard include/engine/*.hpp) \
					$(wildcard include/rooms/*.hpp) \
					$(wildcard include/gameobjs/*.hpp)
OBJS :=				$(subst .cpp,.o,$(subst src/,obj/,$(SRC)))

# Targets

## Helpers

.PHONY: all
ifndef DEBUG
all: $(BIN)
else
all: clean $(BIN)
endif

.PHONY: clean
clean:
	rm -rf obj
	rm -rf $(BIN)

obj/%.o: src/%.cpp $(HFILES)
	mkdir -p obj
	mkdir -p obj/engine
	mkdir -p obj/gameobjs
	mkdir -p obj/rooms
	$(CC) $(CPPFLAGS) -o $@ -c $<

## Main

$(BIN): $(OBJS)
	$(LD) -o $@ $^ $(LDFLAGS)

