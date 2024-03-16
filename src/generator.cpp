#include "generator.hpp"
#include "vertex.hpp"

#include <array>
#include <glm/glm.hpp>
#include <random>
#include <tuple>
#include <vector>

namespace {

constexpr const float alpha = 0.5f;
constexpr std::array<glm::vec4, 8> color_palette{
    glm::vec4(0.0, 0.0, 0.0, alpha), glm::vec4(0.0, 0.0, 1.0, alpha),
    glm::vec4(0.0, 1.0, 0.0, alpha), glm::vec4(0.0, 1.0, 1.0, alpha),
    glm::vec4(1.0, 0.0, 0.0, alpha), glm::vec4(1.0, 0.0, 1.0, alpha),
    glm::vec4(1.0, 1.0, 0.0, alpha), glm::vec4(1.0, 1.0, 1.0, alpha),
};

constexpr const glm::vec3 top(0.0f, 1.0f, 0.0f);
constexpr const glm::vec3 bottom(0.0f, -1.0f, 0.0f);
constexpr const glm::vec3 left(-1.0f, 0.0f, 0.0f);
constexpr const glm::vec3 right(1.0f, 0.0f, 0.0f);
constexpr const glm::vec3 back(0.0f, 0.0f, -1.0f);
constexpr const glm::vec3 front(0.0f, 0.0f, 1.0f);

constexpr const glm::vec3 cbv[8] = {
    {-0.5, -0.5, 0.5}, {0.5, -0.5, 0.5}, {-0.5, 0.5, 0.5}, {0.5, 0.5, 0.5},
    {-0.5, -0.5, -0.5}, {0.5, -0.5, -0.5}, {-0.5, 0.5, -0.5}, {0.5, 0.5, -0.5},
};

constexpr const std::array<unsigned int, 6> cube_indices{0, 1, 2, 1, 3, 2};

std::vector<vertex> cube_front(const glm::vec3 location,
                               const glm::vec4 color) {
  auto normal = front;
  return {
      {
          .p = cbv[0] + location,
          .n = normal,
          .c = color,
      },
      {
          .p = cbv[1] + location,
          .n = normal,
          .c = color,
      },
      {
          .p = cbv[2] + location,
          .n = normal,
          .c = color,
      },
      {
          .p = cbv[3] + location,
          .n = normal,
          .c = color,
      },
  };
}

std::vector<vertex> cube_back(const glm::vec3 location, const glm::vec4 color) {
  auto normal = back;
  return {
      {
          .p = cbv[5] + location,
          .n = normal,
          .c = color,
      },
      {
          .p = cbv[4] + location,
          .n = normal,
          .c = color,
      },
      {
          .p = cbv[7] + location,
          .n = normal,
          .c = color,
      },
      {
          .p = cbv[6] + location,
          .n = normal,
          .c = color,
      },
  };
}

std::vector<vertex> cube_left(const glm::vec3 location, const glm::vec4 color) {
  auto normal = left;
  return {
      {
          .p = cbv[4] + location,
          .n = normal,
          .c = color,
      },
      {
          .p = cbv[0] + location,
          .n = normal,
          .c = color,
      },
      {
          .p = cbv[6] + location,
          .n = normal,
          .c = color,
      },
      {
          .p = cbv[2] + location,
          .n = normal,
          .c = color,
      },
  };
}

std::vector<vertex> cube_right(const glm::vec3 location,
                               const glm::vec4 color) {
  auto normal = right;
  return {
      {
          .p = cbv[1] + location,
          .n = normal,
          .c = color,
      },
      {
          .p = cbv[5] + location,
          .n = normal,
          .c = color,
      },
      {
          .p = cbv[3] + location,
          .n = normal,
          .c = color,
      },
      {
          .p = cbv[7] + location,
          .n = normal,
          .c = color,
      },
  };
}

std::vector<vertex> cube_top(const glm::vec3 location, const glm::vec4 color) {
  auto normal = top;
  return {
      {
          .p = cbv[2] + location,
          .n = normal,
          .c = color,
      },
      {
          .p = cbv[3] + location,
          .n = normal,
          .c = color,
      },
      {
          .p = cbv[6] + location,
          .n = normal,
          .c = color,
      },
      {
          .p = cbv[7] + location,
          .n = normal,
          .c = color,
      },
  };
}

std::vector<vertex> cube_bottom(const glm::vec3 location,
                                const glm::vec4 color) {
  auto normal = bottom;
  return {
      {
          .p = cbv[4] + location,
          .n = normal,
          .c = color,
      },
      {
          .p = cbv[5] + location,
          .n = normal,
          .c = color,
      },
      {
          .p = cbv[0] + location,
          .n = normal,
          .c = color,
      },
      {
          .p = cbv[1] + location,
          .n = normal,
          .c = color,
      },
  };
}

void scale(std::vector<vertex>& vertices, const glm::vec3& scale)
{
    for(auto& v: vertices) {
        v.p = v.p * scale;
    }
}

std::vector<vertex> cube(glm::vec3 location, const glm::vec4 color) {
  std::vector<vertex> result;
  const auto cfront = cube_front(location, color);
  const auto cback = cube_back(location, color);
  const auto cbottom = cube_bottom(location, color);
  const auto ctop = cube_top(location, color);
  const auto cleft = cube_left(location, color);
  const auto cright = cube_right(location, color);
  result.insert(result.end(), cfront.begin(), cfront.end());
  result.insert(result.end(), cback.begin(), cback.end());
  result.insert(result.end(), cbottom.begin(), cbottom.end());
  result.insert(result.end(), ctop.begin(), ctop.end());
  result.insert(result.end(), cleft.begin(), cleft.end());
  result.insert(result.end(), cright.begin(), cright.end());

  return result;
}

std::vector<unsigned int> build_chunk_indices() {
  std::vector<unsigned int> indices;
  std::array<unsigned int, 6> current = cube_indices;
  for (size_t x = 0; x < CHUNK_SIZE; x++) {
    for (size_t y = 0; y < CHUNK_SIZE; y++) {
      for (size_t z = 0; z < CHUNK_SIZE; z++) {
        for (size_t i = 0; i < 6; i++) { // six sides of the cube
          indices.insert(indices.end(), current.begin(), current.end());
          for (auto &index : current) {
            index += 4;
          }
        }
      }
    }
  }
  return indices;
}

} // namespace

