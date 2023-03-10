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
        static ::Array<uint8_t> get_banner(void *banner);
        static ::Array<::String> get_titles(void *banner);
    };

    class U8Parse
    {
    public:
        static void *parse_u8(::Array<uint8_t> data);
        static void drop_u8(void *arc);
        static ::Array<::String> list_dir(void *arc, ::String path);
        static ::Array<uint8_t> get_file(void *arc, ::String path);
    };

    class IMD5Parse
    {
    public:
        static void *parse_imd5(::Array<uint8_t> data);
        static void drop_imd5(void *imd5);
        static ::Array<uint8_t> get(void *imd5);
    };

    class NintyLZ77
    {
    public:
        static ::Array<uint8_t> decompress(::Array<uint8_t> data);
    };

    class TPLParse
    {
    public:
        static void *parse_tpl(::Array<uint8_t> data);
        static void drop_tpl(void *tpl);
        static uint32_t get_num_imgs(void *tpl);
        static uint32_t get_size(void *tpl, uint32_t idx);
        static ::Array<uint8_t> get_tpl_rgba(void *tpl, uint32_t idx);
        static ::Array<uint8_t> save_tpl_img(::Array<uint8_t> data, uint32_t width, uint32_t height);
    };
}