use clap::{Parser, Subcommand};

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

#[derive(Subcommand, Debug, Clone)]
pub enum ViewCommand {
    /// View songs and albums by the artist that is currently playing
    Artist,
    /// View information on the album currently playing
    Album,
}
