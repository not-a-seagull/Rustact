/*
 * src/error.rs
 * Rustact - port of React to Rust
 * 
 * Copyright (c) 2019, not_a_seagull
 * All rights reserved.
 * 
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 * 
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 * 
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 * 
 * 3. Neither the name of the copyright holder nor the names of its
 *    contributors may be used to endorse or promote products derived from
 *    this software without specific prior written permission.
 * 
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE. 
 */

// defines an error type
use std::{
  fmt::{Error as FmtError, Write},
  io::Error as IoError,
  str::Utf8Error
};

/// An error that occurred during runtime.
#[derive(Debug, Error)]
pub enum Error {
  #[error("unspecified error: {0}")]
  StaticMsg(&'static str),

  #[error("unspecified error: {0}")]
  Msg(String),

  #[error("I/O error: {0}")]
  Io(#[from] IoError),

  #[error("UTF-8 parsing error: {0}")]
  Utf8(#[from] Utf8Error),

  #[error("Formatting error: {0}")]
  Fmt(#[from] FmtError) 
}

impl Into<String> for Error {
  fn into(self) -> String {
    match self {
      Error::Msg(s) => s,
      e => {
        let mut buffer = String::new();
        write!(&mut buffer, "{}", &e).expect("Unable to convert error to string");
        buffer
      }
    }
  }
}
