//! Roundtrip serde Options module.

use std::io;

use serde::{de, ser, Deserialize, Serialize};

use crate::de::Deserializer;
use crate::error::Result;
use crate::extensions::Extensions;
use crate::ser::{PrettyConfig, Serializer};

/// Roundtrip serde options.
///
/// # Examples
///
/// ```
/// use ron::{Options, extensions::Extensions};
///
/// let ron = Options::default()
///     .with_default_extension(Extensions::IMPLICIT_SOME);
///
/// let de: Option<i32> = ron.from_str("42").unwrap();
/// let ser = ron.to_string(&de).unwrap();
///
/// assert_eq!(ser, "42");
/// ```
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Options {
    /// Extensions that are enabled by default during serialization and
    ///  deserialization.
    /// During serialization, these extensions do NOT have to be explicitly
    ///  enabled in the parsed RON.
    /// During deserialization, these extensions are used, but their explicit
    ///  activation is NOT included in the output RON.
    /// No extensions are enabled by default.
    pub default_extensions: Extensions,
    /// Private field to ensure adding a field is non-breaking.
    #[serde(skip)]
    _future_proof: (),
}

impl Default for Options {
    fn default() -> Self {
        Self {
            default_extensions: Extensions::empty(),
            _future_proof: (),
        }
    }
}

impl Options {
    #[must_use]
    /// Enable `default_extension` by default during serialization and deserialization.
    pub fn with_default_extension(mut self, default_extension: Extensions) -> Self {
        self.default_extensions |= default_extension;
        self
    }

    #[must_use]
    /// Do NOT enable `default_extension` by default during serialization and deserialization.
    pub fn without_default_extension(mut self, default_extension: Extensions) -> Self {
        self.default_extensions &= !default_extension;
        self
    }
}

impl Options {
    /// A convenience function for reading data from a reader
    /// and feeding into a deserializer.
    pub fn from_reader<R, T>(&self, mut rdr: R) -> Result<T>
    where
        R: io::Read,
        T: de::DeserializeOwned,
    {
        let mut bytes = Vec::new();
        rdr.read_to_end(&mut bytes)?;

        self.from_bytes(&bytes)
    }

    /// A convenience function for building a deserializer
    /// and deserializing a value of type `T` from a string.
    pub fn from_str<'a, T>(&self, s: &'a str) -> Result<T>
    where
        T: de::Deserialize<'a>,
    {
        self.from_bytes(s.as_bytes())
    }

    /// A convenience function for building a deserializer
    /// and deserializing a value of type `T` from bytes.
    pub fn from_bytes<'a, T>(&self, s: &'a [u8]) -> Result<T>
    where
        T: de::Deserialize<'a>,
    {
        let mut deserializer = Deserializer::from_bytes_with_options(s, self.clone())?;

        let value = T::deserialize(&mut deserializer)?;

        deserializer.end()?;

        Ok(value)
    }

    /// Serializes `value` into `writer`
    pub fn to_writer<W, T>(&self, writer: W, value: &T) -> Result<()>
    where
        W: io::Write,
        T: ?Sized + ser::Serialize,
    {
        let mut s = Serializer::with_options(writer, None, self.clone())?;
        value.serialize(&mut s)
    }

    /// Serializes `value` into `writer` in a pretty way.
    pub fn to_writer_pretty<W, T>(&self, writer: W, value: &T, config: PrettyConfig) -> Result<()>
    where
        W: io::Write,
        T: ?Sized + ser::Serialize,
    {
        let mut s = Serializer::with_options(writer, Some(config), self.clone())?;
        value.serialize(&mut s)
    }

    /// Serializes `value` and returns it as string.
    ///
    /// This function does not generate any newlines or nice formatting;
    /// if you want that, you can use `to_string_pretty` instead.
    pub fn to_string<T>(&self, value: &T) -> Result<String>
    where
        T: ?Sized + ser::Serialize,
    {
        let mut output = Vec::new();
        let mut s = Serializer::with_options(&mut output, None, self.clone())?;
        value.serialize(&mut s)?;
        Ok(String::from_utf8(output).expect("Ron should be utf-8"))
    }

    /// Serializes `value` in the recommended RON layout in a pretty way.
    pub fn to_string_pretty<T>(&self, value: &T, config: PrettyConfig) -> Result<String>
    where
        T: ?Sized + ser::Serialize,
    {
        let mut output = Vec::new();
        let mut s = Serializer::with_options(&mut output, Some(config), self.clone())?;
        value.serialize(&mut s)?;
        Ok(String::from_utf8(output).expect("Ron should be utf-8"))
    }
}
