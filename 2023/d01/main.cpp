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

    std::size_t n, f, r = 0;

    while(file >> line) {
        n = sscanf(line.c_str(), "%*[a-z]%1zd", &f);
        if(!n) sscanf(line.c_str(), "%1zd", &f);
        
        r += f * 10;

        std::reverse(line.begin(), line.end());
        n = sscanf(line.c_str(), "%*[a-z]%1zd", &f);
        if(!n) sscanf(line.c_str(), "%1zd", &f);

        r += f;
    }

    return r;
}

std::size_t part2(std::fstream& file) {
    std::string line;
    
    file.clear();
    file.seekg(0);

    std::size_t n, r = 0;

    std::size_t found_pos;
    std::array<uint, 9> found_num;

    auto n_find = [&](std::string this_line, char * buffer, const std::array<std::string, 9> & check_list) -> std::size_t {
        std::size_t f;
        n = sscanf(this_line.c_str(), "%[a-z]%1zd", buffer, &f);
        if(!n) { 
            sscanf(this_line.c_str(), "%1zd", &f);
        } else {
            std::tie(found_pos, found_num) = find_str_num(buffer, check_list);
            if (found_num[found_pos] != uint(-1)) { f = found_pos+1; }
        }

        return f;
    };

    char * fckelfs = new char[2048];
    
    while(file >> line) {
        r += n_find(line, fckelfs, f_numbers) * 10;
        std::reverse(line.begin(), line.end());
        r += n_find(line, fckelfs, r_numbers);
    }

    return r;
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