#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C"
{
#endif

    void *parse_banner(size_t len, const uint8_t *data);
    void drop_banner(void *banner);
    void *list_dir(void *banner, size_t len, const unsigned char *data);
    void *get_file(void *banner, size_t len, const unsigned char *data);

#ifdef __cplusplus
}
#endif