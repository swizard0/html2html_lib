#include <stdio.h>
#include <stdlib.h>
#include <html2html_lib.h>

int main()
{
    char *html = NULL;
    size_t html_buffer_size = 0;

    ssize_t html_size = getdelim(&html, &html_buffer_size, 0, stdin);
    if (html_size < 0)
    {
        fprintf(stderr, "Something went wrong when reading stdin (html_size = %d)\n", (int)html_size);
        return 1;
    }

    enum parse_status status = parse_document(html, (size_t)html_size, 1);
    free(html);
    if (status != RES_OK)
    {
        fprintf(stderr, "Something went wrong when parsing document (status = %d)\n", (int)status);
        return 1;
    }

    return 0;
}
