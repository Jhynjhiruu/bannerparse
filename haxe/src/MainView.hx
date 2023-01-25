package;

import haxe.ui.containers.VBox;

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
			}
		}
	}

	function openFile(file:{name:String, bytes:haxe.io.Bytes}) {
		final newBanner = HaxeRS.Banner.parse(file.bytes);

		if (newBanner.valid()) {
			banner.update(newBanner);
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
}
