#include <iostream>
#include <algorithm>
#include <execution>

#include "elf_io.h"
#include "elf_perf.h"

constexpr std::array<std::string, 9> f_numbers = {"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};
constexpr std::array<std::string, 9> r_numbers = {"eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"};

auto find_str_num(const char * buffer, const std::array<std::string, 9> & check_list) {
    std::string chunk(buffer);
    std::array<uint, 9> found_num;

    std::ranges::generate(found_num.begin(), found_num.end(), [&,n{0}]() mutable {return uint(chunk.find(check_list[n++]));});

    return std::make_tuple(std::min_element(found_num.begin(), found_num.end()) - found_num.begin(), std::move(found_num));
}

std::size_t part1(const std::vector<std::string> & fv) {
    thread_local std::size_t f;

    auto n_find = [&](const std::string & line) -> std::size_t {
        if(!sscanf(line.c_str(), "%*[a-z]%1zd", &f)) sscanf(line.c_str(), "%1zd", &f);
        return f;
    };

    std::size_t r = std::transform_reduce(
        std::execution::par_unseq, 
        fv.begin(), fv.end(), 
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

std::size_t part2(const std::vector<std::string> & fv) {
    thread_local std::size_t f;
    thread_local std::size_t found_pos;
    thread_local std::array<uint, 9> found_num;
    thread_local char fckelfs[2048];

    auto n_find = [&](const std::string & line, char * buffer, const std::array<std::string, 9> & check_list) -> std::size_t {        
        if(!sscanf(line.c_str(), "%[a-z]%1zd", buffer, &f)) { 
            sscanf(line.c_str(), "%1zd", &f);
        } else {
            std::tie(found_pos, found_num) = find_str_num(buffer, check_list);
            f = found_num[found_pos] != uint(-1) ? found_pos+1 : f;
        }

        return f;
    };

    std::size_t r = std::transform_reduce(
        std::execution::par_unseq, 
        fv.begin(), fv.end(), 
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
    auto [inpt, io_time] = Elfperf::execute([&argv](){ return Elfio::read(argv[1]);});
    auto [res1, p1_time] = Elfperf::execute([&inpt](){ return part1(inpt); }, 1000);
    auto [res2, p2_time] = Elfperf::execute([&inpt](){ return part2(inpt); }, 1000);

    std::cout << "I/O   : " << io_time << std::endl;
    std::cout << "Part 1: " << p1_time << " " << res1 << std::endl;
    std::cout << "Part 2: " << p2_time << " " << res2 << std::endl;

    return 0;
}