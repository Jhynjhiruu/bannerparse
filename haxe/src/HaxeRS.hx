typedef VoidPtr = cpp.Pointer.Pointer<cpp.Void.Void>;

@:buildXml('
<files id="haxe">
    <file name="../../ifc/RSHaxe.cpp">
        <depend name="../../ifc/RSHaxe.hpp" />
    </file>
    <file name="../../ifc/HaxeRS.cpp">
        <depend name="../../ifc/HaxeRS.h" />
    </file>
</files>

<target id="haxe">
    <libpath name="../../../rust/target/release" unless="debug"/>
    <libpath name="../../../rust/target/debug" if="debug"/>
    <lib base="banner" if="windows" />
    <lib name="-lbanner" if="linux" />
    <depend name="../../../rust/target/release/libbanner.a" />
</target>
')
extern class HaxeRS_ifc {
	@:native("hxrs::BannerParse::parse_banner")
	static public function parseBanner(data:Array<cpp.UInt8>):VoidPtr;
	@:native("hxrs::BannerParse::drop_banner")
	static public function dropBanner(banner:VoidPtr):Void;
}

@:cppInclude("../../../ifc/HaxeRS.hpp")
class Banner {
	var ptr:VoidPtr = null;

	public function new(?vp:VoidPtr) {
		ptr = vp;
	}

	static public function parse(data:haxe.io.Bytes):Banner {
		return new Banner(HaxeRS_ifc.parseBanner(data.getData()));
	}

	public function drop():Void {
		HaxeRS_ifc.dropBanner(ptr);
		ptr = null;
	}

	public function update(rhs:Banner):Void {
		if (ptr != null) {
			drop();
		}
		ptr = rhs.ptr;
	}

	public function valid():Bool {
		return ptr != null;
	}
}
