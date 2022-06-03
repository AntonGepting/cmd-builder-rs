use crate::{Flag, Opt, Param};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Arg<'a> {
    Flag(Flag<'a>),
    Opt(Opt<'a>),
    Param(Param<'a>),
}

impl<'a> fmt::Display for Arg<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Flag(flag) => flag.to_string(),
            Self::Opt(opt) => opt.to_string(),
            Self::Param(param) => param.to_string(),
        };
        write!(f, "{}", s)
    }
}

impl<'a> Arg<'a> {
    pub fn flag(flag: &'a str) -> Self {
        Self::Flag(Flag::new_short(flag))
    }

    pub fn opt(flag: &'a str, opt: &'a str) -> Self {
        Self::Opt(Opt::new_short(flag, opt))
    }

    pub fn param(param: &'a str) -> Self {
        Self::Param(Param::new(param))
    }
}

#[test]
fn args_list() {}
