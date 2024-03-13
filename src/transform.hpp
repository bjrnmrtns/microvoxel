#pragma once

#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/quaternion.hpp>

struct transform {
  transform() = delete;
  transform(const glm::vec3 &translation, const glm::quat &rotation,
            const glm::vec3 &scale);

  static transform from_identity();
  static transform from_translation(const glm::vec3 &translation);
  static transform from_rotation(const glm::quat &rotation);
  static transform from_translation_rotation(const glm::vec3 &translation,
                                             const glm::quat &rotation);
  static transform from_translation_rotation_scale(const glm::vec3 &translation,
                                                   const glm::quat &rotation,
                                                   const glm::vec3 &scale);
  glm::vec3 mul_vec3(const glm::vec3 &value);
  transform mul_transform(const transform &other);
  glm::vec3 forward();
  glm::mat4 to_matrix();

  glm::vec3 translation;
  glm::quat rotation;
  glm::vec3 scale;
};
