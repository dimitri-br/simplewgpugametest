use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::{self, Sender};
use std::thread;

use rodio::Sink;

// Audio Struct
// Plays audio from files loaded on a seperate thread. Audio are queued, so for each instance of audio, make a new audio instance!
pub struct Audio{
    control_channel: Sender<(bool, String, f32)>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Audio {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let handle = thread::spawn(move || {
            let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            sink.pause();
            while let Ok((should_play, path, volume)) =  rx.recv(){
                // Make sure we have a valid path
                if path != ""{
                    let file = File::open(path).unwrap();
                    let audio_source = rodio::Decoder::new(BufReader::new(file)).unwrap();
                    sink.append(audio_source);
                }
                sink.set_volume(volume);
                if should_play {
                    sink.play();
                } else {
                    sink.pause();
                }
            }
        });
        log::info!("Sucessfully created new audio thread");

        Self {
            control_channel: tx,
            handle: Some(handle)
        }
    }

    pub fn play(&self, path: &str, volume: f32) {
        self.control_channel.send((true, path.to_string(), volume)).unwrap();
    }
    pub fn pause(&self) {
        self.control_channel.send((false, "".to_string(), 0.0)).unwrap();
    }
}

// Safely drop Audio, so if an entity with audio gets destroyed, we don't mess up the threading system
impl Drop for Audio{
    fn drop(&mut self){
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
            log::info!("Sucessfully shut down audio thread");
        }
    }
}