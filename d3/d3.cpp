#include <algorithm>
#include <fstream>
#include <iostream>
#include <iterator>
#include <set>
#include <string>
#include <vector>

using namespace std;

vector<string> parse_input(string path);

void day_one();
void day_two();

int main(int argc, char *argv[]) {
  // day_one();
  day_two();

  return 0;
}

void day_one() {
  vector<string> rucksack_items;
  int sum = 0;

  try {
    rucksack_items = parse_input("inputs/d3");
  } catch (const char *x) {
    cout << x;
  }
  // rucksack_items.push_back("hellow_world!");

  //==========Solution==========
  // a-z = [97-122]
  // A-Z = [65-90]

  // 1. split each string in middle

  // 2. compare which character is repeating in both strings (could use sets
  // for this)
  // 3. use ascii values to compute the priority and add it to sum

  for (const string &s : rucksack_items) {
    set<char> chars_left;
    set<char> chars_right;
    char *common = new char();

    for (char c : s.substr(0, s.size() / 2)) {
      chars_left.insert(c);
    }

    for (char c : s.substr(s.size() / 2)) {
      chars_right.insert(c);
    }

    set_intersection(chars_left.begin(), chars_left.end(), chars_right.begin(),
                     chars_right.end(), common);

    sum += (*common > 96 && *common < 123) ? *common - 96 : *common - 38;
  }

  cout << "[Day 3.1] \x1b[32m" << sum << "\x1b[0m" << endl;
}

void day_two() {
  vector<string> lines;
  int sum = 0;

  try {
    lines = parse_input("inputs/d3");

  } catch (char *c) {
    cout << c << endl;
  }

  for (int i = 0; i < lines.size(); i += 3) {

    int freq1[52] = {0};
    int freq2[52] = {0};
    int freq3[52] = {0};

    for (char c : lines[i]) {
      size_t x = (c >= 'A' && c <= 'Z') ? (c - 'A') + 26 : (c - 'a');
      freq1[x]++;
    }

    for (char c : lines[i + 1]) {
      size_t x = (c >= 'A' && c <= 'Z') ? (c - 'A') + 26 : (c - 'a');
      freq2[x]++;
    }

    for (char c : lines[i + 2]) {
      size_t x = (c >= 'A' && c <= 'Z') ? (c - 'A') + 26 : (c - 'a');
      freq3[x]++;
    }

    for (int j = 0; j < 52; j++) {
      int tmp = min(freq1[j], freq2[j]);

      if (min(tmp, freq3[j]) > 0) {
        sum += (j + 1);
        break;
      }
    }
  }

  cout << sum << endl;
}

/**
 * Read a file line-by-line, pushing each line to a vector and returning it.
 */
vector<string> parse_input(string path) {
  ifstream file(path);
  vector<string> v;
  string buf;

  if (file.is_open()) {

    while (file >> buf) {
      v.push_back(buf);
    }
  } else {
    throw "could not open file!\n"; // this is not best practice but works for
                                    // this simple problem
  }

  return v;
}
