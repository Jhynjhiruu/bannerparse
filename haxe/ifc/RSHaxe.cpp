#include "RSHaxe.hpp"
#include <cstdint>
#include <cstring>
#include <cstdio>

#include <hxcpp.h>

extern "C"
{
    void *alloc(unsigned int size, size_t align)
    {
        uint8_t *allocation = static_cast<uint8_t *>(hx::NewGCBytes(nullptr, size + (align - 1)));
        while ((uintptr_t)allocation % align)
        {
            allocation++;
        }
        return allocation;
    }
    void *construct_array_u8(unsigned int size, const uint8_t *data)
    {
        return new ::Array_obj<uint8_t>(size, size);
    }
    void *new_array_string(void)
    {
        return new ::Array_obj<::String>(0, 0);
    }
    void array_string_push(void *arr, const void *str)
    {
        ::Array_obj<::String> *arr_real = static_cast<::Array_obj<::String> *>(arr);
        const ::String *str_real = static_cast<const ::String *>(str);

        arr_real->push(*str_real);
    }
    void *construct_string(unsigned int size, const char *data)
    {
        return new ::String(::String::create(data, size));
    }
}