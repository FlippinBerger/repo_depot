use repo_depot::{app::App, fs};
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let _ = fs::set_up_dir();
    let mut term = ratatui::init();
    let mut app = App::new();
    let app_res = app.run(&mut term).await;
    ratatui::restore();
    app_res
}
