#[test]
fn cmds_list() {
    use crate::{Cmd, CmdList};

    let mut cmds = CmdList::new();
    cmds.push(
        Cmd::with_name("new-session")
            .opt("-n", "session_name")
            .env("ENVVAR", "EN")
            .to_owned(),
    );
    cmds.push(
        Cmd::with_name("has-session")
            .opt("-t", "session_name")
            .to_owned(),
    );
    cmds.push(
        Cmd::with_name("kill-session")
            .opt("-t", "session_name")
            .to_owned(),
    );

    dbg!(cmds.to_vec());
    dbg!(cmds.to_string());
}
