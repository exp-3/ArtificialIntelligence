#include <chrono>
#include <thread>

#include "Map.hpp"

using namespace std;

int main() {
  Map map = Map(100);
  map.init();
  while(!map.isSteady() && !map.shouldFinish()) {
    map.update();
    this_thread::sleep_for(chrono::milliseconds(100));
    map.draw();
  }

  while(!map.shouldFinish()) {
    map.draw();
  }

  return 0;
}
