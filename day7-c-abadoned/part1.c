#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define ull unsigned long long
#define EQ_LEN 850

// void print_int128(__int128_t num) {
//     printf("%llu%llu\n", (ull)(num >> 64), (ull)(num));
// }

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

typedef struct {
    ull *buff;
    size_t len;
    size_t cap;
} vector;

vector vec_init() {
    ull *buff = (ull*)malloc(sizeof(ull) * 1);
    if (buff == NULL) {
        perror("Allocation failed");
        exit(EXIT_FAILURE);
    }
    vector vec = {buff, 0, 1};
    return vec;
}

void vec_free(vector *vec) {
    memset(vec->buff, 0, sizeof(ull) * vec->len);
    free(vec->buff);
    vec->buff = NULL;
}

void vec_push(vector *vec, ull item) {
    if (vec->len == vec->cap) {
        vec->cap *= 2;
        ull *new_buff = (ull*)realloc(vec->buff, sizeof(ull) * vec->cap);
        if (new_buff == NULL) {
            perror("Reallocation failed");
            vec_free(vec);
            return;
        }

        vec->buff = new_buff;
    }

    vec->buff[vec->len] = item;
    vec->len += 1;
}

ull vec_get(vector *vec, size_t index) {
    if (index >= vec->len) {
        perror("Index out of range");
        return 0;
    }

    return vec->buff[index];
}

void vec_insert(vector *vec, size_t index, ull item) {
    if (index >= vec->len) {
        perror("Index out of bounds");
        return;
    }

    vec->buff[index] = item;
}

int vec_contains(vector *vec, ull num) {
    for (size_t i = 0; i < vec->len; i++) {
        ull n = vec_get(vec, i);
        if (n == num) {
            return 1;
        }
    }

    return 0;
}

typedef struct {
    ull result;
    vector nums;
} equation;

equation eq_init(ull result, vector nums) {
    equation eq = {result, nums};
    return eq;
}

void eq_free(equation *eq) {
    vec_free(&eq->nums);
}

ull ull_from_str(size_t *index, char* input) {
    vector digits = vec_init();
    char ch = input[*index];
    while (ch >= 48 && ch <= 57) {
        vec_push(&digits, ch - 48);
        (*index)++;
        ch = input[*index];
    }

    ull res = 0ULL;
    for (size_t i = 0; i < digits.len; i++) {
        res *= 10ULL;
        res += vec_get(&digits, i);
    }

    vec_free(&digits);
    return res;
}

void read_equations(equation equations[EQ_LEN], char *input) {
    size_t index = 0;
    for (size_t i = 0; i < EQ_LEN; i++) {
        ull result = ull_from_str(&index, input);
        index++; // skip the ':'
        vector nums = vec_init();
        while (input[index] != '\n' && input[index] != '\0') {
            index++;
            ull n = ull_from_str(&index, input);
            vec_push(&nums, n);
        }

        index++; // skip the '\n'

        equations[i] = eq_init(result, nums);
    }
}

int check_if_possible(equation *eq) {
    vector results = vec_init();
    vec_push(&results, 0ULL);

    for (size_t i = 0; i < eq->nums.len; i++) {
        size_t length = results.len;
        ull next = vec_get(&eq->nums, i);

        for (size_t j = 0; j < length; j++) {
            ull num = vec_get(&results, j);
            vec_insert(&results, i, num * next); // multiply the original vector
            vec_push(&results, num + next); // add up the and double the size of vector
        }
    }    

    if (vec_contains(&results, eq->result)) {
        vec_free(&results);
        return 1;
    }

    vec_free(&results);
    return 0;
}

int main(void) {
    // __int128_t num = 193542387263212000;

    // print_int128(num);

    // return 0;
    const char *filename = "input.txt";
    char *input = read_file(filename);
    equation equations[EQ_LEN];
    read_equations(equations, input);


    // for (size_t i = 0; i < EQ_LEN; i++) {
    //     printf("%llu:", equations[i].result);
    //     for (size_t j = 0; j < equations[i].nums.len; j++) {
    //         printf(" %llu", vec_get(&equations[i].nums, j));
    //     }
    //     printf("\n");
    // }

    vector sum = vec_init();
    for (size_t i = 0; i < EQ_LEN; i++) {
        int is_possible = check_if_possible(&equations[i]);
        if (is_possible) {
            vec_push(&sum, equations[i].result);
        }
    }

    ull res = 0ULL;
    for (size_t i = 0; i < sum.len; i++) {
        res += vec_get(&sum, i);
        printf("%llu\n", vec_get(&sum, i));
    }
    
    printf("%llu", res);

    for (size_t i = 0; i < EQ_LEN; i++) {
        eq_free(&equations[i]);
    }
    
    vec_free(&sum);
    free(input);
    return 0;
}