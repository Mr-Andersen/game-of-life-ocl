__kernel void next_iteration(__constant uchar (*prev)[WIDTH], __global uchar (*next)[WIDTH]) {
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
    uchar neighbours_num
        = prev[prev_row][prev_col] + prev[prev_row][col] + prev[prev_row][next_col]
        + prev[row][prev_col] + prev[row][next_col]
        + prev[next_row][prev_col] + prev[next_row][col] + prev[next_row][next_col];
    if (prev[row][col]) {
        if (neighbours_num == 2 || neighbours_num == 3)
            next[row][col] = 1;
        else
            next[row][col] = 0;
    } else {
        if (neighbours_num == 3)
            next[row][col] = 1;
        else
            next[row][col] = 0;
    }
}
