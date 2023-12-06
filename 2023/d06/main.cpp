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

long solve(long t, long d) {
    auto eqa = ((-t + std::sqrt(t*t - 4*(d+1)))/-2);
    auto eqb = ((-t - std::sqrt(t*t - 4*(d+1)))/-2);

    auto eqmin = std::ceil( std::min(eqa, eqb));
    auto eqmax = std::floor(std::max(eqa, eqb));

    return eqmax - eqmin + 1;
}

std::size_t part1(const std::vector<std::string> & fv) {
    std::string times = fv[0];
    std::string dists = fv[1];
    
    auto as_list = [](std::string & s) {
        return s | std::views::split(" "sv) 
                 | std::ranges::views::drop(1)
                 | std::views::transform([](const auto & v) { return std::string(std::string_view(v.begin(), v.end()));})
                 | std::views::filter([](const auto & sv){return sv != ""sv;})
                 | std::views::transform([](const auto & v) { return std::stoi(v);})
                 ;
    };

    auto race_view = std::ranges::views::zip(as_list(times), as_list(dists));

    return std::transform_reduce(
        race_view.begin(), race_view.end(),
        1, std::multiplies<std::size_t>{},
        [](const auto & v) {
            return solve(v.first,v.second);
        }
    );
}

std::size_t part2(const std::vector<std::string> & fv) {
    std::string times = fv[0];
    std::string dists = fv[1];
    
    auto as_nmbr = [](std::string & s) {
        s.erase(std::remove_if(s.begin(), s.end(), [](unsigned char x) { return std::isspace(x); }), s.end());
        return s | std::views::split(":"sv)
                 | std::views::transform([](const auto & v) { return std::stol(std::string(std::string_view(v)));})
                 | std::ranges::views::drop(1)
                 ;
    };

    const auto & [t, d] = std::ranges::views::zip(as_nmbr(times), as_nmbr(dists)).front();

    return solve(t,d);
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