//use compress_tools::*;
//use std::fs::File;
//use std::path::Path;

use add_new_mod::NewModWindowInput;
//use gtk4::glib::clone;
use gtk4::prelude::*;
use gtk4::*;
use relm4::prelude::*;

mod utils;

mod add_new_mod;
mod filelist;
mod gamepath;
mod modlist;

use futures::prelude::*;
use tokio;

struct Mod {
    name: String,
    version: String,
    filepath: String,
}

struct AppModel {
    game_install_path: String,

    gamepath: AsyncController<gamepath::GamePathModel>,
    modlist: Controller<modlist::ModListModel>,
    //filelist: Controller<filelist::FileListModel>,
    new_mod_window: AsyncController<add_new_mod::NewModWindowModel>,
}

#[derive(Debug)]
pub enum AppInput {
    Ignore,

    SetPath(String),

    SelectMod(gtk4::ColumnView),

    // AddNewMod(gtk4::ColumnView),
    AddNewMod,
}

#[relm4::component(async)]
impl AsyncComponent for AppModel {
    type Input = AppInput;
    type Output = ();
    type Init = String;
    //type Root = gtk4::Window;
    type Widgets = AppWidgets;
    type CommandOutput = ();

    view! {
        #[name = "root_window"]
        gtk4::Window {
            set_title: Some("Monster Hunter Mod Manager"),
            set_default_width: 1280,
            set_default_height: 720,

            gtk4::Box {
                set_orientation: gtk4::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,
                set_hexpand: true,
                set_halign: Align::Fill,

                // Game path hbox
                model.gamepath.widget(),

                gtk4::Box {
                    set_orientation: gtk4::Orientation::Horizontal,
                    set_spacing: 15,
                    set_margin_all: 5,
                    set_margin_horizontal: 10,
                    set_hexpand: true,
                    set_halign: Align::Fill,

                    // Mod list frame
                    gtk4::Frame {
                        set_hexpand: true,
                        set_vexpand: true,
                        set_halign: Align::Fill,
                        set_label: Some("Mods"),

                        //#[name(mods_columnview)]
                        model.modlist.widget(),
                    },

                    // Files frame
                    gtk4::Frame {
                        set_hexpand: true,
                        set_halign: Align::Fill,
                        set_label: Some("Mod Files"),
                    },
                },

                // Buttons
                gtk4::Box {
                    set_orientation: gtk4::Orientation::Horizontal,
                    set_spacing: 10,
                    set_margin_all: 5,
                    set_hexpand: true,
                    set_halign: Align::Center,

                    gtk4::Button {
                        set_label: "Disable",
                    },

                    gtk4::Button {
                        set_label: "Uninstall",
                    },

                    gtk4::Button {
                        set_label: "Add new mod",
                        set_css_classes: &["suggested-action"],

                        // connect_clicked[sender, mods_columnview] => move |_|{
                        connect_clicked[sender] => move |_|{
                            // sender.input(AppInput::AddNewMod(mods_columnview.clone()));
                            sender.input(AppInput::AddNewMod);
                        }
                    },
                }
            }
        }
    }

    /// Initialize the UI and model
    async fn init(
        game_install_path: Self::Init,
        window: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        // Game path chooser component
        let gamepath_input = (window.clone(), String::from("~/.steam"));
        let gamepath = gamepath::GamePathModel::builder()
            .launch(gamepath_input)
            .forward(sender.input_sender(), |msg| msg);

        // Mod list compoonent
        let modlist = modlist::ModListModel::builder()
            .launch(())
            .forward(sender.input_sender(), |_msg| AppInput::Ignore);

        // Add new mod window
        let new_mod_window = add_new_mod::NewModWindowModel::builder()
            .launch(true)
            .forward(sender.input_sender(), |_msg| AppInput::Ignore);

        let model = AppModel {
            game_install_path,
            gamepath,
            modlist,

            new_mod_window,
        };

        let widgets = view_output!();
        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        message: Self::Input,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            AppInput::Ignore => {}

            AppInput::SetPath(new_path) => {
                self.game_install_path = new_path;
            }

            #[allow(dead_code)]
            AppInput::SelectMod(mods_columnview) => {}

            // AppInput::AddNewMod(mods_columnview) => {
            AppInput::AddNewMod => {
                self.new_mod_window
                    .sender()
                    .send(NewModWindowInput::Show)
                    .unwrap();

                let factory = gtk4::SignalListItemFactory::new();

                let name_column = gtk4::ColumnViewColumn::builder()
                    .title("Name")
                    .factory(&factory)
                    .expand(true)
                    .build();

                let version_column = gtk4::ColumnViewColumn::builder()
                    .title("Version")
                    .factory(&factory)
                    .expand(true)
                    .build();

                //mods_columnview.insert_column(0, &name_column);
                //mods_columnview.insert_column(1, &version_column);
            }
        }

        // println!("Debug: {}", self.game_install_path);
    }
}

#[tokio::main]
async fn main() {
    let app = RelmApp::new("me.claracf.mh-mod-manager");
    app.run_async::<AppModel>(String::from(""));
}
