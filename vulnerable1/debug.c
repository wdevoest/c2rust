#include <stdio.h>
#include <string.h>
#include <stdlib.h>
struct user {
    char username[10];
    char password[10];
    char bankinfo[10];
};

int lookup_user(char *username, struct user *result)
{
    FILE *file = fopen("users.txt", "r");
    if (file == NULL)
    {
        return 0;
    }

    char line[32];
    while (fgets(line, sizeof(line), file) != NULL)
    {
        char *token = strtok(line, ",");
        if (token == NULL)
        {
            continue;
        }

        if (strcmp(token, username) == 0)
        {
            strcpy(result->username, token);
            token = strtok(NULL, ",");
            if (token == NULL)
            {
                fclose(file);
                return 0;
            }
            strcpy(result->password, token);
            token = strtok(NULL, ",");
            if (token == NULL)
            {
                fclose(file);
                return 0;
            }
            strcpy(result->bankinfo, token);
            fclose(file);
            return 1;
        }
    }

    fclose(file);
    return 0;
}

int main(int argc, char *argv[])
{
    char username[10];
    char password[10];

    puts("Enter your username: ");
    gets(username);
    
    // Lookup the user
    struct user user_result;
    strcpy(user_result.username, "admin");
    strcpy(user_result.password, "secret");
    strcpy(user_result.bankinfo, "1234");
    // if (lookup_user(username, &user_result) == 0)
    // {
    //     puts("User not found");
    //     exit(1);
    // }

    puts("Enter your password: ");
    gets(password);

    for (int i = -15; i < 55; i++)
    {
        printf("username[%d]=%c\n", i, username[i]);
    }
    
    for (int i = -15; i < 55; i++)
    {
        printf("password[%d]=%c\n", i, password[i]);
    }
    for (int i = -15; i < 55; i++)
    {
        printf("user_result[%d]=%c\n", i, user_result.username[i]);
    }

    int username_mem_address = (int)&username;
    int password_mem_address = (int)&password;
    int user_result_mem_address = (int)&user_result;
    printf("username_mem_address: %p\n", username_mem_address);
    printf("password_mem_address: %p\n", password_mem_address);
    printf("user_result_mem_address: %p\n", user_result_mem_address);

    exit(0);
}
