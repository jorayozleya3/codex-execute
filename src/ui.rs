use gtk::prelude::*;
use gtk::{Button, Entry, Label, Window, WindowType};

pub struct App {
    window: Window,
    script_entry: Entry,
    output_label: Label,
}

impl App {
    pub fn new() -> Self {
        let window = Window::new(WindowType::Toplevel);
        window.set_title("Codex Executor");
        window.set_default_size(400, 200);

        let layout = gtk::Box::new(gtk::Orientation::Vertical, 5);
        window.add(&layout);

        let script_entry = Entry::new();
        layout.pack_start(&script_entry, false, false, 0);

        let execute_button = Button::with_label("Execute");
        layout.pack_start(&execute_button, false, false, 0);

        let output_label = Label::new("");
        layout.pack_start(&output_label, false, false, 0);

        execute_button.connect_clicked(move |_| {
            let script = script_entry.get_text().unwrap();
            let output = executor::run_script(&script).unwrap_or_else(|e| e);
            output_label.set_text(&output);
        });

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        window.show_all();
        Self { window, script_entry, output_label }
    }
}