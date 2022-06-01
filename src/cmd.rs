use crate::{Arg, ArgList, CmdList};

//use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::process::Command;

#[derive(Default, Clone, Debug)]
pub struct Cmd<'a> {
    pub env: Option<HashMap<&'a str, &'a str>>,
    pub name: &'a str,
    pub flags: Option<Vec<char>>,
    pub args: Option<ArgList<'a>>,
    pub cmds: Option<CmdList<'a>>,
}

impl<'a> fmt::Display for Cmd<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = self.to_vec();
        let s = v.join(" ");
        write!(f, "{}", s)
    }
}

impl<'a> Cmd<'a> {
    pub fn new() -> Cmd<'a> {
        Cmd::default()
    }

    pub fn with_name(name: &'a str) -> Self {
        Cmd {
            name: name,
            ..Default::default()
        }
    }

    pub fn cmd(&mut self, cmd: Cmd<'a>) -> &mut Self {
        self.cmds.get_or_insert(CmdList::new()).push(cmd);
        self
    }

    pub fn flag1(&mut self, c: char) -> &mut Self {
        self.flags.get_or_insert(Vec::new()).push(c);
        self
    }

    pub fn flag(&mut self, short: &'a str) -> &mut Self {
        self.arg(Arg::flag(short));
        self
    }

    // XXX: -> &mut Self, or Self
    pub fn opt(&mut self, short: &'a str, opt: &'a str) -> &mut Self {
        self.arg(Arg::opt(short, opt));
        self
    }

    pub fn param(&mut self, param: &'a str) -> &mut Self {
        self.arg(Arg::param(param));
        self
    }

    pub fn arg(&mut self, arg: Arg<'a>) -> &mut Self {
        self.args.get_or_insert(ArgList::new()).push(arg);
        self
    }

    pub fn to_command(&self) -> Command {
        let mut command = Command::new(self.name);

        //command.envs
        //if let Some(env) = &self.env {
        //command.envs();
        //}

        if let Some(args) = &self.args {
            command.args(args.to_vec());
        }

        if let Some(cmds) = &self.cmds {
            command.args(cmds.to_vec());
        }

        command
    }

    pub fn to_vec(&self) -> Vec<&'a str> {
        let mut v = Vec::new();

        v.push(self.name);

        //if let Some(flags) = &self.flags {
        //v.extend(flags);
        //}

        if let Some(cmds) = &self.cmds {
            v.extend(&cmds.to_vec());
        }

        //if let Some(args) = &self.args {
        //v.extend(&args.to_vec());
        //}

        v
    }

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
//#[test]
//fn git_test() {
//use crate::cmd::Cmd;

//let mut git = Cmd::with_name("git");
//git.cmd(Cmd::with_name("status"));
//git.flag("a");
//git.flag("b");
//let v = git.to_vec();

//dbg!(v);
//}

#[test]
fn tmux_test() {
    use crate::cmd::Cmd;

    let mut tmux = Cmd::with_name("tmux");
    tmux.cmd(Cmd::with_name("list-commands"));
    tmux.cmd(Cmd::with_name("list-commands"));
    tmux.cmd(Cmd::with_name("list-commands"));

    //tmux.cmd(Cmd::with_name("list-commands"));
    //tmux.cmd(Cmd::with_name("list-commands"));
    //tmux.cmd(Cmd::with_name("list-commands"));
    let v = tmux.to_vec();

    dbg!(&v);

    dbg!(&tmux);
    let s = tmux.to_string();
    dbg!(&s);
}

#[test]
fn cow_test<'a>() {
    use std::borrow::Cow;

    let mut v: Vec<Cow<'a, str>> = Vec::new();

    let mut a: Vec<Cow<'a, str>> = Vec::new();
    a.push(Cow::Borrowed("asdf"));

    let mut b: Vec<Cow<'a, str>> = Vec::new();
    b.push(Cow::Owned("asdf".to_string()));

    v.extend(a);
    v.extend(b);

    for a in v {
        println!("{}", a);
    }
}
