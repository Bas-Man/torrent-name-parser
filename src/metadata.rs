use crate::error::ErrorMatch;
use crate::pattern;
use crate::pattern::Pattern;
use regex::Captures;
use smartstring::alias::String;
use std::borrow::Cow;
use std::cmp::{max, min};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Metadata {
    title: String,
    season: Option<u16>,
    episode: Option<u16>,
    year: Option<u16>,
    resolution: Option<String>,
    quality: Option<String>,
    codec: Option<String>,
    audio: Option<String>,
    group: Option<String>,
    extended: bool,
    hardcoded: bool,
    proper: bool,
    repack: bool,
    widescreen: bool,
    unrated: bool,
    three_d: bool,
    imdb: Option<String>,
}

fn check_pattern_and_extract<'a>(
    pattern: &Pattern,
    torrent_name: &'a str,
    title_start: &mut usize,
    title_end: &mut usize,
    extract_value: impl Fn(Captures<'a>) -> Option<&'a str>,
) -> Option<&'a str> {
    pattern.captures(torrent_name).and_then(|caps| {
        if let Some(cap) = caps.get(0) {
            if pattern.before_title() {
                *title_start = max(*title_start, cap.end());
            } else {
                *title_end = min(*title_end, cap.start());
            }
        }
        extract_value(caps)
    })
}

fn check_pattern<'a>(
    pattern: &Pattern,
    torrent_name: &'a str,
    title_start: &mut usize,
    title_end: &mut usize,
) -> Option<Captures<'a>> {
    pattern.captures(torrent_name).map(|caps| {
        if let Some(cap) = caps.get(0) {
            if pattern.before_title() {
                *title_start = max(*title_start, cap.end());
            } else {
                *title_end = min(*title_end, cap.start());
            }
        }
        caps
    })
}

fn capture_to_string(caps: Option<Captures<'_>>) -> Option<std::string::String> {
    caps.and_then(|c| c.get(0)).map(|m| m.as_str().to_string())
}

