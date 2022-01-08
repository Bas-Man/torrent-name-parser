use std::str::FromStr;

use crate::error::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Quality {
	HDTV,
	Cam,
	BluRay,
	TS,
	HD,
	DVD,
	Web,
}

impl FromStr for Quality {
	type Err = Error;
	fn from_str(input: &str) -> Result<Self, Self::Err> {
		match input {
			"HDTV"|"PPV.HDTV"|"PDTV"|"PPV.PDTV"|"hdtv" => Ok(Self::HDTV),
			"CAM"|"HDCAM"|"CamRip" => Ok(Self::Cam),
			"BrRip"|"BRRip"|"BluRay"|"Bluray"|"bluRay"|"bluray" => Ok(Self::BluRay),
			"TS" => Ok(Self::TS),
			"WEB"|"WEB-"|"WEB-DL"|"WEBDL"|"PPV WEB"|"PPV WEB-"|"PPV WEB-DL"|"PPV WEBDL" => Ok(Self::Web),
			"WEB DVDRip"|"WEB- DVDRip"|"WEB-DL DVDRip"|"WEBDL DVDRip"|"PPV WEB DVDRip"|"PPV WEB- DVDRip"|"PPV WEB-DL DVDRip"|"PPV WEBDL DVDRip" => Ok(Self::Web),
			"WEBRip"|"WEBrip"|"WBBRip"|"WBBrip" => Ok(Self::Web),
			"HDRip"|"HdRip" => Ok(Self::HD),
			"DVDRip"|"DVDRiP"|"DVDRIP"|"DvDScr" => Ok(Self::DVD),
			s => Err(Error::InvalidQuality(s.into()))
		}
	}
}

