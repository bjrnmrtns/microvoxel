#pragma once

#include <vector>
#include "vertex.hpp"

std::vector<unsigned int> cube_indices();
std::vector<vertex> cube_front();
std::vector<vertex> cube_back();
std::vector<vertex> cube_left();
std::vector<vertex> cube_right();
std::vector<vertex> cube_top();
std::vector<vertex> cube_bottom();
