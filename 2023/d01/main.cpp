#include <cstdio>
#include <chrono>
#include <fstream>
#include <iostream>
#include <algorithm>
#include <execution>

constexpr std::array<std::string, 9> f_numbers = {"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};
constexpr std::array<std::string, 9> r_numbers = {"eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"};

auto read(char * filename) {
    std::fstream file(filename);
    std::vector<std::string> lines;

    std::copy_if(
        std::istream_iterator<std::string>(file),
        std::istream_iterator<std::string>(),
        std::back_inserter(lines),
    [](const std::string& t) { return t != "-1"; });

    return std::move(lines);
}

auto find_str_num(const char * buffer,const std::array<std::string, 9> & check_list) {
    std::string chunk(buffer);
    std::array<uint, 9> found_num;

    std::ranges::generate(found_num.begin(), found_num.end(), [&,n{0}]() mutable {return uint(chunk.find(check_list[n++]));});

    return std::make_tuple(std::min_element(found_num.begin(), found_num.end()) - found_num.begin(), std::move(found_num));
}

std::size_t part1(const std::vector<std::string> & filevector) {    
    auto n_find = [&](const std::string & line) -> std::size_t {
        std::size_t f;
        if(!sscanf(line.c_str(), "%*[a-z]%1zd", &f)) sscanf(line.c_str(), "%1zd", &f);
        return f;
    };

    std::size_t r = std::transform_reduce(
        std::execution::par_unseq, 
        filevector.begin(), filevector.end(), 
        0, std::plus<std::size_t>{}, 
        [&](std::string line) -> std::size_t { 
            std::size_t t = 0;
            t += n_find(line) * 10;
            std::reverse(line.begin(), line.end());
            t += n_find(line);
            return t; 
        }
    );

    return r;
}

std::size_t part2(const std::vector<std::string> & filevector) {
    thread_local std::size_t f;
    thread_local std::size_t found_pos;
    thread_local std::array<uint, 9> found_num;
    thread_local char fckelfs[2048];

    auto n_find = [&](const std::string & line, char * buffer, const std::array<std::string, 9> & check_list) -> std::size_t {        
        if(!sscanf(line.c_str(), "%[a-z]%1zd", buffer, &f)) { 
            sscanf(line.c_str(), "%1zd", &f);
        } else {
            std::tie(found_pos, found_num) = find_str_num(buffer, check_list);
            if (found_num[found_pos] != uint(-1)) { f = found_pos+1; }
        }

        return f;
    };

    std::size_t r = std::transform_reduce(
        std::execution::par_unseq, 
        filevector.begin(), filevector.end(), 
        0, std::plus<std::size_t>{}, 
        [&](std::string line) -> std::size_t { 
            std::size_t t = 0;
            t += n_find(line, fckelfs, f_numbers) * 10;
            std::reverse(line.begin(), line.end());
            t += n_find(line, fckelfs, r_numbers);
            return t; 
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