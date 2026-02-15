mod app;
mod ui;
mod sensors;

use relm4::prelude::*;
use app::AppModel;

fn main() {
    // Usamos RelmApp::<AppModel> para decirle a Rust qué modelo usar exactamente
    let app = RelmApp::new("com.nucita.config");
    app.run::<AppModel>(());
}