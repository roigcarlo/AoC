#include <cstdio>
#include <string>
#include <ranges>
#include <numeric>
#include <algorithm>
#include <execution>
#include <unordered_map>
#include <unordered_set>

#include "elf_io.h"
#include "elf_perf.h"
#include "elf_report.h"

std::size_t to_key(const std::string_view & sv) {
    return (sv[0] - 'A') * 26 * 26 + (sv[1] - 'A') * 26 + (sv[2] - 'A');
}

std::size_t part1(const std::vector<std::string> & fv) {
    std::array<std::size_t, 17576> lmap{};
    std::array<std::size_t, 17576> rmap{};
    
    auto route = std::string_view(fv[0]);
    auto map = fv | std::views::drop(1) | std::views::transform([](const std::string & s) { 
        return std::make_tuple(
            std::string_view(s.begin() +  0, s.begin() +  3),
            std::string_view(s.begin() +  7, s.begin() + 10),
            std::string_view(s.begin() + 12, s.begin() + 15)
        ); 
    });

    std::ranges::for_each(map, [&lmap, &rmap](const auto & v) {
        lmap[to_key(std::get<0>(v))] = to_key(std::get<1>(v));
        rmap[to_key(std::get<0>(v))] = to_key(std::get<2>(v));
    });

    auto place = route.cbegin();

    int steps = 0;
    int route_size = static_cast<int>(route.size());

    std::size_t cur = to_key("AAA");
    std::size_t end = to_key("ZZZ");

    while (cur != end) {
        cur = *(route.cbegin() + (steps++ % route_size)) == 'L' ? lmap[cur] : rmap[cur];
    }

    return steps;
}

std::size_t part2(const std::vector<std::string> & fv) {
    std::array<std::size_t, 17576> lmap{};
    std::array<std::size_t, 17576> rmap{};
    std::array<std::size_t, 17576> emap{};

    auto map = fv | std::views::drop(1) | std::views::transform([](const std::string & s) { 
        return std::make_tuple(
            std::string_view(s.begin() +  0, s.begin() +  3),
            std::string_view(s.begin() +  7, s.begin() + 10),
            std::string_view(s.begin() + 12, s.begin() + 15)
        ); 
    });

    std::ranges::for_each(map, [&lmap, &rmap, &emap](const auto & v) {
        lmap[to_key(std::get<0>(v))] = to_key(std::get<1>(v));
        rmap[to_key(std::get<0>(v))] = to_key(std::get<2>(v));
        emap[to_key(std::get<0>(v))] = std::get<0>(v)[2] == 'Z';
    });

    auto route = std::string_view(fv[0]);
    auto calc_cycle = [&](const auto & v) { 
        auto cur = to_key(std::get<0>(v));
        auto place = route.cbegin();

        std::size_t steps = 0;
        std::unordered_set<std::size_t> chk{};

        int route_size = static_cast<int>(route.size());
        
        while (chk.size() != 2) {
            cur = *(route.cbegin() + (steps++ % route_size)) == 'L' ? lmap[cur] : rmap[cur];
            if (emap[cur] && chk.size() < 2) chk.insert(steps);
        }

        return *std::next(chk.begin(), 0) - *std::next(chk.begin(), 1);
    };

    auto cycles = map | std::views::filter([&](const auto & v) { return std::get<0>(v)[2] == 'A'; })     
                      | std::views::transform(calc_cycle);

    std::size_t total_lcm = 1;

    for(auto partial_lcm : cycles) {
        total_lcm = std::lcm(total_lcm, partial_lcm);
    }

    return total_lcm;
}

int main (int argc, char** argv) {
    auto [inpt, io_time] = Elfperf::execute([&argv](){ return Elfio::read(argv[1], Elfio::Mode::Snow);});
    auto [res1, p1_time] = Elfperf::execute([&inpt](){ return part1(inpt); }, 1000);
    auto [res2, p2_time] = Elfperf::execute([&inpt](){ return part2(inpt); }, 1000);

    Elfreport::report(res1, res2, io_time, p1_time, p2_time);

    return 0;
}