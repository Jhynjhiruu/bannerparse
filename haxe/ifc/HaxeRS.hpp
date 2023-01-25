#pragma once

#include <hxcpp.h>

#include <cstdint>

namespace hxrs
{
    class BannerParse
    {
    public:
        static void *parse_banner(Array<uint8_t> data);
        static void drop_banner(void *banner);
    };
}