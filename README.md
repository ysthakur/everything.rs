# everything.rs

![build](https://github.com/ysthakur/everything.rs/actions/workflows/build.yml/badge.svg)

![image](https://user-images.githubusercontent.com/45539777/231678003-ae1b6183-2017-4cf6-a08b-56710710fb50.png)

Do you ever feel like washing your hands after a coding session, disgusted at having to use essential
Unix utilities with not the tiniest bit of Rust in them? If so, this project is for you! If not, this project is
still for you, you just haven't seen the light yet. Head on over to [rust-lang.org](https://www.rust-lang.org/)
to meet your salvation.

Perhaps you've been using one of those "X Unix tool, but in Rust" things popping up everywhere. But that's
not sustainable or convenient, because porting individual tools to Rust is slow, difficult work, and these individual
ports often differ in behavior from the tool they were meant to replace. Most Unix tools don't have Rust ports,
and many of these ports require maintenance. This repository aims to fix that.

Behold, a port of **everything** to Rust.

Want a drop-in Rust-powered replacement for `ls` or `echo`? `everything.rs` has got you
covered. Now you can list directories and print strings blazing fast 🚀, without the guilt of using a
non-Rust tool! Rest assured, `everything.rs` will have the *exact* same behavior as `ls` or `echo`, but with Rust 🦀.

## Installation

Download an executable from [the latest release](https://github.com/ysthakur/everything.rs/releases/latest), or,
if you want hemorrhaging edge releases, download the artifact from the latest green run
[here](https://github.com/ysthakur/everything.rs/actions/workflows/build.yml).

Put it in a folder that's in your `PATH` and rename it to `everything` and you're good to go!

## Usage

To use a Rusty version of `ls`, use
```bash
everything ls
everything ls -al # You can pass arguments too
```

You can use aliases to make it more convenient (e.g. `alias ls=everything ls`).

## FAQ

### How does it work?

`everything.rs` applies the cutting-edge technology pioneered by [`exec-rs`](https://github.com/faradayio/exec-rs)
to perfectly emulate any program. `everything.rs` does require you to have the program installed on your
machine so it can better emulate it.

### Does it work for more than Unix tools?

Of course! Want a Rust-powered replacement for that sluggish bash script you made? No problem,
just stick an `everything` in front of it and use it the same way you would normally (`everything <script name> <args...>`).

### There's no way you've ported *everything*. Does it actually work?

You better believe it.

### What platforms does `everything.rs` support?

Every platform that Rust supports. Any other platform is not worth supporting.

### I love it! How can I support this project?

Despite how useful this software is, I don't want any compensation for it - working on this was fun enough!
