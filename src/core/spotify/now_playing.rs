#[derive(Debug, Clone)]
pub struct NowPlaying {
    pub song: String,
    pub artists: Vec<String>,
    pub album: String,
    pub song_length: u64,
    pub progress: u64,
    pub shuffle: bool,
}

impl NowPlaying {
    pub fn get_song_length_string(&self) -> String {
        Self::milliseconds_to_string(self.song_length)
    }

    pub fn get_progress_string(&self) -> String {
        Self::milliseconds_to_string(self.progress)
    }

    pub fn get_shuffle_string(&self) -> String {
        match self.shuffle {
            true => "Shuffle: On".to_string(),
            false => "Shuffle: Off".to_string(),
        }
    }

    fn milliseconds_to_string(ms: u64) -> String {
        let total_seconds = ms / 1_000;
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;

        format!("{}:{:02}", minutes, seconds)
    }
}
