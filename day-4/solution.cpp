#include <string>

#include <fstream>
#include <iostream>
#include <list>

using namespace std;

list<list<char>> readFile(string fileName = "example.txt") {
    list<list<char>> grid;

    fstream fs(fileName, fstream::in);
    
    if (!fs.is_open()) {
        cout << "[ERROR] Failed to open file: " << fileName << endl;
        return grid;
    }

    string input;
    while (getline(fs, input)) {
        list<char> line;
        for (char c : input) {
            line.push_back(c);
        }
        grid.push_back(line);
    }
    
    fs.close();
    return grid;
}

int main() {
    string fileName = "input.txt"; //presupposes ./

    list<list<char>> grid = readFile(fileName);

    return 0;
}