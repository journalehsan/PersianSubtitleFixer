use std::cell::{OnceCell, RefCell};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use adw::prelude::*;
use adw::{Application, ApplicationWindow};
use glib::{clone, Value};
use gtk4::gdk;
use gtk4::gio::{self, IOErrorEnum};
use gtk4::pango;
use gtk4::{
    self as gtk, Align, CssProvider, DropDown, FileDialog, FontDialog, Label,
    STYLE_PROVIDER_PRIORITY_APPLICATION,
};

use crate::encoding::{self, InputEncoding};

pub const APP_ID: &str = "ir.ehsan.PersianSubtitleFixer";

thread_local! {
    static CONTROLLER: OnceCell<Rc<AppController>> = OnceCell::new();
}

pub fn controller(app: &Application) -> Rc<AppController> {
    let app_clone = app.clone();
    CONTROLLER.with(|cell| cell.get_or_init(|| AppController::build(app_clone)).clone())
}

pub struct AppController {
    window: ApplicationWindow,
    text_view: gtk::TextView,
    text_buffer: gtk::TextBuffer,
    encoding_selector: DropDown,
    status_label: Label,
    font_provider: RefCell<Option<CssProvider>>,
    state: RefCell<AppState>,
}

#[derive(Clone, Debug)]
struct AppState {
    current_file: Option<PathBuf>,
    last_folder: PathBuf,
}

impl AppState {
    fn new() -> Self {
        Self {
            current_file: None,
            last_folder: default_folder(),
        }
    }
}

impl AppController {
    fn build(app: Application) -> Rc<Self> {
        let text_buffer = gtk::TextBuffer::new(None::<&gtk::TextTagTable>);
        let text_view = gtk::TextView::new();
        text_view.set_buffer(Some(&text_buffer));
        text_view.set_wrap_mode(gtk::WrapMode::WordChar);
        text_view.set_monospace(true);
        text_view.set_hexpand(true);
        text_view.set_vexpand(true);
        text_view.set_margin_top(8);
        text_view.set_margin_bottom(8);
        text_view.set_margin_start(8);
        text_view.set_margin_end(8);

        let encoding_labels: Vec<&str> = InputEncoding::ALL.iter().map(|enc| enc.label()).collect();
        let encoding_selector = DropDown::from_strings(&encoding_labels);
        encoding_selector.set_selected(0);
        encoding_selector.set_hexpand(true);
        encoding_selector.set_tooltip_text(Some("Select how the source file should be decoded."));

        let status_label = Label::new(Some("Ready"));
        status_label.set_xalign(0.0);
        status_label.add_css_class("dim-label");

        let window = ApplicationWindow::builder()
            .application(&app)
            .title("Persian Subtitle Fixer")
            .default_width(820)
            .default_height(540)
            .build();

        let controller = Rc::new(Self {
            window,
            text_view,
            text_buffer,
            encoding_selector,
            status_label,
            font_provider: RefCell::new(None),
            state: RefCell::new(AppState::new()),
        });

        controller.build_ui();
        controller
    }

    fn build_ui(self: &Rc<Self>) {
        let header = adw::HeaderBar::new();
        let title = adw::WindowTitle::new(
            "Persian Subtitle Fixer",
            "Convert Persian SRT subtitles to UTF-8",
        );
        header.set_title_widget(Some(&title));

        let open_button = gtk::Button::from_icon_name("document-open-symbolic");
        open_button.add_css_class("flat");
        open_button.set_tooltip_text(Some("Open subtitle file"));
        header.pack_start(&open_button);

        let save_button = gtk::Button::from_icon_name("document-save-symbolic");
        save_button.add_css_class("suggested-action");
        save_button.set_tooltip_text(Some("Save as UTF-8"));
        header.pack_start(&save_button);

        let about_button = gtk::Button::from_icon_name("help-about-symbolic");
        about_button.add_css_class("flat");
        about_button.set_tooltip_text(Some("About Persian Subtitle Fixer"));
        header.pack_end(&about_button);

        let font_button = gtk::Button::from_icon_name("preferences-desktop-font-symbolic");
        font_button.add_css_class("flat");
        font_button.set_tooltip_text(Some("Change preview font"));
        header.pack_end(&font_button);

        self.window.set_titlebar(Some(&header));

        let main_box = gtk::Box::new(gtk::Orientation::Vertical, 12);
        main_box.set_margin_top(12);
        main_box.set_margin_bottom(12);
        main_box.set_margin_start(12);
        main_box.set_margin_end(12);

        let controls_row = gtk::Box::new(gtk::Orientation::Horizontal, 12);
        controls_row.set_valign(Align::Center);
        let encoding_label = gtk::Label::new(Some("Source encoding"));
        encoding_label.set_xalign(0.0);
        controls_row.append(&encoding_label);
        controls_row.append(&self.encoding_selector);
        main_box.append(&controls_row);

        let scrolled = gtk::ScrolledWindow::builder()
            .hscrollbar_policy(gtk::PolicyType::Automatic)
            .vscrollbar_policy(gtk::PolicyType::Automatic)
            .build();
        scrolled.set_child(Some(&self.text_view));
        main_box.append(&scrolled);

        let status_row = gtk::Box::new(gtk::Orientation::Horizontal, 6);
        status_row.append(&self.status_label);
        main_box.append(&status_row);

        self.window.set_content(Some(&main_box));

        self.connect_actions(&open_button, &save_button, &about_button, &font_button);
        self.setup_drop_target();
        self.text_buffer
            .set_text("Drag an SRT file here or use Open to load subtitles.");
    }

