#ifndef MAP
#define MAP

#include <vector>
#include <random>

#include "Point.hpp"
#include "Color.hpp"
#include "Canvas.hpp"
#include "Graph.hpp"

using namespace std;

class Map {
public:
  Map();
  void init(Graph &graph);
  void update();
  void draw();
  bool isSteady();
  bool shouldFinish();

protected:
  int counter;
  Canvas canvas;
  Graph graph;
  int virtices_num;
  bool steady_flag;
  default_random_engine engine;
  uniform_int_distribution<> dist;

  vector<int> color;

  vector<Color> palette = {
    {1.0, 0.0, 0.0},
    {0.0, 1.0, 0.0},
    {0.0, 0.0, 1.0},
    {1.0, 1.0, 0.0}
  };
};

#endif /* end of include guard: MAP */
