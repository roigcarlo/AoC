#include <cstdio>
#include <chrono>
#include <ranges>
#include <string>
#include <fstream>
#include <iostream>
#include <algorithm>
#include <execution>
#include <string_view>

constexpr int RGB_R = 0;
constexpr int RGB_G = 1;
constexpr int RGB_B = 2;

constexpr std::array<int, 3> RGB_COUNT{12, 13, 14};

std::unordered_map<std::string, int> RGB_MAP{
    {"red", RGB_R}, {"green", RGB_G}, {"blue", RGB_B}
};

struct csv_whitespace : std::ctype<char>
{
    static const mask * make_table()
    {
        static std::vector<mask> v(classic_table(), classic_table() + table_size);
        v[' '] &= ~space;
        return &v[0];
    }
 
    csv_whitespace(std::size_t refs = 0) : ctype(make_table(), false, refs) {}
};

auto read(char * filename) {
    std::fstream file(filename);
    std::vector<std::string> lines;

    file.imbue(std::locale(file.getloc(), new csv_whitespace));

    std::copy(
        std::istream_iterator<std::string>(file),
        std::istream_iterator<std::string>(),
        std::back_inserter(lines)
    );

    return std::move(lines);
}

std::size_t part1(const std::vector<std::string> & filevector) {
    std::size_t game, value;
    std::size_t box_r, box_g, box_b;
    std::string tmp;

    std::size_t r = std::transform_reduce(
        std::execution::seq, 
        filevector.begin(), filevector.end(), 
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

std::size_t part2(const std::vector<std::string> & filevector) {
    thread_local std::size_t game;
    thread_local int value, box_r, box_g, box_b;
    thread_local std::string tmp;

    std::size_t r = std::transform_reduce(
        std::execution::seq, 
        filevector.begin(), filevector.end(), 
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
    using namespace std::chrono;

    auto io_s = high_resolution_clock::now();
    auto input = read(argv[1]);
    auto io_e = high_resolution_clock::now();

    auto p1_s = high_resolution_clock::now();
    auto r1 = part1(input);
    for(int i = 0; i < 999; i++) part1(input);
    auto p1_e = high_resolution_clock::now();

    auto p2_s = high_resolution_clock::now();
    auto r2 = part2(input);
    for(int i = 0; i < 999; i++) part2(input);
    auto p2_e = high_resolution_clock::now();

    std::cout << "I/O   : " << duration_cast<microseconds>(io_e - io_s) << std::endl;
    std::cout << "Part 1: " << duration_cast<microseconds>(p1_e - p1_s) / 1000 << " " << r1 << std::endl;
    std::cout << "Part 2: " << duration_cast<microseconds>(p2_e - p2_s) / 1000 << " " << r2 << std::endl;

    return 0;
}