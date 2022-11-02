use std::{error, fmt::Display};

#[derive(Debug)]
pub enum Error {
    GetPlaybackSwitch(alsa::Error),
    SetPlaybackSwitch(alsa::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::GetPlaybackSwitch(e) => write!(f, "unable to get playback switch: {}", e),
            Error::SetPlaybackSwitch(e) => write!(f, "unable to set playback switch: {}", e),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::GetPlaybackSwitch(e) => Some(e),
            Error::SetPlaybackSwitch(e) => Some(e),
        }
    }
}
