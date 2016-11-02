#ifndef MAP
#define MAP

#include <vector>

#include "Point.hpp"
#include "Color.hpp"
#include "Canvas.hpp"

using namespace std;

class Map {
public:
  Map(int virtices_num);
  void init();
  void update();
  void draw();
  bool isSteady();
  bool shouldFinish();

protected:
  Canvas canvas;
  int virtices_num;
  bool steady_flag;

  vector<Point> virtices;
  vector<vector<int>> adjacency;
  vector<int> color;

  vector<Color> palette = {
    {1.0, 0.0, 0.0},
    {0.0, 1.0, 0.0},
    {0.0, 0.0, 1.0},
    {1.0, 1.0, 0.0}
  };
};

#endif /* end of include guard: MAP */
