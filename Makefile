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
LDFLAGS :=			-lSDL2 -lSDL2_image -lSDL2_mixer

## Project

BIN :=				x-craft
SRC :=				$(wildcard src/*.cpp)
HFILES :=			$(wildcard include/*.hpp)
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
	$(CC) $(CPPFLAGS) -o $@ -c $<

## Main

$(BIN): $(OBJS)
	$(LD) -o $@ $^ $(LDFLAGS)

