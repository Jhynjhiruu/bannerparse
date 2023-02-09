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
    ::Array<uint8_t> BannerParse::get_banner(void *banner)
    {
        return ::Array<uint8_t>(static_cast<::Array_obj<uint8_t> *>(::get_banner(banner)));
    }
    ::Array<::String> BannerParse::get_titles(void *banner)
    {
        return ::Array<::String>(static_cast<::Array_obj<::String> *>(::get_titles(banner)));
    }

    void *U8Parse::parse_u8(::Array<uint8_t> data)
    {
        return ::parse_u8(data.__length(), data.CheckGetPtr()->Pointer());
    }
    void U8Parse::drop_u8(void *arc)
    {
        ::drop_u8(arc);
    }
    ::Array<::String> U8Parse::list_dir(void *arc, ::String path)
    {
        return ::Array<::String>(static_cast<::Array_obj<::String> *>(::list_dir(arc, _hx_utf8_length(path), (const unsigned char *)path.utf8_str())));
    }
    ::Array<uint8_t> U8Parse::get_file(void *arc, ::String path)
    {
        return ::Array<uint8_t>(static_cast<::Array_obj<uint8_t> *>(::get_file(arc, _hx_utf8_length(path), (const unsigned char *)path.utf8_str())));
    }

    void *IMD5Parse::parse_imd5(::Array<uint8_t> data)
    {
        return ::parse_imd5(data.__length(), data.CheckGetPtr()->Pointer());
    }
    void IMD5Parse::drop_imd5(void *imd5)
    {
        ::drop_imd5(imd5);
    }
    ::Array<uint8_t> IMD5Parse::get(void *imd5)
    {
        return ::Array<uint8_t>(static_cast<::Array_obj<uint8_t> *>(::get_imd5(imd5)));
    }

    ::Array<uint8_t> NintyLZ77::decompress(::Array<uint8_t> data)
    {
        return ::Array<uint8_t>(static_cast<::Array_obj<uint8_t> *>(::decompress_lz77(data.__length(), data.CheckGetPtr()->Pointer())));
    }

    void *TPLParse::parse_tpl(::Array<uint8_t> data)
    {
        return ::parse_tpl(data.__length(), data.CheckGetPtr()->Pointer());
    }
    void TPLParse::drop_tpl(void *tpl)
    {
        ::drop_tpl(tpl);
    }
    uint32_t TPLParse::get_num_imgs(void *tpl)
    {
        return ::get_tpl_num_imgs(tpl);
    }
    uint32_t TPLParse::get_size(void *tpl, uint32_t img)
    {
        return ::get_tpl_size(tpl, img);
    }
    ::Array<uint8_t> TPLParse::get_tpl_rgba(void *tpl, uint32_t idx)
    {
        return ::Array<uint8_t>(static_cast<::Array_obj<uint8_t> *>(::get_tpl_rgba(tpl, idx)));
    }
}