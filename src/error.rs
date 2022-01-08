#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Couldn't find a title; matches: {0:?}")]
	Match(Vec<(&'static str, Option<String>)>),
	#[error("Invalid resolution: {0}")]
	InvalidResolution(String),
	#[error("Invalid quality: {0}")]
	InvalidQuality(String),
	#[error("Invalid codec: {0}")]
	InvalidCodec(String),
	#[error("Invalid audio: {0}")]
	InvalidAudio(String)
}

