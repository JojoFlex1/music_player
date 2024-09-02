// models.rs

pub struct Song {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub file_path: String,
    pub duration: u32, 
    pub genre: Option<String>, 
}
