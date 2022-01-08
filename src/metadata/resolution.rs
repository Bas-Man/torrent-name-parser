use std::str::FromStr;

use crate::error::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Resolution {
	R240i,
	R240p,
	R320i,
	R320p,
	R480i,
	R480p,
	R576i,
	R576p,
	R720i,
	R720p,
	R1080i,
	R1080p,
	R2160i,
	R2160p,
	R4k,
}

impl FromStr for Resolution {
	type Err = Error;
	fn from_str(input: &str) -> Result<Self, Self::Err> {
		match input {
			"240i"|"240I" => Ok(Self::R240i),
			"240p"|"240P" => Ok(Self::R240p),
			"320i"|"320I" => Ok(Self::R320i),
			"320p"|"320P" => Ok(Self::R320p),
			"480i"|"480I" => Ok(Self::R480i),
			"480p"|"480P" => Ok(Self::R480p),
			"576i"|"576I" => Ok(Self::R576i),
			"576p"|"576P" => Ok(Self::R576p),
			"720i"|"720I" => Ok(Self::R720i),
			"720p"|"720P" => Ok(Self::R720p),
			"1080i"|"1080I" => Ok(Self::R1080i),
			"1080p"|"1080P" => Ok(Self::R1080p),
			"2160i"|"2160I" => Ok(Self::R2160i),
			"2160p"|"2160P" => Ok(Self::R2160p),
			"4k"|"4K" => Ok(Self::R4k),
			s => Err(Error::InvalidResolution(s.into()))
		}
	}
}

