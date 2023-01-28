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
    <libpath name="../../../rust/target/release" unless="debug" />
    <libpath name="../../../rust/target/debug" if="debug" />
	<libpath name="${USERPROFILE}/.cargo/registry/src/github.com-1ecc6299db9ec823/windows_x86_64_msvc-0.42.0/lib" if="windows" />
    <lib base="windows" if="windows" />
    <lib base="banner" if="windows" />
    <lib name="-lbanner" unless="windows" />
    <depend name="../../../rust/target/release/libbanner.a" unless="debug||windows" />
    <depend name="../../../rust/target/debug/libbanner.a" if="debug" unless="windows" />
    <depend name="../../../rust/target/release/banner.lib" unless="debug" if="windows" />
    <depend name="../../../rust/target/debug/banner.lib" if="debug&&windows" />
</target>

<linker id="exe" exe="g++" unless="windows">
    <flag value="-fuse-ld=mold"/>
</linker>
')
extern class HaxeRS_ifc {
	@:native("hxrs::BannerParse::parse_banner")
	static public function parseBanner(data:Array<cpp.UInt8>):VoidPtr;
	@:native("hxrs::BannerParse::drop_banner")
	static public function dropBanner(banner:VoidPtr):Void;
	@:native("hxrs::BannerParse::list_dir")
	static public function listDir(banner:VoidPtr, str:String):Array<String>;
	@:native("hxrs::BannerParse::get_file")
	static public function getFile(banner:VoidPtr, str:String):Array<cpp.UInt8>;

	@:native("hxrs::IMD5Parse::parse_imd5")
	static public function parseIMD5(data:Array<cpp.UInt8>):VoidPtr;
	@:native("hxrs::IMD5Parse::drop_imd5")
	static public function dropIMD5(imd5:VoidPtr):Void;
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
		if (ptr != null) {
			HaxeRS_ifc.dropBanner(ptr);
			ptr = null;
		}
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

	public function listDir(dir:String):Array<String> {
		if (ptr != null) {
			return HaxeRS_ifc.listDir(ptr, dir);
		}
		return null;
	}

	public function getFile(dir:String):haxe.io.Bytes {
		if (ptr != null) {
			return haxe.io.Bytes.ofData(HaxeRS_ifc.getFile(ptr, dir));
		}
		return null;
	}
}

@:cppInclude("../../../ifc/HaxeRS.hpp")
class IMD5 {
	var ptr:VoidPtr = null;

	public function new(?vp:VoidPtr) {
		ptr = vp;
	}

	static public function parse(data:haxe.io.Bytes):IMD5 {
		return new IMD5(HaxeRS_ifc.parseIMD5(data.getData()));
	}

	public function drop():Void {
		if (ptr != null) {
			HaxeRS_ifc.dropIMD5(ptr);
			ptr = null;
		}
	}

	public function update(rhs:IMD5):Void {
		if (ptr != null) {
			drop();
		}
		ptr = rhs.ptr;
	}
}
