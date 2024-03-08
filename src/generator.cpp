#include "generator.hpp"
#include "vertex.hpp"

#include <glm/glm.hpp>
#include <vector>

static constexpr const glm::vec3 up(0.0f, 1.0f, 0.0f);
static constexpr const glm::vec3 down(0.0f, -1.0f, 0.0f);
static constexpr const glm::vec3 left(-1.0f, 0.0f, 0.0f);
static constexpr const glm::vec3 right(1.0f, 0.0f, 0.0f);
static constexpr const glm::vec3 back(0.0f, 0.0f, -1.0f);
static constexpr const glm::vec3 front(0.0f, 0.0f, 1.0f);

static constexpr const glm::vec3 cube[] {
    glm::vec3(0.0f, 0.0f, 0.0f),
    glm::vec3(1.0f, 0.0f, 0.0f),
    glm::vec3(0.0f, 1.0f, 0.0f),
    glm::vec3(1.0f, 1.0f, 0.0f),
    glm::vec3(0.0f, 0.0f, 1.0f),
    glm::vec3(1.0f, 0.0f, 1.0f),
    glm::vec3(0.0f, 1.0f, 1.0f),
    glm::vec3(1.0f, 1.0f, 1.0f),
};

static constexpr unsigned int cube_indices[] {
    0, 1, 2, 1, 3, 2,
};

std::vector<vertex> create_x_min()
{
    return { 
               { .p = { 0.0f, 0.0f, 0.0f }, 
                 .n = { 0.0f, 1.0f, 0.0f }, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
               { .p = { 5.0f, 0.0f, 0.0f }, 
                 .n = { 0.0f, 1.0f, 0.0f }, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
               { .p = { 5.0f, 0.0f, -5.0f }, 
                 .n = { 0.0f, 1.0f, 0.0f }, 
                 .c = { 1.0f, 0.0f, 0.0f, 1.0f },
               },
           };
}

