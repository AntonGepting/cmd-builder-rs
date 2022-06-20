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

    cmd.env("LANGUAGE", "en_US.UTF-8");
    let v = cmd.to_vec();

    assert_eq!(v, vec!["LANGUAGE=en_US.UTF-8", "tmux", "list-commands"]);

    let s = cmd.to_string();
    assert_eq!(s, "LANGUAGE=en_US.UTF-8 tmux list-commands");
}
