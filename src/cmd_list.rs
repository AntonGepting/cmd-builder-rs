use crate::cmd::Cmd;
use std::borrow::Cow;
use std::fmt;
use std::process::Command;

#[derive(Default, Clone, Debug)]
pub struct CmdList<'a> {
    pub cmds: Vec<Cmd<'a>>,
    // custom separator
    pub separator: Option<&'a str>,
}

impl<'a> fmt::Display for CmdList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.to_vec().join(" ");
        write!(f, "{}", s)
    }
}

impl<'a> CmdList<'a> {
    pub fn new() -> Self {
        CmdList {
            cmds: Vec::new(),
            separator: Some(";"),
        }
    }

    pub fn separator(&mut self, separator: &'a str) -> &mut Self {
        self.separator = Some(separator);
        self
    }

    pub fn get_separator(&self) -> Option<&'a str> {
        self.separator
    }

    pub fn push(&mut self, cmd: Cmd<'a>) {
        self.cmds.push(cmd);
    }

    pub fn into_cmds(self) -> Vec<Cmd<'a>> {
        self.cmds
    }

    //
    pub fn to_vec(&self) -> Vec<Cow<'a, str>> {
        let mut v = Vec::new();

        let len = self.cmds.len();
        for (i, cmd) in self.cmds.iter().enumerate() {
            v.extend(cmd.to_vec());

            if let Some(separator) = self.separator {
                if i < len - 1 {
                    v.push(Cow::Borrowed(separator));
                }
            }
        }

        v
    }

    pub fn to_command_vec(&self) -> Vec<Command> {
        let mut v = Vec::new();
        for cmd in &self.cmds {
            v.push(cmd.to_command());
        }
        v
    }

    //pub fn into_vec(self) -> Vec<&'a str> {
    //let mut v = Vec::new();

    //let len = self.cmds.len();
    //for (i, cmd) in self.cmds.into_iter().enumerate() {
    //v.append(&mut cmd.into_vec());

    //if let Some(separator) = self.separator {
    //if i < len - 1 {
    //v.push(separator);
    //}
    //}
    //}

    //v
    //}
}

#[test]
fn cmds_list() {
    use crate::{Cmd, CmdList};

    let mut cmds = CmdList::new();
    cmds.push(
        Cmd::with_name("new-session")
            .opt("n", "session_name")
            .to_owned(),
    );
    cmds.push(
        Cmd::with_name("has-session")
            .opt("t", "session_name")
            .to_owned(),
    );
    cmds.push(
        Cmd::with_name("kill-session")
            .opt("t", "session_name")
            .to_owned(),
    );

    dbg!(cmds.to_vec());
    dbg!(cmds.to_string());
}
