#include <iostream>
#include <string>

using namespace std;

char s[102];
int X[101];
char board[101][101];
long long u[200001];
long long v[200001];
long long w[200001];

int main()
{
    ios_base::sync_with_stdio(false);
    cin.tie(nullptr);

    int a, b, c, K, H, W, N, M;

    cin >> a >> b >> c >> s >> K;
    for (auto i = 0; i < K; i++)
    {
        cin >> X[i];
    }

    cin >> H >> W;
    for (auto i = 0; i < H; i++)
    {
        cin >> board[i];
    }

    cin >> N >> M;
    for (auto i = 0; i < M; i++)
    {
        cin >> u[i] >> v[i] >> w[i];
    }

    auto check = (a ^ b ^ s[0] ^ X[K - 1] ^ board[H - 1][W - 1] ^ N ^ u[M - 1] ^ v[M - 1] ^ w[M - 1]) % 128 ^ c;
    if (check != 0)
    {
        cerr << "WA " << check << endl;
        return 1;
    }

    return 0;
}
