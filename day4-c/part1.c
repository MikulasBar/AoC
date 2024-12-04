#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define PAT "XMAS"
#define PAT_LEN 4


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

int match(char *input, size_t input_size, size_t index, int offset) {
    for (int i = 0; i < PAT_LEN; i++) {
        int j = (int)index + offset * i;

        if (!is_inside(j, input_size)) {
            return 0;
        }

        if (input[j] != PAT[i]) {
            return 0;
        }
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
    int dirs[] = {1, -1, row_size, -row_size, row_size+1, -row_size-1, row_size-1, -row_size+1};
    size_t dirs_len = 8;

    int matched = 0;

    for (size_t i = 0; i < size; i++) {
        if (input[i] != PAT[0]) {
            continue;
        }

        for (size_t j = 0; j < dirs_len; j++) {
            matched += match(input, size, i, dirs[j]);
        }
    }

    printf("%i", matched);

    free(input);
    return 0;
}

int main(void) {
    run();
}
