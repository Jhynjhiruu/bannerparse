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

	function toString() {
		return switch (this) {
			case IMET: "IMET";
			case IMD5: "IMD5";
			case LZ77: "LZ77";
			case U8: "U8";
			case U8File: "U8File";
			case U8Dir: "U8Dir";
			case TPL: "TPL";
			default: "unk";
		}
	}

	@:from
	static function fromBytes(other: haxe.io.Bytes): FileTypes {
		return other.toHex().toUpperCase();
	}
}

@:build(haxe.ui.macros.ComponentMacros.build("assets/bannerfile-rightclick.xml"))
class BannerFileRightClick extends haxe.ui.containers.menus.Menu {
	@:bind(rightclick_export, haxe.ui.events.MouseEvent.CLICK)
	function onRightClickExport(e: haxe.ui.events.MouseEvent) {
		trace(e.target);
	}
}

interface Directory {
	public function listDir(dir: String = ""): Array<String>;
	public function get(path: String = ""): haxe.io.Bytes;
}

@:build(haxe.ui.ComponentBuilder.build("assets/main-view.xml"))
class MainView extends haxe.ui.containers.VBox {
	final BannerType: haxe.ui.containers.dialogs.Dialogs.FileDialogExtensionInfo = {extension: "bnr", label: "Banner Files"};
	final ContentType: haxe.ui.containers.dialogs.Dialogs.FileDialogExtensionInfo = {extension: "app", label: "Content Files"};

	var banner = new HaxeRS.Banner();
	var rootArc = new HaxeRS.U8();
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
			} else {
				trace("failed");
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

	function recurseTree(arc: HaxeRS.U8, root: haxe.ui.containers.TreeViewNode, path: String, prepath: String) {
		trace(path + ", " + prepath);
		final dir = arc.listDir(path);
		for (file in dir) {
			final isDir = file.endsWith("/");
			var added = root.addNode({
				text: file,
				id: file,
				userData: {type: U8File},
			});
			added.expanded = true;
			if (isDir) {
				added.data.text = added.text.substr(0, -1);
				added.data.icon = "haxeui-core/styles/shared/folder-light.png";
				added.data.userData.type = U8Dir;
				sys.FileSystem.createDirectory('$prepath$path$file');
				recurseTree(arc, added, '$path$file', prepath);
			} else {
				var data = arc.get('$path$file');
				var extra = "";

				while (true) {
					var magic = data.sub(0, 4);
					switch (magic: FileTypes) {
						case IMD5:
							final imd5 = HaxeRS.IMD5.parse(data);
							data = imd5.get();
							imd5.drop();

							added = added.addNode({
								text: "imd5",
								id: "imd5/",
								userData: {type: IMD5},
							});
							added.expanded = true;
							sys.FileSystem.createDirectory('$prepath$path$file$extra');
							extra = '$extra/imd5';

						case LZ77:
							data = HaxeRS.NintyLZ77.decompress(data);

							added = added.addNode({
								text: "lz77",
								id: "lz77/",
								userData: {type: LZ77},
							});
							added.expanded = true;
							sys.FileSystem.createDirectory('$prepath$path$file$extra');
							extra = '$extra/lz77';

						case U8:
							final u8 = HaxeRS.U8.parse(data);
							sys.FileSystem.createDirectory('$prepath$path$file$extra');
							recurseTree(u8, added, "/", '$prepath$path$file$extra');
							u8.drop();
							break;

						default:
							trace('${magic.toHex()}, $IMD5, $LZ77, $U8');
							final output = sys.io.File.write('$prepath$path$file$extra', true);

							added.data.userData.type = switch (magic: FileTypes) {
								case TPL:
									final tpl = HaxeRS.TPL.parse(data);
									tpl.drop();

									TPL;

								default:
									"unk";
							}

							output.write(data);
							output.close();
							break;
					}
				}
			}
		}
	}

	function refreshTree() {
		if (!banner.valid()) {
			return;
		}

		if (!rootArc.valid()) {
			return;
		}

		final node = tree.getNodes()[0];
		if (node != null) {
			tree.removeNode(node);
		}

		final root = tree.addNode({
			text: "root",
			id: "",
			icon: "haxeui-core/styles/shared/folder-light.png",
			userData: {type: IMET}
		});
		root.expanded = true;
		recurseTree(rootArc, root, "/", ".");
	}

	function openFile(file: {name: String, bytes: haxe.io.Bytes}) {
		final newBanner = HaxeRS.Banner.parse(file.bytes);

		if (newBanner.valid()) {
			banner.update(newBanner);

			final newU8 = HaxeRS.U8.parse(banner.get());
			if (newU8.valid()) {
				rootArc.update(newU8);

				refreshTree();

				strings = banner.getTitles();
				titlelang.selectedIndex = English;
				titletext.text = strings[English];

				setTitle(file.name);
			} else {
				haxe.ui.containers.dialogs.Dialogs.messageBox('Failed to parse root archive in file ${file.name}', "Error", "error");
			}
		} else {
			haxe.ui.containers.dialogs.Dialogs.messageBox('Failed to open file ${file.name}', "Error", "error");
		}

		trace(banner);
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

	function _getPath(root: Directory, path: String) {
		trace("checking " + path);
		final components = path.split("/");
		for (i in 0...components.length) {
			final comp = components[i];
			final cur = components.slice(0, i).join("/");
			final rem = components.slice(i + 1).join("/");
			trace("\tchecking " + comp + " rem " + rem);
			final dir = root.listDir(cur);
			trace("\t\tdir = " + dir);
			if (dir.contains(comp)) {
				// file
				final data = root.get('$cur/$comp');

				var magic = data.sub(0, 4);
				return switch (magic: FileTypes) {
					case IMD5:
						final imd5 = HaxeRS.IMD5.parse(data);
						final data = _getPath(imd5, rem);
						imd5.drop();
						data;

					case LZ77:
						final lz77 = new HaxeRS.LZ77(data);
						_getPath(lz77, rem);

					case U8:
						final u8 = HaxeRS.U8.parse(data);
						final data = _getPath(u8, rem);
						u8.drop();
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

	function getPath(root: Directory, path: String) {
		final path = if (path.startsWith("root/")) {
			path.substr(5);
		} else {
			path;
		}
		return _getPath(root, path);
	}

	function fillBannerPane() {}

	function fillBannerFolderPane() {}

	function fillBannerFilePane(path: String, name: String, type: FileTypes) {
		currBannerFile = {
			path: path,
			name: name
		};
		filesection.text = currBannerFile.path;
		switch (type) {
			case TPL:
				final data = getPath(rootArc, path);
				trace(data);
				final tpl = HaxeRS.TPL.parse(data);

				final dims = tpl.getSize(0);
				trace(dims);

				tpl.drop();

			default:
		}
	}

	@:bind(fileexport, haxe.ui.events.MouseEvent.CLICK)
	function exportBannerFile(e: haxe.ui.events.MouseEvent) {
		final data = getPath(rootArc, currBannerFile.path);

		var dialog = new haxe.ui.containers.dialogs.SaveFileDialog();
		dialog.options = {
			title: "Save Banner File",
			writeAsBinary: true,
			extensions: [BannerType, ContentType]
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
					fillBannerPane();
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
		trace("rightclick");
		var menu = new BannerFileRightClick();
		menu.left = e.screenX;
		menu.top = e.screenY;
		haxe.ui.core.Screen.instance.addComponent(menu);
	}
}
