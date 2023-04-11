# everything.rs

You may have noticed a bunch of Rust ports of Unix tools popping up everywhere.
Porting individual tools to Rust is slow, difficult work, however. This repository aims
to fix that. Behold, a port of **everything** to Rust.

Want a drop-in Rust-powered replacement for `ls` or `echo`? `everything.rs` has got you
covered. Now you can list directories and print strings blazing fast 🚀, without the guilt of using a
non-Rust tool.

Want a Rust-powered replacement for that sluggish Python script you made? No problem, `everything.rs`
has got you covered there, too.

## Installation

Download the artifact from the latest green run [here](https://github.com/ysthakur/everything.rs/actions/workflows/build.yml).
Put it in a folder that's in your `PATH` and rename it to `everything` or `e` or something.

## Usage

To use a Rusty version of `ls`, use
```bash
everything ls
everything ls -al # You can pass arguments too
```

You can use aliases to make things more convenient for you (e.g. `alias ls=everything ls`).

## FAQ

### How does it work?

`everything.rs` applies the cutting-edge technology pioneered by [`exec-rs`](https://github.com/faradayio/exec-rs)
to perfectly emulate any program. `everything.rs` does require you to have the program installed on your
machine so it can better emulate it.

### There's no way you've ported *everything*. Does it actually work?

You better believe it.

### I love it! When will it have Windows support?

Working on it.

### How can I support this project?

Despite how valuable this software is to all of humanity, I do not seek any compensation for my work.
Knowing that I have benefitted the world is enough payment.
