using StringTools;

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
private extern class HaxeRS_ifc {
	@:native("hxrs::BannerParse::parse_banner")
	static public function parseBanner(data: Array<cpp.UInt8>): VoidPtr;
	@:native("hxrs::BannerParse::drop_banner")
	static public function dropBanner(banner: VoidPtr): Void;
	@:native("hxrs::BannerParse::get_banner")
	static public function getBanner(banner: VoidPtr): Array<cpp.UInt8>;
	@:native("hxrs::BannerParse::get_titles")
	static public function getTitles(banner: VoidPtr): Array<String>;

	@:native("hxrs::U8Parse::parse_u8")
	static public function parseU8(data: Array<cpp.UInt8>): VoidPtr;
	@:native("hxrs::U8Parse::drop_u8")
	static public function dropU8(arc: VoidPtr): Void;
	@:native("hxrs::U8Parse::list_dir")
	static public function listDir(arc: VoidPtr, str: String): Array<String>;
	@:native("hxrs::U8Parse::get_file")
	static public function getFile(arc: VoidPtr, str: String): Array<cpp.UInt8>;

	@:native("hxrs::IMD5Parse::parse_imd5")
	static public function parseIMD5(data: Array<cpp.UInt8>): VoidPtr;
	@:native("hxrs::IMD5Parse::drop_imd5")
	static public function dropIMD5(imd5: VoidPtr): Void;
	@:native("hxrs::IMD5Parse::get")
	static public function getIMD5(imd5: VoidPtr): Array<cpp.UInt8>;

	@:native("hxrs::NintyLZ77::decompress")
	static public function decompressLZ77(data: Array<cpp.UInt8>): Array<cpp.UInt8>;

	@:native("hxrs::TPLParse::parse_tpl")
	static public function parseTPL(data: Array<cpp.UInt8>): VoidPtr;
	@:native("hxrs::TPLParse::drop_tpl")
	static public function dropTPL(tpl: VoidPtr): Void;
	@:native("hxrs::TPLParse::get_num_imgs")
	static public function getTPLNumImgs(tpl: VoidPtr): cpp.UInt32;
	@:native("hxrs::TPLParse::get_size")
	static public function getTPLSize(tpl: VoidPtr, idx: cpp.UInt32): cpp.UInt32;
	@:native("hxrs::TPLParse::get_tpl_rgba")
	static public function getRGBA(tpl: VoidPtr, idx: cpp.UInt32): Array<cpp.UInt8>;
	@:native("hxrs::TPLParse::save_tpl_img")
	static public function saveTPLImg(data: Array<cpp.UInt8>, width: cpp.UInt32, height: cpp.UInt32): Array<cpp.UInt8>;
}

@:cppInclude("../../../ifc/HaxeRS.hpp")
class Banner implements MainView.Directory {
	var ptr: VoidPtr = null;

	public final type = MainView.FileTypes.IMET;

	public function new(?vp: VoidPtr) {
		ptr = vp;
	}

	public function valid(): Bool {
		return ptr != null;
	}

	static public function parse(data: haxe.io.Bytes): Banner {
		return new Banner(HaxeRS_ifc.parseBanner(data.getData()));
	}

	public function drop(): Void {
		if (this.valid()) {
			HaxeRS_ifc.dropBanner(ptr);
			ptr = null;
		}
	}

	public function get(path: String = ""): haxe.io.Bytes {
		if (this.valid()) {
			return haxe.io.Bytes.ofData(HaxeRS_ifc.getBanner(ptr));
		}
		return null;
	}

	public function getTitles(): Array<String> {
		if (this.valid()) {
			return HaxeRS_ifc.getTitles(ptr);
		}
		return null;
	}

	public function listDir(dir: String = ""): Array<String> {
		return ["imet"];
	}
}

@:cppInclude("../../../ifc/HaxeRS.hpp")
class U8 implements MainView.Directory {
	var ptr: VoidPtr = null;

	public final type = MainView.FileTypes.U8;

	public function new(?vp: VoidPtr) {
		ptr = vp;
	}

	public function valid(): Bool {
		return ptr != null;
	}

	static public function parse(data: haxe.io.Bytes): U8 {
		return new U8(HaxeRS_ifc.parseU8(data.getData()));
	}

	public function drop(): Void {
		if (this.valid()) {
			HaxeRS_ifc.dropU8(ptr);
			ptr = null;
		}
	}

	public function listDir(dir: String = ""): Array<String> {
		if (this.valid()) {
			return HaxeRS_ifc.listDir(ptr, dir);
		}
		return null;
	}

