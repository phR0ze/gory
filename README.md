# gory
[![license-badge](https://img.shields.io/crates/l/fungus.svg)](https://opensource.org/licenses/MIT)
[![build](https://github.com/phR0ze/gory/workflows/build/badge.svg?branch=main)](https://github.com/phR0ze/gory/actions)
[![codecov](https://codecov.io/gh/phR0ze/gory/branch/main/graph/badge.svg?token=3KnXWj0qTw)](https://codecov.io/gh/phR0ze/gory)
[![crates.io](https://img.shields.io/crates/v/gory.svg)](https://crates.io/crates/gory)
[![rust-version](https://img.shields.io/badge/rust-latest%20stable-blue.svg)](https://github.com/rust-lang/rust/releases)

Super simple, just `libc` as a dependency; `gory` is intended to just add a little color to your task

**Example:**  
![Display colors](docs/images/colors.png)

### Quick links
* [Contribute](#contribute)
  * [Git-Hook](#git-hook)
* [License](#license)
  * [Contribution](#contribution)
* [Backlog](#backlog)

## Contribute <a name="Contribute"/></a>
Pull requests are always welcome. However understand that they will be evaluated purely on whether
or not the change fits with my goals/ideals for the project.

### Git-Hook <a name="git-hook"/></a>
Enable the git hooks to have automatic version increments
```bash
cd ~/Projects/gory
git config core.hooksPath .githooks
```

## License <a name="license"/></a>
This project is licensed under either of:
 * MIT license [LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT
 * Apache License, Version 2.0 [LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0

### Contribution <a name="contribution"/></a>
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this project by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.

---

## Backlog <a name="backlog"/></a>
* Support more pre-defined colors like `orange`?
* Support styles like `underline`?
