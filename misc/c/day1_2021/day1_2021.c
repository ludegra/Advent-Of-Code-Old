#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main()
{
    FILE *fp;

    fp = fopen("./assets/input.txt", "r");

    char content[5];

    while (fgets(content, 5, fp) != NULL)
    {
        printf("%s", content);

        int len = strlen(content);
        int result = 0;

        // printf("%d", len);

        for (int i = 0; i < len; i++)
        {
            printf("%c", content[i]);
            result = result * 10 + ( content[i] - '0' );
        }

        printf("%d", result);
    }

    fclose(fp);


    return 0;
}