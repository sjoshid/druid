use druid::{AppLauncher, Widget, WidgetExt, WindowDesc};
use druid::widget::{Button, Flex, Label, TextBox};

use crate::{CalendarData, DateDetails};
use crate::widget::{Button, Flex, Label, TextBox};

fn main() {
	// describe the main window
	let main_window = WindowDesc::new(build_root_widget)
		.title(WINDOW_TITLE)
		.window_size((400.0, 400.0));

	// start the application
	AppLauncher::with_window(main_window)
		.launch(())
		.expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<()> {

}