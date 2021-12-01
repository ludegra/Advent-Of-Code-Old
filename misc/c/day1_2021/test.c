#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main()
{
    char text[4] = "2765";

    int len = strlen(text);
    int result = 0;

    for (int i = 0; i < len; i++)
    {
        result = result * 10 + ( text[i] - '0');
    }

    printf("%d\n", result);
}