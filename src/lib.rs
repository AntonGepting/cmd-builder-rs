pub mod arg;
pub mod arg_list;
pub mod cmd;
pub mod cmd_list;
pub mod flag;
pub mod opt;
pub mod param;

pub use crate::arg::Arg;
pub use crate::arg_list::ArgList;
pub use crate::cmd::Cmd;
pub use crate::cmd_list::CmdList;
pub use crate::flag::Flag;
pub use crate::opt::Opt;
pub use crate::param::Param;

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
