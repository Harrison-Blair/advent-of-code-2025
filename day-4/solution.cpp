#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

vector<vector<char>> readFile(string fileName = "example.txt") {
  vector<vector<char>> grid;

  fstream fs(fileName, fstream::in);

  if (!fs.is_open()) {
    cout << "[ERROR] Failed to open file: " << fileName << endl;
    return grid;
  }

  string input;
  while (getline(fs, input)) {
    vector<char> line;
    for (char c : input) {
      line.push_back(c);
    }
    grid.push_back(line);
  }

  fs.close();
  return grid;
}

vector<pair<int, int>> getRollLocations(vector<vector<char>> grid) {
  vector<pair<int, int>> rolls;
  for (int i = 0; i < grid.size(); i++) {
    for (int j = 0; j < grid[i].size(); j++) {
      if (grid[i][j] == '@') rolls.push_back(pair(i, j));  // y, x
    }
  }
  return rolls;
}

bool isRemovable(vector<vector<char>> grid, pair<int, int> roll) {
  int numNeighbors = 0;
  // Check all 8 possible directions
  for (int di = -1; di <= 1; di++) {
    for (int dj = -1; dj <= 1; dj++) {
      if (di == 0 && dj == 0) continue;  // Skip self
      int ni = roll.first + di;
      int nj = roll.second + dj;
      if (ni >= 0 && ni < grid.size() && nj >= 0 && nj < grid[ni].size() &&
          grid[ni][nj] == '@') {
        numNeighbors++;
      }
    }
  }
  if (numNeighbors < 4) {
    return true;
  }
  return false;
}

vector<pair<int, int>> getRemovableRolls(vector<vector<char>> grid,
                                         vector<pair<int, int>> rolls) {
  vector<pair<int, int>> removableRolls;
  for (pair<int, int>& roll : rolls) {
    if (isRemovable(grid, roll)) removableRolls.push_back(roll);
  }
  return removableRolls;
}

vector<vector<char>> removeRolls(vector<vector<char>> grid,
                                 vector<pair<int, int>> removableRolls,
                                 int* total) {
  vector<vector<char>> newGrid = grid;

  for (pair<int, int>& roll : removableRolls) {
    newGrid[roll.first][roll.second] = '.';
    (*total)++;
  }

  return newGrid;
}

int partTwo(vector<vector<char>> grid) {
  int total = 0;
  vector<vector<char>> newGrid = grid;
  vector<pair<int, int>> rolls, removableRolls;

  do {
    rolls = getRollLocations(newGrid);
    removableRolls = getRemovableRolls(newGrid, rolls);

    newGrid = removeRolls(newGrid, removableRolls, &total);

  } while (!removableRolls.empty());

  return total;
}

int partOne(vector<vector<char>> grid) {
  int total = 0;
  vector<pair<int, int>> rolls = getRollLocations(grid);

  for (pair<int, int>& roll : rolls) {
    if (isRemovable(grid, roll)) total++;
  }
  return total;
}

int main() {
  string fileName = "input.txt";

  vector<vector<char>> grid = readFile(fileName);

  cout << partOne(grid) << endl;
  cout << partTwo(grid) << endl;

  return 0;
}