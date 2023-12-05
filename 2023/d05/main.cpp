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

auto build_farm(const std::vector<std::string> & fv) {
    auto reversed = fv | std::ranges::views::drop(2);

    std::size_t level = 0;
    std::vector<std::vector<std::tuple<long, long, long>>> farm(7);

    long dst, src, lng;
    for (auto & x : reversed) {
        auto xr = x | std::views::reverse;

        if (xr[0] == ':') {
            level++;
        } else {
            std::sscanf(x.c_str(), "%ld %ld %ld", &dst, &src, &lng);
            farm[level].emplace_back(std::move(std::make_tuple(dst, src, lng)));
        }
    }

    for (auto & level : farm) {
        std::sort(level.begin(), level.end(), [](const auto & a, const auto & b) {
            return std::get<1>(a) < std::get<1>(b);
        });
    }

    return std::move(farm);
}

template<class TFarm>
long traverse_farm(int level, long v, const TFarm & farm) {

    if (level == 7)
        return v;

    std::vector<long> this_level(farm[level].size(), std::numeric_limits<long>::max());

    auto vn = v;
    for (int i = 0; i < farm[level].size(); i++) {
        if (level < 7 && v >= std::get<1>(farm[level][i]) && v <= std::get<1>(farm[level][i]) + std::get<2>(farm[level][i]))
            vn = v - std::get<1>(farm[level][i]) + std::get<0>(farm[level][i]);
    }

    auto r = traverse_farm(level + 1, vn, farm);

    return r;
}

template<class TFarm>
long traverse_farm_range(int level, std::pair<long, long> v, const TFarm & farm) {
    if (level == 7) return std::get<0>(v);

    std::vector<long> this_level(farm[level].size(), std::numeric_limits<long>::max());
    std::vector<std::pair<long,long>> value_ranges;

    auto itr_rn = 0;

    auto itr_dw = std::get<0>(v);
    auto itr_up = std::min(std::get<1>(v), std::get<1>(farm[level][0]));

    // Ranges below unchanged
    if (itr_dw < itr_up) {
        value_ranges.emplace_back(std::make_pair(itr_dw, itr_up));
    }

    // Ranges overlapping
    for (; itr_rn < farm[level].size() && itr_up < std::get<1>(v) ; itr_rn++) {
        itr_dw = std::max(std::get<0>(v), std::get<1>(farm[level][itr_rn]));
        itr_up = std::min(std::get<1>(v), std::get<1>(farm[level][itr_rn]) + std::get<2>(farm[level][itr_rn]));

        if (itr_dw < itr_up) {
            value_ranges.emplace_back(std::make_pair(
                itr_dw - std::get<1>(farm[level][itr_rn]) + std::get<0>(farm[level][itr_rn]), 
                itr_up - std::get<1>(farm[level][itr_rn]) + std::get<0>(farm[level][itr_rn])
            ));
        }
    }

    itr_dw = std::max(std::get<0>(v), std::get<1>(farm[level][itr_rn-1]) + std::get<2>(farm[level][itr_rn-1]));
    itr_up = std::get<1>(v);

    // Ranges above unchanged
    if (itr_dw < itr_up) {
        value_ranges.emplace_back(std::make_pair(itr_dw, itr_up));
    }

    std::vector<long> range_lows{};

    std::transform(
        value_ranges.begin(), value_ranges.end(),
        std::back_inserter(range_lows),
        [&level, &farm](const auto & v) { 
            return traverse_farm_range(level+1, v, farm);
        }
    );

    return value_ranges.size() ? *std::min_element(range_lows.begin(), range_lows.end()) : std::numeric_limits<long>::max();
}

std::size_t part1(const std::vector<std::string> & fv) {
    auto farm = build_farm(fv);
    auto seeds_view = fv[0] 
                    | std::views::split(" "sv) 
                    | std::ranges::views::drop(1) 
                    | std::views::transform([](const auto & v) { return std::stol(std::string(std::string_view(v.begin(), v.end()))); });

    std::vector<long> seeds;
    std::copy(
        seeds_view.begin(), seeds_view.end(),
        std::back_inserter(seeds)
    );

    std::for_each(
        seeds.begin(), seeds.end(),
        [&farm](auto & v) {
            v = traverse_farm(0, v, farm);
        }
    );

    return *std::min_element(seeds.begin(), seeds.end());
}

std::size_t part2(std::vector<std::string> fv) {
    auto farm = build_farm(fv);
    auto seeds_view = fv[0] 
                    | std::views::split(" "sv) 
                    | std::ranges::views::drop(1) 
                    | std::views::transform([](const auto & v) { return std::stol(std::string(std::string_view(v.begin(), v.end()))); });

    std::vector<long> seeds;
    std::copy(
        seeds_view.begin(), seeds_view.end(),
        std::back_inserter(seeds)
    );

    std::vector<std::pair<long, long>> seed_ranges;

    for (int i = 0; i < seeds.size() / 2; i++) {
        seed_ranges.emplace_back(std::make_pair(seeds[i * 2], seeds[i * 2] + seeds[i * 2 + 1]));
    }

    std::vector<long> range_lows;

    std::transform(
        seed_ranges.begin(), seed_ranges.end(),
        std::back_inserter(range_lows),
        [&farm](const auto & v) { return traverse_farm_range(0, v, farm); }
    );

    return *std::min_element(range_lows.begin(), range_lows.end());
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