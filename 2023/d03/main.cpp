#include <regex>
#include <ranges>
#include <cstdio>
#include <string>
#include <execution>
#include <unordered_set>
#include <unordered_map>

#include "elf_io.h"
#include "elf_perf.h"

std::size_t part1(const std::vector<std::string> & fv) {
    std::regex regexp("[0-9]+"); 
    std::unordered_set symbols{'*', '$', '#', '+', '/', '@', '&', '%', '!', '?', '^', '~', '<', '>', '=', '|', '(', ')', '[', ']', '{', '}', ',', ':', ';', '-', '_', '`', '\'', '"'};

    std::size_t r = 0;
    for(std::size_t l = 0; l < fv.size(); l++) {
        for (std::sregex_iterator match = std::sregex_iterator(fv[l].begin(), fv[l].end(), regexp); match != std::sregex_iterator(); ++match) {
            std::size_t orr = r;
            int mp = match->position();
            auto ms = match->str();
            
            auto back = std::max(mp - 1, 0);
            auto forw = std::min(mp + ms.size() + 1, fv[l].size());

            if (orr == r && back > 0 && symbols.contains(fv[l][back])) {
                r += std::stoi(ms);
            }
            if (orr == r && forw < (fv[l].size() - 1) && symbols.contains(fv[l][forw-1])) {
                r += std::stoi(ms);
            }
            if (orr == r && l > 0 && std::any_of(fv[l-1].begin()+back, fv[l-1].begin()+forw, [&symbols](const auto & e){return symbols.contains(e);})) {
                r += std::stoi(ms);
            }
            if (orr == r && l < fv.size()-1 && std::any_of(fv[l+1].begin()+back, fv[l+1].begin()+forw, [&symbols](const auto & e){return symbols.contains(e);})) {
                r += std::stoi(ms);
            }
        }
    }

    return r;
}

std::size_t part2(const std::vector<std::string> & fv) {
    std::regex regexp("[0-9]+"); 
    std::unordered_map<int,std::unordered_set<int>> gears;

    for(std::size_t l = 0; l < fv.size(); l++) {
        for (std::sregex_iterator match = std::sregex_iterator(fv[l].begin(), fv[l].end(), regexp); match != std::sregex_iterator(); ++match) {
            int mp = match->position();
            auto ms = match->str();
            
            auto back = std::max(mp - 1, 0);
            auto forw = std::min(mp + ms.size() + 1, fv[l].size());

            if (back > 0 && fv[l][back] == '*') {
                gears[l*fv[l].size()+back].insert(std::stoi(ms));
            }
            if (forw < (fv[l].size() - 1) && fv[l][forw-1] == '*') {
                gears[l*fv[l].size()+forw-1].insert(std::stoi(ms));
            }
            if (l > 0) {
                for (std::size_t i = back; i < forw; i++) {
                    if (fv[l-1][i] == '*') {
                        gears[(l-1)*fv[l-1].size()+i].insert(std::stoi(ms));
                    }
                }
            }
            if (l < fv.size()-1) {
                for (std::size_t i = back; i < forw; i++) {
                    if (fv[l+1][i] == '*') {
                        gears[(l+1)*fv[l+1].size()+i].insert(std::stoi(ms));
                    }
                }
            }
        }
    }

    auto r = std::transform_reduce(gears.begin(), gears.end(), 0, std::plus<int>{}, [](const auto & e) {
        if (std::get<1>(e).size() == 2) {
            return std::transform_reduce(std::get<1>(e).begin(), std::get<1>(e).end(), 1, std::multiplies<int>{}, [](const auto & g) { return g; });
        } else {
            return 0;
        }
    });

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