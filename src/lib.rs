#![doc = include_str!("../readme.md")]

use std::{fmt::Debug, str::FromStr};

mod error;
pub use error::ReaderError;

/// A custom [`Reader`] that is backed by a [`String`].  
///
/// Comes with functions to easily step, iterate and consume specific patterns from the string.  
///
/// Heavily taken from:  
/// - **[mojang/brigadier/StringReader.java](https://github.com/Mojang/brigadier/blob/master/src/main/java/com/mojang/brigadier/StringReader.java)**
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Reader {
    buf: String,
    cur: usize,
}

impl Reader {
    const ESCAPE: char = '\\';
    const DOUBLE_QUOTE: char = '"';
    const SINGLE_QUOTE: char = '\'';

    /// Creates a new reader
    #[inline]
    pub fn new(str: impl Into<String>) -> Self {
        Self {
            buf: str.into(),
            cur: 0,
        }
    }

    /// Returns the inner buffer
    #[inline]
    pub fn string(&self) -> &str {
        &self.buf
    }

    /// sets the seeking position
    #[inline]
    pub const fn set_pos(&mut self, pos: usize) {
        self.cur = pos
    }

    /// returns the current seeking position
    #[inline]
    pub const fn pos(&self) -> usize {
        self.cur
    }

    /// How much that hasnt been read yet
    #[inline]
    pub const fn remaining_len(&self) -> usize {
        self.buf.len() - self.cur
    }

    /// Returns everything before the cursor, aka: everything that has been read previously.  
    #[inline]
    pub fn prev(&self) -> &str {
        match self.buf.get(0..self.cur) {
            Some(s) => s,
            None => &self.buf,
        }
    }

    /// Returns the rest of the buffer from the cursor and forwards.  
    #[inline]
    pub fn next(&self) -> &str {
        &self.buf[self.cur..]
    }

    /// If there is enough left in the buffer to read `len` chars.  
    #[inline]
    pub const fn can_read(&self, len: usize) -> bool {
        self.cur + len <= self.buf.len()
    }

    /// See the current position's char + `offset`.  
    ///
    /// If the seeked position is outside the buffer,
    /// it will return the last char
    #[inline]
    pub fn peek(&self, offset: usize) -> char {
        // TODO: should probably move all reads away from bytes incase one happens to be 2 bytes width etc
        let buf = self.buf.as_bytes();
        // if peek is past then we peek at the last
        char::from(if self.cur + offset >= buf.len() {
            buf[buf.len() - 1]
        } else {
            buf[self.cur + offset]
        })
    }

    #[inline]
    pub fn prev_c(&self, offset: usize) -> Option<char> {
        let idx = self.cur.checked_sub(1)?.checked_sub(offset)?;
        Some(self.buf.chars().nth(idx).unwrap())
    }

    /// Peeks `n` bytes into the buffer from the current position
    ///
    /// If the len of `current` + `n` is more than the buffer len;  
    /// the remaining is returned
    #[inline]
    pub fn peek_n(&self, n: usize) -> &str {
        if self.cur + n >= self.buf.len() {
            &self.buf[self.cur..self.buf.len()]
        } else {
            &self.buf[self.cur..self.cur + n]
        }
    }

    /// Reads a char and steps the position
    #[inline]
    pub fn read(&mut self) -> char {
        let char = self.buf.chars().nth(self.cur).expect("must exist");
        self.cur += 1;
        if self.cur > self.buf.len() {
            self.cur -= 1;
        }

        char
    }

    /// Advances the position one step without returning the char
    #[inline]
    pub const fn skip(&mut self) {
        self.cur += 1;
    }

    /// If a char is a valid number to be parsed later
    #[inline]
    pub const fn is_allowed_number(c: char) -> bool {
        c >= '0' && c <= '9' || c == '.' || c == '-'
    }

    /// If a char is of either quote variant
    #[inline]
    pub const fn is_quoted_string_start(c: char) -> bool {
        c == Self::DOUBLE_QUOTE || c == Self::SINGLE_QUOTE
    }

