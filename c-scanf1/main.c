#include <stdio.h>

char s[102];
int X[101];
char board[101][101];
long long u[200001];
long long v[200001];
long long w[200001];

int main()
{
    int a, b, c, K, H, W, N, M;

    scanf("%d %d %d %s %d", &a, &b, &c, s, &K);
    for (int i = 0; i < K; i++)
    {
        scanf("%d", &X[i]);
    }

    scanf("%d %d", &H, &W);
    for (int i = 0; i < H; i++)
    {
        scanf("%s", board[i]);
    }

    scanf("%d %d", &N, &M);
    for (int i = 0; i < M; i++)
    {
        scanf("%lld %lld %lld", &u[i], &v[i], &w[i]);
    }

    long long check = (a ^ b ^ s[0] ^ X[K - 1] ^ board[H - 1][W - 1] ^ N ^ u[M - 1] ^ v[M - 1] ^ w[M - 1]) % 128 ^ c;
    if (check != 0)
    {
        printf("WA %lld", check);
        return 1;
    }

    return 0;
}
