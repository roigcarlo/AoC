#include <cmath>
#include <cstdio>
#include <vector>
#include <string>
#include <algorithm>
#include <execution>
#include <string_view>
#include <unordered_set>

#include <range/v3/all.hpp>

#include "elf_io.h"
#include "elf_perf.h"
#include "elf_report.h"

using namespace ranges;
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
        return s | views::split(" "sv) 
                 | views::drop(1)
                 | views::transform([](const auto & v) { return v | to<std::string>;})
                 | views::filter([](const auto & sv){return sv != ""sv;})
                 | views::transform([](const auto & v) { return std::stoi(v);})
                 ;
    };

    auto race_view = views::zip(as_list(times), as_list(dists));

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
        return s | views::split(":"sv)
                 | views::transform([](const auto & v) { return std::stol(v | to<std::string>());})
                 | views::drop(1)
                 ;
    };

    const auto & [t, d] = views::zip(as_nmbr(times), as_nmbr(dists)).front();

    return solve(t,d);
}

int main (int argc, char** argv) {
    auto [inpt, io_time] = Elfperf::execute([&argv](){ return Elfio::read(argv[1], Elfio::Mode::Snow);});
    auto [res1, p1_time] = Elfperf::execute([&inpt](){ return part1(inpt); }, 1000);
    auto [res2, p2_time] = Elfperf::execute([&inpt](){ return part2(inpt); }, 1000);

    Elfreport::report(res1, res2, io_time, p1_time, p2_time);

    return 0;
}