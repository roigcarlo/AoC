#include <cmath>
#include <cstdio>
#include <chrono>
#include <vector>
#include <string>
#include <ranges>
#include <numeric>
#include <fstream>
#include <iostream>
#include <algorithm>
#include <execution>
#include <string_view>
#include <unordered_set>

using std::operator""sv;

struct csv_whitespace : std::ctype<char>
{
    static const mask * make_table()
    {
        static std::vector<mask> v(classic_table(), classic_table() + table_size);
        v[' '] &= ~space;
        return &v[0];
    }
 
    csv_whitespace(std::size_t refs = 0) : ctype(make_table(), false, refs) {}
};

auto read(char * filename) {
    std::fstream file(filename);
    std::vector<std::string> lines;

    file.imbue(std::locale(file.getloc(), new csv_whitespace));

    std::copy(
        std::istream_iterator<std::string>(file),
        std::istream_iterator<std::string>(),
        std::back_inserter(lines)
    );

    return std::move(lines);
}

std::size_t process_line(std::string & line) {
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

std::size_t part1(std::vector<std::string> & fv) {
    return std::transform_reduce(
        fv.begin(), fv.end(),
        0, std::plus<int>(),
        [](std::string & line) -> int {
            auto matches = process_line(line);
            return matches ? std::pow(2, matches-1) : 0;
        }
    );
}

std::size_t part2(std::vector<std::string> & fv) {
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
    using namespace std::chrono;

    auto io_s = high_resolution_clock::now();
    auto input = read(argv[1]);
    auto io_e = high_resolution_clock::now();

    auto p1_s = high_resolution_clock::now();
    auto r1 = part1(input);
    for(int i = 0; i < 999; i++) part1(input);
    auto p1_e = high_resolution_clock::now();

    auto p2_s = high_resolution_clock::now();
    auto r2 = part2(input);
    for(int i = 0; i < 999; i++) part2(input);
    auto p2_e = high_resolution_clock::now();

    std::cout << "I/O   : " << duration_cast<microseconds>(io_e - io_s) << std::endl;
    std::cout << "Part 1: " << duration_cast<microseconds>(p1_e - p1_s) / 1000 << " " << r1 << std::endl;
    std::cout << "Part 2: " << duration_cast<microseconds>(p2_e - p2_s) / 1000 << " " << r2 << std::endl;

    return 0;
}