#include <cmath>
#include <cstdio>
#include <vector>
#include <string>
#include <ranges>
#include <algorithm>
#include <execution>
#include <string_view>
#include <unordered_set>

#include "elf_io.h"
#include "elf_perf.h"

using std::operator""sv;

int process_line(const std::string & line) {
    auto p_line = line 
                | std::views::split(" "sv) 
                | std::views::transform([](const auto & v) { return std::string_view(v.begin(), v.end()); })
                | std::views::filter([](const auto & sv){return sv != ""sv;}) 
                | std::views::drop(2)
                ;

    auto l_mine = std::ranges::find_if(p_line, [](const auto & v){
        return v == "|"sv;
    });

    auto winners_range = std::ranges::subrange(std::ranges::begin(p_line), l_mine);
    auto players_range = std::ranges::subrange(std::next(l_mine), std::ranges::end(p_line));
    
    std::unordered_set<std::string_view> winners;

    for(auto winner: winners_range) {
        winners.insert(winner);
    }

    return std::transform_reduce(
        players_range.begin(), players_range.end(),
        0, std::plus<>(),
        [&winners](const auto & c) { 
            return winners.contains(c); 
        }
    );
}

std::size_t part1(const std::vector<std::string> & fv) {
    return std::transform_reduce(
        std::execution::par_unseq,
        fv.begin(), fv.end(),
        0, std::plus<int>(),
        [](const std::string & line) -> int {
            auto x = process_line(line);
            return std::pow(2, x-1);
        }
    );
}

std::size_t part2(std::vector<std::string> fv) {
    std::vector<std::size_t> apps(fv.size(), 1);

    std::for_each(
        std::execution::par_unseq,
        fv.begin(), fv.end(),
        [](std::string & line) {
            line = std::to_string(process_line(line));
        }
    );

    for(std::size_t i = 0; i < fv.size(); i++) {
        std::ranges::for_each(
            apps.begin() + i + 1, apps.begin() + i + std::stoi(fv[i]) + 1,
            [&i, &apps](std::size_t & v) { v += apps[i]; }
        );
    }

    return std::reduce(apps.begin(), apps.end());
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