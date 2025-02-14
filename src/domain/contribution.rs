use anyhow::anyhow;
use std::{fmt, str::FromStr};

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    None = 0,
    Open = 1,
    Review = 2,
    Merged = 3,
}

pub type Id = String;

#[derive(Debug, PartialEq, Clone)]
pub struct Contribution {
    pub id: Id,
    pub author: String,
    pub status: Status,
    pub project_id: ProjectId,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Filter {
    pub author: Option<String>,
    pub project: Option<Project>,
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            author: None,
            project: None,
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::None => write!(f, "0"),
            Status::Open => write!(f, "1"),
            Status::Review => write!(f, "2"),
            Status::Merged => write!(f, "3"),
        }
    }
}

impl FromStr for Status {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Status::None),
            "1" => Ok(Status::Open),
            "2" => Ok(Status::Review),
            "3" => Ok(Status::Merged),
            _ => Err(anyhow!("Unable to parse {} into a PR status", s)),
        }
    }
}
