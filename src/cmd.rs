use super::CmdList;
use std::borrow::Cow;
use std::fmt;
use std::process::Command;

const EMPTY_CMD: &str = "";
const CMD_ARG_SEPARATOR: &str = " ";

// mod
//
// input:
//  command and it's arguments, as strings (&str, String)
//
// output:
// .to_vec() -> vec![command, arg1, arg2, ... arg3, subcommand1, ..., subcommand2 ...]
// .to_string() -> "command arg1 arg ... arg3 subcommand1 ... ; subcommand2 ...";

/// Standard command line arguments syntax:
///
/// ```text
/// # name   short flags   long flags        option     parameter  subcommand
/// command [-abcdefghij] [--longflag] [--] [-o value] [param]    [subcommand [...]]
/// ```
#[derive(Debug, Default)]
pub struct Cmd<'a> {
    /// environment variables
    pub envs: Option<Vec<(Cow<'a, str>, Cow<'a, str>)>>,

    /// command name
    pub name: Option<Cow<'a, str>>,

    /// command alias
    pub alias: Option<Cow<'a, str>>,

    // XXX: remove
    /// flags (`[-a] [-b] [-c]`)
    pub flags: Option<Vec<Cow<'a, str>>>,

    /// short flags (`[-a] [-b] [-c]`)
    pub flags_short: Option<String>,

    /// arguments: long flags, options, parameters (`[--longflag] [-o opt] [param]`)
    pub args: Option<Vec<Cow<'a, str>>>,

    /// subcommands list
    pub subcommands: Option<CmdList<'a>>,

    // XXX: Cow<'a, str> or &'a str?
    /// separator between command and it's flags, args, subcommand (" ")
    pub separator: Option<&'a str>,

    // XXX: Cow<'a, str> or &'a str?
    /// flags, args separator (usually double dash `--`)
    pub flags_args_separator: Option<&'a str>,

    /// do not combine multiple single flags into flags line, use them separately (`-f -a` = `-fa`)
    pub not_combine_short_flags: bool,

    /// do not use command alias, use name instead (`new-session` = `new`)
    pub not_use_alias: bool,
    //pub env_remove: Option<Vec<Cow<'a, str>>>,

    //pub env_clear: bool,

    //pub stdin: Option<Stdio>,

    //pub stdout: Option<Stdio>,

    //pub stderr: Option<Stdio>,

    //pub current_dir: Option<Cow<'a, str>>,
}

// XXX: reason?
// s. clap
//macro_rules! tmux_command!("env", "cmd", "-a", "-b", "-arg 0", "param")

impl<'a> fmt::Display for Cmd<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = self
            .to_vec()
            .join(self.separator.unwrap_or(CMD_ARG_SEPARATOR));
        write!(f, "{}", output)
    }
}