    /// Advances the position until theres no more whitespace  
    #[inline]
    pub fn skip_whitespace(&mut self) {
        while self.can_read(1) && self.peek(0).is_whitespace() {
            self.skip();
        }
    }

    /// Reads any number
    pub fn read_num<T>(&mut self) -> Result<T, ReaderError>
    where
        T: FromStr<Err: Debug>,
    {
        let start = self.cur;
        while self.can_read(1) && Self::is_allowed_number(self.peek(0)) {
            self.skip();
        }

        let num = &self.buf[start..self.cur];
        if num.is_empty() {
            return Err(ReaderError::NumberIsEmpty);
        }
        match num.parse() {
            Ok(n) => Ok(n),
            Err(e) => {
                self.cur = start;
                Err(ReaderError::ParseError(format!("{e:?}")))
            }
        }
    }

    /// Valid chars in an unquoted string
    #[inline]
    pub const fn is_allowed_in_unquoted_string(c: char) -> bool {
        c >= '0' && c <= '9'
            || c >= 'A' && c <= 'Z'
            || c >= 'a' && c <= 'z'
            || c == '_'
            || c == '-'
            || c == '.'
            || c == '+'
    }

    /// Reads an unquoted string, eg: `hello`
    pub fn read_unquoted_string(&mut self) -> Result<String, ReaderError> {
        let start = self.cur;
        while self.can_read(1) && Self::is_allowed_in_unquoted_string(self.peek(0)) {
            self.skip();
        }

        Ok(self.buf[start..self.cur].to_owned())
    }

    /// Reads an unquoted string, eg: `hello`
    pub fn read_unquoted_str(&mut self) -> Result<&str, ReaderError> {
        let start = self.cur;
        while self.can_read(1) && Self::is_allowed_in_unquoted_string(self.peek(0)) {
            self.skip();
        }

        Ok(&self.buf[start..self.cur])
    }

    /// Reads a quoted string, eg: `"hello world"`
    pub fn read_quoted_string(&mut self) -> Result<String, ReaderError> {
        if !self.can_read(1) {
            return Ok(String::new());
        }

        let next = self.peek(0);
        if !Self::is_quoted_string_start(next) {
            return Err(ReaderError::ExpectedStartOfQuote);
        }
        self.skip();

        self.read_string_until(next)
    }

    /// Reads any chars until ` `, errors if it doenst find one.  
    ///
    /// Consumes the delimiting ` ` if it finds one.  
    pub fn read_string_until(&mut self, end: char) -> Result<String, ReaderError> {
        let mut result = String::new();
        let mut escaped = false;

        while self.can_read(1) {
            let c = self.read();
            if escaped {
                if c == end || c == Self::ESCAPE {
                    result.push(c);
                    escaped = false;
                } else {
                    self.cur = self.cur - 1;
                    return Err(ReaderError::InvalidEscape);
                }
            } else if c == Self::ESCAPE {
                escaped = true
            } else if c == end {
                return Ok(result);
            } else {
                result.push(c);
            }
        }

        Err(ReaderError::ExpectedEndOfQuote)
    }

    /// Reads any chars until the end of the buffer or a ` ` is encountered.  
    ///
    /// Consumes the delimiting ` ` if it finds one.  
    pub fn read_string_until_end(&mut self) -> Result<String, ReaderError> {
        let mut result = String::new();
        let mut escaped = false;

        while self.can_read(1) {
            let c = self.read();
            if escaped {
                if c == ' ' || c == Self::ESCAPE {
                    result.push(c);
                    escaped = false;
                } else {
                    self.cur = self.cur - 1;
                    return Err(ReaderError::InvalidEscape);
                }
            } else if c == Self::ESCAPE {
                escaped = true
            } else if c == ' ' {
                return Ok(result);
            } else {
                result.push(c);
            }
        }

        Ok(result)
    }

