use std::fmt;
use std::borrow::Cow;

use nom::IResult;
use nom::Parser;
use nom::bytes::complete::{tag, take_till1, take_until};
use nom::character::complete::{char, one_of, none_of};
use nom::branch::alt;
use nom::sequence::{preceded, delimited};
use nom::multi::{many0, fold_many0};
use nom::combinator::opt;

mod error;
#[cfg(test)]
mod test;

type KV = (String, String);

#[derive(Debug, Default)]
pub struct FFMetadata {
  pub global: Vec<KV>,
  pub sections: Vec<(String, Vec<KV>)>,
}

impl FFMetadata {
  pub fn parse(s: &str) -> Result<Self, error::ParseError<'_>> {
    let (remaining, r) = ffmetadata(s)?;
    if !remaining.is_empty() {
      Err(error::ParseError::Remaining(remaining))
    } else {
      Ok(r)
    }
  }
}

impl fmt::Display for FFMetadata {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    writeln!(f, ";FFMETADATA1")?;

    for (k, v) in &self.global {
      writeln!(f, "{}={}", escape(k), escape(v))?;
    }
    writeln!(f)?;

    for (header, section) in &self.sections {
      writeln!(f, "[{header}]")?;
      for (k, v) in section {
        writeln!(f, "{}={}", escape(k), escape(v))?;
      }
      writeln!(f)?;
    }

    Ok(())
  }
}

fn header(input: &str) -> IResult<&str, ()> {
  tag(";FFMETADATA1\n").map(|_| ()).parse(input)
}

fn string(input: &str) -> IResult<&str, String> {
  many0(
    alt((
      preceded(char('\\'), one_of("=;#\\\n")),
      none_of("=;#\\\n"),
    ))
  ).map(|v| v.into_iter().collect()).parse(input)
}

fn kv(input: &str) -> IResult<&str, KV> {
  let (input, key) = string(input)?;
  let (input, _) = char('=')(input)?;
  let (input, value) = string(input)?;
  let (input, _) = char('\n')(input)?;
  Ok((input, (key, value)))
}

fn section_header(input: &str) -> IResult<&str, String> {
  delimited(char('['), take_till1(|c| c == ']'), tag("]\n"))
    .map(String::from).parse(input)
}

fn comment(input: &str) -> IResult<&str, ()> {
  let (input, _) = opt(preceded(
    one_of(";#"),
    take_until("\n")
  )).parse(input)?;
  let (input, _) = char('\n')(input)?;
  Ok((input, ()))
}

fn comment_or_kv(input: &str) -> IResult<&str, Option<KV>> {
  alt((
    comment.map(|_| None),
    kv.map(Some),
  ))(input)
}

fn kvs(input: &str) -> IResult<&str, Vec<KV>> {
  fold_many0(comment_or_kv, Vec::new, |mut acc: Vec<_>, item| {
    if let Some(kv) = item {
      acc.push(kv);
    }
    acc
  })(input)
}

fn section(input: &str) -> IResult<&str, (String, Vec<KV>)> {
  let (input, header) = section_header(input)?;
  let (input, kvs) = kvs(input)?;
  Ok((input, (header, kvs)))
}

fn ffmetadata(input: &str) -> IResult<&str, FFMetadata> {
  let (input, _) = header(input)?;
  let (input, global) = kvs(input)?;
  let (input, sections) = many0(section)(input)?;
  Ok((input, FFMetadata {
    global, sections,
  }))
}

const ESCAPING_CHARS: &[char] = &['=', ';', '#', '\\', '\n'];

fn escape(s: &str) -> Cow<str> {
  if s.contains(ESCAPING_CHARS) {
    let escaped = s.chars()
      .fold(String::new(), |mut s, ch| {
        if ESCAPING_CHARS.contains(&ch) {
          s.push('\\');
        }
        s.push(ch);
        s
      });
    Cow::Owned(escaped)
  } else {
    Cow::Borrowed(s)
  }
}
