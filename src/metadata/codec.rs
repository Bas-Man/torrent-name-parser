use std::str::FromStr;

use crate::error::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Codec {
    X264,
    X265,
    Xvid,
}

impl FromStr for Codec {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_ascii_lowercase().as_str() {
            "x264" | "h264" | "h.264" => Ok(Self::X264),
            "x265" | "h265" | "h.265" => Ok(Self::X265),
            "hvec" => Ok(Self::X265),
            "xvid" => Ok(Self::Xvid),
            s => Err(Error::InvalidCodec(s.into())),
        }
    }
}