std::tuple<std::vector<vertex>, std::vector<unsigned int>>
build_chunk(glm::vec3 chunk_location) {
  std::random_device rd;
  std::mt19937 gen(rd());
  std::uniform_int_distribution<unsigned int> uint_distribution(0, 7);
  std::vector<vertex> resultv;
  std::vector<unsigned int> resulti = build_chunk_indices();
  for (int x = CHUNK_MIN; x < CHUNK_MAX; x++) {
    for (int y = CHUNK_MIN; y < CHUNK_MAX; y++) {
      for (int z = CHUNK_MIN; z < CHUNK_MAX; z++) {
        const auto color_index = uint_distribution(gen);
        const auto cube_location =
            chunk_location * (float)CHUNK_SIZE + glm::vec3(x, y, z);
        const auto c = cube(cube_location, color_palette[color_index]);
        resultv.insert(resultv.end(), c.begin(), c.end());
      }
    }
  }
  return {resultv, resulti};
}

std::array<unsigned int, 6> offset_indices(const std::array<unsigned int, 6>& indices, unsigned int offset)
{
    std::array<unsigned int, 6> result;
    for(size_t i = 0; i < indices.size(); i++) {
        result[i] = indices[i] + offset;
    }
    return result;
}

std::tuple<std::vector<vertex>, std::vector<unsigned int>>
build_lattice() {
  std::vector<vertex> vertices;
  std::vector<unsigned int> indices;
  const float alpha = 0.5f;
  constexpr int size_x = 500;
  constexpr int size_y = 100;
  constexpr int size_z = 250;
  for (int x = -size_x / 2; x < size_x / 2; x++) {
    auto s = cube_left(glm::vec3(x + 0.5, 0.0f, 0.0f), glm::vec4(1.0f, 0.0f, 0.0f, alpha)); 
    scale(s, glm::vec3(1.0f, size_y, size_z));
    const auto o_indices = offset_indices(cube_indices, vertices.size());
    vertices.insert(vertices.end(), s.begin(), s.end());
    indices.insert(indices.end(), o_indices.cbegin(), o_indices.cend());
  }
  for (int y = size_y - 1; y  >= -size_y / 2; y--) {
    auto s = cube_top(glm::vec3(0.0f, y + 0.5, 0.0f), glm::vec4(1.0f, 0.0f, 0.0f, alpha)); 
    scale(s, glm::vec3(size_x, 1.0f, size_z));
    const auto o_indices = offset_indices(cube_indices, vertices.size());
    vertices.insert(vertices.end(), s.begin(), s.end());
    indices.insert(indices.end(), o_indices.cbegin(), o_indices.cend());
  }
  for (int z = -size_z / 2; z < size_z / 2; z++) {
    auto s = cube_front(glm::vec3(0.0f, 0.0f, z + 0.5), glm::vec4(1.0f, 0.0f, 0.0f, alpha)); 
    scale(s, glm::vec3(size_x, size_y, 1.0f));
    const auto o_indices = offset_indices(cube_indices, vertices.size());
    vertices.insert(vertices.end(), s.begin(), s.end());
    indices.insert(indices.end(), o_indices.cbegin(), o_indices.cend());
  }
  for (int x = -size_x / 2; x < size_x / 2; x++) {
    auto s = cube_right(glm::vec3(x + 0.5, 0.0f, 0.0f), glm::vec4(1.0f, 0.0f, 0.0f, alpha)); 
    scale(s, glm::vec3(1.0f, size_y, size_z));
    const auto o_indices = offset_indices(cube_indices, vertices.size());
    vertices.insert(vertices.end(), s.begin(), s.end());
    indices.insert(indices.end(), o_indices.cbegin(), o_indices.cend());
  }
  /*
  for (int y = -size_y / 2; y < size_y / 2; y++) {
    auto s = cube_bottom(glm::vec3(0.0f, y + 0.5, 0.0f), glm::vec4(1.0f, 0.0f, 0.0f, alpha)); 
    scale(s, glm::vec3(size_y, 1.0f, size_z));
    const auto o_indices = offset_indices(cube_indices, vertices.size());
    vertices.insert(vertices.end(), s.begin(), s.end());
    indices.insert(indices.end(), o_indices.cbegin(), o_indices.cend());
  }*/
  for (int z = -size_z / 2; z < size_z / 2; z++) {
    auto s = cube_back(glm::vec3(0.0f, 0.0f, z + 0.5), glm::vec4(1.0f, 0.0f, 0.0f, alpha)); 
    scale(s, glm::vec3(size_x, size_y, 1.0f));
    const auto o_indices = offset_indices(cube_indices, vertices.size());
    vertices.insert(vertices.end(), s.begin(), s.end());
    indices.insert(indices.end(), o_indices.cbegin(), o_indices.cend());
  }
  return {vertices, indices};
}
