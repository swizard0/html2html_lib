#ifndef _HTML2HTML_LIB_H_
#define _HTML2HTML_LIB_H_

#ifdef __cplusplus
extern "C"
{
#endif

    enum parse_status
    {
        RES_OK = 0,
        RES_ERR_INVALID_PARAMETER = 1,
        RES_ERR_PARSE_DOCUMENT = 2,
        RES_ERR_DOCTYPE_WRITE = 3,
        RES_ERR_SERIALIZE = 4
    };

    enum parse_status parse_document(const char *html, size_t html_size, int print_tree);

#ifdef __cplusplus
}
#endif

#endif /* _HTML2HTML_LIB_H_ */
