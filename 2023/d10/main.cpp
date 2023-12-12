#include <queue>
#include <cstdio>
#include <string>
#include <ranges>
#include <numeric>
#include <algorithm>
#include <execution>
#include <unordered_map>
#include <unordered_set>

#include "elf_io.h"
#include "elf_perf.h"
#include "elf_report.h"

int to_key(const char & a) {
    switch(a) {
        case '.': return 0;
        case '|': return 1;
        case '-': return 2;
        case 'L': return 3;
        case 'J': return 4;
        case '7': return 5;
        case 'F': return 6;
        case 'S': return 7;
        default: 
            std::cout << "This cannot be happening" << std::endl;
    }
    return -1;
}

bool from_n(auto & maze, int x, int y) {
    auto & val = maze[x][y];
    return val == 1 || val == 5 || val == 6;
}

bool from_s(auto & maze, int x, int y) {
    auto & val = maze[x][y];
    return val == 1 || val == 3 || val == 4;
}

bool from_w(auto & maze, int x, int y) {
    auto & val = maze[x][y];
    return val == 2 || val == 3 || val == 6;
}

bool from_e(auto & maze, int x, int y) {
    auto & val = maze[x][y];
    return val == 2 || val == 4 || val == 5;
}

void add_cluster(auto & cluster, int c) {
    if(c > 0) cluster.insert(c);
}

std::size_t clusterize(auto & maze, int c, int x, int y) {
    std::queue<std::pair<int,int>> cluster_queue;
    cluster_queue.push({x,y});
    std::size_t cluster_size = 0;

    while(!cluster_queue.empty()) {
        auto [cx,cy] = cluster_queue.front();
        if (maze[cx][cy] == 0) {
            maze[cx][cy] = c;
            cluster_size++;
            if (cx > 0                 && !maze[cx-1][cy]) cluster_queue.push({cx-1,cy});
            if (cx < maze.size()-1     && !maze[cx+1][cy]) cluster_queue.push({cx+1,cy});
            if (cy > 0                 && !maze[cx][cy-1]) cluster_queue.push({cx,cy-1});
            if (cy < maze[cx].size()-1 && !maze[cx][cy+1]) cluster_queue.push({cx,cy+1});
        }
        cluster_queue.pop();
    }

    return cluster_size;
}

