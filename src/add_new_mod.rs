//use compress_tools::*;
//use std::fs::File;
//use std::path::Path;

//use gtk4::glib::clone;
use gtk4::prelude::*;
use gtk4::*;
use relm4::prelude::*;

pub struct NewModWindowModel {
    hidden: bool,
}

#[derive(Debug)]
pub enum NewModWindowInput {
    Show,
    Cancel,
    Add,
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
            }
        }
    }

    /// Initialize the UI and model
    async fn init(
        hidden: Self::Init,
        window: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = NewModWindowModel {
            hidden,
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
            NewModWindowInput::Show => {
                self.hidden = false;
            },

            NewModWindowInput::Cancel => {

                self.hidden = true;
            },

            NewModWindowInput::Add => {

                self.hidden = true;
            },
        }
    }
}
