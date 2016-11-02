#ifndef POINT
#define POINT

struct Point {
  double x;
  double y;
  double distance(Point &other) {
    return (x - other.x) * (x - other.x) + (y - other.y) * (y - other.y);
  }
};

#endif /* end of include guard: POINT */
