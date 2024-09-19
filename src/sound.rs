use rodio::{source::SineWave, OutputStream, Sink, Source};
use std::time::Duration;

pub fn play_tone(frequency: u32, duration_ms: u64) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let source = SineWave::new(frequency).take_duration(Duration::from_millis(duration_ms));
    sink.append(source);
    sink.sleep_until_end();
}
