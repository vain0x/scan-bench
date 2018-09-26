#include <climits>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

int main(int argc, char **args)
{
    auto large = argc >= 2 && string{args[1]} == "--large";
    cerr << "Generate " << (large ? "large" : "small") << " input." << endl;

    auto a = 1, b = 2;
    auto s = string{"test"};
    auto K = 3;
    auto X = vector<int>{(int)-1e9, 0, (int)1e9};
    auto H = 3;
    auto W = 11;
    auto board = vector<string>{};
    auto N = (int)1e5;
    auto M = (large ? (int)2e5 : 10);

    board.push_back(string{"#.#.#.#.#.#"});
    board.push_back(string{".#.#.#.#.#."});
    board.push_back(string{"#.#.#.#.#.#"});

    // checksum
    auto c = (a ^ b ^ s[0] ^ X[2] ^ board[H - 1][W - 1] ^ N ^ M ^ M ^ (M * (int)1e4)) % 128;

    cout << a << endl;
    cout << b << ' ' << c << endl;
    cout << s << endl;

    cout << K << endl;
    cout << X[0] << ' ' << X[1] << ' ' << X[2] << endl;

    cout << H << ' ' << W << endl;
    for (auto &&it : board)
    {
        cout << it << endl;
    }

    cout << N << ' ' << M << endl;
    for (auto i = 1; i <= M; i++)
    {
        auto u = 1, v = 1;
        auto w = i * (int)1e4;
        cout << u << ' ' << v << ' ' << w << endl;
    }
    return 0;
}
