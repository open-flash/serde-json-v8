//! Serialize a Rust data structure into JSON data.

use std::io;

use serde_json::error::Result;
use serde::ser::Serialize;

pub use serde_json::ser::Formatter;

/// A structure for serializing Rust values into JSON.
#[allow(non_snake_case)]
pub mod Serializer {
    use super::{io, Formatter, CompactV8Formatter, PrettyV8Formatter};

    /// Creates a new JSON serializer.
    #[inline]
    pub fn new<W>(writer: W) -> serde_json::ser::Serializer<W, CompactV8Formatter>
        where
          W: io::Write,
    {
        with_formatter(writer, CompactV8Formatter)
    }


    /// Creates a new JSON pretty print serializer.
    #[inline]
    pub fn pretty<'a, W>(writer: W) -> serde_json::ser::Serializer<W, PrettyV8Formatter<'a>>
        where
          W: io::Write,
    {
        with_formatter(writer, PrettyV8Formatter::new())
    }

    /// Creates a new JSON visitor whose output will be written to the writer
    /// specified.
    #[inline]
    pub fn with_formatter<W, F>(writer: W, formatter: F) -> serde_json::ser::Serializer<W, F>
        where
          W: io::Write,
          F: Formatter,
    {
        serde_json::ser::Serializer::with_formatter(writer, formatter)
    }
}

/// This structure compacts a JSON value with no extra whitespace.
#[derive(Clone, Debug)]
pub struct CompactV8Formatter;

impl Formatter for CompactV8Formatter {
    #[inline]
    fn write_f32<W: ?Sized>(&mut self, writer: &mut W, value: f32) -> io::Result<()>
        where
          W: io::Write,
    {
        let nearest_int = value.round() as i64;
        if value == (nearest_int as f32) {
            serde_json::ser::CompactFormatter.write_i64(writer, nearest_int)
        } else {
            serde_json::ser::CompactFormatter.write_f64(writer, value.into())
        }
    }

    #[inline]
    fn write_f64<W: ?Sized>(&mut self, writer: &mut W, value: f64) -> io::Result<()>
        where
          W: io::Write,
    {
        let nearest_int = value.round() as i64;
        if value == (nearest_int as f64) {
            serde_json::ser::CompactFormatter.write_i64(writer, nearest_int)
        } else {
            serde_json::ser::CompactFormatter.write_f64(writer, value)
        }
    }
}


/// This structure pretty prints a JSON value to make it human readable.
#[derive(Clone, Debug)]
pub struct PrettyV8Formatter<'a> {
    inner: serde_json::ser::PrettyFormatter<'a>
}

impl<'a> PrettyV8Formatter<'a> {
    /// Construct a pretty printer formatter that defaults to using two spaces for indentation.
    pub fn new() -> Self {
        PrettyV8Formatter {
            inner: serde_json::ser::PrettyFormatter::new()
        }
    }

    /// Construct a pretty printer formatter that uses the `indent` string for indentation.
    pub fn with_indent(indent: &'a [u8]) -> Self {
        PrettyV8Formatter {
            inner: serde_json::ser::PrettyFormatter::with_indent(indent)
        }
    }
}

impl<'a> Default for PrettyV8Formatter<'a> {
    fn default() -> Self {
        PrettyV8Formatter::new()
    }
}

impl<'a> Formatter for PrettyV8Formatter<'a> {
    #[inline]
    fn write_f32<W: ?Sized>(&mut self, writer: &mut W, value: f32) -> io::Result<()>
        where
          W: io::Write,
    {
        let nearest_int = value.round() as i64;
        if value == (nearest_int as f32) {
            self.inner.write_i64(writer, nearest_int)
        } else {
            self.inner.write_f64(writer, value.into())
        }
    }

    #[inline]
    fn write_f64<W: ?Sized>(&mut self, writer: &mut W, value: f64) -> io::Result<()>
        where
          W: io::Write,
    {
        let nearest_int = value.round() as i64;
        if value == (nearest_int as f64) {
            self.inner.write_i64(writer, nearest_int)
        } else {
            self.inner.write_f64(writer, value)
        }
    }

    #[inline]
    fn begin_array<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
        where
          W: io::Write,
    {
        self.inner.begin_array(writer)
    }

    #[inline]
    fn end_array<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
        where
          W: io::Write,
    {
        self.inner.end_array(writer)
    }

    #[inline]
    fn begin_array_value<W: ?Sized>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
        where
          W: io::Write,
    {
        self.inner.begin_array_value(writer, first)
    }

    #[inline]
    fn end_array_value<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
        where
          W: io::Write,
    {
        self.inner.end_array_value(writer)
    }

    #[inline]
    fn begin_object<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
        where
          W: io::Write,
    {
        self.inner.begin_object(writer)
    }

    #[inline]
    fn end_object<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
        where
          W: io::Write,
    {
        self.inner.end_object(writer)
    }

    #[inline]
    fn begin_object_key<W: ?Sized>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
        where
          W: io::Write,
    {
        self.inner.begin_object_key(writer, first)
    }

    #[inline]
    fn begin_object_value<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
        where
          W: io::Write,
    {
        self.inner.begin_object_value(writer)
    }

    #[inline]
    fn end_object_value<W: ?Sized>(&mut self, writer: &mut W) -> io::Result<()>
        where
          W: io::Write,
    {
        self.inner.end_object_value(writer)
    }
}

/// Serialize the given data structure as JSON into the IO stream.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
#[inline]
pub fn to_writer<W, T: ?Sized>(writer: W, value: &T) -> Result<()>
where
    W: io::Write,
    T: Serialize,
{
    let mut ser = Serializer::new(writer);
    try!(value.serialize(&mut ser));
    Ok(())
}

/// Serialize the given data structure as pretty-printed JSON into the IO
/// stream.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
#[inline]
pub fn to_writer_pretty<W, T: ?Sized>(writer: W, value: &T) -> Result<()>
where
    W: io::Write,
    T: Serialize,
{
    let mut ser = Serializer::pretty(writer);
    try!(value.serialize(&mut ser));
    Ok(())
}

/// Serialize the given data structure as a JSON byte vector.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
#[inline]
pub fn to_vec<T: ?Sized>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut writer = Vec::with_capacity(128);
    try!(to_writer(&mut writer, value));
    Ok(writer)
}

/// Serialize the given data structure as a pretty-printed JSON byte vector.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
#[inline]
pub fn to_vec_pretty<T: ?Sized>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut writer = Vec::with_capacity(128);
    try!(to_writer_pretty(&mut writer, value));
    Ok(writer)
}

/// Serialize the given data structure as a String of JSON.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
#[inline]
pub fn to_string<T: ?Sized>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let vec = try!(to_vec(value));
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}

/// Serialize the given data structure as a pretty-printed String of JSON.
///
/// # Errors
///
/// Serialization can fail if `T`'s implementation of `Serialize` decides to
/// fail, or if `T` contains a map with non-string keys.
#[inline]
pub fn to_string_pretty<T: ?Sized>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let vec = try!(to_vec_pretty(value));
    let string = unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(vec)
    };
    Ok(string)
}
