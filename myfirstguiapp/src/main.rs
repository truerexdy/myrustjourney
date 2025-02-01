use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Label};

fn main() {
    let application = Application::builder()
        .application_id("com.rexdy.myfirstguiapp")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Hello, World!")
            .default_width(400)
            .default_height(300)
            .build();

        let label = Label::builder()
            .label("This is some text.")
            .build();

        window.set_child(Some(&label));

        window.show();
    });

    application.run();
}
