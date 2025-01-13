use spotify_client_tui::{run, AppResult};

#[tokio::main]
async fn main() -> AppResult<()> {
    run().await?;
    Ok(())
}
