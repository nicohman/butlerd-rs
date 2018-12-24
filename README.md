# butlerd-rs [![](https://img.shields.io/crates/v/butlerd.svg?style=flat-square)](https://crates.io/crates/butlerd)
butlerd-rs is a rust interface for itch.io's [butlerd](https://github.com/itchio/butler). Currently in development. No responses to SSE's are implemented yet, though almost everything else from the [API](http://docs.itch.ovh/butlerd/master/#/) is. Working on SSE's, but not currently sure as to how they should be implemented. Very open to feedback and bug reports. Available on [crates.io](https://crates.io/crates/butlerd). Note: The tests will almost certainly fail on any system other than mine due to different CaveIDs. Will be fixed. This repository is a mirror for [the main sr.ht repo](https://git.sr.ht/~nicohman/butlerd-rs)

## Documentation & Dependencies

You can find up-to-date documentation at [docs.rs](https://docs.rs/butlerd). The only dependency is
- [butler](https://github.com/itchio/butler)

## Bug reports/Contributing

Please submit bug reports to [the sr.ht issue tracker](https://todo.sr.ht/~nicohman/butlerd-rs) and patches to [the sr.ht mailing list](https://lists.sr.ht/~nicohman/butlerd-rs). However, I still will accept issues and pull requests from github. Note: I have not tested this on windows or mac due to lack of testing machines. If you use this library on windows or mac, please let me know if there are bugs.
