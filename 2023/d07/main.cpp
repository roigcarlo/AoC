#include <cmath>
#include <cstdio>
#include <vector>
#include <string>
#include <ranges>
#include <algorithm>
#include <execution>
#include <string_view>
#include <unordered_map>
#include <unordered_set>

#include "elf_io.h"
#include "elf_perf.h"
#include "elf_report.h"

#include <bitset>

using std::operator""sv;

int to_index_table(const char & a) {
    return a >= '0' && a <= '9' ? a - '0' : a - 'A' + 10;
}

int eval_hand(const std::string_view & hand) {
    std::vector<std::size_t> reps(16, 0);
    int score = 0;

    for (const auto & c : hand) {reps[to_index_table(c)]++;}
    for (int i = 2; i < 16; i++) {  
        if(reps[i]) score += (1 << (3 * (reps[i]-1)));
    }

    if (score > 5) score &= 0xFFFFFFF8;

    return score;
}

int eval_hand_slow(const std::string_view & hand) {
    std::vector<std::size_t> reps(16, 0);
    int score = 0;

    int max_char = 0;
    int max_vals = 0;

    for (const auto & c : hand) {reps[to_index_table(c)]++;}
    for (int i = 2; i < 16; i++) {
        if ((reps[i] > max_vals || (reps[i] == max_vals && i > max_char))) {
            max_char = i;
            max_vals = reps[i];
        } 
    }

    for (const auto & c : hand) {if (to_index_table(c) == 1) reps[max_char]++;}
    for (int i = 2; i < 16; i++) {  
        if(reps[i]) score += (1 << (3 * (reps[i]-1)));
    }

    if (score > 5) score &= 0xFFFFFFF8;

    return score;
}

bool sort_game(const std::tuple<std::string, int, int> & a, const std::tuple<std::string, int, int> & b) {
    return std::get<2>(a) == std::get<2>(b) ? std::get<0>(a) < std::get<0>(b) : std::get<2>(a) < std::get<2>(b);
}

std::size_t part1(const std::vector<std::string> & fv) {
    auto game_view = fv | std::views::transform([](const std::string & s) { 
        auto split = s.begin() + 5;
        auto hnd = std::string(s.begin(), split);

        std::replace(hnd.begin(), hnd.end(), 'A','E');
        std::replace(hnd.begin(), hnd.end(), 'K','D');
        std::replace(hnd.begin(), hnd.end(), 'Q','C');
        std::replace(hnd.begin(), hnd.end(), 'J','B');
        std::replace(hnd.begin(), hnd.end(), 'T','A');

        auto bet = std::stoi(std::string(split + 1, s.end()));

        return std::make_tuple(hnd, bet, 0); 
    }) | std::ranges::to<std::vector>();

    std::for_each(
        std::execution::par_unseq,
        game_view.begin(), game_view.end(),
        [](auto & v) {
            std::get<2>(v) = eval_hand(std::string_view(std::get<0>(v)));
        }
    );

    std::sort(std::execution::par_unseq, game_view.begin(), game_view.end(), sort_game);

    auto game_rank = std::ranges::views::zip(std::views::iota(1) | std::views::take(game_view.size()) | std::ranges::to<std::vector>(), game_view);

    return std::transform_reduce(
        std::execution::par_unseq,
        game_rank.begin(), game_rank.end(), 0, std::plus<std::size_t>{},
        [](const auto & p) { 
            return std::get<0>(p) * std::get<1>(std::get<1>(p));
        }
    );
}

std::size_t part2(const std::vector<std::string> & fv) {
    auto game_view = fv | std::views::transform([](const std::string & s) { 
        auto split = s.begin() + 5;
        auto hnd = std::string(s.begin(), split);

        std::replace(hnd.begin(), hnd.end(), 'A','E');
        std::replace(hnd.begin(), hnd.end(), 'K','D');
        std::replace(hnd.begin(), hnd.end(), 'Q','C');
        std::replace(hnd.begin(), hnd.end(), 'T','A');
        std::replace(hnd.begin(), hnd.end(), 'J','1');

        auto bet = std::stoi(std::string(split + 1, s.end()));

        return std::make_tuple(hnd, bet, 0); 
    }) | std::ranges::to<std::vector>();

    std::for_each(
        std::execution::par_unseq,
        game_view.begin(), game_view.end(),
        [](auto & v) {
            std::get<2>(v) = eval_hand_slow(std::string_view(std::get<0>(v)));
        }
    );

    std::sort(std::execution::par_unseq, game_view.begin(), game_view.end(), sort_game);

    auto game_rank = std::ranges::views::zip(std::views::iota(1) | std::views::take(game_view.size()) | std::ranges::to<std::vector>(), game_view);

    return std::transform_reduce(
        std::execution::par_unseq,
        game_rank.begin(), game_rank.end(), 0, std::plus<std::size_t>{},
        [](const auto & p) { 
            return std::get<0>(p) * std::get<1>(std::get<1>(p));
        }
    );
}

int main (int argc, char** argv) {
    auto [inpt, io_time] = Elfperf::execute([&argv](){ return Elfio::read(argv[1], Elfio::Mode::Snow);});
    auto [res1, p1_time] = Elfperf::execute([&inpt](){ return part1(inpt); }, 1000);
    auto [res2, p2_time] = Elfperf::execute([&inpt](){ return part2(inpt); }, 1000);

    Elfreport::report(res1, res2, io_time, p1_time, p2_time);

    return 0;
}