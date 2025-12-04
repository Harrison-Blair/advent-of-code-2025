/** Notes:
 * 
 * - Dial from 0 - 99
 * - Dial starts @ 50
 * - Dial loops (like a dial on a safe)
 * - input.txt holds one turn per line (direction + amount)
 * 
 *  - "The actual password is the number of times the dial is left pointing at 0 after any rotation in the sequence."
 */

#include <fstream>
#include <iostream>

#include <queue>
#include <optional>
#include <functional>

using namespace std;

optional<queue<string>> loadInputs(string fileName = "input.txt") {
    queue<string> inputs;

    try { // Get inputs from file
        fstream fs(fileName, fstream::in);

        string input;
        while(getline(fs, input)) {
            inputs.push(input);
            cout << "[DEBUG] pushed '" << input << "' to input list" << endl;
        }
        cout << "[DEBUG] loaded inputs" << endl;
        fs.close();
    } catch (...) { // Catch errors
        cout << "[ERROR] Failed to read from " << fileName << endl;
        return nullopt;
    }

    return inputs;
}

int handleTurn(int dial, bool clockwise, int dist) {
    std::function<int(int, int)> op = clockwise 
        ? std::function<int(int, int)>(plus<int>())
        : std::function<int(int, int)>(minus<int>());

    int temp = op(dial, dist);

    while (temp < 0) {
        temp += 100;
    }
    while (temp > 99) {
        temp -= 100;
    }

    return temp;
}

int main() {
    auto maybeInputs = loadInputs();
    int password = 0;

    if (maybeInputs) {
        int dial = 50;
        queue<string> inputs = *maybeInputs;

        while(!inputs.empty()) {
            string input = inputs.front();
            inputs.pop();

            if (input.empty())
                break;
            
            cout << "[DEBUG] Dial @ " << dial << " | Processing " << input << endl;
            
            bool clockwise = input[0] == 'R';
            int dist = stoi(input.substr(1));

            dial = handleTurn(dial, clockwise, dist);

            if (!(0 <= dial <= 99))
                return -1;

            if (dial == 0) {
                cout << "[DEBUG] Dial at Zero!" << endl;
                password++;
            }
        }
        cout << "[RESULT] Password is : " << password << endl;
    } else {
        return 1;
    }

    return 0;
}
