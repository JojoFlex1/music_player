use fltk::{app, button::Button, input::Input, frame::Frame, prelude::*, window::Window};
use crate::audio::{play_audio, pause_audio, stop_audio, forward_audio, rewind_audio, increase_volume, decrease_volume};
use crate::library::MusicLibrary;
use crate::models::Song;

pub fn run_app() {
    let app = app::App::default();

    let mut wind = Window::new(100, 100, 400, 350, );

    let mut frame = Frame::new(160, 300, 80, 40, "");

    let mut search_input = Input::new(50, 50, 300, 30, "");

    let mut btn_play = Button::new(50, 100, 80, 40, "Play");
    let mut btn_pause = Button::new(150, 100, 80, 40, "Pause");
    let mut btn_stop = Button::new(250, 100, 80, 40, "Stop");
    let mut btn_forward = Button::new(50, 150, 80, 40, "Forward");
    let mut btn_rewind = Button::new(150, 150, 80, 40, "Rewind");

    let mut btn_volume_up = Button::new(50, 200, 80, 40, "Vol Up");
    let mut btn_volume_down = Button::new(150, 200, 80, 40, "Vol Down");

    let mut btn_search = Button::new(150, 250, 80, 40, "Search");

    let mut library = MusicLibrary::new();
    library.add_song(Song {
        title: String::from("Song"),
        artist: String::from("Artist"),
        album: String::from("Album"),
        file_path: String::from(r""C:\\Users\\USER\\Music""), // Change to location of music
    });

    btn_play.set_callback(move |_| {
        play_audio(r""C:\\Users\\USER\\Music""); // Change to location of the music
        frame.set_label("Playing...");
    });

    btn_pause.set_callback(move |_| {
        pause_audio();
        frame.set_label("Paused");
    });

    btn_stop.set_callback(move |_| {
        stop_audio();
        frame.set_label("Stopped");
    });

    btn_forward.set_callback(move |_| {
        forward_audio(10.0); 
        frame.set_label("Forwarded");
    });

    btn_rewind.set_callback(move |_| {
        rewind_audio(5.0); 
        frame.set_label("Rewinded");
    });

    btn_volume_up.set_callback(move |_| {
        increase_volume(1.0); 
        frame.set_label("Volume Up");
    });

    btn_volume_down.set_callback(move |_| {
        decrease_volume(1.0); 
        frame.set_label("Volume Down");
    });

    btn_search.set_callback(move |_| {
        let query = search_input.value();
        let results = library.search(&query);
        frame.set_label(&format!("Found {} songs", results.len()));
    });

    wind.end();
    wind.show();

    app.run().unwrap();
}