impl Metadata {
    pub fn from(name: &str) -> Result<Self, ErrorMatch> {
        let mut title_start = 0;
        let mut title_end = name.len();

        let season = check_pattern_and_extract(
            &pattern::SEASON,
            name,
            &mut title_start,
            &mut title_end,
            |caps| {
                caps.name("short")
                    .or_else(|| caps.name("long"))
                    .or_else(|| caps.name("dash"))
                    .map(|m| m.as_str())
            },
        );

        let episode = check_pattern_and_extract(
            &pattern::EPISODE,
            name,
            &mut title_start,
            &mut title_end,
            |caps| {
                caps.name("short")
                    .or_else(|| caps.name("long"))
                    .or_else(|| caps.name("cross"))
                    .or_else(|| caps.name("dash"))
                    .map(|m| m.as_str())
            },
        );

        let year = check_pattern_and_extract(
            &pattern::YEAR,
            name,
            &mut title_start,
            &mut title_end,
            |caps: Captures<'_>| caps.name("year").map(|m| m.as_str()),
        );

        let resolution = check_pattern_and_extract(
            &pattern::RESOLUTION,
            name,
            &mut title_start,
            &mut title_end,
            |caps| caps.get(1).map(|m| m.as_str()),
        )
        .map(String::from);
        let quality = check_pattern_and_extract(
            &pattern::QUALITY,
            name,
            &mut title_start,
            &mut title_end,
            |caps| caps.get(0).map(|m| m.as_str()),
        )
        .map(String::from);
        let codec = check_pattern_and_extract(
            &pattern::CODEC,
            name,
            &mut title_start,
            &mut title_end,
            |caps| caps.get(0).map(|m| m.as_str()),
        )
        .map(String::from);
        let audio = check_pattern_and_extract(
            &pattern::AUDIO,
            name,
            &mut title_start,
            &mut title_end,
            |caps| caps.get(0).map(|m| m.as_str()),
        )
        .map(String::from);
        let group = check_pattern_and_extract(
            &pattern::GROUP,
            name,
            &mut title_start,
            &mut title_end,
            |caps| caps.get(2).map(|m| m.as_str()),
        )
        .map(String::from);
        let imdb = check_pattern_and_extract(
            &pattern::IMDB,
            name,
            &mut title_start,
            &mut title_end,
            |caps| caps.get(0).map(|m| m.as_str()),
        )
        .map(String::from);

        let extended = check_pattern(&pattern::EXTENDED, name, &mut title_start, &mut title_end);
        let hardcoded = check_pattern(&pattern::HARDCODED, name, &mut title_start, &mut title_end);
        let proper = check_pattern(&pattern::PROPER, name, &mut title_start, &mut title_end);
        let repack = check_pattern(&pattern::REPACK, name, &mut title_start, &mut title_end);
        let widescreen =
            check_pattern(&pattern::WIDESCREEN, name, &mut title_start, &mut title_end);
        let unrated = check_pattern(&pattern::UNRATED, name, &mut title_start, &mut title_end);
        let three_d = check_pattern(&pattern::THREE_D, name, &mut title_start, &mut title_end);

        let region = check_pattern(&pattern::REGION, name, &mut title_start, &mut title_end);
        let container = check_pattern(&pattern::CONTAINER, name, &mut title_start, &mut title_end);
        let language = check_pattern(&pattern::LANGUAGE, name, &mut title_start, &mut title_end);
        let garbage = check_pattern(&pattern::GARBAGE, name, &mut title_start, &mut title_end);
        let website = check_pattern(&pattern::WEBSITE, name, &mut title_start, &mut title_end);

        if title_start >= title_end {
            return Err(ErrorMatch::new(vec![
                ("season", season.map(std::string::String::from)),
                ("episode", episode.map(std::string::String::from)),
                ("year", year.map(std::string::String::from)),
                ("resolution", resolution.map(|s| s.into())),
                ("quality", quality.map(|s| s.into())),
                ("codec", codec.map(|s| s.into())),
                ("audio", audio.map(|s| s.into())),
                ("group", group.map(|s| s.into())),
                ("imdb", imdb.map(|s| s.into())),
                ("extended", capture_to_string(extended)),
                ("proper", capture_to_string(proper)),
                ("repack", capture_to_string(repack)),
                ("widescreen", capture_to_string(widescreen)),
                ("unrated", capture_to_string(unrated)),
                ("three_d", capture_to_string(three_d)),
                ("region", capture_to_string(region)),
                ("container", capture_to_string(container)),
                ("language", capture_to_string(language)),
                ("garbage", capture_to_string(garbage)),
                ("website", capture_to_string(website)),
            ]));
        }

        let mut title = &name[title_start..title_end];
        if let Some(pos) = title.find('(') {
            title = title.split_at(pos).0;
        }
        title = title.trim_start_matches(" -");
        title = title.trim_end_matches(" -");
        let mut title = match !title.contains(' ') && title.contains('.') {
            true => Cow::Owned(title.replace('.', " ")),
            false => Cow::Borrowed(title),
        };
        if title.contains('_') {
            title = Cow::Owned(title.replace('_', " "));
        }
        if title.contains('(') {
            title = Cow::Owned(title.replacen('(', "", 1));
        }
        if title.contains("- ") {
            title = Cow::Owned(title.replacen("- ", "", 1));
        }
        title = match title {
            Cow::Owned(s) => Cow::Owned(s.trim().to_string()),
            Cow::Borrowed(s) => Cow::Borrowed(s.trim()),
        };

        Ok(Metadata {
            title: title.into(),
            season: season.map(|s| s.parse().unwrap()),
            episode: episode.map(|s| s.parse().unwrap()),
            year: year.map(|s| s.parse().unwrap()),
            resolution,
            quality,
            codec,
            audio,
            group,
            extended: extended.is_some(),
            hardcoded: hardcoded.is_some(),
            proper: proper.is_some(),
            repack: repack.is_some(),
            widescreen: widescreen.is_some(),
            unrated: unrated.is_some(),
            three_d: three_d.is_some(),
            imdb,
        })
    }

    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn season(&self) -> Option<u16> {
        self.season
    }
    pub fn episode(&self) -> Option<u16> {
        self.episode
    }
    pub fn year(&self) -> Option<u16> {
        self.year
    }
    pub fn resolution(&self) -> Option<&str> {
        self.resolution.as_deref()
    }
    pub fn quality(&self) -> Option<&str> {
        self.quality.as_deref()
    }
    pub fn codec(&self) -> Option<&str> {
        self.codec.as_deref()
    }
    pub fn audio(&self) -> Option<&str> {
        self.audio.as_deref()
    }
    pub fn group(&self) -> Option<&str> {
        self.group.as_deref()
    }
    pub fn imdb_tag(&self) -> Option<&str> {
        self.imdb.as_deref()
    }
    pub fn extended(&self) -> bool {
        self.extended
    }
    pub fn hardcoded(&self) -> bool {
        self.hardcoded
    }
    pub fn proper(&self) -> bool {
        self.proper
    }
    pub fn repack(&self) -> bool {
        self.repack
    }
    pub fn widescreen(&self) -> bool {
        self.widescreen
    }
    pub fn unrated(&self) -> bool {
        self.unrated
    }
    pub fn three_d(&self) -> bool {
        self.three_d
    }
}
