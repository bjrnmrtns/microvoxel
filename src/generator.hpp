#pragma once

#include <vector>
#include "vertex.hpp"

constexpr int CHUNK_SIZE = 128;
static_assert(CHUNK_SIZE % 2 == 0);
static_assert(CHUNK_SIZE > 0);
constexpr int CHUNK_MIN = -(CHUNK_SIZE / 2);
constexpr int CHUNK_MAX = (CHUNK_SIZE / 2);

std::tuple<std::vector<vertex>, std::vector<unsigned int>> build_chunk(glm::vec3 chunk_offset);
std::tuple<std::vector<vertex>, std::vector<unsigned int>> build_lattice();