	public function get(dir: String = ""): haxe.io.Bytes {
		if (this.valid()) {
			return haxe.io.Bytes.ofData(HaxeRS_ifc.getFile(ptr, dir));
		}
		return null;
	}
}

@:cppInclude("../../../ifc/HaxeRS.hpp")
class IMD5 implements MainView.Directory {
	var ptr: VoidPtr = null;

	public final type = MainView.FileTypes.IMD5;

	public function new(?vp: VoidPtr) {
		ptr = vp;
	}

	public function valid(): Bool {
		return ptr != null;
	}

	static public function parse(data: haxe.io.Bytes): IMD5 {
		return new IMD5(HaxeRS_ifc.parseIMD5(data.getData()));
	}

	public function drop(): Void {
		if (this.valid()) {
			HaxeRS_ifc.dropIMD5(ptr);
			ptr = null;
		}
	}

	public function get(path: String = ""): haxe.io.Bytes {
		if (this.valid()) {
			return haxe.io.Bytes.ofData(HaxeRS_ifc.getIMD5(ptr));
		}
		return null;
	}

	public function listDir(dir: String = ""): Array<String> {
		return ["imd5"];
	}
}

@:cppInclude("../../../ifc/HaxeRS.hpp")
class NintyLZ77 {
	static public function decompress(data: haxe.io.Bytes): haxe.io.Bytes {
		return haxe.io.Bytes.ofData(HaxeRS_ifc.decompressLZ77(data.getData()));
	}
}

class LZ77 implements MainView.Directory {
	final data: haxe.io.Bytes;

	public final type = MainView.FileTypes.LZ77;

	public function new(data: haxe.io.Bytes) {
		this.data = data;
	}

	public function valid(): Bool {
		return true;
	}

	public function drop(): Void {}

	public function listDir(?dir: String): Array<String> {
		return ["lz77"];
	}

	public function get(?path: String): haxe.io.Bytes {
		return NintyLZ77.decompress(data);
	}
}

@:cppInclude("../../../ifc/HaxeRS.hpp")
class TPL implements MainView.Directory {
	var ptr: VoidPtr = null;

	public final type = MainView.FileTypes.TPL;

	public function new(?vp: VoidPtr) {
		ptr = vp;
	}

	public function valid(): Bool {
		return ptr != null;
	}

	static public function parse(data: haxe.io.Bytes): TPL {
		return new TPL(HaxeRS_ifc.parseTPL(data.getData()));
	}

	public function drop(): Void {
		if (this.valid()) {
			HaxeRS_ifc.dropTPL(ptr);
			ptr = null;
		}
	}

	/*public function get(): haxe.io.Bytes {
		if (this.valid()) {
			return haxe.io.Bytes.ofData(HaxeRS_ifc.getIMD5(ptr));
		}
		return null;
	}*/
	public function getNumImages(): Int {
		if (this.valid()) {
			return HaxeRS_ifc.getTPLNumImgs(ptr);
		}
		return 0;
	}

	public function getSize(idx: Int): {width: Int, height: Int} {
		if (this.valid()) {
			final hw = HaxeRS_ifc.getTPLSize(ptr, idx);
			return {width: hw >> 0x10, height: hw & 0xFFFF};
		}
		return {width: 0, height: 0};
	}

	public function getRGBA(idx: Int): haxe.io.Bytes {
		if (this.valid()) {
			return haxe.io.Bytes.ofData(HaxeRS_ifc.getRGBA(ptr, idx));
		}
		return null;
	}

	public function listDir(?dir: String): Array<String> {
		if (this.valid()) {
			return [for (i in 0...getNumImages()) 'tpl$i'];
		}
		return [];
	}

	public function retrieveIndex(path: String = ""): Int {
		if (this.valid() && path.startsWith("/tpl")) {
			final img = Std.parseInt(path.substr(4));
			if (img < getNumImages()) {
				return img;
			}
		}
		return -1;
	}

	public function get(?path: String): haxe.io.Bytes {
		if (this.valid()) {
			final img = retrieveIndex(path);
			if (img >= 0) {
				final dims = getSize(img);
				return toPNG(getRGBA(img), dims.width, dims.height);
			}
		}
		return null;
	}

	static public function toPNG(data: haxe.io.Bytes, width: Int, height: Int): haxe.io.Bytes {
		return haxe.io.Bytes.ofData(HaxeRS_ifc.saveTPLImg(data.getData(), width, height));
	}
}
