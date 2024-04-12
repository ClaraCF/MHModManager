//use compress_tools::*;
//use std::fs::File;
//use std::path::Path;

use relm4::prelude::*;

use tokio;

mod types;
mod utils;

mod components;
use components::*;


#[tokio::main]
async fn main() {
    use compress_tools::*;
    use std::fs::File;

    let mut source = File::open("test.zip").unwrap();
    let file_list = list_archive_files(&mut source).unwrap();

    for i in file_list {
        println!("{}", i);
    }

    // return;

    let app = RelmApp::new("me.claracf.mh-mod-manager");
    app.run_async::<app::AppModel>(String::from(""));
}
