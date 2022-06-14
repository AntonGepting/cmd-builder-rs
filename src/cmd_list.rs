use super::Cmd;
use std::borrow::Cow;
use std::fmt;
use std::io::Error;
use std::process::{Command, Output};

/// command separator [^f1]
///
/// [^f1] "...Each command is terminated by a newline or a semicolon (;) Commands separated by
/// semicolons together form a ‘command sequence’ - if a command in the sequence encounters an
/// error, no subsequent commands are executed..."
/// [[tmux manual](https://man7.org/linux/man-pages/man1/tmux.1.html#COMMAND_PARSING_AND_EXECUTION)]

// TODO: rename TERMINATOR
//const CMDS_SEPARATOR: &str = "\\;";
//const CMDS_SEPARATOR: &str = "\n";
const CMDS_SEPARATOR: &str = ";";
const CMD_SEPARATOR: &str = " ";

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct CmdList<'a> {
    pub cmds: Vec<Cmd<'a>>,

    // XXX: Cow<'a, str> or &'a str?
    pub separator: Option<Cow<'a, str>>,
}

impl<'a> fmt::Display for CmdList<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //let separator = self.separator.as_ref().unwrap_or(&Cow::Borrowed(" "));
        //let output = self.to_vec().join(separator.as_ref());
        let output = self.to_vec().join(CMD_SEPARATOR);
        write!(f, "{}", output)
    }
}

impl<'a> Default for CmdList<'a> {
    fn default() -> Self {
        Self {
            cmds: Vec::new(),
            separator: Some(Cow::Borrowed(CMDS_SEPARATOR)),
        }
    }
}

impl<'a> CmdList<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    // XXX: -> Self?
    pub fn push(&mut self, cmd: Cmd<'a>) {
        self.cmds.push(cmd);
    }

    pub fn cmd(mut self, cmd: Cmd<'a>) -> Self {
        self.cmds.push(cmd);
        self
    }

    // XXX: mb use in display trait?
    pub fn to_vec(&self) -> Vec<Cow<'a, str>> {
        let mut v = Vec::new();

        let len = self.cmds.len();
        for (i, cmd) in self.cmds.iter().enumerate() {
            v.extend(cmd.to_vec());

            if let Some(separator) = &self.separator {
                if i < len - 1 {
                    v.push(separator.clone());
                    //v.push(Cow::Borrowed(separator));
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

    pub fn output(self) -> Vec<Result<Output, Error>> {
        let mut v = Vec::new();

        for cmd in self.cmds {
            v.push(cmd.output())
        }

        v
    }

    // NOTE: from bin
    // XXX: error out
    //pub fn output(&self) -> Result<TmuxOutput, Error> {

    //let mut command = Command::new(&self.tmux.bin.as_ref());

    //for tmux_command in &self.cmds.0 {
    //if let Some(cmd) = &tmux_command.cmd {
    //command.arg(cmd.as_ref());
    //}

    //if let Some(args) = &tmux_command.args {
    //for arg in args {
    //command.arg(arg.as_ref());
    //}
    //}

    //command.arg(";");
    //}

    // NOTE: inherit stdin to prevent tmux fail with error `terminal failed: not a terminal`
    //command.stdin(Stdio::inherit());

    //let output = command.output()?;

    //Ok(TmuxOutput(output))
    //}
    pub fn separator<S: Into<Cow<'a, str>>>(&mut self, separator: S) -> &mut Self {
        self.separator = Some(separator.into());
        self
    }

    pub fn get_separator(&self) -> Option<&Cow<'a, str>> {
        self.separator.as_ref()
    }

    pub fn into_cmds(self) -> Vec<Cmd<'a>> {
        self.cmds
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
