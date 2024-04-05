//use gtk::glib::clone;
//use gtk4::prelude::*;
//use gtk4::*;
use relm4::prelude::*;

pub struct ModListModel {}

#[derive(Debug)]
pub enum ModListInput {}

#[derive(Debug)]
pub enum ModListOutput {}

pub struct ModListWidgets {
    header_factory: gtk4::SignalListItemFactory,

    column_view: gtk4::ColumnView,

    name_column: gtk4::ColumnViewColumn,
    version_column: gtk4::ColumnViewColumn,
    status_column: gtk4::ColumnViewColumn,
}

// #[relm4::component(pub)]
impl SimpleComponent for ModListModel {
    type Input = ModListInput;
    type Output = ModListOutput;
    type Init = ();
    type Root = gtk4::ScrolledWindow;
    type Widgets = ModListWidgets;

    // view! {
    //     gtk4::ScrolledWindow {
    //         set_hscrollbar_policy: gtk4::PolicyType::Never,
    //         set_min_content_height: 360,
    //         set_vexpand: true,
    //
    //         #[name = "mods_columnview"]
    //         gtk4::ColumnView {
    //         },
    //     }
    // }

    fn init_root() -> Self::Root {
        gtk4::ScrolledWindow::new()
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ModListModel {};

        let header_factory = gtk4::SignalListItemFactory::new();

        let column_view = gtk4::ColumnView::builder()
            .hexpand(true)
            .halign(gtk4::Align::Fill)
            .header_factory(&header_factory)
            .build();

        let name_column = gtk4::ColumnViewColumn::builder()
            .title("Name")
            .factory(&header_factory)
            .expand(true)
            .build();

        let version_column = gtk4::ColumnViewColumn::builder()
            .title("Version")
            .factory(&header_factory)
            .expand(true)
            .build();

        let status_column = gtk4::ColumnViewColumn::builder()
            .title("Status")
            .factory(&header_factory)
            .expand(true)
            .build();

        column_view.insert_column(0, &name_column);
        column_view.insert_column(1, &version_column);
        column_view.insert_column(2, &status_column);

        root.set_child(Some(&column_view));

        let widgets = ModListWidgets {
            header_factory,

            column_view,

            name_column,
            version_column,
            status_column,
        };

        ComponentParts { model, widgets }
    }
}
