#include "Graph.hpp"

#include <random>
#include <iostream>

Graph::Graph() {
  ;
}

void Graph::set(vector<Point> &virtices, vector<vector<int>> &adjacency) {
  Graph::virtices_num = virtices.size();
  Graph::virtices = virtices;
  Graph::adjacency = adjacency;
}

void Graph::set_random(int virtices_num) {
  Graph::virtices_num = virtices_num;

  virtices.resize(virtices_num);
  adjacency = vector<vector<int>>(virtices_num, vector<int>(virtices_num, 0));

  random_device seed_gen;
  default_random_engine engine(seed_gen());
  uniform_real_distribution<> dist(-1, 1);

  for(int i = 0; i < virtices_num; i++) {
    virtices[i].x = dist(engine);
    virtices[i].y = dist(engine);
  }

  // ドロネー三角分割による平面グラフと隣接行列の生成
  double epsilon = 1e-6;
  for(int i = 0; i < virtices_num - 2; i++) {
    Point v1 = virtices[i];
    for(int j = i + 1; j < virtices_num - 1; j++) {
      Point v2 = virtices[j];
      for(int k = j + 1; k < virtices_num; k++) {
        Point v3 = virtices[k];
        double tmp = 2.0*((v2.x-v1.x)*(v3.y-v1.y)-(v2.y-v1.y)*(v3.x-v1.x));
        Point center = {((v3.y-v1.y)*(v2.x*v2.x-v1.x*v1.x+v2.y*v2.y-v1.y*v1.y)+
           (v1.y-v2.y)*(v3.x*v3.x-v1.x*v1.x+v3.y*v3.y-v1.y*v1.y))/tmp,
          ((v1.x-v3.x)*(v2.x*v2.x-v1.x*v1.x+v2.y*v2.y-v1.y*v1.y)+
           (v2.x-v1.x)*(v3.x*v3.x-v1.x*v1.x+v3.y*v3.y-v1.y*v1.y))/tmp};
        double r = center.distance(v1) - epsilon;

        bool flag = true;
        for(int l = 0; l < virtices_num; l++) {
          if(center.distance(virtices[l]) < r) {
            flag = false;
            break;
          }
        }

        if(flag) {
          adjacency[i][j] = 1;
          adjacency[i][k] = 1;
          adjacency[j][k] = 1;
          adjacency[j][i] = 1;
          adjacency[k][i] = 1;
          adjacency[k][j] = 1;
        }
      }
    }
  }

  cout << "==== virtices ====" << endl;
  for(int i = 0; i < virtices_num; i++) {
    cout << "(" << virtices[i].x << ", " << virtices[i].y << ")" << endl;
  }

  cout << endl;

  cout << "==== adjacency ====" << endl;
  for(int i = 0; i < virtices_num; i++) {
    cout << adjacency[i][0];
    for(int j = 0; j < virtices_num; j++) {
      cout << ", " << adjacency[i][j];
    }
    cout << endl;
  }
}

int Graph::get_virtices_num() {
  return virtices_num;
}

Point &Graph::get_virtex(int virtex_id) {
  return virtices[virtex_id];
}

int Graph::get_adjacency(int m, int n) {
  return adjacency[m][n];
}
