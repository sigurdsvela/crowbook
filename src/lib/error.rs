// Copyright (C) 2016 Élisabeth HENRY.
//
// This file is part of Crowbook.
//
// Crowbook is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published
// by the Free Software Foundation, either version 2.1 of the License, or
// (at your option) any later version.
//
// Caribon is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received ba copy of the GNU Lesser General Public License
// along with Crowbook.  If not, see <http://www.gnu.org/licenses/>.

use std::error;
use std::result;
use std::fmt;
use std::borrow::Cow;

#[derive(Debug, PartialEq, Clone)]
/// Source of an error file
pub struct Source {
    /// File name of the source
    pub file: Option<String>,
    /// Line number of the source
    pub line: Option<u32>,
}

impl Source {
    /// Create an empty source, with both fields set to None
    pub fn empty() -> Source {
        Source { file: None, line: None }
    }

    /// Create a new source pointing to file
    pub fn new<S: Into<String>>(s: S) -> Source {
        Source { file: Some(s.into()), line: None }
    }

    /// Sets line number of a source (with &mut self)
    pub fn set_line(&mut self, line: u32) {
        self.line = Some(line);
    }

    /// Unsets a line number of a source
    pub fn unset_line(&mut self) {
        self.line = None;
    }
}

impl<'a> From<&'a Source> for Source {
    fn from(s: &'a Source) -> Source {
        s.clone()
    }
}
#[derive(Debug, PartialEq)]
/// Crowbook Error type
pub struct Error {
    /// Origin (file, line) of the error, if there is one
    source: Source,
    inner: Inner,
}

