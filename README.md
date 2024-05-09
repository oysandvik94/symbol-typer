# Symbol Typer

Symbol typer is a typing trainer in the CLI for symbols specifically, allthough
it could be extended for letters.

The motivation for the project is to practice muscle memory for symbol layers
on ergonomic keyboards.

It features a steak system that is persisted on disk, as well as a configurable
timeout for correct answers, meaning if you're too slow your streak is reset.

The project can be built with:

```bash
cargo build
```

And ran with:

```bash
cargo run
```

## Flags

There are two CLI flags, `--clear-highscore` and `--time`. The former will
clear the streak persisted on disk, and the second specified how long you have
to provide a correct symbol, given in milliseconds. To run these with `cargo
run`, delimit them with `--` like this:

```bash
cargo run -- --clear-highscre --time 1500
```

**Disclaimer:** This is a tool created for myself to practice my keyboard, as
well as practice writing rust. I can't guarantee that it's not a mess.
