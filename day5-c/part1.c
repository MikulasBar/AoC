#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define RULES_LEN 1176
#define RULE_LINE_SIZE 6
#define REPORTS_LEN 194
#define SEP_LINE 1177
#define LAST_LINE 1371


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
    memset(vec->buff, 0, sizeof(int) * vec->len);
    free(vec->buff);
    vec->buff = NULL;
}

void vec_push(vector *vec, int item) {
    if (vec->len == vec->cap) {
        vec->cap *= 2;
        vec->buff = realloc(vec->buff, sizeof(int) * vec->cap);
    }

    vec->buff[vec->len] = item;
    vec->len += 1;
}

int vec_get(vector *vec, size_t index) {
    if (index >= vec->len) {
        perror("Index out of range");
        return -1;
    }

    return vec->buff[index];
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
} rule;

void read_rules_from_input(rule rules[RULES_LEN], char *input) {
    for (size_t i = 0; i < RULES_LEN; i++) {
        size_t j = i * RULE_LINE_SIZE;
        int num0 = two_chars_to_int(input[j], input[j+1]);
        int num1 = two_chars_to_int(input[j+3], input[j+4]);

        rules[i].a = num0;
        rules[i].b = num1;
    }
}

void read_reports_from_input(vector reports[REPORTS_LEN], char *input) {
    size_t index = RULE_LINE_SIZE * RULES_LEN + 1 + 1 - 1;
    // printf("Char: %c\n", input[index]);

    for (size_t i = 0; i < REPORTS_LEN; i++) {
        reports[i] = vec_init();
        while (input[index + 2] != '\n' && input[index + 2] != '\0') {
            int num = two_chars_to_int(input[index], input[index + 1]); 
            vec_push(&reports[i], num);
            index += 3;
        }

        int num = two_chars_to_int(input[index], input[index + 1]); 
        vec_push(&reports[i], num);
        index += 3;
    }
}

int contains(rule rules[RULES_LEN], rule pat) {
    for (size_t i = 0; i < RULES_LEN; i++) {
        if (rules[i].a == pat.a && rules[i].b == pat.b) {
            // printf("Rule is matched: %i == %i, %i == %i\n", rules[i].a, pat.a, rules[i].b, pat.b);
            // printf("MAthed");
            return 1;
        } 
    }

    return 0;
}

int is_sorted(vector *report, rule rules[RULES_LEN]) {
    for (size_t i = 0; i < report->len-1; i++) {
        int num1 = vec_get(report, i);
        for (size_t j = i+1; j < report->len; j++) {
            int num2 = vec_get(report, j);
            rule pat = {num2, num1}; // inverse rule
            if (contains(rules, pat)) {
                // printf("Not sorted: %i, %i\n", num1, num2);
                return 0;
            }
        }
    }

    return 1;
}

int main(void) {
    const char *filename = "input.txt";
    char *input = read_file(filename);
    rule rules[RULES_LEN];
    vector reports[REPORTS_LEN];
    read_rules_from_input(rules, input);
    read_reports_from_input(reports, input);

    int res = 0;

    // printf("%i", vec_get(&reports[0], reports[0].len / 2 + 1));
    // return 0;    

    for (size_t i = 0; i < REPORTS_LEN; i++) {
        if (is_sorted(&reports[i], rules)) {
            size_t mid = reports[i].len / 2;
            res += vec_get(&reports[i], mid);
            printf("Report sorted: %u\n", i);
        } else {
            printf("Report not sorted: %u\n", i);
        }
    }

    printf("%i", res);

    for (size_t i = 0; i < REPORTS_LEN; i++) {
        vec_free(&reports[i]);
    }
    
    free(input);
    return 0;
}