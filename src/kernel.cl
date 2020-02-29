#define ALIVE 0x00ffffff
#define DEAD  0x00000000

__kernel void next_iteration(__constant uint (*prev)[WIDTH], __global uint (*next)[WIDTH]) {
    size_t row = get_global_id(0);
    size_t col = get_global_id(1);
    size_t prev_row, next_row;
    if (row == 0) {
        prev_row = HEIGHT - 1;
        next_row = 1;
    } else if (row == HEIGHT - 1) {
        next_row = 0;
        prev_row = HEIGHT - 2;
    } else {
        prev_row = row - 1;
        next_row = row + 1;
    }
    size_t prev_col, next_col;
    if (col == 0) {
        prev_col = WIDTH - 1;
        next_col = 1;
    } else if (col == WIDTH - 1) {
        next_col = 0;
        prev_col = WIDTH - 2;
    } else {
        prev_col = col - 1;
        next_col = col + 1;
    }
    uint neighbours_num
        = (prev[prev_row][prev_col] & 1)
        + (prev[prev_row][col] & 1)
        + (prev[prev_row][next_col] & 1)
        + (prev[row][prev_col] & 1)
        + (prev[row][next_col] & 1)
        + (prev[next_row][prev_col] & 1)
        + (prev[next_row][col] & 1)
        + (prev[next_row][next_col] & 1);
    if (prev[row][col]) {
        if (neighbours_num == 2 || neighbours_num == 3)
            next[row][col] = ALIVE;
        else
            next[row][col] = DEAD;
    } else {
        if (neighbours_num == 3)
            next[row][col] = ALIVE;
        else
            next[row][col] = DEAD;
    }
}
