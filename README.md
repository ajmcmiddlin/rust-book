# Rust book

Projects written while working through the [Rust book](https://doc.rust-lang.org/book/).

## Dev setup

I use Nix and direnv, so there's a [`shell.nix`](shell.nix) that brings in rust tooling from a
mozilla overlay. Nixpkgs is pinned by default. VS Code's Rust extension works happily with the
toolchain in this nix shell.

### The trap

One trap I fell into working this out was that the guide I followed included `rustup` in the shell.
This meant that all the rust tools in PATH were those in the `rustup` derivation's output. These
versions expect rustup to be in use, which turned out to be something I didn't want or need. After
removing rustup from the shell, all the tooling in PATH came from a rust toolchain produced by the
mozilla overlay that didn't expect `rustup`.

I should maybe write up my misadventures in case anyone else hits this.