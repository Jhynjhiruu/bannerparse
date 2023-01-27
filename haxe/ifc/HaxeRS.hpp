#pragma once

#include <hxcpp.h>

#include <cstdint>

namespace hxrs
{
    class BannerParse
    {
    public:
        static void *parse_banner(::Array<uint8_t> data);
        static void drop_banner(void *banner);
        static ::Array<::String> list_dir(void *banner, ::String path);
        static ::Array<uint8_t> get_file(void *banner, ::String path);
    };
}