    fn connect_actions(
        self: &Rc<Self>,
        open_button: &gtk::Button,
        save_button: &gtk::Button,
        about_button: &gtk::Button,
        font_button: &gtk::Button,
    ) {
        open_button.connect_clicked(clone!(@strong self as controller => move |_| {
            controller.open_dialog();
        }));

        save_button.connect_clicked(clone!(@strong self as controller => move |_| {
            controller.save_dialog();
        }));

        about_button.connect_clicked(clone!(@strong self as controller => move |_| {
            controller.show_about();
        }));

        font_button.connect_clicked(clone!(@strong self as controller => move |_| {
            controller.choose_font();
        }));
    }

    fn setup_drop_target(self: &Rc<Self>) {
        let drop_target = gtk::DropTarget::new(gdk::FileList::static_type(), gdk::DragAction::COPY);
        drop_target.set_preload(true);
        drop_target.connect_drop(
            clone!(@strong self as controller => @default-return false, move |_target, value, _, _| {
                controller.handle_drop(value)
            }),
        );
        self.text_view.add_controller(drop_target);
    }

    fn handle_drop(&self, value: &Value) -> bool {
        if let Ok(list) = value.get::<gdk::FileList>() {
            if let Some(file) = list.files().first() {
                if let Some(path) = file.path() {
                    let encoding = self.selected_encoding();
                    self.open_path(&path, encoding);
                    return true;
                }
            }
        }
        false
    }

    pub fn present(self: &Rc<Self>) {
        self.window.present();
    }

    pub fn open_initial(self: &Rc<Self>, path: &Path) {
        self.open_path(path, InputEncoding::Auto);
    }

    fn open_dialog(self: &Rc<Self>) {
        let dialog = FileDialog::builder()
            .title("Open Persian subtitle")
            .accept_label("Open")
            .modal(true)
            .build();

        let last_folder = self.state.borrow().last_folder.clone();
        let _ = dialog.set_initial_folder(Some(&gio::File::for_path(&last_folder)));
        let (filters, default_filter) = subtitle_filters();
        dialog.set_filters(Some(&filters));
        dialog.set_default_filter(Some(&default_filter));

        let weak = Rc::downgrade(self);
        dialog.open(
            Some(&self.window),
            None::<&gio::Cancellable>,
            move |result| {
                if let Some(controller) = weak.upgrade() {
                    match result {
                        Ok(file) => {
                            if let Some(path) = file.path() {
                                let encoding = controller.selected_encoding();
                                controller.open_path(&path, encoding);
                            }
                        }
                        Err(err) if err.matches(IOErrorEnum::Cancelled) => {}
                        Err(err) => controller.show_error("Unable to open file", &err.to_string()),
                    }
                }
            },
        );
    }

    fn save_dialog(self: &Rc<Self>) {
        let dialog = FileDialog::builder()
            .title("Save subtitle as UTF-8")
            .accept_label("Save")
            .modal(true)
            .build();

        let state = self.state.borrow();
        if let Some(current) = &state.current_file {
            let _ = dialog.set_initial_file(Some(&gio::File::for_path(current)));
        } else {
            let _ = dialog.set_initial_folder(Some(&gio::File::for_path(&state.last_folder)));
            dialog.set_initial_name(Some("subtitle.srt"));
        }
        drop(state);

        let weak = Rc::downgrade(self);
        dialog.save(
            Some(&self.window),
            None::<&gio::Cancellable>,
            move |result| {
                if let Some(controller) = weak.upgrade() {
                    match result {
                        Ok(file) => {
                            if let Some(path) = file.path() {
                                controller.save_to_path(&path);
                            }
                        }
                        Err(err) if err.matches(IOErrorEnum::Cancelled) => {}
                        Err(err) => controller.show_error("Unable to save file", &err.to_string()),
                    }
                }
            },
        );
    }

