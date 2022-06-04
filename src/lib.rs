pub mod cmd;
pub mod cmd_list;

pub use crate::cmd::Cmd;
pub use crate::cmd_list::CmdList;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[test]
fn cmd_test() {
    use crate::cmd::Cmd;

    let cmd = Cmd::new();
    dbg!(&cmd);
}
