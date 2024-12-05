#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define RULES_LEN 1176
#define RULE_LINE_SIZE 6
#define INCLUDE 1
#define EXCLUDE 0
#define INC_ERROR -1

typedef struct {
    int *buff;
    size_t len;
    size_t cap;
} vector;

vector vec_init() {
    int *buff = (int*)malloc(sizeof(int) * 1);
    vector vec = {buff, 0, 1};
    return vec;
}

void vec_free(vector *vec) {
    memset(vec->buff, 0, sizeof(int*));
    free(vec->buff);
    vec->buff == NULL;
}

void vec_push(vector *vec, int item) {
    if (vec->len == vec->cap) {
        vec->buff = realloc(vec->buff, sizeof(int) * vec->cap * 2);
        vec->cap *= 2;
    }

    vec->buff[vec->len] = item;
    vec->len += 1;
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

int two_chars_to_int(char char0, char char1) {
    int res = (char0 - 48) * 10 + char1 - 48;
    return res;
}

typedef struct {
    int a;
    int b;
    int inc; // inclusion
} rule;

void read_rules_from_input(rule rules[RULES_LEN], char *input) {
    for (size_t i = 0; i < RULES_LEN; i++) {
        size_t j = i * RULE_LINE_SIZE;
        int num0 = two_chars_to_int(input[j], input[j+1]);
        int num1 = two_chars_to_int(input[j+3], input[j+4]);

        rules[i].a = num0;
        rules[i].b = num1;
        rules[i].inc = INCLUDE;
    }
}

rule get_nth(rule rules[RULES_LEN], size_t n) {
    size_t count = 0;

    for (size_t i = 0; i < RULES_LEN; i++) {
        if (rules[i].inc) {
            if (count == n) {
                return rules[i];
            } else {
                count++;
            }
        }
    }

    perror("Error: no included rules");

    rule r = {0, 0, INC_ERROR};
    return r;
}

int *sorted_by_rules(rule rules[RULES_LEN]) {
    vector left = vec_init();
    vector right = vec_init();
    get_nth(rules, 0);



    vec_free(&left);
    vec_free(&right);
}




int main(void) {
    const char *filename = "input.txt";
    char *input = read_file(filename);
    int rules[RULES_LEN][2];
    read_rules_from_input(rules, input);

    for (size_t i = 0; i < RULES_LEN; i++) {
        printf("%i|%i\n", rules[i][0], rules[i][1]);
    }

    free(input);
    return 0;
}