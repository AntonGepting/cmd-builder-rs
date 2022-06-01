


flag: char or str?
* [x] str
* [ ] conversion needed for vec::push() (ownage?)


separate vec for flags or merge all as enum flag, opt, param
* [x] cmd.flags, if merging can be processed separately, -abc
* [ ] cmd.args, temp vec needed for filtering and merging


hold str or generics in structs for cmds args etc
* [x] str
* [ ] generics, match, to string in the end needed before call `Command::new()`
