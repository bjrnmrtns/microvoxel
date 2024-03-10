#include "generator.hpp"
#include "vertex.hpp"

#include <glm/glm.hpp>
#include <vector>
#include <tuple>

static constexpr const glm::vec3 top(0.0f, 1.0f, 0.0f);
static constexpr const glm::vec3 bottom(0.0f, -1.0f, 0.0f);
static constexpr const glm::vec3 left(-1.0f, 0.0f, 0.0f);
static constexpr const glm::vec3 right(1.0f, 0.0f, 0.0f);
static constexpr const glm::vec3 back(0.0f, 0.0f, -1.0f);
static constexpr const glm::vec3 front(0.0f, 0.0f, 1.0f);

static glm::vec3 cbv(const size_t index, float offset_x, float offset_y, float offset_z)
{
    static glm::vec3 table[] {
    glm::vec3(offset_x + 0.0f, offset_y + 0.0f, offset_z + 0.0f),
    glm::vec3(offset_x + 1.0f, offset_y + 0.0f, offset_z + 0.0f),
    glm::vec3(offset_x + 0.0f, offset_y + 1.0f, offset_z + 0.0f),
    glm::vec3(offset_x + 1.0f, offset_y + 1.0f, offset_z + 0.0f),
    glm::vec3(offset_x + 0.0f, offset_y + 0.0f, offset_z + 1.0f),
    glm::vec3(offset_x + 1.0f, offset_y + 0.0f, offset_z + 1.0f),
    glm::vec3(offset_x + 0.0f, offset_y + 1.0f, offset_z + 1.0f),
    glm::vec3(offset_x + 1.0f, offset_y + 1.0f, offset_z + 1.0f) };
    return table[index];
};

std::vector<unsigned int> cube_indices()
{
    return { 0, 1, 2, 1, 3, 2 };
}

std::vector<vertex> cube_front()
{
    auto normal = front;
    return { 
               { .p = cbv(0, 0.0f, 0.0f, 0.0f), 
                 .n = normal, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
               { .p = cbv(1, 0.0f, 0.0f, 0.0f), 
                 .n = normal, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
               { .p = cbv(2, 0.0f, 0.0f, 0.0f), 
                 .n = normal, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
               { .p = cbv(3, 0.0f, 0.0f, 0.0f), 
                 .n = normal, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
           };
}

std::vector<vertex> cube_back()
{
    auto normal = back;
    return { 
               { .p = cbv(5, 0.0f, 0.0f, 0.0f), 
                 .n = normal, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
               { .p = cbv(4, 0.0f, 0.0f, 0.0f), 
                 .n = normal, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
               { .p = cbv(7, 0.0f, 0.0f, 0.0f), 
                 .n = normal, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
               { .p = cbv(6, 0.0f, 0.0f, 0.0f), 
                 .n = normal, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
           };
}

std::vector<vertex> cube_left()
{
    auto normal = left;
    return { 
               { .p = cbv(4, 0.0f, 0.0f, 0.0f), 
                 .n = normal, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
               { .p = cbv(0, 0.0f, 0.0f, 0.0f), 
                 .n = normal, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
               { .p = cbv(6, 0.0f, 0.0f, 0.0f), 
                 .n = normal, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
               { .p = cbv(2, 0.0f, 0.0f, 0.0f), 
                 .n = normal, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
           };
}

std::vector<vertex> cube_right()
{
    auto normal = right;
    return { 
               { .p = cbv(1, 0.0f, 0.0f, 0.0f), 
                 .n = normal, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
               { .p = cbv(5, 0.0f, 0.0f, 0.0f), 
                 .n = normal, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
               { .p = cbv(3, 0.0f, 0.0f, 0.0f), 
                 .n = normal, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
               { .p = cbv(7, 0.0f, 0.0f, 0.0f), 
                 .n = normal, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
           };
}

std::vector<vertex> cube_top()
{
    auto normal = top;
    return { 
               { .p = cbv(2, 0.0f, 0.0f, 0.0f), 
                 .n = normal, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
               { .p = cbv(3, 0.0f, 0.0f, 0.0f), 
                 .n = normal, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
               { .p = cbv(6, 0.0f, 0.0f, 0.0f), 
                 .n = normal, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
               { .p = cbv(7, 0.0f, 0.0f, 0.0f), 
                 .n = normal, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
           };
}

std::vector<vertex> cube_bottom()
{
    auto normal = bottom;
    return { 
               { .p = cbv(4, 0.0f, 0.0f, 0.0f), 
                 .n = normal, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
               { .p = cbv(5, 0.0f, 0.0f, 0.0f), 
                 .n = normal, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
               { .p = cbv(0, 0.0f, 0.0f, 0.0f), 
                 .n = normal, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
               { .p = cbv(1, 0.0f, 0.0f, 0.0f), 
                 .n = normal, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
           };
}

static std::vector<unsigned int> offset_indices(const std::vector<unsigned int>& indices, unsigned int offset) 
{
    std::vector<unsigned int> result = indices;
    for(auto& i: result) {
        i += offset;
    }
    return result;
}

std::tuple<std::vector<vertex>, std::vector<unsigned int>> cube()
{
    std::vector<vertex> resultv;
    std::vector<unsigned int>  resulti;
    const auto cbottom = cube_bottom();
    const auto ctop = cube_top();
    const auto cleft = cube_left();
    const auto cright = cube_right();
    const auto cfront = cube_front();
    const auto cback = cube_back();
    auto indices = cube_indices();
    resultv.insert(resultv.end(), cbottom.begin(), cbottom.end());
    resulti.insert(resulti.end(), indices.begin(), indices.end());
    resultv.insert(resultv.end(), ctop.begin(), ctop.end());
    indices = offset_indices(indices, 4);
    resulti.insert(resulti.end(), indices.begin(), indices.end());
    resultv.insert(resultv.end(), cleft.begin(), cleft.end());
    indices = offset_indices(indices, 4);
    resulti.insert(resulti.end(), indices.begin(), indices.end());
    resultv.insert(resultv.end(), cright.begin(), cright.end());
    indices = offset_indices(indices, 4);
    resulti.insert(resulti.end(), indices.begin(), indices.end());
    resultv.insert(resultv.end(), cfront.begin(), cfront.end());
    indices = offset_indices(indices, 4);
    resulti.insert(resulti.end(), indices.begin(), indices.end());
    resultv.insert(resultv.end(), cback.begin(), cback.end());
    indices = offset_indices(indices, 4);
    resulti.insert(resulti.end(), indices.begin(), indices.end());

    return { resultv, resulti };
}
