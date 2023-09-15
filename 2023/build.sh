export CC=clang
export CXX=clang++

cmake  -G Ninja -Wno-dev -B"build" \
-DCMAKE_BUILD_TYPE=Release \
.

cmake --build "build"