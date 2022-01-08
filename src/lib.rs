#[macro_use]
extern crate lazy_static;
extern crate regex;

pub mod error;
mod metadata;
mod pattern;
#[cfg(test)]
mod test;

pub use metadata::Metadata;
pub use metadata::MetadataRef;
pub use metadata::Resolution;
pub use metadata::Quality;
pub use metadata::Codec;
pub use metadata::Audio;

