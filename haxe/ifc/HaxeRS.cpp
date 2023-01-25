#include "HaxeRS.hpp"
#include "HaxeRS.h"
#include <hxcpp.h>

namespace hxrs
{
    void *BannerParse::parse_banner(Array<uint8_t> data)
    {
        return ::parse_banner(data.__length(), data.CheckGetPtr()->Pointer());
    }

    void BannerParse::drop_banner(void *banner)
    {
        ::drop_banner(banner);
    }
}