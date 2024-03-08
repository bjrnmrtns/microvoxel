#include "generator.hpp"
#include "vertex.hpp"

#include <GL/glew.h>
#include <GLFW/glfw3.h>
#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/type_ptr.hpp>

#include <vector>
#include <iostream>

void process_input(GLFWwindow *window)
{
    if (glfwGetKey(window, GLFW_KEY_ESCAPE) == GLFW_PRESS)
        glfwSetWindowShouldClose(window, true);
}

void framebuffer_size_callback(GLFWwindow* window, int width, int height)
{
    glViewport(0, 0, width, height);
}

constexpr const unsigned int WIDTH = 800;
constexpr const unsigned int HEIGHT = 600;

const char *vertexShaderSource = "#version 460 core\n"
    "layout (location = 0) in vec3 position;\n"
    "layout (location = 1) in vec3 normal;\n"
    "layout (location = 2) in vec4 color;\n"
    "uniform mat4 projection;\n"
    "uniform mat4 camera;\n"
    "out vec3 pass_position;\n"
    "out vec3 pass_normal;\n"
    "out vec4 pass_color;\n"
    "void main()\n"
    "{\n"
    "   gl_Position = projection * camera * vec4(position.x, position.y, position.z, 1.0);\n"
    "   pass_position = position;\n"
    "   pass_normal = normal;\n"
    "   pass_color = color;\n"
    "}\0";
const char *fragmentShaderSource = "#version 460 core\n"
    "in vec3 pass_position;\n"
    "in vec3 pass_normal;\n"
    "in vec4 pass_color;\n"
    "out vec4 FragColor;\n"
    "const vec3 light_position = vec3(1.0, 1.0, 0.0);\n"
    "void main()\n"
    "{\n"
    "   vec3 p = pass_position;\n"
    "   vec3 N = normalize(pass_normal.xyz);\n"
    "   vec3 L = normalize(light_position - p);\n"
    "   float lambert = max(0.0, dot(N, L));\n"
    "   FragColor = pass_color * (lambert + 0.1);\n"
    "}\0";

struct drawcall_indirect
{
    unsigned int count_indices;
    unsigned int count_instances;
    unsigned int first_index;
    unsigned int base_vertex;
    unsigned int base_instance;
    unsigned int* index;
};

std::vector<vertex> create_chunk()
{
    return cube_top();
}

int main()
{
    glfwInit();
    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 6);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

    GLFWwindow* window = glfwCreateWindow(WIDTH, HEIGHT, "microvoxel", NULL, NULL);
    if (window == NULL)
    {
        std::cout << "Failed to create GLFW window" << std::endl;
        glfwTerminate();
        return -1;
    }
    glfwMakeContextCurrent(window);
    glfwSetFramebufferSizeCallback(window, framebuffer_size_callback);

    GLenum err = glewInit();
    if (err != GLEW_OK) {
        fprintf(stderr, "GLEW initialization error: %s\n", glewGetErrorString(err));
        return -1;
    }

    unsigned int vertexShader = glCreateShader(GL_VERTEX_SHADER);
    glShaderSource(vertexShader, 1, &vertexShaderSource, NULL);
    glCompileShader(vertexShader);
    int success;
    char infoLog[512];
    glGetShaderiv(vertexShader, GL_COMPILE_STATUS, &success);
    if (!success)
    {
        glGetShaderInfoLog(vertexShader, 512, NULL, infoLog);
        std::cout << "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n" << infoLog << std::endl;
    }

    unsigned int fragmentShader = glCreateShader(GL_FRAGMENT_SHADER);
    glShaderSource(fragmentShader, 1, &fragmentShaderSource, NULL);
    glCompileShader(fragmentShader);

    glGetShaderiv(fragmentShader, GL_COMPILE_STATUS, &success);
    if (!success)
    {
        glGetShaderInfoLog(fragmentShader, 512, NULL, infoLog);
        std::cout << "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n" << infoLog << std::endl;
    }

    unsigned int shaderProgram = glCreateProgram();
    glAttachShader(shaderProgram, vertexShader);
    glAttachShader(shaderProgram, fragmentShader);
    glLinkProgram(shaderProgram);

    glGetProgramiv(shaderProgram, GL_LINK_STATUS, &success);
    if (!success) {
        glGetProgramInfoLog(shaderProgram, 512, NULL, infoLog);
        std::cout << "ERROR::SHADER::PROGRAM::LINKING_FAILED\n" << infoLog << std::endl;
    }
    glDeleteShader(vertexShader);
    glDeleteShader(fragmentShader);

    std::vector<vertex> vertices = create_chunk();

    unsigned int vbo, vao;
    glGenVertexArrays(1, &vao);
    glGenBuffers(1, &vbo);
    glBindVertexArray(vao);

    glBindBuffer(GL_ARRAY_BUFFER, vbo);
    glBufferData(GL_ARRAY_BUFFER, vertices.size() * sizeof(vertex), vertices.data(), GL_STATIC_DRAW);

    glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, sizeof(vertex), (void*)0);
    glEnableVertexAttribArray(0);
    glVertexAttribPointer(1, 3, GL_FLOAT, GL_FALSE, sizeof(vertex), (void*)sizeof(glm::vec3));
    glEnableVertexAttribArray(1);
    glVertexAttribPointer(2, 4, GL_FLOAT, GL_FALSE, sizeof(vertex), (void*)(sizeof(glm::vec3) + sizeof(glm::vec3)));
    glEnableVertexAttribArray(2);

    glBindBuffer(GL_ARRAY_BUFFER, 0); 

    glBindVertexArray(0); 

    const auto projection = glm::perspective(60.0f, (float)WIDTH / (float)HEIGHT, 1.0f, 100.0f);
    const auto camera = glm::lookAt(glm::vec3(0.0f, 2.0f, 0.0f), glm::vec3(0.0f, 0.0f, 0.0f), glm::vec3(0.0f, 0.0f, 1.0f)); 
    //glPolygonMode(GL_FRONT_AND_BACK, GL_LINE);

    const auto projection_loc = glGetUniformLocation(shaderProgram, "projection");
    const auto camera_loc = glGetUniformLocation(shaderProgram, "camera");

    while (!glfwWindowShouldClose(window))
    {
        process_input(window);
        glClearColor(0.2f, 0.3f, 0.3f, 1.0f);
        glClear(GL_COLOR_BUFFER_BIT);
        glUseProgram(shaderProgram);
        glUniformMatrix4fv(projection_loc, 1, GL_FALSE, glm::value_ptr(projection));
        glUniformMatrix4fv(camera_loc, 1, GL_FALSE, glm::value_ptr(camera));
        glBindVertexArray(vao);
        glDrawArrays(GL_TRIANGLES, 0, 3);
//        glMultiDrawElementsIndirect(GL_TRIANGLES, GL_UNSIGNED_INT, 0, length, sizeof_DAIC);
        glfwSwapBuffers(window);
        glfwPollEvents();
    }

    glDeleteVertexArrays(1, &vao);
    glDeleteBuffers(1, &vbo);
    glDeleteProgram(shaderProgram);

    glfwTerminate();
    return 0;
}