impl<'a> Cmd<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_name<S: Into<Cow<'a, str>>>(name: S) -> Self {
        Cmd {
            name: Some(name.into()),
            ..Default::default()
        }
    }

    /// set command name
    pub fn name<S: Into<Cow<'a, str>>>(&mut self, cmd: S) -> &mut Self {
        self.name = Some(cmd.into());
        self
    }

    // run command
    //pub fn output(&self) -> Result<Output, Error> {
    //let mut command = Command::from(self);
    //// NOTE: inherit stdin to prevent tmux fail with error `terminal failed: not a terminal`
    //command.stdin(Stdio::inherit());
    //let output = command.output()?;
    //Ok(output)
    //}

    // XXX: really necessary?
    //pub fn spawn(&self) -> Result<Child, Error> {
    //let mut command = Command::from(self);
    //Ok(command.spawn()?)
    //}

    // XXX: really necessary?
    //pub fn status(&self) -> Result<ExitStatus, Error> {
    //let mut command = Command::from(self);
    //Ok(command.status()?)
    //}

    pub fn env<T, U>(&mut self, key: T, value: U) -> &mut Self
    where
        T: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>,
    {
        self.envs
            .get_or_insert(Vec::new())
            .push((key.into(), value.into()));
        self
    }

    // XXX: really necessary?
    //pub fn stdin<T: Into<Stdio>>(&mut self, stdin: T) -> &mut Self {
    //self.stdin = Some(stdin.into());
    //self
    //}

    //// XXX: really necessary?
    //pub fn stdout<T: Into<Stdio>>(&mut self, stdout: T) -> &mut Self {
    //self.stdout = Some(stdout.into());
    //self
    //}

    //// XXX: really necessary?
    //pub fn stderr<T: Into<Stdio>>(&mut self, stderr: T) -> &mut Self {
    //self.stderr = Some(stderr.into());
    //self
    //}

    //// XXX: really necessary?
    //pub fn env_remove<S: Into<Cow<'a, str>>>(&mut self, key: S) -> &mut Self {
    //self.env_remove.get_or_insert(Vec::new()).push(key.into());
    //self
    //}

    //// XXX: really necessary?
    //pub fn current_dir<S: Into<Cow<'a, str>>>(&mut self, current_dir: S) -> &mut Self {
    //self.current_dir = Some(current_dir.into());
    //self
    //}

    // XXX: hard bound to cmd_args
    // if vec doesn't exist, creates it and appends with given arguments
    /// push a single flag (`-x`)
    pub fn push_flag<S: Into<Cow<'a, str>>>(&mut self, flag: S) -> &mut Self {
        self.args.get_or_insert(Vec::new()).push(flag.into());
        self
    }

    pub fn push_flag_short(&mut self, flag: char) -> &mut Self {
        self.flags_short.get_or_insert(String::new()).push(flag);
        self
    }

    // if vec doesn't exist, creates it and appends with given arguments
    /// push an option, flag and value (`-x  <VALUE>`)
    pub fn push_option<U, V>(&mut self, key: U, option: V) -> &mut Self
    where
        U: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        self.args
            .get_or_insert(Vec::new())
            .extend_from_slice(&[key.into(), option.into()]);
        self
    }

    // if vec doesn't exist, creates it and appends with given arguments
    /// push a single parameter (`<VALUE>`)
    pub fn push_param<S: Into<Cow<'a, str>>>(&mut self, param: S) -> &mut Self {
        self.args.get_or_insert(Vec::new()).push(param.into());
        self
    }

    pub fn push_cmd(&mut self, cmd: Cmd<'a>) -> &mut Self {
        self.subcommands.get_or_insert(CmdList::new()).push(cmd);
        self
    }

    pub fn push_cmds(&mut self, cmdlist: CmdList<'a>) -> &mut Self {
        self.subcommands = Some(cmdlist);
        self
    }

    pub fn arg<T, U>(&mut self, flag: T, opt: U) -> &mut Self
    where
        T: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>,
    {
        let v = self.args.get_or_insert(Vec::new());
        v.push(flag.into());
        v.push(opt.into());
        self
    }

    // XXX: -> &mut Self, or Self
    pub fn opt<T, U>(&mut self, short: T, opt: U) -> &mut Self
    where
        T: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>,
    {
        let v = self.args.get_or_insert(Vec::new());
        v.push(short.into());
        v.push(opt.into());
        self
    }

    pub fn param<T: Into<Cow<'a, str>>>(&mut self, param: T) -> &mut Self {
        self.args.get_or_insert(Vec::new()).push(param.into());
        self
    }

    //pub fn arg(&mut self, arg: &'a str) -> &mut Self {
    //self.args.get_or_insert(Vec::new()).push(arg);
    //self
    //}

    // TODO: custom command
    //pub fn custom<S: Into<Cow<'a, str>>>(&self, ) -> &mut Self {
    //}

    //// create `std::process::Command` from `Self` (consuming `Self`)
    //pub fn to_command(&self) -> Command {
    //Command::from(self)
    //}

    pub fn to_vec(&self) -> Vec<Cow<'a, str>> {
        let mut v: Vec<Cow<'a, str>> = Vec::new();

        if let Some(envs) = &self.envs {
            for (key, value) in envs {
                v.push(Cow::Owned(format!("{}={}", key, value)));
            }
        }

        if let Some(cmd) = &self.name {
            v.push(cmd.to_owned());
        }

        if let Some(flags_short) = &self.flags_short {
            if self.not_combine_short_flags {
                for c in flags_short.chars() {
                    v.push(Cow::Owned(format!("-{}", c)));
                }
            } else {
                v.push(Cow::Owned(format!("-{}", flags_short)));
            }
        }

        if let Some(args) = &self.args {
            v.extend(args.to_vec());
        }

        if let Some(cmds) = &self.subcommands {
            v.extend(cmds.to_vec());
        }

        v
    }

    pub fn to_command(self) -> Command {
        let name = self.name.as_ref().unwrap_or(&Cow::Borrowed(""));
        let mut command = Command::new(name.as_ref());

        if let Some(envs) = self.envs {
            command.envs(
                envs.iter()
                    .map(|(key, value)| (key.as_ref(), value.as_ref())),
            );
        }

        if let Some(args) = self.args {
            command.args(args.iter().map(|arg| arg.as_ref()));
        }

        // additional subcommands
        if let Some(cmds) = self.subcommands {
            command.args(cmds.to_vec().iter().map(|arg| arg.as_ref()));
        }

        //if let Some(stdin) = self.stdin {
        //command.stdin(stdin);
        //}

        //if let Some(stdout) = self.stdout {
        //command.stdout(stdout);
        //}

        //if let Some(stderr) = self.stderr {
        //command.stderr(stderr);
        //}

        command
    }

    //pub fn into_tmux_command(self) -> TmuxCommand<'a> {
    //TmuxCommand::default()
    //}

    //pub fn into_tmux_bin_command_ext(self, tmux: TmuxBin<'a>) -> TmuxBinCommand<'a> {
    //TmuxBinCommand {
    //tmux: tmux,
    //command: self,
    //}
    //}

    //pub fn append_to(self, cmds: &mut TmuxCommands<'a>) {
    //cmds.push(self);
    //}

    //pub fn writeln(self, stdin: &mut ChildStdin) -> Result<(), std::io::Error> {
    //writeln!(stdin, "{}", self.to_string())
    //}
    //pub fn into_vec(self) -> Vec<&'a str> {
    //let mut v = Vec::new();

    //v.push(self.name);

    //for cmd in self.cmds.cmds {
    //v.append(&mut cmd.into_vec());
    //}

    //for arg in self.args.args {
    ////v.append(&mut args.into_vec());
    //match arg {
    //Arg::Flag(flag) => {
    //if let Some(short) = flag.get_short() {
    //v.push(short);
    //}
    //}
    //Arg::Opt(opt) => {}
    //Arg::Param(param) => {}
    //_ => {}
    //};
    //}

    //v
    //}
}

