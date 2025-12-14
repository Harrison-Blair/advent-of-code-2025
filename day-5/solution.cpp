#include <fstream>
#include <iostream>
#include <unordered_set>
#include <utility>
#include <vector>

using namespace std;

pair<ulong, ulong> convertRange(string sRange) {
  int dPos = 0;
  for (int i = 0; i < sRange.length(); i++) {
    if (sRange[i] == '-') {
      dPos = i;
      break;
    }
  }

  auto n1 = stoul(sRange.substr(0, dPos));
  auto n2 = stoul(sRange.substr(dPos + 1));

  return pair(n1, n2);
}

pair<vector<pair<ulong, ulong>> , vector<ulong>> readInput(string fileName) {
  vector<pair<ulong, ulong>> codeRanges;
  vector<ulong> codes;

  fstream fs(fileName, fstream::in);

  if (!fs.is_open()) {
    cout << "[ERROR] Error reading from file." << endl;
    return pair(codeRanges, codes);
  }

  string input;
  bool isRange = true;
  while (getline(fs, input)) {
    if (input.empty()) {
      isRange = false;
      continue;
    }
    if (isRange) {
      codeRanges.push_back(convertRange(input));
    } else {
      codes.push_back(stoul(input));
    }
  }
  fs.close();

  return pair(codeRanges, codes);
}

int partOne(vector<pair<ulong, ulong>> codeRanges, vector<ulong> codes) {
  int total = 0;

  for (auto& code : codes) {
    for (auto& range : codeRanges) {
      if (range.first <= code && code <= range.second) {
        cout << range.first << "\t<\t" << code << "\t<\t" << range.second << endl;
        total++;
        break;
      }
    }
  }

  return total;
}

int main() {
  string input = "input.txt";

  auto [validCodes, codes] = readInput(input);

  cout << "[RESULT] " << partOne(validCodes, codes) << endl;

  return 0;
}