    fn choose_font(self: &Rc<Self>) {
        let dialog = FontDialog::builder()
            .title("Choose preview font")
            .modal(true)
            .build();

        let weak = Rc::downgrade(self);
        dialog.choose_font(
            Some(&self.window),
            None::<&pango::FontDescription>,
            None::<&gio::Cancellable>,
            move |result| {
                if let Some(controller) = weak.upgrade() {
                    match result {
                        Ok(desc) => controller.apply_font(&desc),
                        Err(err) if err.matches(IOErrorEnum::Cancelled) => {}
                        Err(err) => {
                            controller.show_error("Unable to change font", &err.to_string())
                        }
                    }
                }
            },
        );
    }

    fn apply_font(&self, desc: &pango::FontDescription) {
        let css = format!("textview {{ font: {}; }}", desc.to_string());
        let mut slot = self.font_provider.borrow_mut();
        let provider = slot.get_or_insert_with(|| {
            let provider = CssProvider::new();
            if let Some(display) = gdk::Display::default() {
                gtk::style_context_add_provider_for_display(
                    &display,
                    &provider,
                    STYLE_PROVIDER_PRIORITY_APPLICATION,
                );
            }
            provider
        });
        provider.load_from_data(&css);
    }

    fn open_path(&self, path: &Path, encoding: InputEncoding) {
        match encoding::read_file(path, encoding) {
            Ok(decoded) => {
                self.text_buffer.set_text(&decoded.text);
                let mut state = self.state.borrow_mut();
                state.current_file = Some(path.to_path_buf());
                if let Some(parent) = path.parent() {
                    state.last_folder = parent.to_path_buf();
                }
                drop(state);
                self.set_encoding_selection(decoded.used_encoding);
                self.update_title(Some(path), decoded.used_encoding);
                self.set_status(&format!(
                    "Opened {} as {}",
                    display_name(path),
                    decoded.used_encoding.label()
                ));
            }
            Err(err) => self.show_error("Unable to open file", &err.to_string()),
        }
    }

    fn save_to_path(&self, path: &Path) {
        let start = self.text_buffer.start_iter();
        let end = self.text_buffer.end_iter();
        let text = self.text_buffer.text(&start, &end, true);
        match encoding::write_utf8(path, text.as_str()) {
            Ok(_) => {
                let mut state = self.state.borrow_mut();
                state.current_file = Some(path.to_path_buf());
                if let Some(parent) = path.parent() {
                    state.last_folder = parent.to_path_buf();
                }
                drop(state);
                self.update_title(Some(path), InputEncoding::Utf8);
                self.set_status(&format!("Saved {} as UTF-8", display_name(path)));
            }
            Err(err) => self.show_error("Unable to save file", &err.to_string()),
        }
    }

    fn selected_encoding(&self) -> InputEncoding {
        let index = self.encoding_selector.selected() as usize;
        InputEncoding::ALL
            .get(index)
            .copied()
            .unwrap_or(InputEncoding::Auto)
    }

    fn set_encoding_selection(&self, encoding: InputEncoding) {
        if let Some(index) = InputEncoding::ALL.iter().position(|&enc| enc == encoding) {
            self.encoding_selector.set_selected(index as u32);
        }
    }

    fn update_title(&self, path: Option<&Path>, encoding: InputEncoding) {
        if let Some(path) = path {
            if let Some(name) = path.file_name().and_then(|f| f.to_str()) {
                self.window.set_title(Some(&format!(
                    "{} — Persian Subtitle Fixer ({})",
                    name,
                    encoding.label()
                )));
                return;
            }
        }
        self.window
            .set_title(Some("Persian Subtitle Fixer — UTF-8"));
    }

    fn set_status(&self, message: &str) {
        self.status_label.set_label(message);
    }

    fn show_about(&self) {
        let about = adw::AboutWindow::builder()
            .transient_for(&self.window)
            .application_name("Persian Subtitle Fixer")
            .developer_name("Ehsan Tork")
            .developers(vec!["Ehsan Tork", "Contributors"])
            .issue_url("https://github.com/ehsator/PersianSubtitleFixer/issues")
            .license_type(gtk::License::Gpl30)
            .comments("Convert Persian SRT subtitles from Windows-1256 to UTF-8.")
            .version("0.4.0")
            .build();
        about.present();
    }

    fn show_error(&self, heading: &str, body: &str) {
        let dialog = adw::MessageDialog::builder()
            .transient_for(&self.window)
            .heading(heading)
            .body(body)
            .build();
        dialog.add_response("close", "Close");
        dialog.set_close_response("close");
        dialog.present();
    }
}

fn subtitle_filters() -> (gio::ListStore, gtk::FileFilter) {
    let store = gio::ListStore::new::<gtk::FileFilter>();
    let filter = gtk::FileFilter::new();
    filter.set_name(Some("Subtitle files"));
    filter.add_pattern("*.srt");
    filter.add_pattern("*.sub");
    store.append(&filter);
    (store, filter)
}

fn display_name(path: &Path) -> String {
    path.file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("subtitle")
        .to_string()
}

fn default_folder() -> PathBuf {
    dirs::home_dir()
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| PathBuf::from("."))
}
