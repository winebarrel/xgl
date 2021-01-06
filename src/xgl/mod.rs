#[cfg(test)]
mod tests;

use anyhow::Result;
use once_cell::sync::Lazy;
use regex::Regex;
use std::io;

#[derive(Debug)]
pub struct Header {
  pub time: String,
  pub id: String,
  pub command: String,
}

// https://github.com/mysql/mysql-server/blob/5.6/sql/log.cc#L1897
static RE_MYSQL56: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r"(?s)^(\d{6}\s+\d{1,2}:\d{2}:\d{2}|\t)\s+(\d+)\s+([^\t]+)\t(.*)").unwrap()
});

// https://github.com/mysql/mysql-server/blob/5.7/sql/log.cc#L783
// NOTE: In Aurora MySQL 5.7, there may be no space between "Time" and "ID"
static RE_MYSQL57: Lazy<Regex> =
  Lazy::new(|| Regex::new(r"(?s)^([^\sZ]+Z)\s*(\d+)\s+([^\t]+)\t(.*)").unwrap());

// https://github.com/mysql/mysql-server/blob/5.6/sql/log.cc#L1676
// https://github.com/mysql/mysql-server/blob/5.7/sql/log.cc#L696
static RE_IGNORE: Lazy<Regex> = Lazy::new(|| {
  Regex::new(concat!(
    r"^(?:",
    r"\S+, Version: \S+ (.+). started with:",
    r"|Tcp port: \d+  Unix socket: \S+",
    r"|Time                 Id Command    Argument",
    r")$"
  ))
  .unwrap()
});

pub fn parse<T, F>(reader: T, mut cb: F) -> Result<()>
where
  T: io::prelude::BufRead,
  F: FnMut(&Header, &str),
{
  let mut re: Option<&Lazy<Regex>> = None;
  let mut header: Option<Header> = None;
  let mut args: Vec<String> = vec![];
  let mut copy_tm = false;
  let mut prev_tm = "".to_string();

  read_line(reader, |str| {
    if RE_IGNORE.is_match(&str) {
      return;
    }

    if re.is_none() {
      re = if RE_MYSQL56.is_match(&str) {
        copy_tm = true;
        Some(&RE_MYSQL56)
      } else if RE_MYSQL57.is_match(&str) {
        Some(&RE_MYSQL57)
      } else {
        return;
      };
    }

    if let Some(m) = re.unwrap().captures(&str) {
      let mut tm = m.get(1).unwrap().as_str().to_string();

      if copy_tm {
        if tm == "\t" {
          tm = prev_tm.clone();
        } else {
          prev_tm = tm.clone();
        }
      }

      if let Some(h) = &header {
        cb(&h, args.join("").trim());
      }

      args.clear();

      header = Some(Header {
        time: tm,
        id: m.get(2).unwrap().as_str().to_string(),
        command: m.get(3).unwrap().as_str().to_string(),
      });

      args.push(m.get(4).unwrap().as_str().to_string());
    } else if header.is_some() {
      args.push(str.to_string());
    }
  })?;

  if let Some(h) = &header {
    cb(&h, args.join("").trim_end_matches("\n"));
  }

  Ok(())
}

fn read_line<T, F>(mut reader: T, mut cb: F) -> Result<()>
where
  T: io::prelude::BufRead,
  F: FnMut(&str),
{
  let mut buf = String::new();

  while reader.read_line(&mut buf)? > 0 {
    cb(&buf);
    buf.clear();
  }

  Ok(())
}
