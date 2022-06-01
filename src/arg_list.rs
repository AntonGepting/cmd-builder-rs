use crate::Arg;
use std::fmt;

#[derive(Default, Clone, Debug)]
pub struct ArgList<'a> {
    pub args: Vec<Arg<'a>>,
    pub separator: Option<&'a str>,
}

impl<'a> ArgList<'a> {
    pub fn new() -> Self {
        Self {
            args: Vec::new(),
            ..Default::default()
        }
    }

    pub fn push(&mut self, arg: Arg<'a>) {
        self.args.push(arg);
    }

    pub fn push_flag(&mut self, flag: &'a str) {
        self.args.push(Arg::flag(flag));
    }

    pub fn to_vec(&self) -> Vec<String> {
        let mut v = Vec::new();

        let len = self.args.len();
        for (i, arg) in self.args.iter().enumerate() {
            v.push(arg.to_string());

            if let Some(separator) = self.separator {
                if i < len - 1 {
                    v.push(separator.to_string());
                }
            }
        }

        v
    }
}

impl<'a> fmt::Display for ArgList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();

        for arg in &self.args {
            s.push_str(&arg.to_string());
        }

        write!(f, "{}", s)
    }
}

impl<'a> From<Arg<'a>> for ArgList<'a> {
    fn from(item: Arg<'a>) -> Self {
        ArgList {
            args: vec![item],
            ..Default::default()
        }
    }
}
