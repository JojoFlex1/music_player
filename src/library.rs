use crate::models::Song;

pub struct MusicLibrary {
    songs: Vec<Song>,
}

impl MusicLibrary {
    
    pub fn new() -> Self {
        MusicLibrary { songs: Vec::new() }
    }

    pub fn add_song(&mut self, song: Song) {
        self.songs.push(song);
    }

    pub fn search(&self, query: &str) -> Vec<&Song> {
        self.songs.iter()
            .filter(|s| s.title.contains(query) || s.artist.contains(query))
            .collect()
    }
}

