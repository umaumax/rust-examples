#include <chrono>
#include <cstdlib>
#include <iostream>

#include <pthread.h>

int main(int argc, char* argv[]) {
  {
    std::chrono::system_clock::time_point start =
        std::chrono::system_clock::now();

    for (int i = 0; i < 1000000; i++) {
      char* _ = getenv("DISPLAY");
    }
    double elapsed = std::chrono::duration_cast<std::chrono::nanoseconds>(
                         std::chrono::system_clock::now() - start)
                         .count();
    std::cout << elapsed / 1000.0 / 1000.0 / 1000.0 << "sec" << std::endl;
  }

  {
    pthread_mutex_t getenv_lock;
    std::chrono::system_clock::time_point start =
        std::chrono::system_clock::now();

    for (int i = 0; i < 1000000; i++) {
      pthread_mutex_lock(&getenv_lock);
      char* _ = getenv("DISPLAY");
      pthread_mutex_unlock(&getenv_lock);
    }
    double elapsed = std::chrono::duration_cast<std::chrono::nanoseconds>(
                         std::chrono::system_clock::now() - start)
                         .count();
    std::cout << elapsed / 1000.0 / 1000.0 / 1000.0 << "sec" << std::endl;
  }

  return 0;
}
