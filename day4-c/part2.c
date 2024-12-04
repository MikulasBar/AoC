#include <stdio.h>
#include <stdlib.h>
#include <string.h>


size_t get_row_size(char *string) {
    size_t size = 0;

    while (string[size] != '\n') {
        size++;
    }

    return size + 1;
}

char* read_file(const char* filename) {
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

int is_inside(int index, size_t size) {
    if (index < 0 || (size_t)index > size) {
        return 0;
    } else {
        return 1;
    }
}

int match(char *input, size_t input_size, int row_size, int index) {
    int lt = index - row_size - 1;
    int rt = index - row_size + 1;
    int lb = index + row_size - 1;
    int rb = index + row_size + 1;

    if (!is_inside(lt, input_size) || !is_inside(rb, input_size)) {
        return 0;
    }

    if (input[lt] + input[rb] != 160 || input[rt] + input[lb] != 160) {
        return 0;
    }

    return 1;
}

int run(void) {
    const char* filename = "input.txt";
    char *input = read_file(filename);
    if (input == NULL) {
        return 1;
    }

    int row_size = (int)get_row_size(input);
    size_t size = strlen(input);

    int matched = 0;

    for (size_t i = 0; i < size; i++) {
        if (input[i] != 'A') {
            continue;
        }

        matched += match(input, size, row_size, (int)i);
    }

    printf("%i", matched);

    free(input);
    return 0;
}

int main(void) {
    run();
}
