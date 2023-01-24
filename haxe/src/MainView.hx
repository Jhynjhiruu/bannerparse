package;

import haxe.ui.containers.VBox;

@:build(haxe.ui.ComponentBuilder.build("assets/main-view.xml"))
class MainView extends VBox {
	final BannerType:haxe.ui.containers.dialogs.Dialogs.FileDialogExtensionInfo = {extension: ".bnr", label: "Banner Files"};

	public function new() {
		super();
	}

	function menuOpen() {
		var dialogue = new haxe.ui.containers.dialogs.OpenFileDialog();
		dialogue.options = {
			readContents: true,
			title: "Open Banner File",
			readAsBinary: true,
			extensions: [BannerType]
		};
		dialogue.onDialogClosed = function(event) {
			if (event.button == haxe.ui.containers.dialogs.Dialog.DialogButton.OK) {
				HaxeRS.parseBanner(dialogue.selectedFiles[0].bytes);
			}
		}
		dialogue.show();
	}

	function menuSave() {}

	function menuSaveAs() {}

	function menuExit() {}

	@:bind(menu, haxe.ui.containers.menus.Menu.MenuEvent.MENU_SELECTED)
	private function onSelectMenu(e:haxe.ui.containers.menus.Menu.MenuEvent) {
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