std::size_t part1(const std::vector<std::string> & fv) {
    auto maze = fv | std::views::transform([](const auto & l) { 
        return l | std::views::transform([](const auto & v) { 
                return to_key(v);
        }) | std::ranges::to<std::vector>(); })
    | std::ranges::to<std::vector>();

    // Get Starting point
    std::pair<int, int> pbeg{-1,-1};
    for(std::size_t i = 0; i < maze.size() && pbeg == std::pair<int, int>{-1,-1}; i++) {
        for(std::size_t j = 0; j <maze[i].size() && pbeg == std::pair<int, int>{-1,-1}; j++) {
            if (maze[i][j] == 7) pbeg = std::make_pair(i,j);
        }
    }

    // Get Possible Starting routes (optimize this, pick first)
    std::vector<std::pair<int, int>> possible_routes;
    if (std::get<0>(pbeg) > 0                                && from_n(maze, std::get<0>(pbeg)-1,std::get<1>(pbeg))) possible_routes.push_back(std::make_pair(std::get<0>(pbeg)-1, std::get<1>(pbeg)));
    if (std::get<0>(pbeg) < maze.size()-1                    && from_s(maze, std::get<0>(pbeg)+1,std::get<1>(pbeg))) possible_routes.push_back(std::make_pair(std::get<0>(pbeg)+1, std::get<1>(pbeg)));
    if (std::get<1>(pbeg) > 0                                && from_w(maze, std::get<0>(pbeg),std::get<1>(pbeg)-1)) possible_routes.push_back(std::make_pair(std::get<0>(pbeg), std::get<1>(pbeg)-1));
    if (std::get<1>(pbeg) < maze[std::get<0>(pbeg)].size()-1 && from_e(maze, std::get<0>(pbeg),std::get<1>(pbeg)+1)) possible_routes.push_back(std::make_pair(std::get<0>(pbeg), std::get<1>(pbeg)+1));

    // Pick one and go
    int dist = -1;
    auto [px,py] = pbeg;
    auto [cx,cy] = possible_routes[0];

    while (maze[cx][cy] != 7) {
        // N
             if (maze[cx][cy] == 1 && cx < px) {px=cx; py=cy; cx-=1;}
        else if (maze[cx][cy] == 5 && cx < px) {px=cx; py=cy; cy-=1;}
        else if (maze[cx][cy] == 6 && cx < px) {px=cx; py=cy; cy+=1;}
        // S
        else if (maze[cx][cy] == 1 && cx > px) {px=cx; py=cy; cx+=1;}
        else if (maze[cx][cy] == 3 && cx > px) {px=cx; py=cy; cy+=1;}
        else if (maze[cx][cy] == 4 && cx > px) {px=cx; py=cy; cy-=1;}
        // W
        else if (maze[cx][cy] == 2 && cy < py) {px=cx; py=cy; cy-=1;}
        else if (maze[cx][cy] == 3 && cy < py) {px=cx; py=cy; cx-=1;}
        else if (maze[cx][cy] == 6 && cy < py) {px=cx; py=cy; cx+=1;}
        // E
        else if (maze[cx][cy] == 2 && cy > py) {px=cx; py=cy; cy+=1;}
        else if (maze[cx][cy] == 4 && cy > py) {px=cx; py=cy; cx-=1;}
        else if (maze[cx][cy] == 5 && cy > py) {px=cx; py=cy; cx+=1;}

        maze[px][py] = dist--;
    }
        
    return (-dist)/2;
}

