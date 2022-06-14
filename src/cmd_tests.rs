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

    let mut cmd = Cmd::with_name("tmux");
    cmd.push_cmd(Cmd::with_name("list-commands"));
    cmd.push_cmd(Cmd::with_name("list-commands"));
    cmd.push_cmd(Cmd::with_name("list-commands"));

    cmd.env("ENVVAR", "EN");
    //cmd.cmd(Cmd::with_name("list-commands"));
    //cmd.cmd(Cmd::with_name("list-commands"));
    //cmd.cmd(Cmd::with_name("list-commands"));
    let v = cmd.to_vec();

    dbg!(&v);

    dbg!(&cmd);
    let s = cmd.to_string();
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