impl Error {
    /// Creates a new default error
    pub fn default<S: Into<Cow<'static, str>>, O: Into<Source>>(source: O, msg: S) -> Error {
        Error {
            source: source.into(),
            inner: Inner::Default(msg.into())
        }
    }

    /// Creates a new parser error
    ///
    /// Error when parsing markdown file
    pub fn parser<S: Into<Cow<'static, str>>, O: Into<Source>>(source: O, msg: S) -> Error {
        Error {
            source: source.into(),
            inner: Inner::Parser(msg.into())
        }
    }

    /// Creates a new config parser error
    ///
    /// Error when parsing book file
    pub fn config_parser<S: Into<Cow<'static, str>>, O: Into<Source>>(source: O, msg: S) -> Error {
        Error {
            source: source.into(),
            inner: Inner::ConfigParser(msg.into())
        }
    }

    /// Creates a new file not found error
    ///
    /// * msg: description of why the file was needed
    /// * file: file name
    pub fn file_not_found<S1: Into<Cow<'static, str>>, S2: Into<Cow<'static, str>>, O: Into<Source>>(source: O, msg: S1, file: S2) -> Error {
        Error {
            source: source.into(),
            inner: Inner::FileNotFound(msg.into(), file.into())
        }
    }

    /// Creates a new render error
    ///
    /// Error when rendering
    pub fn render<S: Into<Cow<'static, str>>, O: Into<Source>>(source: O, msg: S) -> Error {
        Error {
            source: source.into(),
            inner: Inner::Render(msg.into())
        }
    }

    /// Creates a new invalid option error
    ///
    /// Error when trying to set an option
    pub fn invalid_option<S: Into<Cow<'static, str>>, O: Into<Source>>(source: O, msg: S) -> Error {
        Error {
            source: source.into(),
            inner: Inner::InvalidOption(msg.into())
        }
    }

    /// Creates a new zipper error
    ///
    /// Error when moving/copying files to temporary dir, e.g. using `zip` commmand
    pub fn zipper<S: Into<Cow<'static, str>>>(msg: S) -> Error {
        Error {
            source: Source::empty(),
            inner: Inner::Zipper(msg.into())
        }
    }

    /// Creates a new book option error
    ///
    /// Used when converting an error to invalid type
    pub fn book_option<S: Into<Cow<'static, str>>, O: Into<Source>>(source: O, msg: S) -> Error {
        Error {
            source: source.into(),
            inner: Inner::BookOption(msg.into())
        }
    }

    /// Change the source of an error
    pub fn with_source<O: Into<Source>>(mut self, source: O) -> Error {
        self.source = source.into();
        self
    }

    /// Returns true if self is a default option error, false else
    pub fn is_default(&self) -> bool {
        match self.inner {
            Inner::Default(..) => true,
            _ => false,
        }
    }

    /// Returns true if self is a parser error, false else
    pub fn is_parser(&self) -> bool {
        match self.inner {
            Inner::Parser(..) => true,
            _ => false,
        }
    }

    /// Returns true if self is a config parser error, false else
    pub fn is_config_parser(&self) -> bool {
        match self.inner {
            Inner::ConfigParser(..) => true,
            _ => false,
        }
    }

    /// Returns true if self is a file not found error, false else
    pub fn is_file_not_found(&self) -> bool {
        match self.inner {
            Inner::FileNotFound(..) => true,
            _ => false,
        }
    }
    
    /// Returns true if self is a render error, false else
    pub fn is_render(&self) -> bool {
        match self.inner {
            Inner::Render(..) => true,
            _ => false,
        }
    }

    /// Returns true if self is a zipper error, false else
    pub fn is_zipper(&self) -> bool {
        match self.inner {
            Inner::Zipper(..) => true,
            _ => false,
        }
    }
    
    /// Returns true if self is a book option error, false else
    pub fn is_book_option(&self) -> bool {
        match self.inner {
            Inner::BookOption(..) => true,
            _ => false,
        }
    }

    /// Returns true if self is an invalid option error, false else
    pub fn is_invalid_option(&self) -> bool {
        match self.inner {
            Inner::InvalidOption(..) => true,
            _ => false,
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.inner {
            Inner::Default(ref s)
                | Inner::Parser(ref s)
                | Inner::Zipper(ref s)
                | Inner::BookOption(ref s)
                | Inner::ConfigParser(ref s)
                | Inner::InvalidOption(ref s)
                | Inner::Render(ref s)
                => s.as_ref(),
            
            Inner::FileNotFound(..) => "File not found",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let source = &self.source;
        if let Some(ref file) = source.file {
            try!(write!(f, "{}", file));
            if let Some(line) = source.line {
                try!(write!(f, ":{}", line));
            }
            try!(write!(f, ": "));
        }

        try!(match self.inner {
            Inner::Default(ref s) => {
                write!(f, "Error: {}", s)
            },
            Inner::Parser(ref s) => {
                write!(f, "Error parsing markdown: {}", s)
            },
            Inner::ConfigParser(ref s) => {
                try!(f.write_str("Error parsing configuration file: "));
                f.write_str(s)
            },
            Inner::FileNotFound(ref description, ref file) => {
                write!(f, "Could not find file '{}' for {}", file, description)
            },
            Inner::Render(ref s) => {
                try!(f.write_str("Error during rendering: "));
                f.write_str(s)
            },
            Inner::Zipper(ref s) => {
                try!(f.write_str("Error during temporary files editing: "));
                f.write_str(s)
            },
            Inner::BookOption(ref s) => {
                try!(f.write_str("Error converting BookOption: "));
                f.write_str(s)
            },
            Inner::InvalidOption(ref s) => {
                try!(f.write_str("Error accessing book option: "));
                f.write_str(s)
            },
        });
        Ok(())
    }
}

/// Crowbook's Result type, used by many methods that can fail
pub type Result<T> = result::Result<T, Error>;


#[derive(Debug, PartialEq)]
enum Inner {
    /// Default variant
    Default(Cow<'static, str>),
    /// An error in Parsing markdown file
    Parser(Cow<'static, str>),
    /// An error in parsing a book configuration file
    ConfigParser(Cow<'static, str>), 
    /// An error when a file is not found
    FileNotFound(Cow<'static, str>, Cow<'static, str>), //description, file
    /// An error in a renderer
    Render(Cow<'static, str>),
    /// An error during "zipping" processus
    Zipper(Cow<'static, str>),
    /// An error relative to BookOption convertion (usually a type error)
    BookOption(Cow<'static, str>),
    /// An invalid option
    InvalidOption(Cow<'static, str>),
}
