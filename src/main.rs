use fltk::{
    app,
    button::Button,
    frame::Frame,
    group::Pack,
    prelude::*,
    slider::Slider,
    window::Window,
};
use soloud::Soloud;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

mod audio;
use audio::*;


lazy_static! {
    static ref SOLOUND: Arc<Mutex<Soloud>> = {
        let mut sl = Soloud::default().unwrap();
        sl.set_global_volume(50.0);
        Arc::new(Mutex::new(sl))
    };
}

fn main() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 800, 600, "Music Player");

    let mut pack = Pack::new(10, 10, 780, 580, "");
    pack.set_spacing(10);

    let mut play_button = Button::new(0, 0, 100, 40, "Play");
    let mut pause_button = Button::new(110, 0, 100, 40, "Pause");
    let mut stop_button = Button::new(220, 0, 100, 40, "Stop");
    let mut forward_button = Button::new(330, 0, 100, 40, "Forward");
    let mut rewind_button = Button::new(440, 0, 100, 40, "Rewind");

    let mut volume_slider = Slider::new(0, 50, 300, 40, "Volume");
    volume_slider.set_range(0.0, 10.0);
    volume_slider.set_value(4.0);
    
    let mut status = Frame::new(0, 100, 780, 40, "Status");

    pack.end();
    wind.end();
    wind.show();

    play_button.set_callback(move |_| {
        play_audio("C:\\Users\\USER\\Music"); 
        status.set_label("Playing");
    });

    pause_button.set_callback(move |_| {
        pause_audio();
        status.set_label("Paused");
    });

    stop_button.set_callback(move |_| {
        stop_audio();
        status.set_label("Stopped");
    });

    forward_button.set_callback(move |_| {
        forward_audio(10.0); 
        status.set_label("Forwarding");
    });

    rewind_button.set_callback(move |_| {
        rewind_audio(5.0); 
        status.set_label("Rewinding");
    });

    volume_slider.set_callback(move |s| {
        set_volume(s.value());
    });

    app.run().unwrap();
}


