#include "generator.hpp"
#include "vertex.hpp"

#include <array>
#include <glm/glm.hpp>
#include <vector>
#include <random>
#include <tuple>

namespace {

constexpr std::array<glm::vec4, 8> color_palette {
    glm::vec4(0.0, 0.0, 0.0, 1.0),
    glm::vec4(0.0, 0.0, 1.0, 1.0),
    glm::vec4(0.0, 1.0, 0.0, 1.0),
    glm::vec4(0.0, 1.0, 1.0, 1.0),
    glm::vec4(1.0, 0.0, 0.0, 1.0),
    glm::vec4(1.0, 0.0, 1.0, 1.0),
    glm::vec4(1.0, 1.0, 0.0, 1.0),
    glm::vec4(1.0, 1.0, 1.0, 1.0),
};

constexpr const glm::vec3 top(0.0f, 1.0f, 0.0f);
constexpr const glm::vec3 bottom(0.0f, -1.0f, 0.0f);
constexpr const glm::vec3 left(-1.0f, 0.0f, 0.0f);
constexpr const glm::vec3 right(1.0f, 0.0f, 0.0f);
constexpr const glm::vec3 back(0.0f, 0.0f, -1.0f);
constexpr const glm::vec3 front(0.0f, 0.0f, 1.0f);

constexpr const glm::ivec3 cbv[8] = {
    { 0, 0, 0 },
    { 1, 0, 0 },
    { 0, 1, 0 },
    { 1, 1, 0 },
    { 0, 0, 1 },
    { 1, 0, 1 },
    { 0, 1, 1 },
    { 1, 1, 1 },
};

constexpr const std::array<unsigned int, 6> cube_indices { 0, 1, 2, 1, 3, 2 };

std::vector<vertex> cube_front(const glm::ivec3 location, const glm::vec4 color)
{
    auto normal = front;
    return { 
               { .p = cbv[0] + location, 
                 .n = normal, 
                 .c = color,
               },
               { .p = cbv[1] + location, 
                 .n = normal, 
                 .c = color,
               },
               { .p = cbv[2] + location, 
                 .n = normal, 
                 .c = color,
               },
               { .p = cbv[3] + location, 
                 .n = normal, 
                 .c = color,
               },
           };
}

std::vector<vertex> cube_back(const glm::ivec3 location, const glm::vec4 color)
{
    auto normal = back;
    return { 
               { .p = cbv[5] + location, 
                 .n = normal, 
                 .c = color,
               },
               { .p = cbv[4] + location, 
                 .n = normal, 
                 .c = color,
               },
               { .p = cbv[7] + location, 
                 .n = normal, 
                 .c = color,
               },
               { .p = cbv[6] + location, 
                 .n = normal, 
                 .c = color,
               },
           };
}

std::vector<vertex> cube_left(const glm::ivec3 location, const glm::vec4 color)
{
    auto normal = left;
    return { 
               { .p = cbv[4] + location, 
                 .n = normal, 
                 .c = color,
               },
               { .p = cbv[0] + location, 
                 .n = normal, 
                 .c = color,
               },
               { .p = cbv[6] + location, 
                 .n = normal, 
                 .c = color,
               },
               { .p = cbv[2] + location, 
                 .n = normal, 
                 .c = color,
               },
           };
}

std::vector<vertex> cube_right(const glm::ivec3 location, const glm::vec4 color)
{
    auto normal = right;
    return { 
               { .p = cbv[1] + location, 
                 .n = normal, 
                 .c = color,
               },
               { .p = cbv[5] + location, 
                 .n = normal, 
                 .c = color,
               },
               { .p = cbv[3] + location, 
                 .n = normal, 
                 .c = color,
               },
               { .p = cbv[7] + location, 
                 .n = normal, 
                 .c = color,
               },
           };
}

std::vector<vertex> cube_top(const glm::ivec3 location, const glm::vec4 color)
{
    auto normal = top;
    return { 
               { .p = cbv[2] + location, 
                 .n = normal, 
                 .c = color,
               },
               { .p = cbv[3] + location, 
                 .n = normal, 
                 .c = color,
               },
               { .p = cbv[6] + location, 
                 .n = normal, 
                 .c = color,
               },
               { .p = cbv[7] + location, 
                 .n = normal, 
                 .c = color,
               },
           };
}

std::vector<vertex> cube_bottom(const glm::ivec3 location, const glm::vec4 color)
{
    auto normal = bottom;
    return { 
               { .p = cbv[4] + location, 
                 .n = normal, 
                 .c = color,
               },
               { .p = cbv[5] + location, 
                 .n = normal, 
                 .c = color,
               },
               { .p = cbv[0] + location, 
                 .n = normal, 
                 .c = color,
               },
               { .p = cbv[1] + location, 
                 .n = normal, 
                 .c = color,
               },
           };
}

std::vector<vertex> cube(const glm::ivec3 location, const glm::vec4 color)
{
    std::vector<vertex> result;
    const auto cbottom = cube_bottom(location, color);
    const auto ctop = cube_top(location, color);
    const auto cleft = cube_left(location, color);
    const auto cright = cube_right(location, color);
    const auto cfront = cube_front(location, color);
    const auto cback = cube_back(location, color);
    result.insert(result.end(), cbottom.begin(), cbottom.end());
    result.insert(result.end(), ctop.begin(), ctop.end());
    result.insert(result.end(), cleft.begin(), cleft.end());
    result.insert(result.end(), cright.begin(), cright.end());
    result.insert(result.end(), cfront.begin(), cfront.end());
    result.insert(result.end(), cback.begin(), cback.end());

    return result;
}

std::vector<unsigned int> build_chunk_indices()
{
    std::vector<unsigned int> indices;
    std::array<unsigned int, 6> current = cube_indices;
    for(size_t x = 0; x < CHUNK_SIZE; x++) {
        for(size_t y = 0; y < CHUNK_SIZE; y++) {
           for(size_t z = 0; z < CHUNK_SIZE; z++) {
               for(size_t i = 0; i < 6; i++) { // six sides of the cube
                   indices.insert(indices.end(), current.begin(), current.end());
                   for(auto& index: current) {
                       index += 4;
                   }
               }
           }
        }
    }
    return indices;
}

}

std::tuple<std::vector<vertex>, std::vector<unsigned int>> build_chunk(glm::ivec3 chunk_location)
{
    std::random_device rd;
    std::mt19937 gen(rd());
    std::uniform_int_distribution<unsigned int> uint_distribution(0, 7);
    std::vector<vertex> resultv;
    std::vector<unsigned int> resulti = build_chunk_indices();
    glm::vec4 red {1.0f, 0.0, 0.0, 1.0};
    for(size_t x = 0; x < CHUNK_SIZE; x++) {
        for(size_t y = 0; y < CHUNK_SIZE; y++) {
           for(size_t z = 0; z < CHUNK_SIZE; z++) {
               const auto color_index = uint_distribution(gen);
               const auto cube_location = chunk_location * CHUNK_SIZE + glm::ivec3(x, y, z);
               const auto c = cube(cube_location, color_palette[color_index]);
               resultv.insert(resultv.end(), c.begin(), c.end());
           }
        }
    }
    return { resultv, resulti };
}
