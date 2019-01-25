# butlerd-rs [![](https://img.shields.io/crates/v/butlerd.svg?style=flat-square)](https://crates.io/crates/butlerd)
butlerd-rs is a rust interface for itch.io's [butlerd](https://github.com/itchio/butler). Currently in development. No responses to SSE's are implemented yet, though almost everything else from the [API](http://docs.itch.ovh/butlerd/master/#/) is. Very open to feedback and bug reports. Available on [crates.io](https://crates.io/crates/butlerd). This repository is a mirror for [the main sr.ht repo](https://git.sr.ht/~nicohman/butlerd-rs). This was originally made to support [eidolon](https://git.sr.ht/~nicohman/eidolon), though it now covers almost all of the API.

## Getting Started

### Documentation & Dependencies

You can find up-to-date documentation at [docs.rs](https://docs.rs/butlerd). The only dependency is

- [butler](https://github.com/itchio/butler)

### Examples

- The [tests](https://git.sr.ht/%7Enicohman/butlerd-rs/tree/master/tests/lib.rs) are a good example of basic usage of the API

## Bug reports/Contributing

Please submit bug reports to [the sr.ht issue tracker](https://todo.sr.ht/~nicohman/butlerd-rs) and patches to [the sr.ht mailing list](https://lists.sr.ht/~nicohman/butlerd-rs). However, I still will accept issues and pull requests from github. Note: I have not tested this on windows or mac due to lack of testing machines. If you use this library on windows or mac, please let me know if there are bugs.
