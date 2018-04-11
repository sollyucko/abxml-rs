#![recursion_limit = "1024"]

//! Library that decodes the binary documents contained on an APK (both resources.arsc and binary
//! XMLs).
//! It exposes also structures to query this binary files on a structured way. For example, it's
//! possible to check which chunks of data a document contains, and perform specific queries
//! depending on the type of chunk.

extern crate byteorder;
extern crate encoding;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
extern crate xml;
#[cfg(feature = "zip_decode")]
extern crate zip;

pub mod encoder;
pub mod chunks;
pub mod visitor;
pub mod model;
pub mod decoder;
#[cfg(test)]
pub mod test;
#[cfg(test)]
pub mod raw_chunks;
#[cfg(feature = "zip_decode")]
pub mod apk;

/// Contents of android's resources.arsc
pub const STR_ARSC: &'static [u8] = include_bytes!("../resources/resources.arsc");
