#ifndef CANVAS
#define CANVAS

#include <vector>
#include <GLFW/glfw3.h>
#include "Point.hpp"
#include "Color.hpp"

using namespace std;

class Canvas {
public:
  Canvas();
  void init();
  void draw_circle(Point &p, Color &c);
  void draw_line(Point &start, Point &end);
  void clear();
  void flush();
  bool should_close();
  int get_window_width();
  int get_window_height();
  void poll_events();
  void wait_events();
  void update();

protected:
  int width = 640;
  int height = 480;
  double circle_radius = 0.05;
  int circle_virtices_num = 32;
  vector<Point> circle_virtices;
  GLFWwindow *window;
};

#endif /* end of include guard: CANVAS */