// create ready to exec [`std::process::Command`]
// * create [`std::process::Command`]
// * push environment variables
// * push binary arguments
// * push subcommand
impl<'a> From<&Cmd<'a>> for Command {
    fn from(cmd: &Cmd) -> Self {
        // user given command or blank command
        let name = cmd.name.as_ref().unwrap_or(&Cow::Borrowed(EMPTY_CMD));
        let mut command = Command::new(name.as_ref());

        // environment variables
        if let Some(envs) = &cmd.envs {
            command.envs(
                envs.iter()
                    .map(|(key, value)| (key.as_ref(), value.as_ref())),
            );
        }

        // arguments
        if let Some(args) = &cmd.args {
            command.args(args.iter().map(|arg| arg.as_ref()));
        }

        // subcommands
        if let Some(cmds) = &cmd.subcommands {
            command.args(cmds.to_vec().iter().map(|arg| arg.as_ref()));
        }

        command
    }
}

// create ready to exec [`std::process::Command`]
// * create [`std::process::Command`]
// * push environment variables
// * push binary arguments
// * push subcommand
impl<'a> From<Cmd<'a>> for Command {
    fn from(cmd: Cmd) -> Self {
        Command::from(&cmd)
    }
}

/* NOTE: from bin
    /// Returns mutable reference to tmux executable name `Cow<'a, str>`
    ///
    /// # Examples
    /// ```
    /// use std::borrow::Cow;
    /// use tmux_interface::commands::tmux_bin::TmuxBin;
    ///
    /// let mut tmux = TmuxBin::default();
    /// let bin = tmux.bin();
    /// assert_eq!(bin, &Cow::Borrowed("tmux"));
    /// ```
    pub fn bin(&self) -> &Cow<'a, str> {
        &self.bin
    }

    /// Returns mutable reference to tmux executable name `Cow<'a, str>`
    ///
    /// # Examples
    /// ```
    /// use std::borrow::Cow;
    /// use tmux_interface::commands::tmux_bin::TmuxBin;
    ///
    /// let mut tmux = TmuxBin::default();
    /// *tmux.bin_mut() = Cow::Borrowed("/usr/bin/tmux");
    /// assert_eq!(tmux.bin, Cow::Borrowed("/usr/bin/tmux"));
    /// ```
    /// or
    /// ```
    /// use std::borrow::Cow;
    /// use tmux_interface::commands::tmux_bin::TmuxBin;
    ///
    /// let mut tmux = TmuxBin::default();
    /// *tmux.bin_mut() = "/usr/bin/tmux".into();
    /// assert_eq!(tmux.bin, Cow::Borrowed("/usr/bin/tmux"));
    /// ```
    pub fn bin_mut(&mut self) -> &mut Cow<'a, str> {
        &mut self.bin
    }

*/
