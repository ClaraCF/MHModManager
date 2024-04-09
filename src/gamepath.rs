//use gtk::glib::clone;
use futures::prelude::*;
use gtk4::prelude::*;
use gtk4::*;
use relm4::prelude::*;
//use tokio::*;

use crate::utils;

pub struct GamePathModel {
    root_window: gtk4::Window,
    default_game_path: String,
}

#[derive(Debug)]
pub enum GamePathInput {
    Submit(gtk4::Entry),
    Clear(gtk4::Entry),
    Browse(gtk4::Entry),
}

// #[derive(Debug)]
// pub enum GamePathOutput {
//     SetPath(String),
// }

pub struct GamePathWidgets {}

#[relm4::component(pub, async)]
impl AsyncComponent for GamePathModel {
    type Input = GamePathInput;
    // type Output = GamePathOutput;
    type Output = crate::AppInput;
    type Init = (gtk4::Window, String);
    //type Root = gtk4::Box;
    //type Widgets = GamePathWidgets;
    type CommandOutput = ();

    view! {
        gtk4::Box {
            set_orientation: gtk4::Orientation::Horizontal,
            set_spacing: 5,
            set_margin_all: 5,
            set_margin_horizontal: 10,
            set_margin_top: 10,
            set_hexpand: true,
            set_halign: gtk4::Align::Fill,

            gtk4::Label {
                set_label: "Game path: ",
            },

            #[name ="game_path_entry"]
            gtk4::Entry {
                set_hexpand: true,
                set_halign: gtk4::Align::Fill,
                connect_activate[sender] => move |entry|{
                    sender.input(GamePathInput::Submit(entry.clone()));
                }
            },

            gtk4::Button {
                set_label: "Submit",
                connect_clicked[sender, game_path_entry] => move |_| {
                    sender.input(GamePathInput::Submit(game_path_entry.clone()));
                }
            },

            gtk4::Button {
                set_label: "Clear",
                connect_clicked[sender, game_path_entry] => move |_| {
                    sender.input(GamePathInput::Clear(game_path_entry.clone()));
                }
            },

            gtk4::Button {
                set_label: "Browse",
                connect_clicked[sender, game_path_entry] => move |_| {
                    sender.input(
                        GamePathInput::Browse(game_path_entry.clone())
                    );
                }
            },
        },
    }

    /// Initialize the UI and model
    async fn init(
        input: Self::Init,
        window: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        // Define the model of the component
        let model = GamePathModel {
            root_window: input.0,
            default_game_path: input.1,
        };

        // Collect the widgets of the component
        let widgets = view_output!();

        // Set the default entry path
        set_entry_text(&widgets.game_path_entry, &model.default_game_path).await;

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        message: Self::Input,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            GamePathInput::Submit(entry) => {
                let text = get_entry_text(&entry).await;
                _sender.output(Self::Output::SetPath(text)).unwrap();
            }

            GamePathInput::Clear(entry) => {
                // Clear the entry
                set_entry_text(&entry, "").await;
                _sender.input(GamePathInput::Submit(entry));
            }

            GamePathInput::Browse(entry) => {
                match utils::choose_directory(&self.root_window).await {
                    Some(path) => {
                        println!("[DEBUG] gamepath.rs: {}", path);
                        set_entry_text(&entry, &path).await;
                        _sender.input(GamePathInput::Submit(entry));
                    }
                    None => println!("[DEBUG] gamepath.rs: None"),
                }
            }
        }
    }
}

async fn get_entry_text(entry: &gtk4::Entry) -> String {
    let buffer = entry.buffer();
    buffer.text().into()
}

async fn set_entry_text(entry: &gtk4::Entry, new_text: &str) {
    let buffer = entry.buffer();
    buffer.set_text(new_text);
}
