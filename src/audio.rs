use soloud::*;
use std::sync::{Arc, Mutex};
use thiserror::Error;

lazy_static::lazy_static! {
    static ref SOLOUND: Arc<Mutex<Soloud>> = {
        let sl = Soloud::default().unwrap();
        sl.set_global_volume(50.0);
        Arc::new(Mutex::new(sl))
    };
}

#[derive(Error, Debug)]
pub enum AudioError {
    #[error("Failed to load audio file: {0}")]
    LoadError(String),
    #[error("Playback error: {0}")]
    PlaybackError(String),
    #[error("Seek error: {0}")]
    SeekError(String),
    #[error("Volume error: {0}")]
    VolumeError(String),
}

pub fn play_audio(file_path: &str) -> Result<(), AudioError> {
    let mut wav = Wav::default();
    wav.load(file_path).map_err(|_| AudioError::LoadError(file_path.to_string()))?;

    let sl = SOLOUND.lock().unwrap();
    sl.play(&wav);
    Ok(())
}

pub fn pause_audio() -> Result<(), AudioError> {
    let sl = SOLOUND.lock().unwrap();
    sl.pause_all();
    Ok(())
}

pub fn stop_audio() -> Result<(), AudioError> {
    let sl = SOLOUND.lock().unwrap();
    sl.stop_all();
    Ok(())
}

pub fn forward_audio(seconds: f32) -> Result<(), AudioError> {
    let sl = SOLOUND.lock().unwrap();

    let current_position = sl.stream_time().unwrap_or(0.0);
    let new_position = (current_position + seconds).clamp(0.0, sl.stream_time_max().unwrap_or(0.0));
    sl.seek(new_position).map_err(|_| AudioError::SeekError("Failed to seek forward".to_string()))?;
    Ok(())
}

pub fn rewind_audio(seconds: f32) -> Result<(), AudioError> {
    let sl = SOLOUND.lock().unwrap();

    let current_position = sl.stream_time().unwrap_or(0.0);
    let new_position = (current_position - seconds).clamp(0.0, sl.stream_time_max().unwrap_or(0.0));
    sl.seek(new_position).map_err(|_| AudioError::SeekError("Failed to seek backward".to_string()))?;
    Ok(())
}

pub fn set_volume(volume: f32) -> Result<(), AudioError> {
    let sl = SOLOUND.lock().unwrap();
    sl.set_global_volume(volume);
    Ok(())
}

pub fn increase_volume(amount: f32) -> Result<(), AudioError> {
    let sl = SOLOUND.lock().unwrap();
    let new_volume = (sl.global_volume() + amount).clamp(0.0, 100.0);
    sl.set_global_volume(new_volume);
    Ok(())
}

pub fn decrease_volume(amount: f32) -> Result<(), AudioError> {
    let sl = SOLOUND.lock().unwrap();
    let new_volume = (sl.global_volume() - amount).clamp(0.0, 100.0);
    sl.set_global_volume(new_volume);
    Ok(())
}


