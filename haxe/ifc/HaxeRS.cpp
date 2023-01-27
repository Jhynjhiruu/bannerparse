#include "HaxeRS.hpp"
#include "HaxeRS.h"
#include <hxcpp.h>
#include <cstring>

namespace hxrs
{
    void *BannerParse::parse_banner(::Array<uint8_t> data)
    {
        return ::parse_banner(data.__length(), data.CheckGetPtr()->Pointer());
    }

    void BannerParse::drop_banner(void *banner)
    {
        ::drop_banner(banner);
    }

    ::Array<::String> BannerParse::list_dir(void *banner, ::String path)
    {
        void *arr_ptr = ::list_dir(banner, _hx_utf8_length(path), (const unsigned char *)path.utf8_str());
        ::Array<::String> arr_real;
        std::memcpy(&arr_real, &arr_ptr, sizeof(void *));
        return arr_real;
    }
    ::Array<uint8_t> BannerParse::get_file(void *banner, ::String path)
    {
        void *file_ptr = ::get_file(banner, _hx_utf8_length(path), (const unsigned char *)path.utf8_str());
        ::Array<uint8_t> file_real;
        std::memcpy(&file_real, &file_ptr, sizeof(void *));
        return file_real;
    }
}