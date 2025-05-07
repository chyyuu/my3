#include <locale.h>
#include <ncurses.h>

int main() {
    // 设置本地化信息以支持 UTF - 8
    setlocale(LC_ALL, "");
    // 初始化 ncurses
    initscr();
    // 显示 UTF - 8 字符
    printw("你好，世界!");
    refresh();
    getch();
    endwin();
    return 0;
}
