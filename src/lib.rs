// XXX: name command-string-builder?
// XXX: name command-line-builder?

//! # `Cmd`
//!
//! `Cmd` library containin few functions for building command line for later use with
//! [`std::process::Command`]
//!
//! ## Input
//!
//! owned or borrowed strings
//!
//! ## Output
//!
//! * [`Command`][`std::process::Command`] - structure, generated using
//! [`Command::new()`][`std::process::Command::new()`]
//! * [`Vec<Cow<'a, str>>`][`Vec`] - vector containing all commands and parameters with separators as elements
//! * [`String`] - string containing full command line
//!
//! ## Example
//!
//! ```
//! use cmd::Cmd;
//!
//! let mut cmd = Cmd::with_name("git");
//! cmd.env("LANGUAGE", "en_US.UTF-8");
//! cmd.push_cmd(Cmd::with_name("status"));
//!
//! let v = cmd.to_vec();
//! assert_eq!(v, vec!["LANGUAGE=en_US.UTF-8", "git", "status"]);
//!
//! let s = cmd.to_string();
//! assert_eq!(s, "LANGUAGE=en_US.UTF-8 git status");
//!
//! let output = cmd.to_command().output();
//! ```
//!
//! # Why
//!
//! * structure fields holding data as [`Cow<'a, str>`][std::borrow::Cow] type. Allowing modifying, or
//! using constants.
//! * ~~macro for generation~~
//! * short or long flags can be used, short flags can be combined together
//! * command name or command alias can be used
//! * sometimes we need a temporary structure for holding command line before it will be executed
//!
//!
//! # Modules
//!
//! * [`Cmd`] - Main struct, holding environment variables, command name, alias, flags, arguments,
//! subcommads, separator etc
//! * [`CmdList`] - Wrapper around [`Vec<Cmd>`] including separator
//!
//! # Details
//!
//! Standard command line arguments syntax:
//! ```text
//! [[ENV_VAR1=value1] ...] command [-abc...] [--longflag ...] [--] [args ...] [subcommand [...]]
//! ```
//!
//! ```text
//! [[ENV_VAR1=value1] ...] - environment variables (separator: space)
//! [command]               - name (command name or alias)
//! [-abc...]               - short flags (combined or not)
//! [--longflag ...]        - long flags
//! [--]                    - arguments from flags separator (double hyphen)
//! [-o value]              - option (flag (short or long) and value (no separator, space, eq))
//! [param]                 - parameter (value)
//! [subcommand [...]]      - subcommand
//! ```
//!
//! # [`Cmd`]
//!
//! New:
//! * [`::new()`][`Cmd::new()`] - Create default (short flags will be combined, command alias instead of
//! * [`::default()`][`Cmd::default()`] - Create default (short flags will be combined, command alias instead of
//! name will be used)
//! * [`::new_full()`][`Cmd::new_full()`] - Create with command name and not combined short flags will be used
//! * [`::with_name()`][`Cmd::with_name()`] - Create with name
//!
//! Setters:
//! * [`.name()`][`Cmd::name()`] - set command name
//! * [`.alias()`][`Cmd::alias()`] - set command alias
//! * [`.env()`][`Cmd::env()`] - add environment variable
//! * [`.push_flag()`][`Cmd::push_flag()`] - add flag
//! * [`.push_flag_short()`][`Cmd::push_flag_short()`] - add short flag
//! * [`.push_option()`][`Cmd::push_option()`] - add option
//! * [`.push_param()`][`Cmd::push_param()`] - add param
//! * [`.push_cmd()`][`Cmd::push_cmd()`] - add subcommand
//! * [`.push_cmds()`][`Cmd::push_cmds()`] - add subcommands
//! * [`.arg()`][`Cmd::arg()`] - add arg
//! * [`.opt()`][`Cmd::opt()`] - add opt
//! * [`.param()`][`Cmd::param()`] - add param
//!
//! Output:
//! * [`.to_vec()`][`Cmd::to_vec()`] - [`Cmd`] to [`Vec<Cow<'a, str>>`][`Vec`]
//! * [`.to_command()`][`Cmd::to_command()`] - [`Cmd`] struct to [`Command`][`std::process::Command`]
//!
//! # [`CmdList`]
//!
//! New:
//! * [`::new()`][`CmdList::new()`] -
//! * [`::default()`][`CmdList::default()`] -
//!
//! Setters:
//! * [`.push()`][`CmdList::push()`] -
//! * [`.cmd()`][`CmdList::cmd()`] -
//! * [`.separator()`][`CmdList::separator()`] -
//!
//! Output:
//! * [`.to_vec()`][`CmdList::to_vec()`] -
//! * [`.to_command_vec()`][`CmdList::to_command_vec()`] -
//!
//!
//!
pub mod cmd;
pub mod cmd_list;

pub mod cmd_list_tests;
pub mod cmd_tests;

pub use crate::cmd::Cmd;
pub use crate::cmd_list::CmdList;
