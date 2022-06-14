// XXX: name command-string-builder?
// XXX: name command-line-builder?
//
/// Wrapper for preparing data for [`std::process::Command`]
///
///
/// [`Cmd`]
/// [`CmdList`] - Wrapper around [`Vec<Cmd>`]
pub mod cmd;
pub mod cmd_list;

pub mod cmd_list_tests;
pub mod cmd_tests;

pub use crate::cmd::Cmd;
pub use crate::cmd_list::CmdList;
