#pragma once

#include <hxcpp.h>

#include <cstdint>

namespace hxrs
{
    class BannerParse
    {
        static void *parse_banner(Array<uint8_t> data);
    };
}