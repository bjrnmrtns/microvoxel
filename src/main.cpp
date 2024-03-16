#include "fps.hpp"
#include "generator.hpp"
#include "transform.hpp"
#include "vertex.hpp"

#include <GL/glew.h>
#include <GLFW/glfw3.h>
#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/type_ptr.hpp>

#include <array>
#include <random>
#include <iostream>

void process_input(GLFWwindow *window, float &rotate_x, float& rotate_z, float& zoom) {
  if (glfwGetKey(window, GLFW_KEY_ESCAPE) == GLFW_PRESS)
    glfwSetWindowShouldClose(window, true);
  if (glfwGetKey(window, GLFW_KEY_A) == GLFW_PRESS)
    rotate_z -= 0.01;
  if (glfwGetKey(window, GLFW_KEY_D) == GLFW_PRESS)
    rotate_z += 0.01;
  if (glfwGetKey(window, GLFW_KEY_W) == GLFW_PRESS)
    rotate_x += 0.01;
  if (glfwGetKey(window, GLFW_KEY_S) == GLFW_PRESS)
    rotate_x -= 0.01;
  if (glfwGetKey(window, GLFW_KEY_Z) == GLFW_PRESS)
    zoom -= 1.0;
  if (glfwGetKey(window, GLFW_KEY_X) == GLFW_PRESS)
    zoom += 1.0;
}

void framebuffer_size_callback(GLFWwindow *window, int width, int height) {
  glViewport(0, 0, width, height);
}

constexpr const unsigned int WIDTH = 1920;
constexpr const unsigned int HEIGHT = 1200;

const char *vertexShaderSource =
    "#version 460 core\n"
    "layout (location = 0) in vec3 position;\n"
    "layout (location = 1) in vec3 normal;\n"
    "layout (location = 2) in vec4 color;\n"
    "uniform mat4 projection;\n"
    "uniform mat4 camera;\n"
    "uniform mat4 world;\n"
    "out vec3 pass_position;\n"
    "out vec3 pass_normal;\n"
    "out vec4 pass_color;\n"
    "void main()\n"
    "{\n"
    "   gl_Position = projection * camera * world * vec4(position.x, "
    "position.y, position.z, 1.0);\n"
    "   pass_position = position;\n"
    "   pass_normal = normal;\n"
    "   pass_color = color;\n"
    "}\0";
const char *fragmentShaderSource =
    "#version 460 core\n"
    "uniform sampler3D type_buffer;\n"
    "in vec3 pass_position;\n"
    "in vec3 pass_normal;\n"
    "in vec4 pass_color;\n"
    "out vec4 FragColor;\n"
    "const vec3 light_position = vec3(1.0, 1.0, 0.0);\n"
    "vec3 palette[8] = {\n"
    "    vec3(0.0, 0.0, 0.0),\n"
    "    vec3(0.0, 0.0, 1.0),\n"
    "    vec3(0.0, 1.0, 0.0),\n"
    "    vec3(0.0, 1.0, 1.0),\n"
    "    vec3(1.0, 0.0, 0.0),\n"
    "    vec3(1.0, 0.0, 1.0),\n"
    "    vec3(1.0, 1.0, 0.0),\n"
    "    vec3(1.0, 1.0, 1.0)\n"
    "};\n"
    "void main()\n"
    "{\n"
    "   uint x = uint(round(pass_position.x + 0.5 + 250));\n"
    "   uint y = uint(round(pass_position.y + 0.5 + 50));\n"
    "   uint z = uint(round(pass_position.z + 0.5 + 125));\n"
//    "   vec4 color = vec4(float(x % 10) * 0.1, float(y % 10) * 0.1, float(z % 10) * 0.1, 1.0f);\n"
    "   float index = texture(type_buffer, vec3(ivec3(x, y, z))).r;\n"
    "   vec4 color = vec4(palette[uint(index * 255.0)], 1.0f);\n"
    "   vec3 p = pass_position;\n"
    "   vec3 N = normalize(pass_normal.xyz);\n"
    "   vec3 L = normalize(light_position - p);\n"
    "   float lambert = max(0.0, dot(N, L));\n"
    "   FragColor = color;//pass_color;// * (lambert + 0.1);\n"
    "}\0";

