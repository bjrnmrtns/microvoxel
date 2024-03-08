cmake_minimum_required(VERSION 3.27)
project(microvoxel)

set(CMAKE_CXX_STANDARD 20)

find_package(OpenGL REQUIRED)
find_package(glfw3 REQUIRED)
find_package(GLEW REQUIRED)
find_package(glm REQUIRED)

add_executable(${PROJECT_NAME}
    src/main.cpp
)

target_link_libraries(${PROJECT_NAME}
    ${OPENGL_LIBRARIES}
    glfw 
    GLEW::GLEW
    glm::glm
)
