use crate::CmdList;

use std::borrow::Cow;
use std::fmt;
use std::process::Command;

#[derive(Default, Clone, Debug)]
pub struct Cmd<'a> {
    pub envs: Option<Vec<Cow<'a, str>>>,
    pub name: Cow<'a, str>,
    //alias
    pub flags_short: Option<String>,
    pub args: Option<Vec<Cow<'a, str>>>,
    pub cmds: Option<CmdList<'a>>,
    pub separator: Option<&'a str>,
}

impl<'a> fmt::Display for Cmd<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = self.to_vec();
        let s = v.join(self.separator.unwrap_or(" "));
        write!(f, "{}", s)
    }
}

impl<'a> Cmd<'a> {
    pub fn new() -> Cmd<'a> {
        Cmd::default()
    }

    // XXX: rename new()
    pub fn with_name<T: Into<Cow<'a, str>>>(name: T) -> Self {
        Cmd {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn cmd(&mut self, cmd: Cmd<'a>) -> &mut Self {
        self.cmds.get_or_insert(CmdList::new()).push(cmd);
        self
    }

    pub fn flag_short(&mut self, c: char) -> &mut Self {
        self.flags_short.get_or_insert(String::new()).push(c);
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

    // env(key, val)
    // envs(iterator)
    pub fn to_command(&self) -> Command {
        let mut command = Command::new(self.name.as_ref());

        //if let Some(envs) = &self.envs {
        //for env in envs {
        //command.env(env.as_ref());
        //}
        //}

        if let Some(args) = &self.args {
            for arg in args {
                command.arg(arg.as_ref());
            }
        }

        if let Some(cmds) = &self.cmds {
            //command.args(cmds) // XXX: iterator?
            for arg in cmds.to_vec() {
                command.arg(arg.as_ref());
            }
        }

        command
    }

    pub fn to_vec(&self) -> Vec<Cow<'a, str>> {
        let mut v: Vec<Cow<'a, str>> = Vec::new();

        v.push(self.name.clone());

        if let Some(flags_short) = &self.flags_short {
            v.push(Cow::Owned(format!("-{}", flags_short)));
        }

        if let Some(args) = &self.args {
            v.extend(args.to_vec());
        }

        if let Some(cmds) = &self.cmds {
            v.extend(cmds.to_vec());
        }

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
