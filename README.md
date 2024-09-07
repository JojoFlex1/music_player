# Music Player App 🎵

A simple music player app built with Rust using FLTK for the graphical user interface and SoLoud for audio playback. This app allows users to play, pause, stop, forward, and rewind songs, manage playlists, and search for songs within the music library.

## Features

- **Play, Pause, and Stop Audio:** Control your music playback with intuitive buttons.
- **Forward and Rewind:** Easily skip forward or go back to your favorite parts of a song.
- **Volume Control:** Adjust the volume to your liking.
- **Search Bar:** Quickly find songs in your library using the search functionality.
- **Playlist Management:** Organize your songs into playlists for easy access.

## Dependencies

To run the music player app, you need to have the following dependencies in your `Cargo.toml` file:

```toml
[dependencies]
fltk = "1.3"
rodio = "0.17"        
soloud = "1.0.5"
fltk-sys = { version = "1.4"}
fltk = "1.4" 
lazy_static = "1.4"
thiserror = "1.0"
```

### How to Use

- Add Songs: Load your music files into the app's library.
- Search: Use the search bar to find specific songs by name.
- Play Music: Select a song from the library and hit the play button to start listening.
- Control Playback: Use the pause, stop, forward, and rewind buttons to control your listening experience.
- Manage Playlists: Organize your songs into playlists for easy access.

### Contribution
Contributions are welcome! If you'd like to add features, fix bugs, or improve the codebase, feel free to submit a pull request.

### License
This project is licensed under the MIT License.
