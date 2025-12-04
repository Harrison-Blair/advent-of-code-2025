#include <fstream>
#include <iostream>

#include <queue>
#include <optional>
#include <functional>

using namespace std;

bool debug = true;

optional<queue<string>> loadInputs(string fileName = "input.txt")
{
    queue<string> inputs;

    try
    { // Get inputs from file
        fstream fs(fileName, fstream::in);

        string input;
        while (getline(fs, input))
        {
            inputs.push(input);
            if (debug)
                cout << "[DEBUG] pushed '" << input << "' to input list" << endl;
        }
        if (debug)
            cout << "[DEBUG] loaded inputs" << endl;
        fs.close();
    }
    catch (...)
    { // Catch errors
        cout << "[ERROR] Failed to read from " << fileName << endl;
        return nullopt;
    }

    return inputs;
}

tuple<bool, int> handleInput(string input)
{
    bool clockwise = input[0] == 'R';
    int dist = stoi(input.substr(1));

    return make_tuple(clockwise, dist);
}

int handleTurn(int dial, bool clockwise, int dist)
{
    std::function<int(int, int)> op = clockwise
                                          ? std::function<int(int, int)>(plus<int>())
                                          : std::function<int(int, int)>(minus<int>());

    int temp = op(dial, dist);

    while (temp < 0)
    {
        temp += 100;
    }
    while (temp > 99)
    {
        temp -= 100;
    }

    return temp;
}

int handleInterTurn(int dial, bool clockwise, int dist, int &password)
{
    std::function<int(int, int)> op = clockwise
                                          ? std::function<int(int, int)>(plus<int>())
                                          : std::function<int(int, int)>(minus<int>());

    int temp = op(dial, dist);
    
    while (temp < 0)
    {
        temp += 100;
        password++;
        cout << "[DEBUG] Crossed Zero! | Password: " << password << endl;
    }
    while (temp > 99)
    {
        temp -= 100;
        password++;
        cout << "[DEBUG] Crossed Zero! | Password: " << password << endl;
    }

    return temp;
}

void partOne(queue<string> inputs)
{
    int password = 0;
    int dial = 50;

    cout << "---[PART1]---" << endl;

    while (!inputs.empty())
    {
        bool clockwise;
        int dist;

        if (inputs.front().empty())
            break;

        if (debug)
            cout << "[DEBUG] Dial @ " << dial << " | Processing " << inputs.front() << endl;

        tie(clockwise, dist) = handleInput(inputs.front());
        inputs.pop();

        dial = handleTurn(dial, clockwise, dist);

        if (!(0 <= dial <= 99))
            return;

        if (dial == 0)
        {
            password++;
            if (debug)
                cout << "[DEBUG] Dial at Zero! | Password: " << password << endl;
        }
    }
    cout << "[RESULT] Password is : " << password << endl;
    return;
}

void partTwo(queue<string> inputs)
{
    int password = 0;
    int dial = 50;

    cout << "---[PART2]---" << endl;

    while (!inputs.empty())
    {
        bool clockwise;
        int dist;

        if (inputs.front().empty())
            break;

        if (debug)
            cout << "[DEBUG] Dial @ " << dial << " | Processing " << inputs.front() << endl;

        tie(clockwise, dist) = handleInput(inputs.front());
        inputs.pop();

        if (dial == 0 && !clockwise)
            password--;
        
        dial = handleInterTurn(dial, clockwise, dist, password);

        if (dial == 0 && !clockwise) {
            password++;
            cout << "[DEBUG] Ended at Zero | Password : " << password << endl;
        }
    }
    cout << "[RESULT] Password is : " << password << endl
         << endl;
    return;
}

int main()
{
    auto maybeInputs = loadInputs("example.txt");

    partOne(*maybeInputs);

    partTwo(*maybeInputs);

    return 0;
}
