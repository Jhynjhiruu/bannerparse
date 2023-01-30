#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C"
{
#endif

    void *parse_banner(size_t len, const uint8_t *data);
    void drop_banner(void *banner);
    void *get_banner(void *banner);
    void *get_titles(void *banner);

    void *parse_u8(size_t len, const uint8_t *data);
    void drop_u8(void *arc);
    void *list_dir(void *arc, size_t len, const unsigned char *data);
    void *get_file(void *arc, size_t len, const unsigned char *data);

    void *parse_imd5(size_t len, const uint8_t *data);
    void drop_imd5(void *imd5);
    void *get_imd5(void *imd5);

    void *decompress_lz77(size_t len, const uint8_t *data);

#ifdef __cplusplus
}
#endif