export CC=clang
export CXX=clang++

cmake -Wno-dev -B"build" \
-DCMAKE_BUILD_TYPE=Release \
.

cmake --build "build"