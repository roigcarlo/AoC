#include <queue>
#include <cstdio>
#include <string>
#include <ranges>
#include <numeric>
#include <algorithm>
#include <execution>
#include <set>

#include "elf_io.h"
#include "elf_perf.h"
#include "elf_report.h"

std::size_t part1(const std::vector<std::string> & fv) {
    std::set<int> non_void_x;
    std::set<int> non_void_y;

    std::vector<std::pair<int,int>> stars;
    std::vector<int> stars_x(fv.size(), 0);
    std::vector<int> stars_y(fv.size(), 0);

    for(std::size_t i = 0; i < fv.size(); i++) {
        for(std::size_t j = 0; j <fv[i].size(); j++) {
            if (fv[i][j] == '#') {
                stars.push_back({i,j});
                stars_x[i] = 1;
                stars_y[j] = 1;
            }
        }
    }
    
    for(std::size_t i = 0; i < fv.size() - 1; i++) {
        stars_x[i+1] += stars_x[i];
        stars_y[i+1] += stars_y[i];
    }

    std::size_t t_dist = 0;
    for(auto src = stars.begin(); src != stars.end(); src++) {
        auto dx_b = stars_x[src->first];
        auto dy_b = stars_y[src->second];
        for(auto dst = src; dst != stars.end(); dst++) {
            if (src != dst) {
                auto dx_e = stars_x[dst->first];
                auto dy_e = stars_y[dst->second];
                auto dx = std::abs(dx_e - dx_b);
                auto dy = std::abs(dy_e - dy_b);

                t_dist += std::abs(src->first - dst->first) + std::abs(src->second - dst->second)
                    + ((std::abs(src->first  - dst->first) - dx) 
                    + (std::abs(src->second  - dst->second) - dy)) * 1;
            }
        }
    }

    return t_dist;
}

unsigned long part2(const std::vector<std::string> & fv) {
    std::set<int> non_void_x;
    std::set<int> non_void_y;

    std::vector<std::pair<int,int>> stars;
    std::vector<int> stars_x(fv.size(), 0);
    std::vector<int> stars_y(fv.size(), 0);

    for(std::size_t i = 0; i < fv.size(); i++) {
        for(std::size_t j = 0; j <fv[i].size(); j++) {
            if (fv[i][j] == '#') {
                stars.push_back({i,j});
                stars_x[i] = 1;
                stars_y[j] = 1;
            }
        }
    }
    
    for(std::size_t i = 0; i < fv.size() - 1; i++) {
        stars_x[i+1] += stars_x[i];
        stars_y[i+1] += stars_y[i];
    }

    unsigned long t_dist = 0;
    for(auto src = stars.begin(); src != stars.end(); src++) {
        auto dx_b = stars_x[src->first];
        auto dy_b = stars_y[src->second];
        for(auto dst = src; dst != stars.end(); dst++) {
            if (src != dst) {
                auto dx_e = stars_x[dst->first];
                auto dy_e = stars_y[dst->second];
                auto dx = std::abs(dx_e - dx_b);
                auto dy = std::abs(dy_e - dy_b);

                t_dist += std::abs(src->first - dst->first) + std::abs(src->second - dst->second)
                    + ((std::abs(src->first  - dst->first) - dx) 
                    + (std::abs(src->second  - dst->second) - dy)) * (1000000 - 1);
            }
        }
    }

    return t_dist;
}

int main (int argc, char** argv) {
    auto [inpt, io_time] = Elfperf::execute([&argv](){ return Elfio::read(argv[1]);});
    auto [res1, p1_time] = Elfperf::execute([&inpt](){ return part1(inpt); }, 1000);
    auto [res2, p2_time] = Elfperf::execute([&inpt](){ return part2(inpt); }, 1000);

    Elfreport::report(res1, res2, io_time, p1_time, p2_time);

    return 0;
}