use std::io;

use audio_queue::AudioConsumer;

#[cfg(target_os="macos")]
mod coreaudio;
#[cfg(target_os="macos")]
pub use self::coreaudio::CoreAudioSink;

#[cfg(target_os="linux")]
mod alsa;
#[cfg(target_os="linux")]
pub use self::alsa::AlsaSink;


#[cfg(target_os="macos")]
pub type DefaultSink = CoreAudioSink;
#[cfg(target_os="linux")]
pub type DefaultSink = AlsaSink;

pub trait Sink {
    fn open(arg: Option<String>, queue: AudioConsumer<i16>) -> io::Result<Self>
        where Self: Sized;

    fn start(&mut self) -> io::Result<()>;
    fn pause(&mut self) -> io::Result<()>;
}
