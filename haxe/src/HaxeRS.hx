typedef VoidPtr = cpp.Pointer.Pointer<cpp.Void.Void>;

extern class HaxeRS_ifc {
	@:native("hxrs::BannerParse::parse_banner")
	static public function parseBanner(data:Array<cpp.UInt8>):VoidPtr;
}

@:buildXml('
<files id="haxe">
    <file name="../../../ifc/HaxeRS.cpp">
        <depend name="../../../ifc/HaxeRS.h" />
        <depend name="../../../ifc/HaxeRS.hpp" />
    </file>
</files>

<target id="haxe">
    <libpath path="../../../rust/target/release" />
    <lib base="banner" />
</target>
')
class HaxeRS {
	static public function parseBanner(data:haxe.io.Bytes):VoidPtr {
		return HaxeRS_ifc.parseBanner(data.getData());
	}
}
