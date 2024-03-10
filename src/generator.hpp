#pragma once

#include <vector>
#include "vertex.hpp"

constexpr int CHUNK_SIZE = 1;

std::tuple<std::vector<vertex>, std::vector<unsigned int>> build_chunk(glm::vec3 chunk_offset);
