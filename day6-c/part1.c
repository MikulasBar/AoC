#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define SIZE 130

typedef struct {
    size_t x;
    size_t y;
} position;

position pos_init(size_t x, size_t y) {
    position pos = {x, y};
    return pos;
}

position pos_from_char(char ch) {
    switch (ch) {
        case '<':
            return pos_init(-1, 0);   
        case '>':
            return pos_init(1, 0);
        case '^':
            return pos_init(0, -1);
        case 'V':
            return pos_init(0, 1);
    }

    perror("Reached unreachable point");
    return pos_init(1, 0);
}

char pos_get(position pos, char *input) {
    return input[pos.x + pos.y * (SIZE + 1)];
}

void pos_rotate_right(position *dir) {
    size_t x = dir->x;
    dir->x = -dir->y;
    dir->y = x;
}

position pos_add(position a, position b) {
    return pos_init(
        a.x + b.x,
        a.y + b.y
    );
}

char *read_file(const char *filename) {
    FILE *file = fopen(filename, "r");
    if (file == NULL) {
        perror("Error opening file");
        return NULL;
    }

    fseek(file, 0, SEEK_END);
    long file_size = ftell(file);
    rewind(file);

    char *buffer = (char*)malloc(file_size + 1);
    if (buffer == NULL) {
        perror("Memory allocation failed");
        fclose(file);
        return NULL;
    }

    size_t read_size = fread(buffer, 1, file_size, file);
    buffer[read_size] = '\0';

    fclose(file);

    return buffer;
}

int is_inside(position pos) {
    if (pos.x >= SIZE || pos.y >= SIZE) {
        return 0;
    }

    return 1;
}

void insert_at(position pos, char ch, char *input) {
    input[pos.x + pos.y * (SIZE + 1)] = ch;
}

position locate_guard(char *input) {
    size_t index = 0;
    char ch = input[index];
    size_t y = 0;

    while (ch != '^') {
        if (ch == '\n') {
            y++;
        }

        index++;
        ch = input[index];
    }

    size_t x = index % (SIZE + 1);
    return pos_init(x, y);
}

int main(void) {
    const char *filename = "input.txt";
    char *input = read_file(filename);
    position guard = locate_guard(input);
    position dir = pos_from_char(pos_get(guard, input));
    size_t tiles = 1;


    while (1) {
        insert_at(guard, 'X', input);
        position next_guard = pos_add(guard, dir);
        if (!is_inside(next_guard)) {
            break;
        }

        char next_char = pos_get(next_guard, input);
        switch (next_char) {
            case '.':
                tiles++;
                break;
            case '#':
                pos_rotate_right(&dir);
                continue;
        }

        guard = next_guard;
    }
    
    printf("%s\n", input);
    printf("%i\n", tiles);

    free(input);
    return 0;
}