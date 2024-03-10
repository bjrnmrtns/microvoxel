cmake -E make_directory bin/release &&
cmake -E chdir bin/release cmake -DCMAKE_BUILD_TYPE=Release -G Ninja ../.. &&
cmake -E chdir bin/release cmake --build .
