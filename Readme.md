# dots-cli

The `dots-cli` is a tool for quickly installing and linking groups of dotfiles across computers. In
short, it allows you to do the following:

- Quickly add remote "dots" (a git repo of dotfiles) to your computer's `~/.dots` directory.
- Use a `Dot.toml` to specify where you want your dotfiles to be linked.
- Link multiple "dots" in one go and get a report of any conflicts that might happen.

## Getting Started

First, you'll need to make sure you have `cargo` installed. Visit their [Installation Guide][1] and
follow their instructions. Once you have `cargo` installed, run the following to install the `dots-cli`:

```bash
cargo install dots
```

Then, create a git repo with a `Dot.toml` at it's root that describes where files in that repo
should be linked. A simple `Dot.toml` might looks something like this:

```toml
[package]
name = "webdesserts"
authors = [ "Michael Mullins" ]

[link]
"~/.bashrc" = "bash/bashrc"
"~/.bash_profile" = "bash/bash_profile"
```

In the above example "bash/bashrc" and "bash/bash_profile" would be files that live in the "bash"
directory of your git repo. You can use the `install` command to download your repo and link
everything to the desired location.

```
dots install git@github.com:webdesserts/dot.git
```

You can run `dots help` to see more commands. Feel free to check out [my own dot][2] for a better
idea of what you can do with one.

## v1.x Roadmap

At this point the cli is getting close to where I want it to be for a v1.x release. Most of what's
left is polish, testing and usability improvements. One issue that's still outstanding is that if
an install fails half-way, it does not safely recover your previous configuration. I have some
ideas on how to solve this, but this will probably take some time to implement. If you find any
other issues or if you have any suggestions, please feel free to file an issue.

[1]: https://doc.rust-lang.org/cargo/getting-started/installation.html
[2]: https://github.com/webdesserts/dot
