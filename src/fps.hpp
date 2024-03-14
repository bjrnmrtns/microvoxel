#pragma once

#include <array>
#include <chrono>

class fps {
public:
  static constexpr size_t size = 10;
  void add_timepoint();
  unsigned int value();
private:
  std::array<std::chrono::high_resolution_clock::time_point, size> time_points;
  size_t current = 0;
};
