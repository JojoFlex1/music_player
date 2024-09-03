use std::sync::{Arc, Mutex};
use soloud::{Soloud, Wav, Handle};
use soloud::prelude::*; 
use std::io::Read;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AudioError {
    #[error("Load error: {0}")]
    LoadError(String),
    #[error("Seek error: {0}")]
    SeekError(String),
}

lazy_static::lazy_static! {
    static ref SOLOUND: Arc<Mutex<Soloud>> = {
        let mut sl = Soloud::default().unwrap();
        sl.set_global_volume(50.0); // Set the default volume
        Arc::new(Mutex::new(sl))
    };
}

static CURRENT_HANDLE: Mutex<Option<Handle>> = Mutex::new(None);

pub fn load_audio(file_path: &str) -> Result<Wav, AudioError> {
    let mut wav = Wav::default();
    let mut file = std::fs::File::open(file_path)
        .map_err(|_| AudioError::LoadError("Unable to open file".into()))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|_| AudioError::LoadError("Unable to read file".into()))?;
    wav.load_mem(&buffer)
        .map_err(|_| AudioError::LoadError("Unable to load file into Wav".into()))?;
    Ok(wav)
}

pub fn play_audio(file_path: &str) -> Result<(), AudioError> {
    let wav = load_audio(file_path)?;
    let mut sl = SOLOUND.lock().unwrap();
    let handle = sl.play(&wav);
    *CURRENT_HANDLE.lock().unwrap() = Some(handle);
    Ok(())
}

pub fn pause_audio() {
    let mut sl = SOLOUND.lock().unwrap();
    if let Some(handle) = *CURRENT_HANDLE.lock().unwrap() {
        sl.pause(handle);
    }
}

pub fn stop_audio() {
    let mut sl = SOLOUND.lock().unwrap();
    if let Some(handle) = *CURRENT_HANDLE.lock().unwrap() {
        sl.stop(handle);
        *CURRENT_HANDLE.lock().unwrap() = None;
    }
}

pub fn forward_audio(seconds: f32) {
    let mut sl = SOLOUND.lock().unwrap();
    if let Some(handle) = *CURRENT_HANDLE.lock().unwrap() {
        let position = sl.stream_position(handle);
        sl.seek(handle, position + seconds as f64);
    }
}

pub fn rewind_audio(seconds: f32) {
    let mut sl = SOLOUND.lock().unwrap();
    if let Some(handle) = *CURRENT_HANDLE.lock().unwrap() {
        let position = sl.stream_position(handle);
        sl.seek(handle, position - seconds as f64);
    }
}
















