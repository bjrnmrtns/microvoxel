#include "fps.hpp"

void fps::add_timepoint() {
  time_points[current] = std::chrono::high_resolution_clock::now();
  current = (current + 1) % size;
}

unsigned int fps::value() {
  unsigned int milliseconds = 0;
  for (size_t i = 0; i < size - 1; i++) { // nine steps
    const auto first = (current  + i) % size;
    const auto second = (current + i + 1) % size;
    const auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(
        time_points[second] - time_points[first]);
    milliseconds += duration.count();
  }
  return 1000.0f / (milliseconds / 9.0f /* nine steps */);
}
