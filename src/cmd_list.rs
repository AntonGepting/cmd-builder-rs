use super::Cmd;
use std::borrow::Cow;
use std::fmt;
use std::process::Command;

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

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CmdList<'a> {
    pub commands: Vec<Cmd<'a>>,

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

// None = "", Some = ";", Some = "\n"
impl<'a> Default for CmdList<'a> {
    fn default() -> Self {
        Self {
            commands: Vec::new(),
            separator: Some(Cow::Borrowed(CMDS_SEPARATOR)),
        }
    }
}

impl<'a> CmdList<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    // XXX: -> Self?
    pub fn push(&mut self, command: Cmd<'a>) {
        self.commands.push(command);
    }

    // XXX: same fn push?
    pub fn cmd(mut self, command: Cmd<'a>) -> Self {
        self.commands.push(command);
        self
    }

    // XXX: mb use in display trait?
    pub fn to_vec(&self) -> Vec<Cow<'a, str>> {
        let mut v = Vec::new();

        let len = self.commands.len();
        for (i, command) in self.commands.iter().enumerate() {
            v.extend(command.to_vec());

            if let Some(separator) = &self.separator {
                if i < len - 1 {
                    v.push(separator.clone());
                    //v.push(Cow::Borrowed(separator));
                }
            }
        }

        v
    }

    pub fn to_command_vec(self) -> Vec<Command> {
        let mut v = Vec::new();
        for cmd in self.commands {
            v.push(cmd.to_command());
        }
        v
    }

    pub fn separator<S: Into<Cow<'a, str>>>(&mut self, separator: S) -> &mut Self {
        self.separator = Some(separator.into());
        self
    }

    pub fn get_separator(&self) -> Option<&Cow<'a, str>> {
        self.separator.as_ref()
    }

    pub fn into_cmds(self) -> Vec<Cmd<'a>> {
        self.commands
    }
}
