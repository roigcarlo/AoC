#include <vector>
#include <fstream>
#include <iostream>
#include <iterator>

namespace Elfio {
    enum Mode {
        Stnd = 0,
        Snow = 1 << 1
    };

    struct csv_whitespace : std::ctype<char> {
        static const mask * make_table()
        {
            static std::vector<mask> v(classic_table(), classic_table() + table_size);
            v[' '] &= ~space;
            return &v[0];
        }
    
        csv_whitespace(std::size_t refs = 0) : ctype(make_table(), false, refs) {}
    };

    auto read(char * filename, Mode snow_flaks = Mode::Stnd) {
        std::fstream file(filename);

        std::string cline;
        std::vector<std::string> lines;

        // if(snow_flaks && Mode::Snow) {
        //     file.imbue(std::locale(file.getloc(), new csv_whitespace));
        // }

        while(std::getline(file, cline)) {
            lines.emplace_back(cline);
         }

        return std::move(lines);
    }
}