//use compress_tools::*;
//use std::fs::File;
//use std::path::Path;

//use gtk4::glib::clone;
use gtk4::prelude::*;
use gtk4::*;
use relm4::prelude::*;

use crate::types::*;
use crate::utils;

use crate::components::*;

pub struct NewModWindowModel {
    hidden: bool,

    name_entry: Option<gtk4::Entry>,
    version_entry: Option<gtk4::Entry>,
    filepath_entry: Option<gtk4::Entry>,
}

#[derive(Debug)]
pub enum NewModWindowInput {
    Show,
    Cancel,
    Add,
    Browse(gtk4::Window, gtk4::Entry),
}

#[relm4::component(async, pub)]
impl AsyncComponent for NewModWindowModel {
    type Init = bool;
    type Input = NewModWindowInput;
    type Output = app::AppInput;
    //type Root = gtk4::Window;
    type Widgets = NewModWindowWidgets;
    type CommandOutput = ();

    view! {
        #[name = "root_window"]
        gtk4::Window {
            set_title: Some("Add new mod"),
            set_default_width: 960,
            set_default_height: 540,

            #[watch]
            set_visible: !model.hidden,

            gtk4::Box {
                set_orientation: gtk4::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 50,
                //set_margin_top: 20,
                set_hexpand: true,
                set_halign: Align::Fill,

                // Title
                gtk4::Label {
                    set_margin_all: 20,
                    set_margin_top: 0,
                    set_markup: "<span font=\"36\">Add new mod</span>",
                },

                // Name field
                gtk4::Box {
                    set_orientation: gtk4::Orientation::Horizontal,
                    set_spacing: 10,
                    set_margin_all: 5,
                    set_margin_horizontal: 100,
                    set_margin_vertical: 20,
                    set_hexpand: true,
                    set_halign: Align::Fill,

                        gtk4::Label {
                            set_text: "   Name:",
                        },

                        #[name = "name_entry"]
                        gtk4::Entry {
                            set_hexpand: true,
                            set_halign: gtk4::Align::Fill,
                            connect_activate[sender] => move |entry|{
                                //sender.input(GamePathInput::Submit(entry.clone()));
                            }
                        },
                },

                // Version field
                gtk4::Box {
                    set_orientation: gtk4::Orientation::Horizontal,
                    set_spacing: 10,
                    set_margin_all: 5,
                    set_margin_horizontal: 100,
                    set_hexpand: true,
                    set_halign: Align::Fill,
                    set_margin_vertical: 20,

                        gtk4::Label {
                            set_text: "Version:",
                        },

                        #[name = "version_entry"]
                        gtk4::Entry {
                            set_hexpand: true,
                            set_halign: gtk4::Align::Fill,
                            connect_activate[sender] => move |entry|{
                                //sender.input(GamePathInput::Submit(entry.clone()));
                            }
                        },
                },

                // File field
                gtk4::Box {
                    set_orientation: gtk4::Orientation::Horizontal,
                    set_spacing: 10,
                    set_margin_all: 5,
                    set_margin_horizontal: 100,
                    set_hexpand: true,
                    set_halign: Align::Fill,
                    set_margin_vertical: 20,

                        gtk4::Label {
                            set_text: "         File:",
                        },

                        #[name = "file_entry"]
                        gtk4::Entry {
                            set_hexpand: true,
                            set_halign: gtk4::Align::Fill,
                            connect_activate[sender] => move |entry|{
                                //sender.input(GamePathInput::Submit(entry.clone()));
                            }
                        },

                    gtk4::Button {
                        set_label: "Browse",
                        connect_clicked[sender, root_window, file_entry] => move |_| {
                            sender.input(
                                NewModWindowInput::Browse(root_window.clone(), file_entry.clone())
                            );
                        }
                    },
                },

                // Buttons
                gtk4::Box {
                    set_margin_vertical: 20,
                    set_orientation: gtk4::Orientation::Horizontal,
                    set_spacing: 20,
                    set_margin_all: 5,
                    set_hexpand: true,
                    set_halign: Align::Center,

                    gtk4::Box {
                        set_margin_vertical: 20,
                        set_orientation: gtk4::Orientation::Horizontal,
                        set_spacing: 20,
                        set_margin_all: 5,
                        set_hexpand: true,
                        set_halign: Align::Fill,

                        gtk4::Button {
                            set_hexpand: true,
                            set_label: "Cancel",
                            connect_clicked[sender] => move |_|{
                                sender.input(NewModWindowInput::Cancel);
                            }
                        },

                        gtk4::Button {
                            set_hexpand: true,
                            set_label: "   Add   ",
                            connect_clicked[sender] => move |_|{
                                sender.input(NewModWindowInput::Add);
                            }
                        },
                    }
                }
            }
        }
    }

    /// Initialize the UI and model
    async fn init(
        hidden: Self::Init,
        window: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let mut model = NewModWindowModel {
            hidden,
            name_entry: None,
            version_entry: None,
            filepath_entry: None,
        };

        let widgets = view_output!();

        model.name_entry = Some(widgets.name_entry.clone());
        model.version_entry = Some(widgets.version_entry.clone());
        model.filepath_entry = Some(widgets.file_entry.clone());

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        message: Self::Input,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            NewModWindowInput::Show => {
                self.hidden = false;
            }

            NewModWindowInput::Cancel => {
                self.hidden = true;
            }

            NewModWindowInput::Add => {
                self.hidden = true;

                let new_mod = Mod::new(
                    utils::get_entry_text(self.name_entry.as_ref().unwrap()).await,
                    utils::get_entry_text(self.version_entry.as_ref().unwrap()).await,
                    utils::get_entry_text(self.filepath_entry.as_ref().unwrap()).await,
                );

                _sender.output(Self::Output::InsertMod(new_mod)).unwrap();
            }

            NewModWindowInput::Browse(root, entry) => {
                utils::set_entry_text(&entry, &utils::choose_file(&root, "lol").await.unwrap())
                    .await;
            }
        }
    }
}
