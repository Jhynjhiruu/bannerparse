#pragma once

#include <hxcpp.h>

extern "C"
{
    void *alloc(unsigned int size, size_t align);
    void *construct_array_u8(unsigned int size, const uint8_t *data);
    void *new_array_string(void);
    void array_string_push(void *arr, const void *str);
    void *construct_string(unsigned int size, const char *data);
}