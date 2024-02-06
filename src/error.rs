use std::fmt;
use std::error::Error;

type NomError<'a> = nom::Err<nom::error::Error<&'a str>>;

#[derive(Debug)]
pub enum ParseError<'a> {
  Malformatted(NomError<'a>),
  Remaining(&'a str),
}

impl<'a> From<NomError<'a>> for ParseError<'a> {
  fn from(err: NomError<'a>) -> Self {
    ParseError::Malformatted(err)
  }
}

impl<'a> Error for ParseError<'a> {}

impl<'a> fmt::Display for ParseError<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      ParseError::Malformatted(e) => write!(f, "malformatted input: {}", e),
      ParseError::Remaining(s) => write!(f, "remaining: {}", s),
    }
  }
}
