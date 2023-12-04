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

std::size_t process_line(const std::string & line) {
    auto p_line = line 
                | std::views::split(" "sv) 
                | std::views::drop(2)
                | std::views::filter([](const auto & v){
                    return std::string_view(v.begin(), v.end()) != ""sv;
                });

    auto l_mine = std::ranges::find_if(p_line, [](const auto & v){
        return std::string_view(v.begin(), v.end()) == "|";
    });

    auto winners_range = std::ranges::subrange(std::ranges::begin(p_line), l_mine);
    auto players_range = std::ranges::subrange(l_mine, std::ranges::end(p_line))
                       | std::views::drop(1);
    
    std::unordered_set<std::string_view> winners;

    for(auto winner: winners_range) {
        winners.insert(std::string_view(winner.begin(), winner.end()));
    }

    return std::transform_reduce(
        players_range.begin(), players_range.end(),
        0, std::plus<>(),
        [&winners](const auto & c) { 
            return winners.contains(std::string_view(c.begin(), c.end())); 
        }
    );
}

std::size_t part1(const std::vector<std::string> & fv) {
    return std::transform_reduce(
        fv.begin(), fv.end(),
        0, std::plus<int>(),
        [](const std::string & line) -> int {
            auto matches = process_line(line);
            return matches ? std::pow(2, matches-1) : 0;
        }
    );
}

std::size_t part2(const std::vector<std::string> & fv) {
    std::vector<std::size_t> apps(fv.size(), 0);

    for(std::size_t i = 0; i < fv.size(); i++) {
        apps[i] += 1;

        if(auto matches = process_line(fv[i])) {
            std::ranges::for_each(
                apps.begin() + i + 1, apps.begin() + i + matches + 1,
                [&i, &apps](std::size_t & v) { v += apps[i]; }
            );
        }
    }

    return std::transform_reduce(apps.begin(), apps.end(), 0, std::plus<>(), [](const auto & v) { return v; });
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