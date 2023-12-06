#include <cstdio>
#include <ranges>
#include <string>
#include <algorithm>
#include <execution>
#include <unordered_map>

#include "elf_io.h"
#include "elf_perf.h"

constexpr int RGB_R = 0;
constexpr int RGB_G = 1;
constexpr int RGB_B = 2;

constexpr std::array<int, 3> RGB_COUNT{12, 13, 14};

std::unordered_map<std::string, int> RGB_MAP{
    {"red", RGB_R}, {"green", RGB_G}, {"blue", RGB_B}
};

std::size_t part1(const std::vector<std::string> & fv) {
    std::size_t game, value;
    std::size_t box_r, box_g, box_b;
    std::string tmp;

    std::size_t r = std::transform_reduce(
        fv.begin(), fv.end(), 
        0, std::plus<std::size_t>{}, 
        [&](std::string line) -> std::size_t { 
            std::stringstream ss{line+"."};

            box_r = 0, box_g = 0, box_b = 0;

            ss >> tmp >> game >> tmp;

            while(ss >> value >> tmp) {
                auto del = tmp.back(); tmp.pop_back();

                switch(RGB_MAP[tmp]) {
                    case RGB_R: box_r += value; break;
                    case RGB_G: box_g += value; break;
                    case RGB_B: box_b += value; break;
                        break;
                }

                if (box_r > RGB_COUNT[RGB_R] || box_g > RGB_COUNT[RGB_G] || box_b > RGB_COUNT[RGB_B]) {
                    return 0;
                }

                if (del == ';' || del == '.') { 
                    box_r = 0, box_g = 0, box_b = 0;
                }       
            }

            return game;
        }
    );  

    return r;
}

std::size_t part2(const std::vector<std::string> & fv) {
    thread_local std::size_t game;
    thread_local int value, box_r, box_g, box_b;
    thread_local std::string tmp;

    std::size_t r = std::transform_reduce(
        fv.begin(), fv.end(), 
        0, std::plus<std::size_t>{}, 
        [&](std::string line) -> std::size_t { 
            std::stringstream ss{line+"."};

            box_r = std::numeric_limits<int>::min(), 
            box_g = std::numeric_limits<int>::min(), 
            box_b = std::numeric_limits<int>::min();
            
            ss >> tmp >> game >> tmp;

            while(ss >> value >> tmp) {
                auto del = tmp.back(); tmp.pop_back();

                switch(RGB_MAP[tmp]) {
                    case RGB_R: box_r = std::max(box_r, value); break;
                    case RGB_G: box_g = std::max(box_g, value); break;
                    case RGB_B: box_b = std::max(box_b, value); break;
                        break;
                }    
            }

            return box_r * box_g * box_b;
        }
    );  

    return r;
}

int main (int argc, char** argv) {
    auto [inpt, io_time] = Elfperf::execute([&argv](){ return Elfio::read(argv[1], Elfio::Mode::Snow);});
    auto [res1, p1_time] = Elfperf::execute([&inpt](){ return part1(inpt); }, 1000);
    auto [res2, p2_time] = Elfperf::execute([&inpt](){ return part2(inpt); }, 1000);

    std::cout << "I/O   : " << io_time << std::endl;
    std::cout << "Part 1: " << p1_time << " " << res1 << std::endl;
    std::cout << "Part 2: " << p2_time << " " << res2 << std::endl;

    return 0;
}