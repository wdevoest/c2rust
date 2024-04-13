#include <stdio.h>
#include <string.h>

struct Passwords
{
    char userPass[17];
    char verificationPass[17];
};

int main()
{
    struct Passwords passwords = {.verificationPass = "0123456789123456"};
    printf("Enter your 16 characters crypto password: \n");
    gets(passwords.userPass);
    printf("verificationpass: %s\n", passwords.verificationPass);
    if (strcmp(passwords.userPass, passwords.verificationPass) == 0)
    {
        printf("PASSWORD VERIFIED. \n");
    }
    else
    {
        printf("Invalid password!'\n");
    }
    return 0;
}