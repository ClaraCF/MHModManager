use gtk4::prelude::*;
use relm4::{
    prelude::*,
    typed_view::column::{RelmColumn, TypedColumnView},
};

use crate::types::*;

struct NameColumn;
struct VersionColumn;
struct StatusColumn;

impl RelmColumn for NameColumn {
    type Root = gtk4::Box;
    type Widgets = gtk4::Label;
    type Item = Mod;

    const COLUMN_NAME: &'static str = "Name";

    const ENABLE_RESIZE: bool = false;
    const ENABLE_EXPAND: bool = true;

    fn setup(_item: &gtk::ListItem) -> (Self::Root, Self::Widgets) {
        let container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        let label = gtk4::Label::new(None);

        container.append(&label);

        (container, label)
    }

    fn bind(item: &mut Self::Item, widgets: &mut Self::Widgets, _root: &mut Self::Root) {
        widgets.set_text(&item.name);
    }
}

impl RelmColumn for VersionColumn {
    type Root = gtk4::Box;
    type Widgets = gtk4::Label;
    type Item = Mod;

    const COLUMN_NAME: &'static str = "Version";

    const ENABLE_RESIZE: bool = false;
    const ENABLE_EXPAND: bool = true;

    fn setup(_item: &gtk::ListItem) -> (Self::Root, Self::Widgets) {
        let container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        let label = gtk4::Label::new(None);

        container.append(&label);

        (container, label)
    }

    fn bind(item: &mut Self::Item, widgets: &mut Self::Widgets, _root: &mut Self::Root) {
        widgets.set_text(&item.version);
    }
}

impl RelmColumn for StatusColumn {
    type Root = gtk4::Box;
    type Widgets = gtk4::Label;
    type Item = Mod;

    const COLUMN_NAME: &'static str = "Status";

    const ENABLE_RESIZE: bool = false;
    const ENABLE_EXPAND: bool = true;

    fn setup(_item: &gtk::ListItem) -> (Self::Root, Self::Widgets) {
        let container = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
        let label = gtk4::Label::new(None);

        container.append(&label);

        (container, label)
    }

    fn bind(item: &mut Self::Item, widgets: &mut Self::Widgets, _root: &mut Self::Root) {
        widgets.set_text(match item.status {
            ModStatus::Enabled => "Enabled",
            ModStatus::Disabled => "Disabled",
            ModStatus::NotInstalled => "Not Installed",
        });
    }
}

// #[allow(dead_code)]
// #[derive(Clone)]
// pub struct ModListWidgets {
// }

pub struct ModListModel {
    // widgets: ModListWidgets,
    view_wrapper: TypedColumnView<Mod, gtk::SingleSelection>,
    // mod_count: u32,
}

#[derive(Debug)]
pub enum ModListInput {
    Insert(Mod),
}

#[derive(Debug)]
pub enum ModListOutput {}

#[relm4::component(pub, async)]
impl AsyncComponent for ModListModel {
    type Input = ModListInput;
    type Output = ModListOutput;
    type Init = ();
    // type Root = gtk4::ScrolledWindow;
    type Widgets = ModListWidgets;
    type CommandOutput = ();

    view! {
        gtk4::ScrolledWindow {
            // set_hscrollbar_policy: gtk::PolicyType::Never,
            set_halign: gtk4::Align::Fill,
        }
    }

    // fn init_root() -> Self::Root {
    //     gtk4::ScrolledWindow::new()
    // }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        // Initialize the ListView wrapper
        let mut view_wrapper = TypedColumnView::<Mod, gtk::SingleSelection>::new();

        view_wrapper.view.set_reorderable(false);

        view_wrapper.append_column::<NameColumn>();
        view_wrapper.append_column::<VersionColumn>();
        view_wrapper.append_column::<StatusColumn>();

        root.set_child(Some(&view_wrapper.view));

        // let widgets = ModListWidgets {
        // };

        let widgets = view_output!();

        // widgets.scrolled_window.set_child(Some(&view_wrapper.view));

        let model = ModListModel {
            // widgets,
            view_wrapper,
            // mod_count: init,
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
