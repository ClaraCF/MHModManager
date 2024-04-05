//use compress_tools::*;
//use std::fs::File;
//use std::path::Path;

//use gtk4::glib::clone;
use gtk4::prelude::*;
use gtk4::*;
use relm4::prelude::*;

mod filelist;
mod gamepath;
mod modlist;

struct AppModel {
    game_install_path: String,

    gamepath: Controller<gamepath::GamePathModel>,
    modlist: Controller<modlist::ModListModel>,
    //filelist: Controller<filelist::FileListModel>,
}

#[derive(Debug)]
pub enum AppInput {
    Ignore,

    SetPath(String),

    SelectMod(gtk4::ColumnView),

    AddNewMod(gtk4::ColumnView),
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Input = AppInput;
    type Output = ();
    type Init = String;
    //type Root = gtk4::Window;
    type Widgets = AppWidgets;

    view! {
        #[name = "root_window"]
        gtk4::Window {
            set_title: Some("Monster Hunter Mod Manager"),
            set_default_width: 960,
            set_default_height: 540,

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
                        //     sender.input(AppInput::AddNewMod(mods_columnview.clone()));
                        // }
                    },
                }
            }
        }
    }

    /// Initialize the UI and model
    fn init(
        game_install_path: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        // Game path chooser component
        let gamepath_input = (window.clone(), String::from("~/.steam"));
        let gamepath = gamepath::GamePathModel::builder()
            .launch(gamepath_input)
            .forward(sender.input_sender(), |msg| msg);
        // .forward(sender.input_sender(), |msg| match msg {
        //     gamepath::GamePathOutput::SetPath(new_path) => AppInput::SetPath(new_path),
        // });

        // Mod list compoonent
        let modlist =
            modlist::ModListModel::builder()
                .launch(())
                .forward(sender.input_sender(), |msg| match msg {
                    _ => AppInput::Ignore,
                });

        let model = AppModel {
            game_install_path,
            gamepath,
            modlist,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppInput::Ignore => {}

            AppInput::SetPath(new_path) => {
                self.game_install_path = new_path;
            }

            AppInput::SelectMod(mods_columnview) => {}

            AppInput::AddNewMod(mods_columnview) => {
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

                mods_columnview.insert_column(0, &name_column);
                mods_columnview.insert_column(1, &version_column);
            }
        }

        println!("Debug: {}", self.game_install_path);
    }
}

fn main() {
    let app = RelmApp::new("me.claracf.mh-mod-manager");
    app.run::<AppModel>(String::from(""));
}
