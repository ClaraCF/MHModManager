//use compress_tools::*;
//use std::fs::File;
//use std::path::Path;

//use gtk4::glib::clone;
use gtk4::prelude::*;
use gtk4::*;
use relm4::prelude::*;

use crate::utils;

pub struct NewModWindowModel {
    hidden: bool,
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
    type Output = ();
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
        let model = NewModWindowModel { hidden };

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
            NewModWindowInput::Show => {
                self.hidden = false;
            }

            NewModWindowInput::Cancel => {
                self.hidden = true;
            }

            NewModWindowInput::Add => {
                self.hidden = true;
            }

            NewModWindowInput::Browse(root, entry) => {
                utils::set_entry_text(
                    &entry,
                    &utils::choose_file(&root, "lol").await.unwrap()
                ).await;
            }
        }
    }
}
