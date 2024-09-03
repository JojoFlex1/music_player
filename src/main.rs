use fltk::{app, button::Button, prelude::*, window::Window};
use soloud::Soloud;
use std::sync::Arc;
use std::sync::Mutex;
use lazy_static::lazy_static;
use thiserror::Error;

mod audio;
use audio::{play_audio, pause_audio, stop_audio, forward_audio, rewind_audio}; // Import only used functions

lazy_static! {
    static ref SOLOUND: Arc<Mutex<Soloud>> = {
        let mut sl = Soloud::default().unwrap();
        sl.set_global_volume(50.0); // Set the default volume
        Arc::new(Mutex::new(sl))
    };
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Audio error: {0}")]
    AudioError(#[from] audio::AudioError),
}

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Music Player");

    // Create buttons
    let mut play_button = Button::new(160, 210, 80, 40, "Play");
    let mut pause_button = Button::new(240, 210, 80, 40, "Pause");
    let mut stop_button = Button::new(320, 210, 80, 40, "Stop");
    let mut forward_button = Button::new(160, 260, 80, 40, "Forward");
    let mut rewind_button = Button::new(240, 260, 80, 40, "Rewind");

    // Set up button callbacks
    play_button.set_callback({
        move |_| {
            println!("Play button pressed");
            // Call your play function here
        }
    });

    pause_button.set_callback({
        move |_| {
            println!("Pause button pressed");
            // Call your pause function here
        }
    });

    stop_button.set_callback({
        move |_| {
            println!("Stop button pressed");
            // Call your stop function here
        }
    });

    forward_button.set_callback({
        move |_| {
            println!("Forward button pressed");
            // Call your forward function here
        }
    });

    rewind_button.set_callback({
        move |_| {
            println!("Rewind button pressed");
            // Call your rewind function here
        }
    });

    wind.end();
    wind.show();

    app.run().unwrap();
}




