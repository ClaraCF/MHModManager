//use gtk::glib::clone;
//use gtk4::prelude::*;
//use gtk4::*;
use relm4::{
    binding::{Binding, U8Binding},
    prelude::*,
    typed_view::{
        column::{LabelColumn, RelmColumn, TypedColumnView},
        OrdFn,
    },
    RelmObjectExt,
};

use crate::types::*;

struct NameColumn;
struct VersionColumn;
struct StatusColumn;

impl LabelColumn for NameColumn {
    type Item = Mod;
    type Value = String;

    const COLUMN_NAME: &'static str = "Name";

    const ENABLE_SORT: bool = true;
    const ENABLE_RESIZE: bool = true;

    fn get_cell_value(the_mod: &Self::Item) -> Self::Value {
        the_mod.name.clone()
    }
}

impl LabelColumn for VersionColumn {
    type Item = Mod;
    type Value = String;

    const COLUMN_NAME: &'static str = "Version";

    const ENABLE_SORT: bool = true;
    const ENABLE_RESIZE: bool = true;

    fn get_cell_value(the_mod: &Self::Item) -> Self::Value {
        the_mod.version.clone()
    }
}

impl LabelColumn for StatusColumn {
    type Item = Mod;
    type Value = String;

    const COLUMN_NAME: &'static str = "Status";

    const ENABLE_SORT: bool = true;
    const ENABLE_RESIZE: bool = true;

    fn get_cell_value(the_mod: &Self::Item) -> Self::Value {
        match the_mod.status {
            ModStatus::Enabled => String::from("Enabled"),
            ModStatus::Disabled => String::from("Disabled"),
            ModStatus::NotInstalled => String::from("Not Installed"),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct ModListWidgets {
}

pub struct ModListModel {
    widgets: ModListWidgets,
    view_wrapper: TypedColumnView::<Mod, gtk::SingleSelection>,
    mod_count: u32,
}

#[derive(Debug)]
pub enum ModListInput {
    Insert(Mod),
}

#[derive(Debug)]
pub enum ModListOutput {}


// #[relm4::component(pub)]
impl AsyncComponent for ModListModel {
    type Input = ModListInput;
    type Output = ModListOutput;
    type Init = u32;
    type Root = gtk4::ScrolledWindow;
    type Widgets = ModListWidgets;
    type CommandOutput = ();

    fn init_root() -> Self::Root {
        gtk4::ScrolledWindow::new()
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        // Initialize the ListView wrapper
        let mut view_wrapper = TypedColumnView::<Mod, gtk::SingleSelection>::new();
        view_wrapper.append_column::<NameColumn>();
        view_wrapper.append_column::<VersionColumn>();
        view_wrapper.append_column::<StatusColumn>();

        root.set_child(Some(&view_wrapper.view));

        let widgets = ModListWidgets {
        };

        let model = ModListModel {
            widgets: widgets.clone(),
            view_wrapper,
            mod_count: init,
        };

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        message: Self::Input,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            ModListInput::Insert(new_mod) => {
                self.view_wrapper.append(new_mod);
            }
        }
    }
}
