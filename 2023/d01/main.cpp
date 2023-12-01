#include <cstdio>
#include <fstream>
#include <iostream>
#include <chrono>
#include <algorithm>

constexpr std::array<std::string, 9> f_numbers = {"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};
constexpr std::array<std::string, 9> r_numbers = {"eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"};

std::fstream read(char * filename) {
    return std::fstream(filename);
}

auto find_str_num(const char * buffer,const std::array<std::string, 9> & check_list) {
    std::string chunk(buffer);
    std::array<uint, 9> found_num;

    std::ranges::generate(found_num.begin(), found_num.end(), [&,n{0}]() mutable {return uint(chunk.find(check_list[n++]));});

    return std::make_tuple(std::min_element(found_num.begin(), found_num.end()) - found_num.begin(), std::move(found_num));
}

std::size_t part1(std::fstream& file) {
    std::string line;
    
    file.clear();
    file.seekg(0);

    std::size_t f, l, r1 = 0, r2 = 0;

    while(file >> line) {
        auto es = 'z' + line + 'z';
        sscanf(es.c_str(), "%*[a-z]%1zd", &f);
        std::reverse(es.begin(), es.end());
        sscanf(es.c_str(), "%*[a-z]%1zd", &l);

        r1 += f;
        r2 += l;
    }

    return r1 * 10 + r2;
}

std::size_t part2(std::fstream& file) {
    std::string line;
    
    file.clear();
    file.seekg(0);

    std::size_t f, l, r1 = 0, r2 = 0;

    std::size_t found_pos;
    std::array<uint, 9> found_num;

    while(file >> line) {
        auto es = 'z' + line + 'z';
        char * fckelfs = new char[es.length() + 1];

        sscanf(es.c_str(), "%[a-z]%1zd", fckelfs, &f);
        std::tie(found_pos, found_num) = find_str_num(fckelfs, f_numbers);

        if (found_num[found_pos] != uint(-1)) { f = found_pos+1; }

        std::reverse(es.begin(), es.end());
        sscanf(es.c_str(), "%[a-z]%1zd", fckelfs, &l);
        std::tie(found_pos, found_num) = find_str_num(fckelfs, r_numbers);

        if (found_num[found_pos] != uint(-1)) { l = found_pos+1; }

        r1 += f;
        r2 += l;
    }

    return r1 * 10 + r2;
}

int main (int argc, char** argv) {
    using namespace std::chrono;

    auto t0 = high_resolution_clock::now();
    auto input = read(argv[1]);
    auto t1 = high_resolution_clock::now();
    auto r1 = part1(input);
    // Repeat 1000 because is too quick
    for(int i = 0; i < 999; i++) part1(input);
    auto t2 = high_resolution_clock::now();
    auto r2 = part2(input);
    // Repeat 1000 because is too quick
    for(int i = 0; i < 999; i++) part2(input);
    auto t3 = high_resolution_clock::now();

    std::cout << "I/O   : " << duration_cast<microseconds>(t1 - t0) << std::endl;
    std::cout << "Part 1: " << duration_cast<microseconds>(t2 - t1) / 1000 << " " << r1 << std::endl;
    std::cout << "Part 2: " << duration_cast<microseconds>(t3 - t2) / 1000 << " " << r2 << std::endl;

    return 0;
}