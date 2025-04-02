use anyhow::Result;
use app_lib::App;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let res = App::default().run(&mut ratatui::init()).await;
    ratatui::restore();
    res
}