use gtk::{Application, ApplicationWindow, traits::GtkWindowExt};

pub fn display_config_window(app: &Application) {
	let window = ApplicationWindow::builder()
		.application(app)
		.title("Mango Config")
		.modal(true)
		.default_height(50)
		.default_width(50)
		.build();

	window.present();
}