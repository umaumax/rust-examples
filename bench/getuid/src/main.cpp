#include <sys/types.h>
#include <unistd.h>
#include <chrono>
#include <cstdlib>
#include <iostream>

int main(int argc, char* argv[]) {
  {
    std::chrono::system_clock::time_point start =
        std::chrono::system_clock::now();

    for (int i = 0; i < 1000000; i++) {
      uid_t uid = getuid();
    }
    double elapsed = std::chrono::duration_cast<std::chrono::nanoseconds>(
                         std::chrono::system_clock::now() - start)
                         .count();
    std::cout << elapsed / 1000.0 / 1000.0 / 1000.0 << "sec" << std::endl;
  }
  return 0;
}
