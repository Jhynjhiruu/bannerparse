package;

using StringTools;

enum abstract FileTypeStack(Int) from Int to Int {
	var Banner;
	var BannerFolder;
	var BannerFile;
}

enum abstract Languages(Int) from Int to Int {
	var Japanese;
	var English;
	var German;
	var French;
	var Spanish;
	var Italian;
	var Dutch;
	var SimplifiedChinese;
	var TraditionalChinese;
	var Korean;
}

enum abstract FileTypes(String) from String to String {
	var IMET = "494D4554"; // "IMET"
	var IMD5 = "494D4435"; // "IMD5"
	var LZ77 = "4C5A3737"; // "LZ77"
	var U8 = "55AA382D"; // "Uª8-""
	var U8File = "u8file";
	var U8Dir = "u8dir";
	var TPL = "0020AF30"; // "� ¯0"
	var BNS = "424E5320"; // "BNS "

	function toString() {
		return switch (this) {
			case IMET: "IMET";
			case IMD5: "IMD5";
			case LZ77: "LZ77";
			case U8: "U8";
			case U8File: "U8File";
			case U8Dir: "U8Dir";
			case TPL: "TPL";
			case BNS: "BNS";
			default: "unk";
		}
	}

	@:from
	static function fromBytes(other: haxe.io.Bytes): FileTypes {
		return other.toHex().toUpperCase();
	}

	public static function getFileExtension(data: haxe.io.Bytes): String {
		final type: FileTypes = data.sub(0, 4);

		return switch (type) {
			case IMET: "imet";
			case IMD5: "imd5";
			case LZ77: "lz77";
			case U8: "arc";
			case TPL: "tpl";
			case BNS: "bns";
			default: "";
		}
	}

	public static function parse(data: haxe.io.Bytes): {type: FileTypes, file: Directory} {
		final type: FileTypes = data.sub(0, 4);

		return switch (type) {
			case IMD5: {type: type, file: HaxeRS.IMD5.parse(data)};
			case LZ77: {type: type, file: new HaxeRS.LZ77(data)};
			case U8: {type: type, file: HaxeRS.U8.parse(data)};
			case TPL: {type: type, file: HaxeRS.TPL.parse(data)};
			default:
				final imet = HaxeRS.Banner.parse(data);
				if (imet.valid()) {
					{type: IMET, file: imet};
				} else {
					{type: type, file: null};
				}
		}
	}
}

final fileTypeDescriptions = [
	"imet" => "IMET Files",
	"imd5" => "IMD5 Files",
	"lz77" => "LZ77 Archives",
	"arc" => "U8 Archives",
	"tpl" => "TPL Images",
	"bin" => "Binary Files",
	"brlyt" => "Binary Revolution Layout Files",
	"brlan" => "Binary Revolution Animation Files",
	"bns" => "Banner Sound Files",
];

@:build(haxe.ui.macros.ComponentMacros.build("assets/bannerfile-rightclick.xml"))
class BannerFileRightClick extends haxe.ui.containers.menus.Menu {
	@:bind(this, haxe.ui.events.MouseEvent.CLICK)
	function onRightClickExport(e: haxe.ui.events.MouseEvent) {
		trace(e.target);
	}
}

interface Directory {
	public function listDir(dir: String = ""): Array<String>;
	public function get(path: String = ""): haxe.io.Bytes;
	public function valid(): Bool;
	public function drop(): Void;
	final type: FileTypes;
}

@:forward
abstract OneOf<A, B>(haxe.ds.Either<A, B>) from haxe.ds.Either<A, B> to haxe.ds.Either<A, B> {
	@:from inline static function fromA<A, B>(a: A): OneOf<A, B> {
		return haxe.ds.Either.Left(a);
	}

	@:from inline static function fromB<A, B>(b: B): OneOf<A, B> {
		return haxe.ds.Either.Right(b);
	}

	@:to inline function toA(): Null<A>
		return switch (this) {
			case Left(a): a;
			default: null;
		}

	@:to inline function toB(): Null<B>
		return switch (this) {
			case Right(b): b;
			default: null;
		}

	public function isRight(): Bool
		return switch (this) {
			case Left(_): false;
			case Right(_): true;
		}
}

@:forward
typedef TreeViewType = OneOf<haxe.ui.containers.TreeView, haxe.ui.containers.TreeViewNode>;

