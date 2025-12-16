#include <fstream>
#include <iostream>

#include <algorithm>

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

pair<vector<pair<ulong, ulong>>, vector<ulong>> readInput(string fileName) {
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
        total++;
        break;
      }
    }
  }

  return total;
}

ulong partTwo(vector<pair<ulong, ulong>> codeRanges) {
  ulong total = 0;
  vector<pair<ulong, ulong>> merged;
  sort(codeRanges.begin(), codeRanges.end());

  merged.push_back(codeRanges[0]);
  for (int i = 1; i < codeRanges.size(); i++) {
    pair<ulong, ulong>& last = merged.back(), cur = codeRanges[i];

    if (cur.first <= last.second + 1)
      last.second = max(last.second, cur.second);
    else
      merged.push_back(cur);
  }

  for (auto& range : merged)
    total += range.second - range.first + 1;

  return total;
}

int main() {
  string input = "input.txt";

  auto [validCodes, codes] = readInput(input);

  cout << "[RESULT] " << partOne(validCodes, codes) << endl;
  cout << "[RESULT] " << partTwo(validCodes) << endl;

  return 0;
}