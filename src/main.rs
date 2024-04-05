//use compress_tools::*;
//use std::fs::File;
//use std::path::Path;

use gtk::glib::clone;
use gtk::prelude::*;
use gtk4::*;
use relm4::prelude::*;

struct AppModel {
    game_path: String,
}

#[derive(Debug)]
enum AppInput {
    GamePathSubmit(gtk4::Entry),
    GamePathClear(gtk4::Entry),
    GamePathBrowse(gtk4::Window, gtk4::Entry),
}

struct AppWidgets {}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Input = AppInput;
    type Output = ();
    type Init = String;
    //type Root = gtk::Window;
    //type Widgets = AppWidgets;

    view! {
        #[root]
        #[name = "root_window"]
        gtk::Window {
            set_title: Some("Monster Hunter Mod Manager"),
            set_default_width: 960,
            set_default_height: 540,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,
                set_hexpand: true,
                set_halign: Align::Fill,

                // Game path hbox
                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 5,
                    set_margin_all: 5,
                    set_hexpand: true,
                    set_halign: Align::Fill,

                    gtk::Label {
                        set_label: "Game path: ",
                    },

                    #[name ="game_path_entry"]
                    gtk::Entry {
                        set_hexpand: true,
                        set_halign: Align::Fill,
                        connect_activate[sender] => move |entry|{
                            sender.input(AppInput::GamePathSubmit(entry.clone()));
                        }
                    },

                    gtk::Button {
                        set_label: "Submit",
                        connect_clicked[sender, game_path_entry] => move |_| {
                            sender.input(AppInput::GamePathSubmit(game_path_entry.clone()));
                        }
                    },

                    gtk::Button {
                        set_label: "Clear",
                        connect_clicked[sender, game_path_entry] => move |_| {
                            sender.input(AppInput::GamePathClear(game_path_entry.clone()));
                        }
                    },

                    gtk::Button {
                        set_label: "Browse",
                        connect_clicked[sender, root_window, game_path_entry] => move |_| {
                            sender.input(
                                AppInput::GamePathBrowse(root_window.clone(), game_path_entry.clone())
                            );
                        }
                    },
                }
            }
        }
    }

    /// Initialize the UI and model
    fn init(
        game_path: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = AppModel { game_path };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppInput::GamePathSubmit(entry) => {
                // Grab the current text from the entry
                let buffer = entry.buffer();
                let text: String = buffer.text().into();

                // Set that text as the game path
                self.game_path = text;
            }
            AppInput::GamePathClear(entry) => {
                // Set the game path as an empty string
                self.game_path = String::from("");

                // Clear the entry
                let buffer = entry.buffer();
                buffer.set_text("");
            }
            AppInput::GamePathBrowse(root_window, entry) => {
                let cancellable = gio::Cancellable::new();
                let chooser = FileDialog::builder()
                    .modal(true)
                    .title("Choose the game installation directory")
                    .build();

                // Make the user select a folder
                chooser.select_folder(Some(&root_window), Some(&cancellable), move |result| {
                    // Get the absolute path of the selected folder
                    let path = result.unwrap().path().unwrap();
                    let path: String = path.as_path().display().to_string();

                    // Set it as the current buffer of the game path entry
                    let buffer = entry.buffer();
                    buffer.set_text(path);

                    // Send the submit signal
                    _sender.input(AppInput::GamePathSubmit(entry));
                })
            }
        }

        println!("Debug: {}", self.game_path);
    }
}

fn main() {
    let app = RelmApp::new("me.claracf.mh-mod-manager");
    app.run::<AppModel>(String::from(""));
}
