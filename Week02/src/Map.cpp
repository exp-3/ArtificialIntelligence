#include "Map.hpp"

#include <iostream>
#include <random>

using namespace std;

Map::Map() {
  ;
}

void Map::init(Graph &graph) {
  counter = 0;

  random_device seed_gen;
  engine = default_random_engine(seed_gen());
  dist = uniform_int_distribution<>(0, 1);

  steady_flag = false;
  virtices_num = graph.get_virtices_num();
  Map::graph = graph;

  color = vector<int>(virtices_num, 0);
  canvas.init();
}

void Map::update() {
  counter++;
  steady_flag = true; // 状態が収束したかどうか
  for(int i = 0; i < virtices_num - 1; i++) {
    for(int j = i + 1; j < virtices_num; j++) {
      // もしi番目のノードとj番目のノードが隣接していなければスキップ
      if(!graph.get_adjacency(i, j)) {
        continue;
      }

      // もしi番目のノードとj番目のノードの色が同じなら
      if(color[i] == color[j]) {
        // まだ状態は収束していない
        steady_flag = false;

        // 片方のノードの色を変更する
        // 発振状態に陥らないように、どちらを変更するかはランダム
        if(dist(engine)) {
          color[j] = (color[j] + 1) % 4;
        } else {
          color[i] = (color[i] + 1) % 4;
        }
        break;
      }
    }

    if(!steady_flag) {
      break;
    }
  }
}

void Map::draw() {
  canvas.clear();
  for(int i = 0; i < virtices_num - 1; i++) {
    for(int j = i + 1; j < virtices_num; j++) {
      if(graph.get_adjacency(i, j)) {
        canvas.draw_line(graph.get_virtex(i), graph.get_virtex(j));
      }
    }
  }

  for(int i = 0; i < virtices_num; i++) {
    canvas.draw_circle(graph.get_virtex(i), palette[color[i]]);
  }

  canvas.flush();

  if(!isSteady()) {
    string filename = "images/image" + to_string(counter) + ".jpg";
    canvas.save(filename);
  }
  canvas.poll_events();
  canvas.update();
}

bool Map::isSteady() {
  return steady_flag;
}

bool Map::shouldFinish() {
  return canvas.should_close();
}
