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
    GamePathSubmit(String),
    GamePathClear,
    GamePathBrowse,
}

struct AppWidgets {}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Input = AppInput;
    type Output = ();
    type Init = String;
    //type Root = gtk::Window;
    //type Widgets = AppWidgets;

    /// Initialize the root window
    /*fn init_root() -> Self::Root {
        gtk::Window::builder()
            .title("Monster Hunter Mod Manager")
            .default_width(960)
            .default_height(540)
            .build()
    }*/

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

                        connect_activate[sender] => move |entry| {
                            let buffer = entry.clone().buffer();
                            let text = buffer.text().to_string();
                            sender.input(AppInput::GamePathSubmit(text));
                        }
                    },

                    gtk::Button {
                        set_label: "Submit",
                        connect_clicked[sender, game_path_entry] => move |_| {
                            let buffer = game_path_entry.buffer();
                            let text = buffer.text().to_string();
                            sender.input(AppInput::GamePathSubmit(text));
                        }
                    },

                    gtk::Button {
                        set_label: "Clear",
                        connect_clicked[sender, game_path_entry] => move |_| {
                            let buffer = game_path_entry.buffer();
                            buffer.delete_text(0, None);
                            sender.input(AppInput::GamePathClear);
                        }
                    },

                    gtk::Button {
                        set_label: "Browse",
                        connect_clicked[sender, root_window, game_path_entry] => move |_| {
                            let cancellable = gio::Cancellable::new();
                            let chooser = FileDialog::builder()
                                .modal(true)
                                .title("Choose the game installation directory")
                                .build();
                            chooser.select_folder(
                                Some(&root_window), Some(&cancellable), clone!(
                                    @strong game_path_entry, @strong sender => move |result|{
                                        let path: String = result.unwrap().path().unwrap().as_path().display().to_string();
                                        let buffer = game_path_entry.buffer();
                                        buffer.set_text(path.clone());
                                        sender.input(AppInput::GamePathSubmit(path.clone()));
                                }))
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
            AppInput::GamePathSubmit(new_path) => {
                self.game_path = new_path;
            }
            AppInput::GamePathClear => {
                self.game_path = String::from("");
            }
            _ => {}
        }

        println!("Debug: {}", self.game_path);
    }
}

fn main() {
    let app = RelmApp::new("me.clara.mh-mod-manager");
    app.run::<AppModel>(String::from("~/.steam"));
}
