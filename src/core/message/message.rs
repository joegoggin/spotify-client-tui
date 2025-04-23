use crate::screens::Screen;

#[derive(Clone)]
pub enum Message {
    ChangeScreen { new_screen: Box<dyn Screen> },
    GoToPrevScreen,
    GoToNextScreen,
    RefreshNowPlaying,
    SetAuthCode { code: String },
    PausePlay,
    Shuffle,
    NextSong,
    PrevSong,
    RefreshDevice,
    SetDevice { name: String, id: String },
    PlaySongOnAlbum { track_number: u64, album_id: String },
    RefreshSong,
    RefreshAlbum,
    RefreshArtist,
}
