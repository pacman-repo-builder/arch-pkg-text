# parse-arch-pkg-desc

Pure Rust library to parse pacman package description text and `.SRCINFO`. "Pure Rust" means not needing `libalpm`.

## Why?

Relying on `libalpm` has 2 limitations:
* The program would only work on Arch Linux.
* Every time `libalpm` updates, the program would need to be recompiled. And since Arch Linux is rolling release, `libalpm` would update frequently, forcing the program to recompile frequently.

This library aims to provide parsers for packaging related structured text formats without the above limitations.

## Usage

Read the [documentation](https://docs.rs/parse-arch-pkg-desc).

## License

[MIT](https://github.com/KSXGitHub/parse-arch-pkg-desc/blob/master/LICENSE.md) © [Hoàng Văn Khải](https://github.com/KSXGitHub).
