#include <chrono>

namespace Elfperf {
    template <class F> 
    auto execute(const F& f, std::size_t times=1) {
        using namespace std::chrono;

        auto chron_beg = high_resolution_clock::now();
        
        auto r = f();
        for(int i = 0; i < times-1; i++) f();

        auto chron_end = high_resolution_clock::now();

        return std::make_pair(r, (chron_end - chron_beg) / times);
    }
}