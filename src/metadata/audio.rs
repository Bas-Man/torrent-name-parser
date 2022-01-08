use std::str::FromStr;

use crate::error::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Audio {
	MP3,
	Dolby51,
	Dual,
	Line,
	DTS,
	AAC,
	AC3
}

impl FromStr for Audio {
	type Err = Error;
	fn from_str(input: &str) -> Result<Self, Self::Err> {
		match input {
			"MP3" => Ok(Self::MP3),
			"DD51"|"DD5.1" => Ok(Self::Dolby51),
			"Dual-Audio"|"Dual Audio" => Ok(Self::Dual),
			"LiNE" => Ok(Self::Line),
			"DTS" => Ok(Self::DTS),
			"AAC"|"AAC2.0"|"AAC.2.0" => Ok(Self::AAC),
			"AC3"|"AC35.1"|"AC3.5.1" => Ok(Self::AC3),
			s => Err(Error::InvalidAudio(s.into()))
		}
	}
}

