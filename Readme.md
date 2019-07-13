# dots-cli

The `dots-cli` is a tool for quickly installing and linking groups of dotfiles across computers.

- Quickly installs a "dot" (a collection of dotfiles) via git to a `~/.dots` directory.
- Use a dot `Dot.toml` to specify where you want your dotfiles to be linked.
- Links multiple "dots" in one go and reports any conflicts between each dot or between a dot and existing files on the system.


## Getting Started

First, you'll need to make sure you have `cargo` installed. Visit their [Installation Guide][1] and follow their instructions. Once you have `cargo` installed, run the following to install the `dots-cli`:

```bash
cargo install dots
```

Then, create a git repo with a `Dot.toml` at it's root that describes where files in that repo should be linked. A simple `Dot.toml` might looks something like this:

```toml
[package]
name = "webdesserts"
authors = [ "Michael Mullins" ]

[link]
bashrc = "~/.bashrc"
bash_profile = "~/.bash_profile"
```

You can then `add` your dotfiles from a repo like so...

```bash
dots add git@github.com:webdesserts/dot.git
```

...and then use the `install` command to link everything to their desired location.

```
dots install
```

You can run `dots help` to see more commands. Feel free to check out [my own dot][2] for a better idea of what you can do with one.

## Roadmap

There are two main problems with this repo as it is:

1. Because it does not track what it has previously installed, it has no way to clean up old links
2. If an install fails half-way, it does not safely recover your previous configuration.

I'm currently working on both of these issues in the [v1.x][3] branch. If you have any other suggestions feel free to file an issue.

[1]:[https://doc.rust-lang.org/cargo/getting-started/installation.html]
[2]:[https://github.com/webdesserts/dot]
[3]:[https://github.com/webdesserts/dots-cli/tree/v1.x]