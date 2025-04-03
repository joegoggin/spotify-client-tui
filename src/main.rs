use spotify_client_tui::{App, AppResult};

#[tokio::main]
async fn main() -> AppResult<()> {
    let mut app = App::new()?;

    app.run().await?;

    Ok(())
}
