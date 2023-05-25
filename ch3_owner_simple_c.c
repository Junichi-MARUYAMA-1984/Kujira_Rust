#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main() {
    // メモリを確保して文字列をコピー
    char* g1 = (char*)malloc(100);
    strcpy(g1, "Hello, world!");
    // 変数g2にg1を代入
    char* g2 = g1;
    // g2の内容を表示
    printf("%s\n", g2);
    // メモリの破棄
    free(g2);
    // うっかり以下を実行するとメモリの二重解放
    // free(g1);
    return 0;
}

