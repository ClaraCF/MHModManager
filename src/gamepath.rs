//use gtk::glib::clone;
use gtk4::prelude::*;
use gtk4::*;
use relm4::prelude::*;

pub struct GamePathModel {
    root_window: gtk4::Window,
    game_path: String,
}

#[derive(Debug)]
pub enum GamePathInput {
    SubmitPath(gtk4::Entry),
    ClearPath(gtk4::Entry),
    BrowsePath(gtk4::Entry),
}

#[derive(Debug)]
pub enum GamePathOutput {}

pub struct GamePathWidgets {}

#[relm4::component(pub)]
impl SimpleComponent for GamePathModel {
    type Input = GamePathInput;
    type Output = GamePathOutput;
    type Init = (gtk4::Window, String);
    //type Root = gtk4::Box;
    //type Widgets = GamePathWidgets;

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
                    sender.input(GamePathInput::SubmitPath(entry.clone()));
                }
            },

            gtk4::Button {
                set_label: "Submit",
                connect_clicked[sender, game_path_entry] => move |_| {
                    sender.input(GamePathInput::SubmitPath(game_path_entry.clone()));
                }
            },

            gtk4::Button {
                set_label: "Clear",
                connect_clicked[sender, game_path_entry] => move |_| {
                    sender.input(GamePathInput::ClearPath(game_path_entry.clone()));
                }
            },

            gtk4::Button {
                set_label: "Browse",
                connect_clicked[sender, game_path_entry] => move |_| {
                    sender.input(
                        GamePathInput::BrowsePath(game_path_entry.clone())
                    );
                }
            },
        },
    }

    /// Initialize the UI and model
    fn init(
        input: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        // Define the model of the component
        let model = GamePathModel {
            root_window: input.0,
            game_path: input.1,
        };

        // Collect the widgets of the component
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            GamePathInput::SubmitPath(entry) => {
                // Grab the current text from the entry
                let buffer = entry.buffer();
                let text: String = buffer.text().into();

                // Set that text as the game path
                self.game_path = text;
            }

            GamePathInput::ClearPath(entry) => {
                // Set the game path as an empty string
                self.game_path = String::from("");

                // Clear the entry
                let buffer = entry.buffer();
                buffer.set_text("");
            }

            GamePathInput::BrowsePath(entry) => {
                let cancellable = gio::Cancellable::new();
                let chooser = FileDialog::builder()
                    .modal(true)
                    .title("Choose the game installation directory")
                    .build();

                // Make the user select a folder
                chooser.select_folder(Some(&self.root_window), Some(&cancellable), move |result| {
                    // Get the absolute path of the selected folder
                    let path = result.unwrap().path().unwrap();
                    let path: String = path.as_path().display().to_string();

                    // Set it as the current buffer of the game path entry
                    let buffer = entry.buffer();
                    buffer.set_text(path);

                    // Send the submit signal
                    _sender.input(GamePathInput::SubmitPath(entry));
                })
            }

        }
    }
}
