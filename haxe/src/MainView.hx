package;

import haxe.ui.containers.VBox;

enum abstract FileTypeStack(Int) from Int to Int {
	var Banner;
	var BannerFolder;
	var BannerFile;
}

@:build(haxe.ui.ComponentBuilder.build("assets/main-view.xml"))
class MainView extends VBox {
	final BannerType:haxe.ui.containers.dialogs.Dialogs.FileDialogExtensionInfo = {extension: "bnr", label: "Banner Files"};
	final ContentType:haxe.ui.containers.dialogs.Dialogs.FileDialogExtensionInfo = {extension: "app", label: "Content Files"};

	var banner = new HaxeRS.Banner();

	public function new() {
		super();
		final args = Sys.args();
		if (args.length > 0) {
			final fileName = args[0];

			if (sys.FileSystem.exists(fileName) && !sys.FileSystem.isDirectory(fileName)) {
				final fileData = sys.io.File.getBytes(fileName);
				openFile({name: fileName, bytes: fileData});
			} else {
				trace("failed");
			}
		}
	}

	function recurseTree(root:haxe.ui.containers.TreeViewNode, path:String) {
		trace(path);
		final dir = banner.listDir(path);
		for (file in dir) {
			final isDir = file.charAt(file.length - 1) == '/';
			final added = root.addNode({
				text: file,
				id: file,
				/*, userData: {type: "U8Node"}*/
			});
			added.expanded = true;
			if (isDir) {
				added.data.text = added.text.substr(0, -1);
				added.data.icon = "haxeui-core/styles/shared/folder-light.png";
				sys.FileSystem.createDirectory('.$path$file');
				recurseTree(added, '$path$file');
			} else {
				final output = sys.io.File.write('.$path$file', true);
				output.write(banner.getFile('$path$file'));
				output.close();
			}
		}
	}

	function refreshTree() {
		if (!banner.valid()) {
			return;
		}

		tree.removeNode(tree.getNodes()[0]);
		final root = tree.addNode({
			text: "root",
			id: "",
			icon: "haxeui-core/styles/shared/folder-light.png"
			/*, userData: {type: "U8Root"}*/
		});
		root.expanded = true;
		recurseTree(root, "/");
	}

	function openFile(file:{name:String, bytes:haxe.io.Bytes}) {
		final newBanner = HaxeRS.Banner.parse(file.bytes);

		if (newBanner.valid()) {
			banner.update(newBanner);
		} else {
			haxe.ui.containers.dialogs.Dialogs.messageBox('Failed to open file ${file.name}', "Error", "error");
		}

		trace(banner);
		refreshTree();
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
	function onSelectMenu(e:haxe.ui.containers.menus.Menu.MenuEvent) {
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

	function fillBannerPane() {}

	function fillBannerFolderPane() {}

	function fillBannerFilePane() {}

	@:bind(tree, haxe.ui.events.UIEvent.CHANGE)
	function onSelectTree(e:haxe.ui.events.UIEvent) {
		final node = tree.selectedNode;
		if (node != null) {
			trace(node.nodePath("text"));
			trace(node.data.id);
			final isDir = node.data.id.charAt(node.data.id.length - 1) == '/';
			trace(isDir);

			datapane.selectedIndex = if (node.text == "root") {
				// fillBannerPane();
				Banner;
			} else if (isDir) {
				// fillBannerFolderPane();
				BannerFolder;
			} else {
				// fillBannerFilePane();
				BannerFile;
			}
		}
	}

	@:bind(titlelang, haxe.ui.events.UIEvent.CHANGE)
	function onSelectTitleLang(e:haxe.ui.events.UIEvent) {
		trace("changed lang");
	}
}