struct drawcall_indirect {
  unsigned int count_indices;
  unsigned int count_instances;
  unsigned int first_index;
  unsigned int base_vertex;
  unsigned int base_instance;
  unsigned int *index;
};

int main() {
  glfwInit();
  glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
  glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 6);
  glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

  GLFWwindow *window =
      glfwCreateWindow(WIDTH, HEIGHT, "microvoxel", NULL, NULL);
  if (window == NULL) {
    std::cout << "Failed to create GLFW window" << std::endl;
    glfwTerminate();
    return -1;
  }
  glfwMakeContextCurrent(window);
  glfwSwapInterval(0); // uncomment for unlimited framerate
  glfwSetFramebufferSizeCallback(window, framebuffer_size_callback);

  GLenum err = glewInit();
  if (err != GLEW_OK) {
    fprintf(stderr, "GLEW initialization error: %s\n", glewGetErrorString(err));
    return -1;
  }
  // enable back face culling
  glEnable(GL_CULL_FACE);
  glCullFace(GL_BACK);
  glFrontFace(GL_CCW);

  // enable the z-buffer so order of pixels is correctly drawn
  glEnable(GL_DEPTH_TEST);

  // enable transparency
  glEnable(GL_BLEND);
  glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

  unsigned int vertexShader = glCreateShader(GL_VERTEX_SHADER);
  glShaderSource(vertexShader, 1, &vertexShaderSource, NULL);
  glCompileShader(vertexShader);
  int success;
  char infoLog[512];
  glGetShaderiv(vertexShader, GL_COMPILE_STATUS, &success);
  if (!success) {
    glGetShaderInfoLog(vertexShader, 512, NULL, infoLog);
    std::cout << "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n"
              << infoLog << std::endl;
  }

  unsigned int fragmentShader = glCreateShader(GL_FRAGMENT_SHADER);
  glShaderSource(fragmentShader, 1, &fragmentShaderSource, NULL);
  glCompileShader(fragmentShader);

  glGetShaderiv(fragmentShader, GL_COMPILE_STATUS, &success);
  if (!success) {
    glGetShaderInfoLog(fragmentShader, 512, NULL, infoLog);
    std::cout << "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n"
              << infoLog << std::endl;
  }

  unsigned int shaderProgram = glCreateProgram();
  glAttachShader(shaderProgram, vertexShader);
  glAttachShader(shaderProgram, fragmentShader);
  glLinkProgram(shaderProgram);

  glGetProgramiv(shaderProgram, GL_LINK_STATUS, &success);
  if (!success) {
    glGetProgramInfoLog(shaderProgram, 512, NULL, infoLog);
    std::cout << "ERROR::SHADER::PROGRAM::LINKING_FAILED\n"
              << infoLog << std::endl;
  }
  glDeleteShader(vertexShader);
  glDeleteShader(fragmentShader);

  const auto [vertices, indices] = build_lattice(); // build_chunk({0, 0, 0});
  std::cout << vertices.size() << " " << indices.size() << "\n";

  unsigned int vbo, vao, ebo;
  glGenVertexArrays(1, &vao);
  glGenBuffers(1, &vbo);
  glGenBuffers(1, &ebo);

  glBindVertexArray(vao);
  glBindBuffer(GL_ARRAY_BUFFER, vbo);
  glBufferData(GL_ARRAY_BUFFER, vertices.size() * sizeof(vertex),
               vertices.data(), GL_STATIC_DRAW);

  glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, ebo);
  glBufferData(GL_ELEMENT_ARRAY_BUFFER, sizeof(unsigned int) * indices.size(),
               indices.data(), GL_STATIC_DRAW);

  glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, sizeof(vertex), (void *)0);
  glEnableVertexAttribArray(0);
  glVertexAttribPointer(1, 3, GL_FLOAT, GL_FALSE, sizeof(vertex),
                        (void *)sizeof(glm::vec3));
  glEnableVertexAttribArray(1);
  glVertexAttribPointer(2, 4, GL_FLOAT, GL_FALSE, sizeof(vertex),
                        (void *)(sizeof(glm::vec3) + sizeof(glm::vec3)));
  glEnableVertexAttribArray(2);

  glBindVertexArray(0);

  const auto projection = glm::perspective(
      glm::radians(45.0f), (float)WIDTH / (float)HEIGHT, 1.0f, 10000.0f);

