package;

import haxe.ui.HaxeUIApp;

class Main {
	public static function main() {
		var app = new HaxeUIApp();
		app.ready(function() {
			app.addComponent(new MainView());

			try {
				app.start();
			} catch (e:Any) {
				trace(e);
			}
		});
	}
}