@:forward
abstract Node(TreeViewType) from TreeViewType to TreeViewType {
	public function addNode(data: Dynamic) {
		return switch (this: haxe.ds.Either<haxe.ui.containers.TreeView, haxe.ui.containers.TreeViewNode>) {
			case Left(l):
				l.addNode(data);
			case Right(r):
				r.addNode(data);
		}
	}
}

@:build(haxe.ui.ComponentBuilder.build("assets/main-view.xml"))
class MainView extends haxe.ui.containers.VBox {
	final BannerType: haxe.ui.containers.dialogs.Dialogs.FileDialogExtensionInfo = {extension: "bnr", label: "Banner Files"};
	final ContentType: haxe.ui.containers.dialogs.Dialogs.FileDialogExtensionInfo = {extension: "app", label: "Content Files"};

	var banner: Directory = new HaxeRS.Banner();
	var strings = new Array<String>();
	var currBannerFile: {path: String, name: String};

	public function new() {
		super();
		setTitle("");
		final args = Sys.args();
		if (args.length > 0) {
			final fileName = args[0];

			if (sys.FileSystem.exists(fileName) && !sys.FileSystem.isDirectory(fileName)) {
				final fileData = sys.io.File.getBytes(fileName);
				openFile({name: fileName, bytes: fileData});
			}

			haxe.ui.HaxeUIApp.instance.icon = "haxeui-core/styles/default/haxeui_tiny.png";
		}
	}

	function setTitle(title) {
		var pad = if (title != "") {
			" - ";
		} else {
			"";
		};
		haxe.ui.HaxeUIApp.instance.title = 'bannerparse$pad$title';
	}

	function recurseTree(arc: Directory, root: Node, path: String) {
		final dir = arc.listDir(path);
		for (file in dir) {
			if (file.endsWith("/")) {
				// directory

				final added = root.addNode({
					text: file.substr(0, -1),
					id: file,
					userData: {
						type: switch (arc.type) {
							case U8: U8Dir;
							default: arc.type;
						}
					},
				});
				added.expanded = true;

				recurseTree(arc, added, '$path$file');
			} else {
				// file

				final data = arc.get('$path$file');

				var added = root.addNode({
					text: file,
					id: file,
					userData: {
						type: switch (arc.type) {
							case U8: U8File;
							default: arc.type;
						}
					},
				});

				final temp = FileTypes.parse(data);
				final file = temp.file;
				final type = temp.type;

				switch (type) {
					case IMD5 | LZ77 | U8:
						recurseTree(file, added, "/");
						added.expanded = true;
						file.drop();

					default:
						added.data.userData.type = switch (type) {
							case TPL:
								TPL;

							case IMET:
								IMET;

							default:
								"unk";
						}
				}
			}
		}
	}

	function refreshTree() {
		if (!banner.valid()) {
			return;
		}

		final node = tree.getNodes()[0];
		if (node != null) {
			tree.removeNode(node);
		}

		recurseTree(banner, tree, "/");
	}

	function openFile(file: {name: String, bytes: haxe.io.Bytes}) {
		final temp = FileTypes.parse(file.bytes);
		final newBanner = temp.file;

		if (newBanner != null && newBanner.valid()) {
			banner.drop();
			banner = newBanner;

			refreshTree();

			setTitle(file.name);
		} else {
			haxe.ui.containers.dialogs.Dialogs.messageBox('Failed to open file ${file.name}', "Error", "error");
		}
	}

	function menuOpen() {
		var dialogue = new haxe.ui.containers.dialogs.OpenFileDialog();
		dialogue.options = {
			readContents: true,
			title: "Open Banner File",
			readAsBinary: true,
			extensions: [BannerType, ContentType]
		};
		dialogue.onDialogClosed = function(event) {
			if (event.button == haxe.ui.containers.dialogs.Dialog.DialogButton.OK) {
				openFile(dialogue.selectedFiles[0]);
			}
		}
		dialogue.show();
	}

	function menuSave() {}

	function menuSaveAs() {}

	function menuExit() {}

	@:bind(menu, haxe.ui.containers.menus.Menu.MenuEvent.MENU_SELECTED)
	function onSelectMenu(e: haxe.ui.containers.menus.Menu.MenuEvent) {
		switch (e.menuItem.id) {
			case "open":
				menuOpen();

			case "save":
				menuSave();

			case "saveas":
				menuSaveAs();

			case "exit":
				menuExit();

			default:
				trace("Unhandled menu event " + e.menuItem.id);
		}
	}

