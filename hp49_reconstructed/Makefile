# ============================================================================
# Makefile - HP 49G YUser.exe Reconstructed Build
# ============================================================================

CXX ?= i686-w64-mingw32-g++
CXXFLAGS = -O2 -Wall -Iinclude -DUNICODE -D_UNICODE
LDFLAGS = -mwindows -luser32 -lgdi32 -lshell32 -lcomctl32

SRC = src/YUser.cpp
OBJ = $(SRC:.cpp=.o)
TARGET = YUser.exe

all: $(TARGET)

$(TARGET): $(OBJ)
	$(CXX) $(OBJ) -o $(TARGET) $(LDFLAGS)

%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

clean:
	rm -f $(OBJ) $(TARGET)

.PHONY: all clean
