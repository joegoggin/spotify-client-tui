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
    /// View songs and albums by the artist that is currently playing
    ViewArtist,
    /// View information on the album currently playing
    ViewAlbum,
    /// View and edit current queue
    Queue,
    /// Search for an album, song, or artist on Spotify
    Search,
    /// View songs, albums, artists and playlist from your Spotify library
    Library,
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
    Device { id: String },
}
