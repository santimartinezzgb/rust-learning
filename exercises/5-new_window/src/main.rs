// packages
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

fn main() {
    // Create a new application
    let app = Application::new(Some("com.example.basic"), Default::default());

    // Connect to "activate" signal of `app`
    app.connect_activate(|app| {
        // Create a new window and set its properties
        let win = ApplicationWindow::new(app);
        win.set_title(Some("Hello from Rust"));
        win.set_default_size(300, 100);
        win.show();
    });

    // Run the application
    app.run();
}

// sudo apt install libgtk-4-dev --> cargo run

// Git LFS: search archives >50MB (example: *.rlib, *.bin)
// # git lfs track "*.rlib"
// # git add .gitattributes
// # git commit -m "Configurate LFS"
// # git push
// Hide repo slowness/size, download on demand

// References
// https://git-lfs.com/
