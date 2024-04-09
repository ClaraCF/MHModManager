use gtk4::prelude::*;
use gtk4::*;

pub async fn choose_directory(parent_window: &gtk4::Window) -> Option<String> {
    let chooser = FileDialog::builder()
        .modal(true)
        .title("Choose the game installation directory")
        .build();

    let result = chooser.select_folder_future(Some(parent_window)).await;

    if result.as_ref().err() != None {
        return None;
    }

    Some(
        result
            .unwrap()
            .path()
            .unwrap()
            .as_path()
            .display()
            .to_string(),
    )
}
