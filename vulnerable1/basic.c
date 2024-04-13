#include <stdio.h>
#include <string.h>
#include <stdlib.h>

struct student
{
    char name[10];
    char grade[5];
};

int main(int argc, char *argv[])
{
    struct student stu;

    strcpy(stu.grade, "nil");

    gets(stu.name);

    printf("Hi %s! Your grade is %s.\n", stu.name, stu.grade);

    exit(0);
}
