cmake -E make_directory bin/debug &&
cmake -E chdir bin/debug cmake -DCMAKE_BUILD_TYPE=Debug -G Ninja ../.. &&
cmake -E chdir bin/debug cmake --build .
