use gtk4::prelude::*;
use gtk4::*;

pub async fn get_file_path(file: &gio::File) -> String {
    file.path().unwrap().as_path().display().to_string()
}

pub async fn get_entry_text(entry: &gtk4::Entry) -> String {
    let buffer = entry.buffer();
    buffer.text().into()
}

pub async fn set_entry_text(entry: &gtk4::Entry, new_text: &str) {
    let buffer = entry.buffer();
    buffer.set_text(new_text);
}

pub async fn choose_directory(parent_window: &gtk4::Window, title: &str) -> Option<String> {
    let chooser = FileDialog::builder()
        .modal(true)
        // .title("Choose the game installation directory")
        .title(title)
        .build();

    let result = chooser.select_folder_future(Some(parent_window)).await;

    if result.as_ref().err() != None {
        return None;
    }

    Some(get_file_path(&result.unwrap()).await)
}

pub async fn choose_file(parent_window: &gtk4::Window, title: &str) -> Option<String> {
    let chooser = FileDialog::builder()
        .modal(true)
        // .title("Choose the game installation directory")
        .title(title)
        .build();

    let result = chooser.open_future(Some(parent_window)).await;

    if result.as_ref().err() != None {
        return None;
    }

    Some(get_file_path(&result.unwrap()).await)
}
