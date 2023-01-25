#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C"
{
#endif

    void *parse_banner(size_t len, const uint8_t *data);
    void drop_banner(void *banner);

#ifdef __cplusplus
}
#endif