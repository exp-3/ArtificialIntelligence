#ifndef GRAPH
#define GRAPH

#include <vector>
#include "Point.hpp"

using namespace std;

class Graph {
public:
  Graph();
  void set(vector<Point> &virtices, vector<vector<int>> &adjacency);
  void set_random(int virtices_num);
  int get_virtices_num();
  Point &get_virtex(int virtex_id);
  int get_adjacency(int m, int n);

protected:
  int virtices_num;
  vector<Point> virtices;
  vector<vector<int>> adjacency;
};

#endif /* end of include guard: GRAPH */
