#include <fstream>
#include <iostream>
#include <string>
using namespace std;

int main(int argc, char *argv[]) {

  ifstream file("inputs/d4");
  string buf;

  if (file.is_open()) {
    while (file >> buf) {
      cout << buf << endl;
    }
  }

  return 0;
}
