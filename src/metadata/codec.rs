use std::str::FromStr;

use crate::error::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Codec {
	X264,
	X265,
	Xvid
}

impl FromStr for Codec {
	type Err = Error;
	fn from_str(input: &str) -> Result<Self, Self::Err> {
		match input {
			"X264"|"x264"|"H264"|"h264"|"H.264"|"h.264" => Ok(Self::X264),
			"X265"|"x265"|"H265"|"h265"|"H.265"|"h.265" => Ok(Self::X265),
			"HEVC"|"HEVc"|"HEvC"|"HEvc"|"HeVC"|"HeVc"|"HevC"|"Hevc" => Ok(Self::X265),
			"hEVC"|"hEVc"|"hEvC"|"hEvc"|"heVC"|"heVc"|"hevC"|"hevc" => Ok(Self::X265),
			"XVID"|"XVId"|"XViD"|"XVid"|"XvID"|"XvId"|"XviD"|"Xvid" => Ok(Self::Xvid),
			"xVID"|"xVId"|"xViD"|"xVid"|"xvID"|"xvId"|"xviD"|"xvid" => Ok(Self::Xvid),
			s => Err(Error::InvalidCodec(s.into()))
		}
	}
}

