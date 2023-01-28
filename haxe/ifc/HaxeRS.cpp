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
        return ::Array<::String>(static_cast<::Array_obj<::String> *>(::list_dir(banner, _hx_utf8_length(path), (const unsigned char *)path.utf8_str())));
    }
    ::Array<uint8_t> BannerParse::get_file(void *banner, ::String path)
    {
        return ::Array<uint8_t>(static_cast<::Array_obj<uint8_t> *>(::get_file(banner, _hx_utf8_length(path), (const unsigned char *)path.utf8_str())));
    }

    void *IMD5Parse::parse_imd5(::Array<uint8_t> data)
    {
        return ::parse_imd5(data.__length(), data.CheckGetPtr()->Pointer());
    }
    void IMD5Parse::drop_imd5(void *imd5)
    {
        ::drop_imd5(imd5);
    }

}