	function getPath(root: Directory, path: String) {
		final components = path.split("/");
		for (i in 0...components.length) {
			final comp = components[i];
			final cur = components.slice(0, i).join("/");
			final rem = components.slice(i + 1).join("/");
			final dir = root.listDir(cur);
			if (dir.contains(comp)) {
				// file
				final data = root.get('$cur/$comp');

				if (rem == "") {
					return data;
				}

				final temp = FileTypes.parse(data);
				final file = temp.file;
				final type = temp.type;

				return switch (type) {
					case IMD5 | U8 | LZ77:
						final data = getPath(file, rem);
						file.drop();
						data;

					default:
						data;
				};
			} else if (!dir.contains('$comp/')) {
				// directory not found
				return null;
			}
		}
		return null;
	}

	function fillBannerPane(path: String, name: String, type: FileTypes) {
		final banner = if (path != "imet") {
			HaxeRS.Banner.parse(getPath(banner, path));
		} else {
			cast(banner, HaxeRS.Banner);
		};

		strings = banner.getTitles();
		titlelang.selectedIndex = English;
		titletext.text = strings[English];

		if (path != "imet") {
			banner.drop();
		}
	}

	function fillBannerFolderPane() {}

	function fillBannerFilePane(path: String, name: String, type: FileTypes) {
		currBannerFile = {
			path: path,
			name: name
		};
		filesection.text = currBannerFile.path;
		switch (type) {
			case TPL:
				final data = getPath(banner, path);
				final tpl = HaxeRS.TPL.parse(data);
				final rgba = tpl.getRGBA(0);
				final file = sys.io.File.write("tpl.rgba");
				file.write(rgba);
				file.close();

				final dims = tpl.getSize(0);

				tpl.drop();

			default:
		}
	}

	@:bind(fileexport, haxe.ui.events.MouseEvent.CLICK)
	function exportBannerFile(e: haxe.ui.events.MouseEvent) {
		final data = getPath(banner, currBannerFile.path);
		final ext = haxe.io.Path.extension(currBannerFile.path);

		var dialog = new haxe.ui.containers.dialogs.SaveFileDialog();
		dialog.options = {
			title: "Save Banner File",
			writeAsBinary: true,
			extensions: if (ext != "") {
				final label = fileTypeDescriptions[ext];
				if (label != null) {
					[{extension: ext, label: label}];
				} else {
					[];
				}
			} else {
				final ext = FileTypes.getFileExtension(data);
				if (ext != "") {
					final label = fileTypeDescriptions[ext];
					if (label != null) {
						[{extension: ext, label: label}];
					} else {
						[];
					}
				} else {
					[];
				}
			}
		}
		dialog.onDialogClosed = function(event) {
			if (event.button == haxe.ui.containers.dialogs.Dialog.DialogButton.OK) {
				haxe.ui.containers.dialogs.Dialogs.Dialogs.messageBox("File saved!", "Save Result",
					haxe.ui.containers.dialogs.MessageBox.MessageBoxType.TYPE_INFO);
			}
		}
		dialog.fileInfo = {
			name: currBannerFile.name,
			bytes: data
		}
		dialog.show();
	}

	@:bind(tree, haxe.ui.events.UIEvent.CHANGE)
	function onSelectTree(e: haxe.ui.events.UIEvent) {
		final node = tree.selectedNode;
		if (node != null) {
			final path = node.nodePath("text");
			final name = node.data.id;
			final type: FileTypes = node.data.userData.type;

			datapane.selectedIndex = switch (type) {
				case IMET:
					fillBannerPane(path, name, type);
					0;
				case U8Dir: 1;
				case U8File:
					fillBannerFilePane(path, name, type);
					2;
				default:
					fillBannerFilePane(path, name, type);
					2;
			}
		}
	}

	@:bind(titlelang, haxe.ui.events.UIEvent.CHANGE)
	function onSelectTitleLang(e: haxe.ui.events.UIEvent) {
		titletext.text = strings[titlelang.selectedIndex];
	}

	@:bind(tree, haxe.ui.events.MouseEvent.RIGHT_CLICK)
	function onRightClickTreeView(e: haxe.ui.events.MouseEvent) {
		var rightClickMenu = new BannerFileRightClick();
		rightClickMenu.left = e.screenX;
		rightClickMenu.top = e.screenY;
		haxe.ui.core.Screen.instance.addComponent(rightClickMenu);
	}
}