//  glPolygonMode(GL_FRONT_AND_BACK, GL_LINE);

  GLuint type_buffer_id;
  std::vector<std::byte> type_buffer(500 * 100 * 250);
  std::random_device rd;
  std::mt19937 gen(rd());
  std::uniform_int_distribution<unsigned int> uint_distribution(0, 7);
  for(auto& type_id: type_buffer) {
      type_id = (std::byte)uint_distribution(gen);
  }

  glGenTextures(1, &type_buffer_id);
  glBindTexture(GL_TEXTURE_3D, type_buffer_id);

  glTexStorage3D(GL_TEXTURE_3D, 1, GL_R8, 500, 100, 250);
  glTexSubImage3D(GL_TEXTURE_3D, 0, 0, 0, 0, 500, 100, 250, GL_RED, GL_UNSIGNED_BYTE, type_buffer.data());

  glBindTexture(GL_TEXTURE_3D, 0);

  glActiveTexture(GL_TEXTURE0);
  GLuint sampler_location = glGetUniformLocation(shaderProgram, "type_buffer");
  glUniform1i(sampler_location, 0);

  const auto projection_loc = glGetUniformLocation(shaderProgram, "projection");
  const auto camera_loc = glGetUniformLocation(shaderProgram, "camera");
  const auto world_loc = glGetUniformLocation(shaderProgram, "world");

  float rotation = 0.0f;
  fps fps;
  float rotate_x = 0.0f;
  float rotate_z = 0.0f;
  float zoom = 0.0f;
  while (!glfwWindowShouldClose(window)) {
  const auto camera =
      glm::lookAt(glm::vec3(0.0f, 6.0f + zoom, 0.0f), glm::vec3(0.0f, 0.0f, 0.0f),
                  glm::vec3(0.0f, 0.0f, 1.0f));
    const auto q_x_rot = glm::rotate(glm::mat4(1.0f), rotate_x, glm::vec3(1.0f, 0.0f, 0.0f));
    const auto q_z_rot = glm::rotate(glm::mat4(1.0f), rotate_z, glm::vec3(0.0f, 0.0f, 1.0f));
    auto world_transform = transform::from_rotation(q_x_rot * q_z_rot);
    fps.add_timepoint();
    std::cout << "fps: " << fps.value() << "\n";
    const auto world = glm::mat4(1.0f);
    rotation += 0.01f;
    process_input(window, rotate_x, rotate_z, zoom);
    glClearColor(0.0f, 0.0f, 0.0f, 0.0f);
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
    glUseProgram(shaderProgram);
    glUniformMatrix4fv(projection_loc, 1, GL_FALSE, glm::value_ptr(projection));
    glUniformMatrix4fv(camera_loc, 1, GL_FALSE, glm::value_ptr(camera));
    glUniformMatrix4fv(world_loc, 1, GL_FALSE,
                       glm::value_ptr(world_transform.to_matrix()));
    glBindVertexArray(vao);
    glActiveTexture(GL_TEXTURE0);
    glBindTexture(GL_TEXTURE_3D, type_buffer_id);
    glDrawElements(GL_TRIANGLES, indices.size(), GL_UNSIGNED_INT, 0);
    glBindVertexArray(0);
    //        glMultiDrawElementsIndirect(GL_TRIANGLES, GL_UNSIGNED_INT, 0,
    //        length, sizeof_DAIC);
    glfwSwapBuffers(window);
    glfwPollEvents();
  }

  glDeleteVertexArrays(1, &vao);
  glDeleteBuffers(1, &vbo);
  glDeleteBuffers(1, &ebo);
  glDeleteProgram(shaderProgram);

  glfwTerminate();
  return 0;
}