    /// Reads any chars until it finds any of the chars in `end`.  
    ///
    /// If no chars from `end` were found, it consumes the rest of the string and returns it.  
    ///
    /// This does **not** consume the char if one is found from `end`.  
    pub fn read_string_until_vec(&mut self, end: &[char]) -> Result<String, ReaderError> {
        let mut result = String::new();
        let mut escaped = false;

        while self.can_read(1) {
            let c = self.read();
            if escaped {
                if end.contains(&c) || c == Self::ESCAPE {
                    result.push(c);
                    escaped = false;
                } else {
                    self.cur = self.cur - 1;
                    return Err(ReaderError::InvalidEscape);
                }
            } else if c == Self::ESCAPE {
                escaped = true
            } else if end.contains(&c) {
                // note that compared to the non_vec variant
                // we dont consume the last char
                // we just let it be in the next buffer
                // since the normal one usually is from " to "
                // but this one is like just end it on ANY of these chars
                // like 'name[' or 'name ', both these would end the search
                self.cur = self.cur - 1;
                return Ok(result);
            } else {
                result.push(c);
            }
        }

        // if it was never found we just consumed the rest of the buf
        Ok(result)
    }

    /// Reads a string of either unquoted or quoted variant.  
    pub fn read_string(&mut self) -> Result<String, ReaderError> {
        if !self.can_read(1) {
            return Ok(String::new());
        }

        let next = self.peek(0);
        if Self::is_quoted_string_start(next) {
            self.skip();
            self.read_string_until(next)
        } else {
            self.read_unquoted_string()
        }
    }

    /// Reads `n` chars from the buffer
    pub fn read_n(&mut self, n: usize) -> Result<String, ReaderError> {
        let mut result = String::new();

        while self.can_read(1) && result.len() != n {
            result.push(self.read());
        }

        Ok(result)
    }

    /// Reads `n` chars from the buffer
    pub fn get_n_ref(&self, n: usize) -> Result<&str, ReaderError> {
        if self.remaining_len() < n {
            return Err(ReaderError::ExpectedChar);
        }
        Ok(&self.buf[self.pos()..self.pos() + n])
    }

    pub unsafe fn get_n_range(&self, n: std::ops::Range<usize>) -> &str {
        &self.buf[n]
    }

    /// Reads a boolean from the buffer
    pub fn read_bool(&mut self) -> Result<bool, ReaderError> {
        let start = self.cur;
        let val = self.read_string()?;
        if val.is_empty() {
            return Err(ReaderError::ExpectedBool);
        }

        match &val[..] {
            "true" => Ok(true),
            "false" => Ok(false),
            n => {
                self.cur = start;
                Err(ReaderError::InvalidBool(n.to_owned()))
            }
        }
    }

    /// Check if the current position is `c` char, consumes the char if so
    pub fn expect(&mut self, c: char) -> Result<(), ReaderError> {
        if !self.can_read(1) || self.peek(0) != c {
            return Err(ReaderError::ExpectedChar);
        }
        self.skip();
        Ok(())
    }

    /// Reads from the buffer until `target.1` has been found.  
    ///  
    /// Includes anything in the buffer between the current position and `target.0`
    pub fn read_until_balanced(&mut self, target: (char, char)) -> Result<String, ReaderError> {
        let mut result = String::new();
        let mut depth = 0;
        let mut in_quotes = false;
        let mut found_first_target = false;

        while self.can_read(1) {
            let c = self.read();

            if !found_first_target && c == target.1 {
                return Err(ReaderError::FoundClosingBeforeOpening);
            }

            result.push(c);

            if Self::is_quoted_string_start(c) {
                in_quotes = !in_quotes;
            }

            if !in_quotes {
                if c == target.0 {
                    depth += 1;
                    found_first_target = true;
                } else if c == target.1 {
                    depth -= 1;

                    if depth == 0 {
                        return Ok(result);
                    }
                }
            }
        }

        if found_first_target {
            Err(ReaderError::NoClosing)
        } else {
            Ok(result)
        }
    }
}
