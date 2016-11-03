#include <chrono>
#include <thread>
#include <vector>

#include "Map.hpp"
#include "Graph.hpp"
#include "Point.hpp"

using namespace std;

int main() {
  Map map;

  vector<Point> virtices = {
    {0, 0.8},
    {-0.4, 0.4},
    {0.4, 0.4},
    {-0.8, 0},
    {0, 0},
    {0.8, 0},
    {-0.4, -0.4},
    {0.4, -0.4},
    {0, -0.8}
  };

  vector<vector<int>> adjacency = {
    {0, 1, 1, 1, 0, 1, 0, 0, 0},
    {1, 0, 1, 1, 1, 0, 1, 0, 0},
    {1, 1, 0, 0, 1, 1, 0, 1, 0},
    {1, 1, 0, 0, 0, 0, 1, 0, 1},
    {0, 1, 1, 0, 0, 0, 1, 1, 0},
    {1, 0, 1, 0, 0, 0, 0, 1, 1},
    {0, 1, 0, 1, 1, 0, 0, 1, 1},
    {0, 0, 1, 0, 1, 1, 1, 0, 1},
    {0, 0, 0, 1, 0, 1, 1, 1, 0}
  };

  Graph graph;
  graph.set(virtices, adjacency);

  // graphをランダムに生成する場合は下の行のコメントを外す
  // graph.set_random(30);

  map.init(graph);
  while(!map.isSteady() && !map.shouldFinish()) {
    this_thread::sleep_for(chrono::milliseconds(100));
    map.draw();
    map.update();
  }

  while(!map.shouldFinish()) {
    map.draw();
  }

  return 0;
}
