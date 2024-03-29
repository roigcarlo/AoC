#include <cstdio>
#include <string>
#include <numeric>
#include <algorithm>
#include <execution>

#include <range/v3/all.hpp>

#include "elf_io.h"
#include "elf_perf.h"
#include "elf_report.h"

using namespace ranges;

std::size_t to_key(const std::string_view & sv) {
    return (sv[0] - 'A') * 26 * 26 + (sv[1] - 'A') * 26 + (sv[2] - 'A');
}

std::size_t part1(const std::vector<std::string> & fv) {
    std::array<std::size_t, 17576> lmap{};
    std::array<std::size_t, 17576> rmap{};
    
    auto map = fv | views::drop(1) | views::transform([](const std::string & s) { 
        return std::make_tuple(
            std::string_view(&*(s.begin()) +  0, 3),
            std::string_view(&*(s.begin()) +  7, 3),
            std::string_view(&*(s.begin()) + 12, 3)
        ); 
    });

    for_each(map, [&lmap, &rmap](const auto & v) {
        lmap[to_key(std::get<0>(v))] = to_key(std::get<1>(v));
        rmap[to_key(std::get<0>(v))] = to_key(std::get<2>(v));
    });

    auto route = std::string_view(fv[0]);
    auto route_size = route.size();

    std::size_t steps = 0;
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

    auto map = fv | views::drop(1) | views::transform([](const std::string & s) { 
        return std::make_tuple(
            std::string_view(&*(s.begin()) +  0, 3),
            std::string_view(&*(s.begin()) +  7, 3),
            std::string_view(&*(s.begin()) + 12, 3)
        ); 
    });

    for_each(map, [&lmap, &rmap, &emap](const auto & v) {
        lmap[to_key(std::get<0>(v))] = to_key(std::get<1>(v));
        rmap[to_key(std::get<0>(v))] = to_key(std::get<2>(v));
        emap[to_key(std::get<0>(v))] = std::get<0>(v)[2] == 'Z';
    });

    auto route = std::string_view(fv[0]);
    auto route_size = route.size();
    auto calc_cycle = [&](auto cur) { 
        return 2 + *(views::iota(0) | views::take_while([&](const auto & i) { 
            return !emap[cur = *(route.cbegin() + (i % route_size)) == 'L' ? lmap[cur] : rmap[cur]];
        }) | views::reverse).begin();
    };

    auto cycles = map | views::filter([&](const auto & v) { return std::get<0>(v)[2] == 'A'; })     
                      | views::transform([&](const auto & v){ return calc_cycle(to_key(std::get<0>(v))); });

    return std::accumulate(cycles.begin(), cycles.end(), static_cast<std::size_t>(1), std::lcm<std::size_t, std::size_t>);
}

int main (int argc, char** argv) {
    auto [inpt, io_time] = Elfperf::execute([&argv](){ return Elfio::read(argv[1], Elfio::Mode::Snow);});
    auto [res1, p1_time] = Elfperf::execute([&inpt](){ return part1(inpt); }, 1000);
    auto [res2, p2_time] = Elfperf::execute([&inpt](){ return part2(inpt); }, 1000);

    Elfreport::report(res1, res2, io_time, p1_time, p2_time);

    return 0;
}