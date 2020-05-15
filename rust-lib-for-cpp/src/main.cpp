#include <iostream>

#include "example.hpp"

int main(int argc, char const* argv[]) {
  int input  = 10;
  int output = double_input(input);
  std::cout << input << " * 2 = " << output << std::endl;

  return 0;
}
