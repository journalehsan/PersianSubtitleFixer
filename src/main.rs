mod app;
mod encoding;

use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use adw::prelude::*;
use app::{controller, APP_ID};
use gtk4::gio;

fn main() -> glib::ExitCode {
    let _ = adw::init();

    let pending_path = Rc::new(RefCell::new(std::env::args_os().nth(1).map(PathBuf::from)));

    let application = adw::Application::builder()
        .application_id(APP_ID)
        .flags(gio::ApplicationFlags::HANDLES_OPEN)
        .build();

    application.connect_activate(glib::clone!(@strong pending_path => move |app| {
        let controller = controller(app);
        controller.present();
        if let Some(path) = pending_path.borrow_mut().take() {
            controller.open_initial(&path);
        }
    }));

    application.connect_open(
        glib::clone!(@strong pending_path => move |app, files, _hint| {
            pending_path.borrow_mut().take();
            let controller = controller(app);
            controller.present();
            if let Some(file) = files.first() {
                if let Some(path) = file.path() {
                    controller.open_initial(&path);
                }
            }
        }),
    );

    application.run()
}
