//#![warn(missing_doc)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![forbid(non_camel_case_types)]
#![type_length_limit = "988530600"]

//! This crate currently provides an almost XML 1.0/1.1-compliant pull parser.

#[cfg(test)]
extern crate encoding;
#[cfg(test)]
extern crate quickcheck;

pub use encoding_rs;

pub use self::reader::Reader;
pub use self::reader::ReaderConfig;

pub use self::reader_old::EventReader;
pub use self::reader_old::ParserConfig;
pub use self::writer::EmitterConfig;
pub use self::writer::EventWriter;

pub mod attribute_old;
pub mod chars;
pub mod event;
#[macro_use]
pub mod macros;
pub mod name_old;
pub mod namespace;
pub mod position;
pub mod reader_old;
pub mod util;
pub mod writer;

pub mod attribute;
pub mod name;

pub mod reader;
