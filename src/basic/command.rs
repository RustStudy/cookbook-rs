use std::process::Command;
use ::regex::Regex;

use std::error::Error;
use std::string::FromUtf8Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum CmdError {
    UtfError(FromUtf8Error),
    IoError(io::Error),
    RegexError(::regex::Error),
}

impl From<FromUtf8Error> for CmdError {
    fn from(err: FromUtf8Error) -> CmdError {
        CmdError::UtfError(err)
    }
}

impl From<io::Error> for CmdError {
    fn from(err: io::Error) -> CmdError {
        CmdError::IoError(err)
    }
}

impl From<::regex::Error> for CmdError {
    fn from(err: ::regex::Error) -> CmdError {
        CmdError::RegexError(err)
    }
}

impl Error for CmdError {
    fn description(&self) -> &str {
        match *self {
            CmdError::UtfError(ref err) => err.description(),
            CmdError::IoError(ref err) => err.description(),
            CmdError::RegexError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        Some(match *self {
            CmdError::UtfError(ref err) => err as &Error,
            CmdError::IoError(ref err) => err as &Error,
            CmdError::RegexError(ref err) => err as &Error,
        })
    }
}

impl fmt::Display for CmdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CmdError::UtfError(ref err) => fmt::Display::fmt(err, f),
            CmdError::IoError(ref err) => fmt::Display::fmt(err, f),
            CmdError::RegexError(ref err) => fmt::Display::fmt(err, f),
        }
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
pub struct Commit {
    hash: String,
    message: String,
}

pub fn run() -> Result<(), CmdError> {
    let output = Command::new("git").arg("log").arg("--oneline").output()?;

    if !output.status.success() {
        panic!("Command executed with failing error code");
    }

    let pattern = Regex::new(r"(?x)
                               ([0-9a-fA-F]+) # commit hash
                               (.*)           # The commit message")?;

    let stdout = String::from_utf8(output.stdout)?;
    let commits = stdout
        .lines()
        .filter_map(|line| pattern.captures(line))
        .map(|cap| {
                 Commit {
                     hash: cap[1].to_string(),
                     message: cap[2].trim().to_string(),
                 }
             })
        .take(5);

    for commit in commits {
        println!("{:?}", commit);
    }

    Ok(())
}
