mod app;
mod ui;
mod sensors;

use relm4::prelude::*;
use app::AppModel;

fn main() {
    let app = RelmApp::new("com.nucita.config");
    app.run::<AppModel>(());
}