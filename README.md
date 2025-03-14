# arch-pkg-text

Pure Rust library to parse Arch Linux packages' structured text formats. "Pure Rust" means not needing `libalpm`.

## Why?

Relying on `libalpm` has 2 limitations:
* The program would only work on Arch Linux.
* Every time `libalpm` updates, the program would need to be recompiled. And since Arch Linux is rolling release, `libalpm` would update frequently, forcing the program to recompile frequently.

This library aims to provide parsers for packaging related structured text formats without the above limitations.

## Usage

Read the [documentation](https://docs.rs/arch-pkg-text).

## License

[MIT](https://github.com/pacman-repo-builder/arch-pkg-text/blob/master/LICENSE.md) © [Hoàng Văn Khải](https://github.com/KSXGitHub).
