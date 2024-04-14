#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

int main() {
    // Read in data.txt (each line is a different number)
    int* data = malloc(1000000 * sizeof(int));
    FILE* file = fopen("data.txt", "r");
    if (file == NULL) {
        printf("Error opening file\n");
        return 1;
    }
    int num;
    int i = 0;
    while (fscanf(file, "%d", &num) == 1) {
        data[i] = num;
        i++;
    }
    int num_entries = i;
    fclose(file);
    // Create a hashset
    int* hashset = malloc(1000000 * sizeof(int));
    for (int i = 0; i < 1000000; i++) {
        hashset[i] = 0;
    }
    // Iterate through the data and check if each number is in the hashset
    for (int i = 0; i < num_entries; i++) {
        if (hashset[data[i]] == 1) {
            printf("Duplicate found: %d\n", data[i]);
            return 0;
        }
        hashset[data[i]] = 1;
    }
    printf("No duplicates found\n");
    return 0;
}
