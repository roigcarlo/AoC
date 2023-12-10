#include <cstdio>
#include <string>
#include <ranges>
#include <numeric>
#include <algorithm>
#include <execution>

#include "elf_io.h"
#include "elf_perf.h"
#include "elf_report.h"

std::size_t part1(const std::vector<std::string> & fv) {
    std::size_t sum = 0;
    for(auto & f: fv) {
        auto pira = f | std::views::split(' ') 
                      | std::views::transform([](const auto & v) { return std::stoi(std::string(std::string_view(v.begin(), v.end()))); })
                      | std::ranges::to<std::vector>();

        std::size_t pad = 1;
        while(std::any_of(pira.begin(), pira.end()-pad, [](const auto & v) { return v != 0; })) {
            for(std::size_t i = 0; i < pira.size() - pad; i++) {
                pira[i] = pira[i+1] - pira[i];
            }
            pad++;
        }
        while(pad != 0) {
            sum += pira[pira.size()-(pad)] ;
            pad--;
        }
    }

    return sum;
}

int part2(const std::vector<std::string> & fv) {
    int sum = 0;
    for(auto & f: fv) {
        auto pira = f | std::views::split(' ') 
                      | std::views::transform([](const auto & v) { return std::stoi(std::string(std::string_view(v.begin(), v.end()))); })
                      | std::ranges::to<std::vector>();

        std::size_t pad = 1;
        while(std::any_of(pira.begin(), pira.end()-pad, [](const auto & v) { return v != 0; })) {
            int tmp = pira[0];
            for(std::size_t i = 0; i < pira.size() - pad; i++) {
                pira[i] = pira[i+1] - pira[i];
            }
            pira[pira.size()-pad] = tmp;
            pad++;
        }
        int psum = 0;
        while(pad != 1) {
            psum = pira[pira.size()-(pad)+1] - psum;
            pad--;
        }
        sum += psum;
    }

    return sum;
}

int main (int argc, char** argv) {
    auto [inpt, io_time] = Elfperf::execute([&argv](){ return Elfio::read(argv[1], Elfio::Mode::Snow);});
    auto [res1, p1_time] = Elfperf::execute([&inpt](){ return part1(inpt); }, 1000);
    auto [res2, p2_time] = Elfperf::execute([&inpt](){ return part2(inpt); }, 1000);

    Elfreport::report(res1, res2, io_time, p1_time, p2_time);

    return 0;
}