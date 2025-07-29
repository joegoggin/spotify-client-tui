use clap::{Parser, Subcommand};
use color_eyre::eyre::eyre;

use crate::screens::{
    devices::DevicesScreen,
    library::LibraryScreen,
    now_playing::NowPlayingScreen,
    queue::QueueScreen,
    search::SearchScreen,
    view::{album::ViewAlbumScreen, artist::ViewArtistScreen},
    Screen,
};

use super::{
    app::{App, AppResult},
    spotify::{device::Device, player::SpotifyPlayer},
};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
/// Spotify Client TUI - Control Spotify From Your Terminal
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// Control Spotify with terminal commands
    Player {
        #[command(subcommand)]
        player_command: PlayerCommand,
    },
    /// Display information about song that is currently playing and control Pause/Play, Skip Song,
    /// Previous Song, and Shuffle
    NowPlaying,
    /// View information based on the song that is currently playing
    View {
        #[command(subcommand)]
        view_command: ViewCommand,
    },
    /// Search for an album, song, or artist on Spotify
    Search,
    /// View and edit current queue
    Queue,
    /// View songs, albums, artists and playlist from your Spotify library
    Library,
    /// Change what device Spotify is playing on
    Devices,
}

impl Command {
    pub fn is_player_command(&self) -> bool {
        match self {
            Command::Player { .. } => true,
            _ => false,
        }
    }

    pub async fn handle_command(
        &self,
        app: &mut App,
        current_screen: &mut Box<dyn Screen>,
    ) -> AppResult<()> {
        match self {
            Command::Player { player_command } => {
                player_command.handle_command(app).await?;
            }
            Command::NowPlaying => {
                app.history.prev.push(current_screen.clone_screen_box());
                *current_screen = Box::new(NowPlayingScreen::default());
            }
            Command::View { view_command } => match view_command {
                ViewCommand::Album => {
                    app.history.prev.push(current_screen.clone_screen_box());
                    *current_screen = Box::new(ViewAlbumScreen::default());
                }
                ViewCommand::Artist => {
                    app.history.prev.push(current_screen.clone_screen_box());
                    *current_screen = Box::new(ViewArtistScreen::default());
                }
            },
            Command::Queue => {
                app.history.prev.push(current_screen.clone_screen_box());
                *current_screen = Box::new(QueueScreen::default());
            }
            Command::Search => {
                app.history.prev.push(current_screen.clone_screen_box());
                *current_screen = Box::new(SearchScreen::default());
            }
            Command::Library => {
                app.history.prev.push(current_screen.clone());
                *current_screen = Box::new(LibraryScreen::default());
            }
            Command::Devices => {
                app.history.prev.push(current_screen.clone());
                *current_screen = Box::new(DevicesScreen::default());
            }
        }

        Ok(())
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum PlayerCommand {
    /// Toggle Pause And Play
    PausePlay,
    /// Play To Next Song
    NextSong,
    /// Play Previous Song
    PreviousSong,
    /// Toggle Shuffle
    Shuffle,
    /// List Available Devices
    Devices,
    /// Play On Device
    SetDevice { id: String },
}

impl PlayerCommand {
    pub async fn handle_command(&self, app: &mut App) -> AppResult<()> {
        match app.spotify_client.as_mut() {
            Some(mut spotify_client) => {
                let player = SpotifyPlayer::new();
                let mut device = Device::default();

                device.refresh(&mut spotify_client).await?;

                match self {
                    PlayerCommand::PausePlay => {
                        player.toggle_pause_play(&mut spotify_client).await?;
                    }
                    PlayerCommand::NextSong => {
                        player.next_song(&mut spotify_client).await?;
                    }
                    PlayerCommand::PreviousSong => {
                        player.previous_song(&mut spotify_client).await?;
                    }
                    PlayerCommand::Shuffle => {
                        player.toggle_shuffle(&mut spotify_client).await?;
                    }
                    PlayerCommand::Devices => {
                        device.print_devices(&mut spotify_client).await?;
                    }
                    PlayerCommand::SetDevice { id } => {
                        device
                            .set_current_device(&mut spotify_client, id.to_string())
                            .await?;
                    }
                }

                Ok(())
            }
            None => Err(eyre!("No `SpotifyClient` set on `App`.")),
        }
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum ViewCommand {
    /// View songs and albums by the artist that is currently playing
    Artist,
    /// View information on the album currently playing
    Album,
}
