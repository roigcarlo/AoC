#include <iostream>

namespace Elfreport {
    template <class TTime>
    std::string cast_time(const TTime & time) {
        using namespace std::chrono;

        auto time_ns = time.count();

        if(time_ns < 1000) {
            return std::to_string(time_ns) + "ns";
        } else if(time_ns < 1000000) {
            return std::to_string(time_ns / 1000) + "us";
        } else if(time_ns < 1000000000) {
            return std::to_string(time_ns / 1000000) + "ms";
        } else if(time_ns < 60000000000) {                              // For day 10 and above
            return std::to_string(time_ns / 1000000000) + "s";
        } else if(time_ns < 3600000000000) {                            // For day 12 and above
            return std::to_string(time_ns / 60000000000) + "m";
        } else if(time_ns < 86400000000000) {
            return std::to_string(time_ns / 3600000000000) + "h";
        } else if(time_ns < 604800000000000) {
            return std::to_string(time_ns / 86400000000000) + "d";
        } else if(time_ns < 31536000000000000) {                        // For day 19 and above
            return std::to_string(time_ns / 604800000000000) + "w";
        } else {
            return std::to_string(time_ns / 31536000000000000) + "y";
        }
    }

    template <class TRes1, class TRes2, class TIoTime, class TRes1Tme, class TRes2Tme> 
    auto report(
        const TRes1 & res1, const TRes2 & res2, 
        const TIoTime & io_time, const TRes1Tme & p1_time, const TRes2Tme & p2_time
    ) {
        std::cout << "I/O   : " << io_time << std::endl;
        std::cout << "Part 1: " << cast_time(p1_time) << " " << res1 << std::endl;
        std::cout << "Part 2: " << cast_time(p2_time) << " " << res2 << std::endl;
    }
}