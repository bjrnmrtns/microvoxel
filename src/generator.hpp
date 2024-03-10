#pragma once

#include <vector>
#include "vertex.hpp"

constexpr int CHUNK_SIZE = 4;

std::tuple<std::vector<vertex>, std::vector<unsigned int>> build_chunk(glm::ivec3 chunk_offset);