int part2(const std::vector<std::string> & fv) {
    auto maze = fv | std::views::transform([](const auto & l) { 
        return l | std::views::transform([](const auto & v) { 
                return to_key(v);
        }) | std::ranges::to<std::vector>(); })
    | std::ranges::to<std::vector>();

    // Get Starting point
    std::pair<int, int> pbeg{-1,-1};
    for(std::size_t i = 0; i < maze.size() && pbeg == std::pair<int, int>{-1,-1}; i++) {
        for(std::size_t j = 0; j <maze[i].size() && pbeg == std::pair<int, int>{-1,-1}; j++) {
            if (maze[i][j] == 7) pbeg = std::make_pair(i,j);
        }
    }

    // Get Possible Starting routes (optimize this, pick first)
    std::vector<std::pair<int, int>> possible_routes;
    if (std::get<0>(pbeg) > 0                                && from_n(maze, std::get<0>(pbeg)-1,std::get<1>(pbeg))) possible_routes.push_back(std::make_pair(std::get<0>(pbeg)-1, std::get<1>(pbeg)));
    if (std::get<0>(pbeg) < maze.size()-1                    && from_s(maze, std::get<0>(pbeg)+1,std::get<1>(pbeg))) possible_routes.push_back(std::make_pair(std::get<0>(pbeg)+1, std::get<1>(pbeg)));
    if (std::get<1>(pbeg) > 0                                && from_w(maze, std::get<0>(pbeg),std::get<1>(pbeg)-1)) possible_routes.push_back(std::make_pair(std::get<0>(pbeg), std::get<1>(pbeg)-1));
    if (std::get<1>(pbeg) < maze[std::get<0>(pbeg)].size()-1 && from_e(maze, std::get<0>(pbeg),std::get<1>(pbeg)+1)) possible_routes.push_back(std::make_pair(std::get<0>(pbeg), std::get<1>(pbeg)+1));

    // Pick one and go
    int dist = -1;
    int turn = 0;
    auto [px,py] = pbeg;
    auto [cx,cy] = possible_routes[0];
    maze[px][py] = -maze[px][py];

    while (maze[cx][cy] != -7) {
        // N
             if (maze[cx][cy] == 1 && cx < px) {px=cx; py=cy; cx-=1;}
        else if (maze[cx][cy] == 5 && cx < px) {px=cx; py=cy; cy-=1;}
        else if (maze[cx][cy] == 6 && cx < px) {px=cx; py=cy; cy+=1;}
        // S
        else if (maze[cx][cy] == 1 && cx > px) {px=cx; py=cy; cx+=1;}
        else if (maze[cx][cy] == 3 && cx > px) {px=cx; py=cy; cy+=1;}
        else if (maze[cx][cy] == 4 && cx > px) {px=cx; py=cy; cy-=1;}
        // W
        else if (maze[cx][cy] == 2 && cy < py) {px=cx; py=cy; cy-=1;}
        else if (maze[cx][cy] == 3 && cy < py) {px=cx; py=cy; cx-=1;}
        else if (maze[cx][cy] == 6 && cy < py) {px=cx; py=cy; cx+=1;}
        // E
        else if (maze[cx][cy] == 2 && cy > py) {px=cx; py=cy; cy+=1;}
        else if (maze[cx][cy] == 4 && cy > py) {px=cx; py=cy; cx-=1;}
        else if (maze[cx][cy] == 5 && cy > py) {px=cx; py=cy; cx+=1;}

        maze[px][py] = -maze[px][py];
    }

    for(std::size_t i = 0; i < maze.size(); i++) {
        for(std::size_t j = 0; j <maze[i].size(); j++) {
            if(maze[i][j] > 0) maze[i][j] = 0;
        }
    }

    // For every point possible, assign it to a cluster
    int cluster = 1;
    std::unordered_map<int, int> cluster_map;
    for(std::size_t i = 0; i < maze.size(); i++) {
        for(std::size_t j = 0; j <maze[i].size(); j++) {
            if(maze[i][j] == 0) {
                std::size_t cluster_size = clusterize(maze,cluster,i,j);
                cluster_map.insert({-cluster, cluster_size});
                cluster++;
            }
        }
    }

    // Set two clusters
    std::unordered_set<int> clusters_a = {};
    std::unordered_set<int> clusters_b = {};

    cx = possible_routes[0].first;
    cy = possible_routes[0].second;
    while (maze[cx][cy] != -7) {
        // N
             if (maze[cx][cy] == -1 && cx < px) {px=cx; py=cy; cx-=1;}
        else if (maze[cx][cy] == -5 && cx < px) {
            if (cx > 0)                                         add_cluster(clusters_b,maze[cx-1][cy]);
            if (cy < maze[cx].size()-1)                         add_cluster(clusters_b,maze[cx][cy+1]);
            if (cx > 0 && cy < maze[cx].size()-1)               add_cluster(clusters_b,maze[cx-1][cy+1]);
                                                                add_cluster(clusters_a,maze[cx+1][cy-1]);
            px=cx; py=cy; cy-=1; turn+=1;
        }
        else if (maze[cx][cy] == -6 && cx < px) {
            if (cx > 0)                                         add_cluster(clusters_a,maze[cx-1][cy]);
            if (cy > 0)                                         add_cluster(clusters_a,maze[cx][cy-1]);
            if (cx > 0 && cy > 0)                               add_cluster(clusters_a,maze[cx-1][cy-1]);
                                                                add_cluster(clusters_b,maze[cx+1][cy+1]);
            px=cx; py=cy; cy+=1; turn-=1;
        }
        // S
        else if (maze[cx][cy] == -1 && cx > px) {px=cx; py=cy; cx+=1;}
        else if (maze[cx][cy] == -3 && cx > px) {
            if (cx < maze.size()-1)                             add_cluster(clusters_b,maze[cx+1][cy]);
            if (cy > 0)                                         add_cluster(clusters_b,maze[cx][cy-1]);
            if (cx < maze.size()-1 && cy > 0)                   add_cluster(clusters_b,maze[cx+1][cy-1]);
                                                                add_cluster(clusters_a,maze[cx-1][cy+1]);
            px=cx; py=cy; cy+=1; turn+=1;
        }
        else if (maze[cx][cy] == -4 && cx > px) {
            if (cx < maze.size()-1)                             add_cluster(clusters_a,maze[cx+1][cy]);
            if (cy < maze[cx].size()-1)                         add_cluster(clusters_a,maze[cx][cy+1]);
            if (cx < maze.size()-1 && cy < maze[cx].size()-1)   add_cluster(clusters_a,maze[cx+1][cy+1]);
                                                                add_cluster(clusters_b,maze[cx-1][cy-1]);
            px=cx; py=cy; cy-=1; turn-=1;
        }
        // W
        else if (maze[cx][cy] == -2 && cy < py) {px=cx; py=cy; cy-=1;}
        else if (maze[cx][cy] == -3 && cy < py) {
            if (cx < maze.size()-1)                             add_cluster(clusters_a,maze[cx+1][cy]);
            if (cy > 0)                                         add_cluster(clusters_a,maze[cx][cy-1]);
            if (cx < maze.size()-1 && cy > 0)                   add_cluster(clusters_a,maze[cx+1][cy-1]);
                                                                add_cluster(clusters_b,maze[cx-1][cy+1]);
            px=cx; py=cy; cx-=1; turn-=1;
        }
        else if (maze[cx][cy] == -6 && cy < py) {
            if (cx > 0)                                         add_cluster(clusters_b,maze[cx-1][cy]);
            if (cy > 0)                                         add_cluster(clusters_b,maze[cx][cy-1]);
            if (cx > 0 && cy > 0)                               add_cluster(clusters_b,maze[cx-1][cy-1]);
                                                                add_cluster(clusters_a,maze[cx+1][cy+1]);
            px=cx; py=cy; cx+=1; turn+=1;
        }
        // E
        else if (maze[cx][cy] == -2 && cy > py) {px=cx; py=cy; cy+=1;}
        else if (maze[cx][cy] == -4 && cy > py) {
            if (cx < maze.size()-1)                             add_cluster(clusters_b,maze[cx+1][cy]);
            if (cy < maze[cx].size()-1)                         add_cluster(clusters_b,maze[cx][cy+1]);
            if (cx < maze.size()-1 && cy > 0)                   add_cluster(clusters_b,maze[cx+1][cy+1]);
                                                                add_cluster(clusters_a,maze[cx-1][cy-1]);
            px=cx; py=cy; cx-=1; turn+=1;
        }
        else if (maze[cx][cy] == -5 && cy > py) {
            if (cx > 0)                                         add_cluster(clusters_a,maze[cx-1][cy]);
            if (cy < maze[cx].size()-1)                         add_cluster(clusters_a,maze[cx][cy+1]);
            if (cx > 0 && cy < maze[cx].size()-1)               add_cluster(clusters_a,maze[cx-1][cy+1]);
                                                                add_cluster(clusters_b,maze[cx+1][cy-1]);
            px=cx; py=cy; cx+=1; turn-=1;
        }
    }

    auto & in_cluster = turn > 0 ? clusters_a : clusters_b;

    std::size_t result = 0;
    for(auto & c: in_cluster) {
        result += cluster_map[-c];
    }

    return result;
}

int main (int argc, char** argv) {
    auto [inpt, io_time] = Elfperf::execute([&argv](){ return Elfio::read(argv[1]);});
    auto [res1, p1_time] = Elfperf::execute([&inpt](){ return part1(inpt); }, 1000);
    auto [res2, p2_time] = Elfperf::execute([&inpt](){ return part2(inpt); }, 1000);

    Elfreport::report(res1, res2, io_time, p1_time, p2_time);

    return 0;
}