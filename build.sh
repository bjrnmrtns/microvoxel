cmake -E make_directory bin &&
cmake -E chdir bin cmake -G Ninja .. &&
cmake -E chdir bin cmake --build .
