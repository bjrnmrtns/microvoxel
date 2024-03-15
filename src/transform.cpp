#include "transform.hpp"

transform::transform(const glm::vec3 &translation, const glm::quat &rotation,
                     const glm::vec3 &scale)
    : translation(translation), rotation(rotation), scale(scale) {}

transform transform::from_identity() {
  return transform(glm::zero<glm::vec3>(), glm::identity<glm::quat>(),
                   glm::one<glm::vec3>());
}

transform transform::from_translation(const glm::vec3 &translation) {
  return transform(translation, glm::identity<glm::quat>(),
                   glm::one<glm::vec3>());
}

transform transform::from_rotation(const glm::quat &rotation) {
  return transform(glm::zero<glm::vec3>(), rotation, glm::one<glm::vec3>());
}

transform transform::from_translation_rotation(const glm::vec3 &translation,
                                               const glm::quat &rotation) {
  return transform(translation, rotation, glm::one<glm::vec3>());
}

transform
transform::from_translation_rotation_scale(const glm::vec3 &translation,
                                           const glm::quat &rotation,
                                           const glm::vec3 &scale) {
  return transform(translation, rotation, scale);
}

glm::vec3 transform::mul_vec3(const glm::vec3 &value) {
  glm::vec3 result = rotation * value;
  result = scale * result;
  result += translation;
  return result;
}

transform transform::mul_transform(const transform &other) {
  return transform(mul_vec3(other.translation), rotation * other.rotation,
                   scale * other.scale);
}

glm::vec3 transform::forward() {
  return rotation * glm::vec3(0.0, 0.0, 1.0f); // forward in z direction
}

glm::mat4 transform::to_matrix() {
  return glm::translate(glm::mat4(1.0f), translation) *
         glm::mat4_cast(rotation) * glm::scale(glm::mat4(1.0f), scale);
}
