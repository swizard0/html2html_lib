#include <stdio.h>
#include <stdlib.h>
#include <html2html_lib.h>

int main()
{
    char *html = NULL;
    size_t html_size = 0;

    ssize_t res = getline(&html, &html_size, stdin);
    if (res < 0)
    {
        fprintf(stderr, "Something went wrong when reading stdin (res = %d)\n", (int)res);
        return 1;
    }

    enum parse_status status = parse_document(html, html_size, 1);
    if (status != RES_OK)
    {
        fprintf(stderr, "Something went wrong when parsing document (status = %d)\n", (int)status);
        return 1;
    }

    return 0;
}
