#include "RSHaxe.hpp"

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
}