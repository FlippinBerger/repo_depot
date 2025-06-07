use repo_depot::app::App;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut term = ratatui::init();
    let mut app = App::new();
    let app_res = app.run(&mut term).await;
    ratatui::restore();
    app_res
}
